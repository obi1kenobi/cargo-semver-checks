#[derive(Debug, Clone)]
pub(crate) struct Manifest<'a> {
    path: &'a std::path::Path,
    parsed: cargo_toml::Manifest,
}

impl<'a> Manifest<'a> {
    pub(crate) fn parse(path: &'a std::path::Path) -> anyhow::Result<Self> {
        let manifest_text = std::fs::read_to_string(path)
            .map_err(|e| anyhow::format_err!("Failed when reading {}: {}", path.display(), e))?;
        let parsed = toml::from_str(manifest_text.as_str())
            .map_err(|e| anyhow::format_err!("Failed to parse {}: {}", path.display(), e))?;

        Ok(Self { path, parsed })
    }
}

pub(crate) fn get_package_name(manifest: &Manifest) -> anyhow::Result<String> {
    let package = manifest.parsed.package.as_ref().ok_or_else(|| {
        anyhow::format_err!(
            "Failed to parse {}: no `package` table",
            manifest.path.display()
        )
    })?;
    Ok(package.name.clone())
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
