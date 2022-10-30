#[derive(Debug, Clone)]
pub(crate) struct Manifest<'a> {
    pub(crate) path: &'a std::path::Path,
    pub(crate) doc: toml_edit::Document,
}

impl<'a> Manifest<'a> {
    pub(crate) fn parse(path: &'a std::path::Path) -> anyhow::Result<Self> {
        let manifest_text = std::fs::read_to_string(path)
            .map_err(|e| anyhow::format_err!("Failed when reading {}: {}", path.display(), e))?;
        let doc = manifest_text
            .parse()
            .map_err(|e| anyhow::format_err!("Failed to parse {}: {}", path.display(), e))?;

        Ok(Self { path, doc })
    }
}

pub(crate) fn get_package_name(manifest: &Manifest) -> anyhow::Result<String> {
    let package = manifest.doc.get("package").ok_or_else(|| {
        anyhow::format_err!(
            "Failed to parse {}: no `package` table",
            manifest.path.display()
        )
    })?;
    let crate_name = package["name"].as_str().ok_or_else(|| {
        anyhow::format_err!(
            "Failed to parse {}: invalid package.name",
            manifest.path.display()
        )
    })?;
    Ok(crate_name.to_owned())
}

pub(crate) fn get_lib_target_name(manifest: &Manifest) -> anyhow::Result<String> {
    // If there's a [lib] section, return the name it specifies, if any.
    if let Some(lib_target) = manifest.doc.get("lib") {
        let lib_name = lib_target
            .as_table()
            .ok_or_else(|| {
                anyhow::format_err!(
                    "Failed to parse {}: invalid lib item",
                    manifest.path.display()
                )
            })?
            .get("name");

        if let Some(lib_name) = lib_name {
            return lib_name.as_str().map(|x| x.to_owned()).ok_or_else(|| {
                anyhow::format_err!(
                    "Failed to parse {}: invalid lib.name",
                    manifest.path.display()
                )
            });
        }
    }

    // Otherwise, assume the crate is a lib crate with the default lib target name:
    // the same name as the package but with dashes replaced with underscores.
    Ok(get_package_name(manifest)?.replace('-', "_"))
}

pub(crate) fn get_first_bin_target_name(manifest: &Manifest) -> anyhow::Result<String> {
    // If there's a [[bin]] section, return the first item's name.
    if let Some(lib_target) = manifest.doc.get("bin") {
        let bin_name = lib_target
            .as_array_of_tables()
            .ok_or_else(|| {
                anyhow::format_err!(
                    "Failed to parse {}: invalid bin item",
                    manifest.path.display()
                )
            })?
            .into_iter()
            .next()
            .ok_or_else(|| {
                anyhow::format_err!(
                    "Failed to parse {}: invalid bin first item",
                    manifest.path.display()
                )
            })?
            .get("name");

        if let Some(bin_name) = bin_name {
            return bin_name.as_str().map(|x| x.to_owned()).ok_or_else(|| {
                anyhow::format_err!(
                    "Failed to parse {}: invalid name in first bin item",
                    manifest.path.display()
                )
            });
        }
    }

    // Otherwise, assume the crate is a bin crate with the default bin target name:
    // the same name as the package but with dashes replaced with underscores.
    Ok(get_package_name(manifest)?.replace('-', "_"))
}
