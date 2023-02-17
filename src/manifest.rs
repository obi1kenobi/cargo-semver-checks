use anyhow::Context;

#[derive(Debug, Clone)]
pub(crate) struct Manifest {
    pub(crate) path: std::path::PathBuf,
    pub(crate) parsed: cargo_toml::Manifest,
}

impl Manifest {
    pub(crate) fn parse(path: std::path::PathBuf) -> anyhow::Result<Self> {
        // Normally, we'd like to parse directly via `cargo_toml::Manifest::from_path()`.
        // Using the path is preferable to parsing from a string, because auto-detection of
        // surrounding files is sometimes necessary to determine the existence of lib targets
        // and ensure proper handling of workspace inheritance.
        //
        // However, `cargo_toml` currently has buggy handling of renamed library targets:
        // https://gitlab.com/crates.rs/cargo_toml/-/merge_requests/16/
        //
        // This is a workaround for that bug, added in:
        // https://github.com/obi1kenobi/cargo-semver-checks/pull/371/files#diff-a44cbf177b0e747788be2cc21913a967ce8a76f371b98a6c17cd908c400bf1a9
        let manifest_text = std::fs::read_to_string(&path)
            .map_err(|e| anyhow::format_err!("Failed when reading {}: {}", path.display(), e))?;
        let parsed = match toml::from_str(&manifest_text) {
            Ok(parsed) => parsed,
            Err(_) => {
                // Parsing failed, possibly because of workspace inheritance. Retry with cargo_toml.
                cargo_toml::Manifest::from_path(&path)?
            }
        };
        Ok(Self { path, parsed })
    }
}

pub(crate) fn get_package_name(manifest: &Manifest) -> anyhow::Result<&str> {
    let package = manifest.parsed.package.as_ref().ok_or_else(|| {
        anyhow::format_err!(
            "Failed to parse {}: no `package` table",
            manifest.path.display()
        )
    })?;
    Ok(&package.name)
}

pub(crate) fn get_package_version(manifest: &Manifest) -> anyhow::Result<&str> {
    let package = manifest.parsed.package.as_ref().ok_or_else(|| {
        anyhow::format_err!(
            "Failed to parse {}: no `package` table",
            manifest.path.display()
        )
    })?;
    let version = package.version.get().map_err(|e| {
        anyhow::format_err!(
            "Failed to retrieve package version from {}: {}",
            manifest.path.display(),
            e
        )
    })?;
    Ok(version)
}

pub(crate) fn get_lib_target_name(manifest: &Manifest) -> anyhow::Result<String> {
    // If there's a [lib] section, return the name it specifies, if any.
    if let Some(product) = &manifest.parsed.lib {
        if let Some(lib_name) = &product.name {
            return Ok(lib_name.clone());
        }
    }

    // Otherwise, assume the crate is a lib crate with the default lib target name:
    // the same name as the package but with dashes replaced with underscores.
    Ok(get_package_name(manifest)?.replace('-', "_"))
}

pub(crate) fn get_first_bin_target_name(manifest: &Manifest) -> anyhow::Result<String> {
    // If there's a [[bin]] section, return the first item's name.
    if let Some(product) = manifest.parsed.bin.first() {
        if let Some(bin_name) = &product.name {
            return Ok(bin_name.clone());
        }
    }

    // Otherwise, assume the crate is a bin crate with the default bin target name:
    // the same name as the package but with dashes replaced with underscores.
    Ok(get_package_name(manifest)?.replace('-', "_"))
}

pub(crate) fn get_project_dir_from_manifest_path(
    manifest_path: &std::path::Path,
) -> anyhow::Result<std::path::PathBuf> {
    assert!(
        manifest_path.ends_with("Cargo.toml"),
        "path {} isn't pointing to a manifest",
        manifest_path.display()
    );
    let dir_path = manifest_path
        .parent()
        .context("manifest path doesn't have a parent")?;
    Ok(dir_path.to_path_buf())
}
