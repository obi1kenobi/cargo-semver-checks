//! Logic for [`WitnessLogic::ExtractMethodArgs`](crate::query::WitnessLogic::ExtractMethodArgs)

use std::{collections::BTreeMap, sync::Arc};

use anyhow::{Context, Result};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustdoc_types::{Crate, Function, Id, Impl, Item, ItemEnum};
use trustfall::FieldValue;

use crate::witness_gen::{
    WitnessRustdocPaths,
    lint_logic::{insert_new_result, load_file_data, rustdoc_fmt::FormatRustdoc},
};

/// Extracts the method arguments from the rustdoc data
///
/// Expects a full path to the ImplOwner on `path`, the name of the Impl on `impl_name`, and a method name on `method_name`
///
/// Inserts values to `baseline_arg_types`, `baseline_arg_names`, `current_arg_types` and `current_arg_names`
pub(crate) fn extract_method_args(
    mut witness_results: BTreeMap<Arc<str>, FieldValue>,
    rustdoc_paths: &WitnessRustdocPaths,
) -> Result<BTreeMap<Arc<str>, FieldValue>> {
    let path_value = witness_results
        .get("path")
        .context("failure extracting path, key is not present")?;

    let impl_name_value = witness_results
        .get("impl_name")
        .context("failure extracting impl_name, key is not present")?;

    let method_name_value = witness_results
        .get("method_name")
        .context("failure extracting method_name, key is not present")?;

    let FieldValue::List(path) = path_value else {
        anyhow::bail!("failure extracting path, expected a List, found {path_value:?}")
    };

    let impl_name = if let FieldValue::String(impl_name) = impl_name_value {
        Some(impl_name)
    } else if let FieldValue::Null = impl_name_value {
        None
    } else {
        anyhow::bail!("failure extracting impl_name, expected a String, found {impl_name_value:?}")
    };

    let FieldValue::String(method_name) = method_name_value else {
        anyhow::bail!(
            "failure extracting method_value, expected a String, found {method_name_value:?}"
        )
    };

    let path = path.iter()
        .map(|segment| match segment {
            FieldValue::String(segment) => Ok(Arc::clone(segment)),
            _ => anyhow::bail!("failure extracting path, expected a List of Strings, at least one value was not a String"),
        })
        .collect::<Result<Vec<Arc<str>>>>()?;

    let baseline = load_file_data(&rustdoc_paths.baseline)?;
    let current = load_file_data(&rustdoc_paths.current)?;

    let baseline_method = find_method(&path, impl_name, method_name, &baseline)
        .context("failed to extract baseline method while extracting method args")?;
    let current_method = find_method(&path, impl_name, method_name, &current)
        .context("failed to extract current method while extracting method args")?;

    let mut baseline_types = vec![];
    let mut baseline_names = vec![];
    for (name, arg_type) in baseline_method.sig.inputs.iter().skip(1) {
        baseline_types.push(FieldValue::String(arg_type.format_rustdoc()?.into()));
        baseline_names.push(FieldValue::String(name.as_str().into()));
    }

    let mut current_types = vec![];
    let mut current_names = vec![];
    for (name, arg_type) in current_method.sig.inputs.iter().skip(1) {
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

fn find_method<'a>(
    full_path: &[Arc<str>],
    impl_name: Option<&Arc<str>>,
    method_name: &Arc<str>,
    rustdoc: &'a Crate,
) -> Result<&'a Function> {
    let root = rustdoc
        .index
        .get(&rustdoc.root)
        .context("crate root could not be found")?;

    let impls = recursively_find_impls(&full_path[1..], root, rustdoc)?;

    if let Some(impl_name) = impl_name {
        let implementation = get_impl_on_implementor(impls, impl_name, rustdoc)?;

        for item in &implementation.items {
            if let Some(Item {
                inner: ItemEnum::Function(func),
                name: Some(name),
                ..
            }) = rustdoc.index.get(item)
                && *name == **method_name
            {
                return Ok(func);
            }
        }
    } else {
        let filtered_methods = impls
            .iter()
            .filter_map(|id| rustdoc.index.get(id))
            .filter_map(|item| {
                if let Item {
                    inner: ItemEnum::Impl(implementation),
                    name: None,
                    ..
                } = item
                {
                    Some(&implementation.items)
                } else {
                    None
                }
            })
            .flatten();

        for item in filtered_methods {
            if let Some(Item {
                inner: ItemEnum::Function(func),
                name: Some(name),
                ..
            }) = rustdoc.index.get(item)
                && *name == **method_name
            {
                return Ok(func);
            }
        }
    }

    anyhow::bail!("failed to find method {method_name}");
}

fn recursively_find_impls<'a>(
    remaining_path: &[Arc<str>],
    previous: &'a Item,
    rustdoc: &'a Crate,
) -> Result<&'a [Id]> {
    if let Some(target) = remaining_path.first() {
        match previous {
            Item {
                inner: ItemEnum::Module(module),
                ..
            } => {
                let next = module.items.par_iter().find_map_any(|id| {
                    if let Some(item) = rustdoc.index.get(id)
                        && let Some(name) = &item.name
                        && *name == **target
                    {
                        Some(item)
                    } else {
                        None
                    }
                });
                next.with_context(|| format!("Next item not found at {target}"))
                    .and_then(|next| recursively_find_impls(&remaining_path[1..], next, rustdoc))
            }
            _ => anyhow::bail!("Module not found at {target}"),
        }
    } else {
        match &previous.inner {
            ItemEnum::Struct(item) => Ok(&item.impls),
            ItemEnum::Enum(item) => Ok(&item.impls),
            ItemEnum::Union(item) => Ok(&item.impls),
            _ => anyhow::bail!("reached end of path and value was not an struct, enum, or union"),
        }
    }
}

fn get_impl_on_implementor<'a>(
    impls: &'a [Id],
    impl_name: &Arc<str>,
    rustdoc: &'a Crate,
) -> Result<&'a Impl> {
    for item in impls {
        if let Some(Item {
            inner: ItemEnum::Impl(implementation),
            name: Some(name),
            ..
        }) = rustdoc.index.get(item)
            && *name == **impl_name
        {
            return Ok(implementation);
        }
    }
    anyhow::bail!("failed to find implementation {impl_name}");
}
