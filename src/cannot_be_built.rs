use std::{cell::RefCell, collections::BTreeMap, rc::Rc, sync::Arc};

use anyhow::bail;
use itertools::{Itertools, EitherOrBoth};
use rustdoc_types::Crate;
use trustfall_core::{
    frontend::parse, interpreter::execution::interpret_ir, ir::FieldValue, schema::Schema,
};

use crate::{adapter::RustdocAdapter, GlobalConfig};

#[derive(Debug, Clone, Copy)]
enum ItemKind {
    Struct,
    Enum,
}

fn run_query<'a>(
    schema: &Schema,
    adapter: Rc<RefCell<RustdocAdapter<'a>>>,
    query: &str,
    args: BTreeMap<&str, FieldValue>,
) -> anyhow::Result<impl Iterator<Item = BTreeMap<Arc<str>, FieldValue>> + 'a> {
    let parsed_query = parse(schema, query)?;
    let arguments = Arc::new(
        args.into_iter()
            .map(|(k, v)| (Arc::from(k.to_string()), v))
            .collect(),
    );

    interpret_ir(adapter, parsed_query, arguments).map_err(|e| e.into())
}

fn is_struct_or_enum(
    schema: &Schema,
    adapter: Rc<RefCell<RustdocAdapter>>,
    item_path: &str,
) -> anyhow::Result<ItemKind> {
    let components: Vec<_> = item_path.split("::").collect();
    let struct_query = r#"
    {
        Crate {
            item {
                ... on Struct {
                    name @filter(op: "=", value: ["$name"]) @output

                    path {
                        path @filter(op: "=", value: ["$path"])
                    }
                }
            }
        }
    }
"#;
    let mut struct_args = BTreeMap::new();
    struct_args.insert("name", components.last().copied().unwrap().into());
    struct_args.insert("path", components.clone().into());

    if run_query(schema, adapter.clone(), struct_query, struct_args)?
        .next()
        .is_some()
    {
        return Ok(ItemKind::Struct);
    }

    let enum_query = r#"
    {
        Crate {
            item {
                ... on Struct {
                    name @filter(op: "=", value: ["$name"])

                    path {
                        path @filter(op: "=", value: ["$path"])
                    }
                }
            }
        }
    }
"#;
    let mut enum_args = BTreeMap::new();
    enum_args.insert("name", components.last().copied().unwrap().into());
    enum_args.insert("path", components.into());

    if run_query(schema, adapter, enum_query, enum_args)?
        .next()
        .is_some()
    {
        return Ok(ItemKind::Enum);
    }

    Err(anyhow::anyhow!(
        "The struct or enum to be checked was not found at path {item_path}"
    ))
}

#[derive(Debug, Clone)]
enum Visibility {
    Public,
    Restricted(Vec<String>),
}

impl Visibility {
    fn is_visible_from(&self, path: &[&str]) -> bool {
        match self {
            Self::Public => true,
            Self::Restricted(visible_from) => {
                for element in visible_from.iter().zip_longest(path.iter().copied()) {
                    match element {
                        EitherOrBoth::Both(l, r) => {
                            if l.as_str() != r {
                                return false;
                            }
                        }
                        EitherOrBoth::Left(..) => return false,
                        EitherOrBoth::Right(..) => return true,
                    }
                }

                true
            }
        }
    }
}

fn parse_visibility(crate_name: &str, visibility: &str) -> Visibility {
    match visibility {
        "public" => Visibility::Public,
        "crate" => Visibility::Restricted(vec![crate_name.to_string()]),
        vis if vis.starts_with("restricted (") && vis.ends_with(')') => {
            let path_content = vis
                .trim_start_matches("restricted (")
                .trim_start_matches("::")
                .trim_end_matches(')');
            Visibility::Restricted(
                [crate_name.to_string()]
                    .into_iter()
                    .chain(path_content.split("::").map(str::to_string))
                    .collect(),
            )
        }
        _ => unreachable!("{}", visibility),
    }
}

fn is_externally_visible(
    schema: &Schema,
    adapter: Rc<RefCell<RustdocAdapter>>,
    item_path: &str,
    boundary_module: &str,
) -> anyhow::Result<bool> {
    let item_components: Vec<_> = item_path.split("::").collect();
    let boundary_components: Vec<_> = boundary_module.split("::").collect();
    let query_text = r#"
    {
        Crate {
            item {
                name @filter(op: "=", value: ["$name"])
                visibility: visibility_limit @output

                path {
                    path @filter(op: "=", value: ["$path"])
                }
            }
        }
    }
"#;
    let mut args = BTreeMap::new();
    args.insert("name", item_components.last().copied().unwrap().into());
    args.insert("path", item_components.clone().into());

    let result = run_query(schema, adapter.clone(), query_text, args)?
        .next()
        .expect("could not find item");

    let crate_name = *item_components.first().unwrap();
    let visibility = parse_visibility(crate_name, result["visibility"].as_str().unwrap());
    if !visibility.is_visible_from(&boundary_components) {
        println!("item visibility {visibility:?} is not visible from {boundary_module}");
        return Ok(false);
    }

    Ok(true)
}

fn is_externally_constructible_with_struct_literal(
    schema: &Schema,
    adapter: Rc<RefCell<RustdocAdapter>>,
    struct_path: &str,
    boundary_module: &str,
) -> anyhow::Result<bool> {
    let item_components: Vec<_> = struct_path.split("::").collect();
    let boundary_components: Vec<_> = boundary_module.split("::").collect();
    let query_text = r#"
    {
        Crate {
            item {
                ... on Struct {
                    name @filter(op: "=", value: ["$name"])

                    path {
                        path @filter(op: "=", value: ["$path"])
                    }

                    # look for non-public fields
                    field_: field @fold {
                        names: name @output
                        visibilities: visibility_limit @filter(op: "!=", value: ["$public"]) @output
                    }
                }
            }
        }
    }
"#;
    let mut args = BTreeMap::new();
    args.insert("name", item_components.last().copied().unwrap().into());
    args.insert("path", item_components.clone().into());
    args.insert("public", "public".into());

    let result = run_query(schema, adapter.clone(), query_text, args)?
        .next()
        .expect("could not find struct");

    let crate_name = *item_components.first().unwrap();

    let field_names: Vec<_> = match &result["field_names"] {
        FieldValue::List(l) => l.iter().map(|v| v.as_str().unwrap()).collect(),
        _ => unreachable!(),
    };
    let field_visibilities: Vec<_> = match &result["field_visibilities"] {
        FieldValue::List(l) => l.iter().map(|v| parse_visibility(crate_name, v.as_str().unwrap())).collect(),
        _ => unreachable!(),
    };
    assert_eq!(field_names.len(), field_visibilities.len());

    for (field_name, visibility) in field_names.iter().copied().zip(field_visibilities.iter()) {
        if !visibility.is_visible_from(&boundary_components) {
            println!(
                "struct field {field_name} is not visible from {boundary_module}, \
                 so this struct is not constructible with a literal from there."
            );
            return Ok(false);
        }
    }

    // If the boundary is not in the item's crate, check for #[non_exhaustive] on the struct.
    if boundary_components.first().unwrap() == item_components.first().unwrap() {
        let query_text = r#"
            {
                Crate {
                    item {
                        name @filter(op: "=", value: ["$name"])
                        visibility: visibility_limit @output
                        attrs @filter(op: "contains", value: ["$non_exhaustive"])

                        path {
                            path @filter(op: "=", value: ["$path"])
                        }
                    }
                }
            }"#;
        let mut args = BTreeMap::new();
        args.insert("name", item_components.last().copied().unwrap().into());
        args.insert("path", item_components.clone().into());
        args.insert("non_exhaustive", "#[non_exhaustive]".into());

        let mut result_iter = run_query(schema, adapter.clone(), query_text, args)?;
        if result_iter.next().is_some() {
            println!(
                "the struct is marked #[non_exhaustive], so it is not constructible with a literal \
                from outside its own crate."
            );
            return Ok(false);
        }
    }

    Ok(true)
}

fn handle_struct_cannot_be_built(
    _config: GlobalConfig,
    schema: &Schema,
    adapter: Rc<RefCell<RustdocAdapter>>,
    struct_check_path: &str,
    boundary_module: &str,
) -> anyhow::Result<()> {
    if !is_externally_constructible_with_struct_literal(schema, adapter.clone(), struct_check_path, boundary_module)? {
        return Ok(());
    }

    bail!(
        "Could not prove that item {struct_check_path} cannot be constructed from {boundary_module} \
         or outside it"
    );
}

pub(crate) fn handle_cannot_be_built(
    config: GlobalConfig,
    current_crate: Crate,
    check_path: &str,
    boundary_module: &str,
) -> anyhow::Result<()> {
    let schema = RustdocAdapter::schema();
    let adapter = Rc::new(RefCell::new(RustdocAdapter::new(&current_crate, None)));

    if !is_externally_visible(&schema, adapter.clone(), check_path, boundary_module)? {
        return Ok(());
    }

    let item_kind = is_struct_or_enum(&schema, adapter.clone(), check_path)?;
    println!("item_kind = {item_kind:?}");

    match item_kind {
        ItemKind::Struct => {
            handle_struct_cannot_be_built(config, &schema, adapter, check_path, boundary_module)
        }
        ItemKind::Enum => todo!(),
    }
}
