use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::Context;
use rustdoc_types::Crate;

pub(crate) const SCOPE: &str = "semver-checks";

pub(crate) fn load_rustdoc_from_file(path: &Path) -> anyhow::Result<Crate> {
    // Parsing JSON after fully reading a file into memory is much faster than
    // parsing directly from a file, even if buffered:
    // https://github.com/serde-rs/json/issues/160
    let mut s = String::new();
    File::open(path)
        .with_context(|| format!("Failed to open rustdoc JSON output file {:?}", path))?
        .read_to_string(&mut s)
        .with_context(|| format!("Failed to read rustdoc JSON output file {:?}", path))?;

    serde_json::from_str(&s)
        .with_context(|| format!("Failed to parse rustdoc JSON output file {:?}", path))
}

pub(crate) fn slugify(value: &str) -> String {
    value
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>()
}
