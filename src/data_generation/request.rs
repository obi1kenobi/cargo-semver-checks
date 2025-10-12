use std::path::PathBuf;
use std::{borrow::Cow, collections::BTreeSet, path::Path};

use anyhow::Context;
use sha2::Digest as _;
use trustfall_rustdoc::{LoadingError, VersionedStorage};

use crate::manifest::Manifest;
use crate::util::slugify;
use crate::witness_gen::CoupledRustdocPath;

use super::error::{IntoTerminalResult, TerminalError};
use super::generate::GenerationSettings;
use super::progress::{CallbackHandler, ProgressCallbacks};

#[derive(Debug, Clone)]
pub(crate) struct RegistryRequest<'a> {
    pub(crate) index_entry: &'a tame_index::IndexVersion,
}

#[derive(Debug, Clone)]
pub(crate) struct ProjectRequest<'a> {
    pub(crate) manifest: &'a Manifest,
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

    pub(super) fn version(&self) -> anyhow::Result<&str> {
        Ok(match self {
            Self::Registry(RegistryRequest { index_entry }) => index_entry.version.as_str(),
            Self::LocalProject(ProjectRequest { manifest }) => {
                crate::manifest::get_package_version(manifest)?
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

        let key = {
            if let Some(build_target) = request.build_target {
                format!("{}-{}", request.cache_slug()?, build_target)
            } else {
                request.cache_slug()?
            }
        };

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
                fs_err::copy(rustdoc_json, json_path)?;
                fs_err::write(metadata_path, serde_json::to_string(metadata)?)?;
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

    /// Fingerprint of the feature selections, for use in disambiguating between artifacts.
    features_fingerprint: String,
}

impl<'a> CrateDataRequest<'a> {
    pub(crate) fn from_index(
        index_entry: &'a tame_index::IndexVersion,
        default_features: bool,
        extra_features: BTreeSet<Cow<'a, str>>,
        build_target: Option<&'a str>,
        is_baseline: bool,
    ) -> Self {
        let features_fingerprint = make_features_hash(default_features, &extra_features);
        Self {
            kind: RequestKind::Registry(RegistryRequest { index_entry }),
            default_features,
            extra_features,
            build_target,
            is_baseline,
            features_fingerprint,
        }
    }

    pub(crate) fn from_local_project(
        manifest: &'a Manifest,
        default_features: bool,
        extra_features: BTreeSet<Cow<'a, str>>,
        build_target: Option<&'a str>,
        is_baseline: bool,
    ) -> Self {
        let features_fingerprint = make_features_hash(default_features, &extra_features);
        Self {
            kind: RequestKind::LocalProject(ProjectRequest { manifest }),
            default_features,
            extra_features,
            build_target,
            is_baseline,
            features_fingerprint,
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
    ) -> Result<CoupledRustdocPath<VersionedStorage>, TerminalError> {
        let mut callbacks = CallbackHandler::new(
            self.kind
                .name()
                .context("failed to get crate name")
                .into_terminal_result()?,
            self.kind
                .version()
                .context("failed to get crate version")
                .into_terminal_result()?,
            self.is_baseline,
            callbacks,
        );

        // We treat failures to even set up a cache to use as fatal,
        // since they almost always indicate a serious bug in our mental model.
        // An example of a failure here would be "crates don't always have a name, actually"
        // which is something we want to know about ASAP.
        let cache = CacheUse::new(self, cache_settings).into_terminal_result()?;

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
        let build_dir = target_root.join(self.build_path_slug().into_terminal_result()?);
        let (data_path, metadata) = super::generate::generate_rustdoc(
            self,
            &build_dir,
            generation_settings,
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

    /// A path-safe unique identifier that includes the crate's source, name, version, and features.
    fn build_path_slug(&self) -> anyhow::Result<String> {
        Ok(format!(
            "{}-{}",
            match &self.kind {
                RequestKind::Registry { .. } => "registry",
                RequestKind::LocalProject { .. } => "local",
            },
            self.cache_slug()?,
        ))
    }

    /// A path-safe unique identified that includes the crate's name, version, and features.
    fn cache_slug(&self) -> anyhow::Result<String> {
        Ok(format!(
            "{}-{}-{}-{}",
            slugify(self.kind.name()?),
            slugify(self.kind.version()?),
            slugify(self.build_target.unwrap_or("default")),
            &self.features_fingerprint,
        ))
    }
}

fn load_rustdoc_with_optional_metadata(
    json_path: &Path,
    metadata: cargo_metadata::Metadata,
    callbacks: &mut CallbackHandler<'_>,
) -> anyhow::Result<CoupledRustdocPath<VersionedStorage>> {
    match trustfall_rustdoc::load_rustdoc(json_path, Some(metadata)) {
        Ok(data) => Ok(CoupledRustdocPath::new(data, json_path.to_path_buf())),
        Err(e @ LoadingError::MetadataParsing(..)) => {
            // Metadata parsing is brand new and might have unforeseen issues.
            // Be resilient: report the problem but don't crash --
            // instead, drop back to not checking manifest data.
            callbacks.non_fatal_error(anyhow::Error::from(e).context("skipping package metadata due to failure to load it; package manifest checks will not discover any breakage"));

            trustfall_rustdoc::load_rustdoc(json_path, None)
                .map(|data| CoupledRustdocPath::new(data, json_path.to_path_buf()))
                .map_err(anyhow::Error::from)
        }
        Err(e) => Err(anyhow::Error::from(e)),
    }
}

fn make_features_hash(default_features: bool, extra_features: &BTreeSet<Cow<'_, str>>) -> String {
    // Use newlines as the record separator, since newlines are not valid in feature names.
    let mut hasher = sha2::Sha256::new();

    // The default feature is always first. The name `default` is reserved for default features.
    if default_features {
        hasher.update("default\n".as_bytes());
    }

    for feature in extra_features {
        hasher.update(feature.as_bytes());
        hasher.update("\n".as_bytes());
    }

    // Store the hash as string with hex number (leading zeros added)
    let mut hash = format!("{:0>64x}", hasher.finalize());

    // First 16 characters is good enough for our use case.
    // For birthday paradox to occur, a single crate version must be run
    // with approximately 2**32 feature configurations.
    hash.truncate(16);
    hash
}
