mod rustdoc_fmt;

mod extract_func_args;
mod extract_method_args;

use std::{collections::BTreeMap, fs::File, io::Read, path::Path, sync::Arc};

use anyhow::{Context, Result};

pub(crate) use extract_func_args::extract_func_args;
pub(crate) use extract_method_args::extract_method_args;

use trustfall::FieldValue;

fn load_file_data(path: &Path) -> Result<rustdoc_types::Crate> {
    let mut file_data = String::new();
    File::open(path)
        .with_context(|| {
            format!(
                "error opening rustdoc file {} for witness checks",
                path.display()
            )
        })?
        .read_to_string(&mut file_data)
        .with_context(|| {
            format!(
                "error reading rustdoc file {} for witness checks",
                path.display()
            )
        })?;

    // TODO: Detect version, currently just always uses v56 as a default
    serde_json::from_str(&file_data).with_context(|| {
        format!(
            "error parsing rustdoc file {} with version v56 for witness checks",
            path.display()
        )
    })
}

fn insert_new_result(
    results: &mut BTreeMap<Arc<str>, FieldValue>,
    key: &str,
    value: FieldValue,
) -> Result<()> {
    let key = Arc::from(key);
    match results.insert(Arc::clone(&key), value) {
        None => Ok(()),
        Some(_) => anyhow::bail!(
            "error inserting new value in witness results at `{key}`, entry is occupied"
        ),
    }
}
