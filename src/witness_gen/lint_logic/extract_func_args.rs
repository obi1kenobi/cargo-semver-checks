//! Logic for [`WitnessLogic::ExtractFuncArgs`](crate::query::WitnessLogic::ExtractFuncArgs)

use std::{collections::BTreeMap, fs::File, io::Read, path::Path, sync::Arc};

use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustdoc_types::{Crate, Function, Item, ItemEnum, ItemKind};
use trustfall::FieldValue;

use crate::witness_gen::{WitnessRustdocPaths, lint_logic::rustdoc_fmt::FormatRustdoc};

/// Extracts the function arguments from the rustdoc data
///
/// Expects a full path on `path`
///
/// Inserts values to `baseline_arg_types`, `baseline_arg_names`, `current_arg_types` and `current_arg_names`
pub(super) fn extract_func_args(
    mut witness_results: BTreeMap<Arc<str>, FieldValue>,
    rustdoc_paths: &WitnessRustdocPaths,
) -> Result<BTreeMap<Arc<str>, FieldValue>> {
    let value = witness_results
        .get("path")
        .context("failure extracting path, key is not present")?;

    let FieldValue::List(path) = value else {
        anyhow::bail!("failure extracting path, expected a List, found {value:?}")
    };

    let path = path.iter()
        .map(|segment| match segment {
            FieldValue::String(segment) => Ok(Arc::clone(segment)),
            _ => anyhow::bail!("failure extracting path, expected a List of Strings, at least one value was not a String"),
        })
        .collect::<Result<Vec<Arc<str>>>>()?;

    let baseline = load_file_data(&rustdoc_paths.baseline)?;
    let current = load_file_data(&rustdoc_paths.current)?;

    let baseline_func = find_function(&path, &baseline)?;
    let current_func = find_function(&path, &current)?;

    let mut baseline_types = vec![];
    let mut baseline_names = vec![];
    for (name, arg_type) in baseline_func.sig.inputs.iter() {
        baseline_types.push(FieldValue::String(arg_type.format_rustdoc()?.into()));
        baseline_names.push(FieldValue::String(name.as_str().into()));
    }

    let mut current_types = vec![];
    let mut current_names = vec![];
    for (name, arg_type) in current_func.sig.inputs.iter() {
        current_types.push(FieldValue::String(arg_type.format_rustdoc()?.into()));
        current_names.push(FieldValue::String(name.as_str().into()));
    }

    insert_new_result(
        &mut witness_results,
        "baseline_arg_types",
        FieldValue::List(baseline_types.into()),
    )?;
    insert_new_result(
        &mut witness_results,
        "baseline_arg_names",
        FieldValue::List(baseline_names.into()),
    )?;
    insert_new_result(
        &mut witness_results,
        "current_arg_types",
        FieldValue::List(current_types.into()),
    )?;
    insert_new_result(
        &mut witness_results,
        "current_arg_names",
        FieldValue::List(current_names.into()),
    )?;

    Ok(witness_results)
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

fn find_function<'a>(full_path: &[Arc<str>], rustdoc: &'a Crate) -> Result<&'a Function> {
    let (id, _) = rustdoc
        .paths
        .par_iter()
        .find_any(|(_, summary)| {
            // Find a function with the same full path as what we're looking for
            summary.kind == ItemKind::Function
                && summary
                    .path
                    .iter()
                    .zip(full_path)
                    .all(|(left, right)| *left.as_str() == **right)
        })
        .with_context(|| {
            format!(
                "error finding function, no function exists at the path {}",
                full_path.join("::")
            )
        })?;

    match rustdoc.index.get(id) {
        Some(Item {
            inner: ItemEnum::Function(func),
            ..
        }) => Ok(func),
        _ => unreachable!(
            "id is matched as being function and as existing, that should not vary here"
        ),
    }
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
