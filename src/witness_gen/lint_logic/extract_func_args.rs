//! Logic for [`WitnessLogic::ExtractFuncArgs`](crate::query::WitnessLogic::ExtractFuncArgs)

use std::{collections::BTreeMap, fs::File, io::Read, path::Path, sync::Arc};

use anyhow::{Context, Result};
use rustdoc_types::Crate;
use trustfall::FieldValue;

use crate::witness_gen::WitnessRustdocPaths;

/// Extracts the function arguments from the rustdoc data
///
/// Expects a function signature outputted on `old_signature`
pub(super) fn extract_func_args(
    witness_results: BTreeMap<Arc<str>, FieldValue>,
    rustdoc_paths: &WitnessRustdocPaths,
) -> Result<BTreeMap<Arc<str>, FieldValue>> {
    let value = witness_results
        .get("old_signature")
        .context("failure extracting old_signature, key is not present")?;

    let signature = match value {
        FieldValue::String(signature) => signature,
        value => {
            anyhow::bail!("failure extracing old_signature, expected a String, found {value:?}")
        }
    };

    let baseline = load_file_data(&rustdoc_paths.baseline)?;
    let current = load_file_data(&rustdoc_paths.current)?;

    anyhow::bail!(
        "Failed intentionally: {:?}",
        witness_results.get("old_signature")
    )
}

fn load_file_data(path: &Path) -> Result<Crate> {
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
