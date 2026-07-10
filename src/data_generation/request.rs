use std::{
    borrow::Cow,
    collections::BTreeSet,
    fmt::Write as _,
    path::{Path, PathBuf},
};

use anyhow::Context;
use sha2::Digest as _;
use trustfall_rustdoc::{LoadingError, VersionedStorage};

use crate::manifest::Manifest;
use crate::util::{atomic_write, slugify};

use super::error::{IntoTerminalResult, TerminalError};
use super::generate::{GenerationSettings, RustdocBuildEnvironment};
use super::progress::{CallbackHandler, ProgressCallbacks};

#[derive(Debug, Clone)]
pub(crate) struct RegistryRequest<'a> {
    index_entry: &'a tame_index::IndexVersion,
}

#[derive(Debug, Clone)]
pub(crate) struct ProjectRequest<'a> {
    pub(super) manifest: &'a Manifest,
}

#[derive(Debug, Clone)]
pub(crate) enum RequestKind<'a> {
    Registry(RegistryRequest<'a>),
    LocalProject(ProjectRequest<'a>),
}

impl RequestKind<'_> {
    pub(super) fn name(&self) -> anyhow::Result<&str> {
        Ok(match self {
            Self::Registry(RegistryRequest { index_entry }) => &index_entry.name,
            Self::LocalProject(ProjectRequest { manifest }) => {
                crate::manifest::get_package_name(manifest)?
            }
        })
    }

    pub(super) fn version(&self) -> anyhow::Result<Cow<'_, str>> {
        Ok(match self {
            Self::Registry(RegistryRequest { index_entry }) => {
                Cow::Borrowed(index_entry.version.as_str())
            }
            Self::LocalProject(ProjectRequest { manifest }) => {
                Cow::Owned(crate::manifest::get_package_version(manifest)?)
            }
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) enum CacheSettings<T> {
    /// No caching at all.
    None,

    /// A read-only cache. We do *not* populate it on cache miss.
    ReadOnly(T),

    /// A read-write cache, populated on cache miss.
    ReadWrite(T),

    /// A cache that will ignore present data (because maybe it's invalid or stale!),
    /// but will write newly generated items, overwriting anything present at that path.
    WriteOnly(T),
}

impl CacheSettings<()> {
    pub(crate) fn with_path<'a>(&self, path: &'a Path) -> CacheSettings<&'a Path> {
        match self {
            CacheSettings::None => CacheSettings::None,
            CacheSettings::ReadOnly(_) => CacheSettings::ReadOnly(path),
            CacheSettings::ReadWrite(_) => CacheSettings::ReadWrite(path),
            CacheSettings::WriteOnly(_) => CacheSettings::WriteOnly(path),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CacheUse<'a> {
    /// Invariant: always `None` if the cache settings are [`CacheSettings::None`],
    /// and always `Some` otherwise.
    json_cache_location: Option<PathBuf>,

    /// Invariant: always `None` if the cache settings are [`CacheSettings::None`],
    /// and always `Some` otherwise.
    metadata_cache_location: Option<PathBuf>,

    settings: CacheSettings<&'a Path>,
}

#[derive(Debug, Clone)]
struct CacheEntry<'a> {
    json: &'a Path,
    metadata: &'a Path,
}

impl<'a> CacheUse<'a> {
    fn new(
        request: &CrateDataRequest<'a>,
        build_environment: &RustdocBuildEnvironment,
        settings: CacheSettings<&'a Path>,
    ) -> anyhow::Result<Self> {
        // We can only cache registry crates. For local crates, we have no idea of the state
        // of the local filesystem: it can point to an arbitrary git commit, have dirty repo state,
        // or might not be part of a git repository at all. We cannot guarantee consistency,
        // so we don't offer caching.
        let settings = if matches!(request.kind, RequestKind::LocalProject(..)) {
            CacheSettings::None
        } else {
            settings
        };

        let key = request.artifact_slug(build_environment)?;

        let (json_cache_location, metadata_cache_location) = {
            match settings {
                CacheSettings::None => (None, None),
                CacheSettings::ReadOnly(path)
                | CacheSettings::ReadWrite(path)
                | CacheSettings::WriteOnly(path) => (
                    Some(path.join(format!("{key}.json"))),
                    Some(path.join(format!("{key}.metadata.json"))),
                ),
            }
        };

        Ok(Self {
            json_cache_location,
            metadata_cache_location,
            settings,
        })
    }

    fn read(&self) -> anyhow::Result<Option<CacheEntry<'_>>> {
        match self.settings {
            CacheSettings::ReadWrite(..) | CacheSettings::ReadOnly(..) => {
                let json_path = self
                    .json_cache_location
                    .as_ref()
                    .expect("invariant violation: no cache path for readable cache");
                let metadata_path = self
                    .metadata_cache_location
                    .as_ref()
                    .expect("invariant violation: no metadata path for readable cache");

                if json_path.exists() && metadata_path.exists() {
                    return Ok(Some(CacheEntry {
                        json: json_path,
                        metadata: metadata_path,
                    }));
                }
            }
            CacheSettings::WriteOnly(..) | CacheSettings::None => {}
        }

        Ok(None)
    }

    /// If the cache policy allows writing to the cache, attempt to cache a path.
    ///
    /// If the cache policy does not allow writing, this returns `Ok(..)`.
    /// Errors are genuine failures to write to the cache, such as I/O errors.
    fn populate(
        &self,
        rustdoc_json: &Path,
        metadata: &cargo_metadata::Metadata,
    ) -> anyhow::Result<bool> {
        match self.settings {
            CacheSettings::ReadWrite(path) | CacheSettings::WriteOnly(path) => {
                let json_path = self
                    .json_cache_location
                    .as_ref()
                    .expect("invariant violation: no cache path for readable cache");
                let metadata_path = self
                    .metadata_cache_location
                    .as_ref()
                    .expect("invariant violation: no metadata path for readable cache");

                fs_err::create_dir_all(path)?;
                // Do not use `fs_err::copy` or `std::fs::copy` here. Those APIs open the
                // destination path themselves and stream bytes into it directly, so another process
                // can observe a partially copied cache file if it reads while the copy is in
                // progress or if the copy fails partway through. Routing the copy through
                // `atomic_write` publishes only a completed temp-file copy.
                atomic_write(json_path, |writer| {
                    let mut rustdoc_json_file = fs_err::File::open(rustdoc_json)?;
                    std::io::copy(&mut rustdoc_json_file, writer).with_context(|| {
                        format!(
                            "failed to copy rustdoc JSON cache file from {} to {}",
                            rustdoc_json.display(),
                            json_path.display()
                        )
                    })?;
                    Ok(())
                })?;
                atomic_write(metadata_path, |writer| {
                    serde_json::to_writer(writer, metadata)?;
                    Ok(())
                })?;
                Ok(true)
            }
            CacheSettings::None | CacheSettings::ReadOnly(..) => Ok(false),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CrateDataRequest<'a> {
    pub(crate) kind: RequestKind<'a>,

    // N.B.: `--no-default-features --feature default` is not the same as using default features,
    //       since it will fail if the crate has no "default" feature definition.
    pub(super) default_features: bool,

    pub(super) extra_features: BTreeSet<Cow<'a, str>>,
    pub(super) build_target: Option<&'a str>,

    /// Purely for progress reporting purposes. Does not change behavior.
    pub(super) is_baseline: bool,
}

impl<'a> CrateDataRequest<'a> {
    pub(crate) fn from_index(
        index_entry: &'a tame_index::IndexVersion,
        default_features: bool,
        extra_features: BTreeSet<Cow<'a, str>>,
        build_target: Option<&'a str>,
        is_baseline: bool,
    ) -> Self {
        Self {
            kind: RequestKind::Registry(RegistryRequest { index_entry }),
            default_features,
            extra_features,
            build_target,
            is_baseline,
        }
    }

    pub(crate) fn from_local_project(
        manifest: &'a Manifest,
        default_features: bool,
        extra_features: BTreeSet<Cow<'a, str>>,
        build_target: Option<&'a str>,
        is_baseline: bool,
    ) -> Self {
        Self {
            kind: RequestKind::LocalProject(ProjectRequest { manifest }),
            default_features,
            extra_features,
            build_target,
            is_baseline,
        }
    }

    pub(crate) fn package_name(&self) -> anyhow::Result<&str> {
        self.kind.name()
    }

    pub(crate) fn exact_version(&self) -> anyhow::Result<String> {
        Ok(format!("={}", self.kind.version()?.as_ref()))
    }

    pub(crate) fn local_project_dir(&self) -> anyhow::Result<Option<PathBuf>> {
        match &self.kind {
            RequestKind::Registry(..) => Ok(None),
            RequestKind::LocalProject(ProjectRequest { manifest }) => Ok(Some(
                crate::manifest::get_project_dir_from_manifest_path(&manifest.path)?,
            )),
        }
    }

    pub(crate) fn default_features_enabled(&self) -> bool {
        self.default_features
    }

    pub(crate) fn extra_features(&self) -> impl Iterator<Item = &str> + '_ {
        self.extra_features.iter().map(Cow::as_ref)
    }

    pub(crate) fn build_target(&self) -> Option<&str> {
        self.build_target
    }

    /// Best-effort Rust import name for this crate as it would appear in
    /// downstream code. This is intentionally not always the same as the Cargo
    /// package name: local projects may customize their library target name,
    /// while registry packages with dashes are imported with underscores.
    pub(crate) fn fallback_import_name(&self) -> anyhow::Result<String> {
        match &self.kind {
            RequestKind::Registry(..) => Ok(self.package_name()?.replace('-', "_")),
            RequestKind::LocalProject(ProjectRequest { manifest }) => {
                crate::manifest::get_library_target_name(manifest)
            }
        }
    }

    /// Load data for the requested crate, using the specified directories.
    ///
    /// `target_root` is the directory where we'll perform any necessary code generation
    /// to load the data. The specific operations performed there are not public API.
    ///
    /// `cache` specifies how we may use a cache to speed up our data requests:
    /// read-write, read-only, or not at all.
    pub(crate) fn resolve<'slf>(
        &'slf self,
        target_root: &Path,
        cache_settings: CacheSettings<&'a Path>,
        generation_settings: GenerationSettings,
        callbacks: &'slf mut dyn ProgressCallbacks<'slf>,
    ) -> Result<VersionedStorage, TerminalError> {
        let version = self
            .kind
            .version()
            .context("failed to get crate version")
            .into_terminal_result()?;
        let mut callbacks = CallbackHandler::new(
            self.kind
                .name()
                .context("failed to get crate name")
                .into_terminal_result()?,
            version,
            self.is_baseline,
            callbacks,
        );

        // We treat failures to even set up a cache to use as fatal,
        // since they almost always indicate a serious bug in our mental model.
        // An example of a failure here would be "crates don't always have a name, actually"
        // which is something we want to know about ASAP.
        let build_environment =
            RustdocBuildEnvironment::from_env_and_config_for_target(self.build_target())
                .into_terminal_result()?;
        let cache =
            CacheUse::new(self, &build_environment, cache_settings).into_terminal_result()?;

        // Can we satisfy the request from cache?
        match cache.read() {
            // Cache hit!
            Ok(Some(entry)) => {
                callbacks.rustdoc_cache_hit();
                callbacks.parse_rustdoc_start(true);

                match std::fs::read_to_string(entry.metadata) {
                    Ok(text) => match serde_json::from_str(&text) {
                        Ok(metadata) => {
                            match load_rustdoc_with_optional_metadata(
                                entry.json,
                                metadata,
                                &mut callbacks,
                            ) {
                                Ok(data) => {
                                    callbacks.parse_rustdoc_success(true);
                                    return Ok(data);
                                }
                                Err(e) => {
                                    callbacks.non_fatal_error(
                                        e.context("failed to load cached rustdoc JSON"),
                                    );
                                }
                            }
                        }
                        Err(e) => {
                            callbacks.non_fatal_error(
                                anyhow::anyhow!(e).context("failed to parse cached metadata"),
                            );
                        }
                    },
                    Err(e) => callbacks.non_fatal_error(
                        anyhow::anyhow!(e).context("failed to read cached metadata file"),
                    ),
                }
            }

            // Cache miss. Move on.
            Ok(None) => {}

            // Error in the cache lookup. Record it, then move on as if cache miss.
            Err(e) => {
                callbacks.non_fatal_error(
                    e.context("failed to perform cache lookup, continuing as if cache miss"),
                );
            }
        }

        // Generate the data we need.
        let build_dir = target_root.join(
            self.build_path_slug(&build_environment)
                .into_terminal_result()?,
        );
        let (data_path, metadata) = super::generate::generate_rustdoc(
            self,
            &build_dir,
            generation_settings,
            &build_environment,
            &mut callbacks,
        )?;

        // Check if we need to populate the cache.
        // If the cache doesn't need to be populated, this returns `Ok(false)`.
        // Errors are genuine failures to populate the cache, such as I/O problems.
        let mut clean_up_build_dir = false;
        match cache.populate(data_path.as_path(), &metadata) {
            // Populated the cache.
            Ok(true) => {
                callbacks.rustdoc_cache_populated();

                // Clean up our build dir, since we don't need it anymore.
                clean_up_build_dir = true;
            }

            // Did not populate the cache.
            Ok(false) => {}

            // Encountered an error while attempting to populate the cache.
            Err(e) => {
                callbacks.non_fatal_error(e.context("failed to populate rustdoc cache"));
            }
        }

        // This time, failure to read the rustdoc is fatal.
        callbacks.parse_rustdoc_start(false);
        let data = load_rustdoc_with_optional_metadata(&data_path, metadata, &mut callbacks)
            .into_terminal_result()?;
        callbacks.parse_rustdoc_success(false);

        if clean_up_build_dir {
            let outcome = std::fs::remove_dir_all(&build_dir);
            if let Err(e) = outcome {
                callbacks.non_fatal_error(
                    anyhow::Error::from(e)
                        .context("failed to clean up build dir after populating rustdoc cache"),
                );
            }
        }

        Ok(data)
    }

    /// A path-safe unique identifier for the placeholder build directory.
    fn build_path_slug(
        &self,
        build_environment: &RustdocBuildEnvironment,
    ) -> anyhow::Result<String> {
        Ok(format!(
            "{}-{}",
            match &self.kind {
                RequestKind::Registry { .. } => "registry",
                RequestKind::LocalProject { .. } => "local",
            },
            self.artifact_slug(build_environment)?,
        ))
    }

    /// A path-safe unique identifier for the generated rustdoc artifact.
    fn artifact_slug(&self, build_environment: &RustdocBuildEnvironment) -> anyhow::Result<String> {
        Ok(format!(
            "{}-{}-{}-{}",
            slugify(self.kind.name()?),
            slugify(self.kind.version()?.as_ref()),
            slugify(&build_environment.target_triple),
            self.artifact_fingerprint(build_environment)?,
        ))
    }

    fn artifact_fingerprint(
        &self,
        build_environment: &RustdocBuildEnvironment,
    ) -> anyhow::Result<String> {
        // Bump this when the fields or semantics hashed into rustdoc artifact identity change, so
        // old cache entries and placeholder build directories cannot collide with the new
        // interpretation.
        const RUSTDOC_ARTIFACT_FINGERPRINT_SCHEMA: &str = "rustdoc-artifact-v1";

        let mut hasher = sha2::Sha256::new();
        update_artifact_hash(&mut hasher, "schema", RUSTDOC_ARTIFACT_FINGERPRINT_SCHEMA);
        update_artifact_hash(
            &mut hasher,
            "source",
            match &self.kind {
                RequestKind::Registry { .. } => "registry",
                RequestKind::LocalProject { .. } => "local",
            },
        );
        update_artifact_hash(&mut hasher, "name", self.kind.name()?);
        update_artifact_hash(&mut hasher, "version", self.kind.version()?.as_ref());
        update_artifact_hash(
            &mut hasher,
            "default_features",
            if self.default_features {
                "true"
            } else {
                "false"
            },
        );
        for feature in &self.extra_features {
            update_artifact_hash(&mut hasher, "feature", feature);
        }
        update_artifact_hash(&mut hasher, "target", &build_environment.target_triple);
        update_artifact_hash(
            &mut hasher,
            "rustflags",
            build_environment.cargo_rustflags.as_ref(),
        );
        update_artifact_hash(
            &mut hasher,
            "rustdocflags",
            build_environment.cargo_rustdocflags.as_ref(),
        );
        update_artifact_hash(
            &mut hasher,
            "toolchain_version",
            &build_environment.toolchain_version,
        );

        // First 16 hex characters are good enough for our use case.
        // For birthday paradox to occur, a single crate version must be run
        // with approximately 2**32 artifact configurations.
        let digest = hasher.finalize();
        let mut hash = String::with_capacity(16);
        for byte in digest.as_slice().iter().take(8) {
            write!(&mut hash, "{byte:02x}").expect("writing to a String should not fail");
        }
        Ok(hash)
    }
}

fn load_rustdoc_with_optional_metadata(
    json_path: &Path,
    metadata: cargo_metadata::Metadata,
    callbacks: &mut CallbackHandler<'_>,
) -> anyhow::Result<VersionedStorage> {
    match trustfall_rustdoc::load_rustdoc(json_path, Some(metadata)) {
        Ok(data) => Ok(data),
        Err(e @ LoadingError::MetadataParsing(..)) => {
            // Metadata parsing is brand new and might have unforeseen issues.
            // Be resilient: report the problem but don't crash --
            // instead, drop back to not checking manifest data.
            callbacks.non_fatal_error(anyhow::Error::from(e).context("skipping package metadata due to failure to load it; package manifest checks will not discover any breakage"));

            trustfall_rustdoc::load_rustdoc(json_path, None).map_err(anyhow::Error::from)
        }
        Err(e) => Err(anyhow::Error::from(e)),
    }
}

fn update_artifact_hash(hasher: &mut sha2::Sha256, label: &str, value: &str) {
    // Length-prefix each field so adjacent records cannot collide by concatenating ambiguously.
    for entry in [label, value] {
        let bytes = entry.as_bytes();
        hasher.update(bytes.len().to_le_bytes());
        hasher.update(bytes);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::*;

    fn test_manifest() -> Manifest {
        Manifest::parse(PathBuf::from(
            "test_crates/cfg_conditional_compilation/old/Cargo.toml",
        ))
        .expect("failed to load test manifest")
    }

    fn build_environment(
        target_triple: &str,
        rustdocflags: &'static str,
    ) -> RustdocBuildEnvironment {
        RustdocBuildEnvironment {
            target_triple: target_triple.to_owned(),
            cargo_rustflags: Cow::Borrowed("--cap-lints=allow"),
            cargo_rustdocflags: Cow::Borrowed(rustdocflags),
            toolchain_version: String::from("1.95.0"),
        }
    }

    #[test]
    fn artifact_slug_uses_canonical_target_triple() {
        let implicit_build_environment =
            RustdocBuildEnvironment::from_env_and_config_for_target(None)
                .expect("implicit build environment failed");
        let target_triple = implicit_build_environment.target_triple.clone();
        let explicit_build_environment =
            RustdocBuildEnvironment::from_env_and_config_for_target(Some(&target_triple))
                .expect("explicit build environment failed");

        let implicit_manifest = test_manifest();
        let explicit_manifest = test_manifest();
        let implicit = CrateDataRequest::from_local_project(
            &implicit_manifest,
            true,
            BTreeSet::new(),
            None,
            false,
        );
        let explicit = CrateDataRequest::from_local_project(
            &explicit_manifest,
            true,
            BTreeSet::new(),
            Some(&target_triple),
            false,
        );

        assert_eq!(
            implicit
                .artifact_slug(&implicit_build_environment)
                .expect("implicit artifact slug failed"),
            explicit
                .artifact_slug(&explicit_build_environment)
                .expect("explicit artifact slug failed"),
        );
    }

    #[test]
    fn artifact_slug_includes_effective_rustdocflags() {
        let manifest = test_manifest();
        let request =
            CrateDataRequest::from_local_project(&manifest, true, BTreeSet::new(), None, false);
        let host_environment = RustdocBuildEnvironment::from_env_and_config_for_target(None)
            .expect("build environment failed");
        let without_cfg = build_environment(
            &host_environment.target_triple,
            "-Z unstable-options --document-private-items --document-hidden-items --output-format=json --cap-lints=allow",
        );
        let with_cfg = build_environment(
            &host_environment.target_triple,
            "--cfg custom -Z unstable-options --document-private-items --document-hidden-items --output-format=json --cap-lints=allow",
        );

        assert_ne!(
            request
                .artifact_slug(&without_cfg)
                .expect("artifact slug without cfg failed"),
            request
                .artifact_slug(&with_cfg)
                .expect("artifact slug with cfg failed"),
        );
    }
}
