pub fn get_package_name(manifest_path: &std::path::Path) -> anyhow::Result<String> {
    let manifest = std::fs::read_to_string(manifest_path).map_err(|e| {
        anyhow::format_err!("Failed when reading {}: {}", manifest_path.display(), e)
    })?;
    let manifest: toml_edit::Document = manifest
        .parse()
        .map_err(|e| anyhow::format_err!("Failed to parse {}: {}", manifest_path.display(), e))?;
    let crate_name = manifest["package"]["name"].as_str().ok_or_else(|| {
        anyhow::format_err!(
            "Failed to parse {}: invalid package.name",
            manifest_path.display()
        )
    })?;
    Ok(crate_name.to_owned())
}
