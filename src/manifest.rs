use anyhow::Context;

#[derive(Debug, Clone)]
pub(crate) struct Manifest {
    pub(crate) path: std::path::PathBuf,
    pub(crate) parsed: cargo_toml::Manifest,
}

impl Manifest {
    pub(crate) fn parse(path: std::path::PathBuf) -> anyhow::Result<Self> {
        // Parsing via `cargo_toml::Manifest::from_path()` is preferable to parsing from a string,
        // because inspection of surrounding files is sometimes necessary to determine
        // the existence of lib targets and ensure proper handling of workspace inheritance.

        let parsed = cargo_toml::Manifest::from_path(&path)
            .with_context(|| format!("Failed when reading {}", path.display()))?;

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
    let version = package.version.get().with_context(|| {
        format!(
            "Failed to retrieve package version from {}",
            manifest.path.display(),
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
