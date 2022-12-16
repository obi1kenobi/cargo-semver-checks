use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use trustfall_core::ir::TransparentValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum RequiredSemverUpdate {
    Major,
    Minor,
}

impl RequiredSemverUpdate {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ActualSemverUpdate {
    Major,
    Minor,
    Patch,
    NotChanged,
}

impl ActualSemverUpdate {
    pub(crate) fn supports_requirement(&self, required: RequiredSemverUpdate) -> bool {
        match (*self, required) {
            (ActualSemverUpdate::Major, _) => true,
            (ActualSemverUpdate::Minor, RequiredSemverUpdate::Major) => false,
            (ActualSemverUpdate::Minor, _) => true,
            (_, _) => false,
        }
    }
}

/// A query that can be executed on a pair of rustdoc output files,
/// returning instances of a particular kind of semver violation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SemverQuery {
    pub(crate) id: String,

    pub(crate) human_readable_name: String,

    pub(crate) description: String,

    pub(crate) required_update: RequiredSemverUpdate,

    #[serde(default)]
    pub(crate) reference: Option<String>,

    #[serde(default)]
    pub(crate) reference_link: Option<String>,

    pub(crate) query: String,

    #[serde(default)]
    pub(crate) arguments: BTreeMap<String, TransparentValue>,

    /// The top-level error describing the semver violation that was detected.
    /// Even if multiple instances of this semver issue are found, this error
    /// message is displayed only at most once.
    pub(crate) error_message: String,

    /// Optional template that can be combined with each query output to produce
    /// a human-readable description of the specific semver violation that was discovered.
    #[serde(default)]
    pub(crate) per_result_error_template: Option<String>,
}

impl SemverQuery {
    pub(crate) fn all_queries() -> BTreeMap<String, SemverQuery> {
        let mut queries = BTreeMap::default();
        for query_text in get_query_text_contents() {
            let query: SemverQuery = ron::from_str(&query_text).unwrap_or_else(|e| {
                panic!(
                    "\
Failed to parse a query: {}
```ron
{}
```",
                    e, query_text
                );
            });
            let id_conflict = queries.insert(query.id.clone(), query);
            assert!(id_conflict.is_none(), "{:?}", id_conflict);
        }

        queries
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::BTreeMap, path::Path};

    use anyhow::Context;
    use trustfall_core::ir::TransparentValue;
    use trustfall_core::{frontend::parse, ir::FieldValue};
    use trustfall_rustdoc::{
        load_rustdoc, VersionedCrate, VersionedIndexedCrate, VersionedRustdocAdapter,
    };

    use crate::query::SemverQuery;
    use crate::templating::make_handlebars_registry;

    fn load_pregenerated_rustdoc(crate_pair: &str, crate_version: &str) -> VersionedCrate {
        let path = format!(
            "./localdata/test_data/{}/{}/rustdoc.json",
            crate_pair, crate_version
        );
        load_rustdoc(Path::new(&path))
            .with_context(|| format!("Could not load {} file, did you forget to run ./scripts/regenerate_test_rustdocs.sh ?", path))
            .expect("failed to load baseline rustdoc")
    }

    #[test]
    fn all_queries_parse_correctly() {
        let current_crate = load_pregenerated_rustdoc("template", "new");
        let indexed_crate = VersionedIndexedCrate::new(&current_crate);
        let adapter =
            VersionedRustdocAdapter::new(&indexed_crate, None).expect("failed to create adapter");

        let schema = adapter.schema();
        for semver_query in SemverQuery::all_queries().into_values() {
            let _ = parse(schema, semver_query.query).expect("not a valid query");
        }
    }

    #[test]
    fn pub_use_handling() {
        let current_crate = load_pregenerated_rustdoc("pub_use_handling", "new");
        let current = VersionedIndexedCrate::new(&current_crate);

        let query = r#"
            {
                Crate {
                    item {
                        ... on Struct {
                            name @filter(op: "=", value: ["$struct"])

                            canonical_path {
                                canonical_path: path @output
                            }

                            importable_path @fold {
                                path @output
                            }
                        }
                    }
                }
            }"#;
        let mut arguments = BTreeMap::new();
        arguments.insert("struct", "CheckPubUseHandling");

        let adapter =
            VersionedRustdocAdapter::new(&current, None).expect("could not create adapter");

        let results_iter = adapter
            .run_query(query, arguments)
            .expect("failed to run query");
        let actual_results: Vec<BTreeMap<_, _>> = results_iter
            .map(|res| res.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
            .collect();

        let expected_result: FieldValue =
            vec!["pub_use_handling", "inner", "CheckPubUseHandling"].into();
        assert_eq!(1, actual_results.len(), "{actual_results:?}");
        assert_eq!(
            expected_result, actual_results[0]["canonical_path"],
            "{actual_results:?}"
        );

        let mut actual_paths = actual_results[0]["path"]
            .as_vec(|val| val.as_vec(FieldValue::as_str))
            .expect("not a Vec<Vec<&str>>");
        actual_paths.sort_unstable();

        let expected_paths = vec![
            vec!["pub_use_handling", "CheckPubUseHandling"],
            vec!["pub_use_handling", "inner", "CheckPubUseHandling"],
        ];
        assert_eq!(expected_paths, actual_paths);
    }

    fn get_test_crate_names() -> Vec<String> {
        std::fs::read_dir("./test_crates/")
            .expect("directory test_crates/ not found")
            .map(|dir_entry| dir_entry.expect("failed to list test_crates/"))
            .filter(|dir_entry| {
                dir_entry
                    .metadata()
                    .expect("failed to retrieve test_crates/* metadata")
                    .is_dir()
            })
            .map(|dir_entry| {
                String::from(
                    String::from(
                        dir_entry
                            .path()
                            .to_str()
                            .expect("failed to convert dir_entry to String"),
                    )
                    .strip_prefix("./test_crates/")
                    .expect(
                        "the dir_entry doesn't start with './test_crates/', which is unexpected",
                    ),
                )
            })
            .collect()
    }

    pub(in crate::query) fn check_query_execution(query_name: &str) {
        let query_text = std::fs::read_to_string(format!("./src/lints/{query_name}.ron")).unwrap();
        let semver_query: SemverQuery = ron::from_str(&query_text).unwrap();

        let expected_result_text =
            std::fs::read_to_string(format!("./test_outputs/{query_name}.output.ron"))
            .with_context(|| format!("Could not load test_outputs/{}.output.ron expected-outputs file, did you forget to add it?", query_name))
            .expect("failed to load expected outputs");
        let mut expected_results: BTreeMap<String, Vec<BTreeMap<String, FieldValue>>> =
            ron::from_str(&expected_result_text)
                .expect("could not parse expected outputs as ron format");

        let mut actual_results: BTreeMap<String, Vec<BTreeMap<_, _>>> = get_test_crate_names()
            .into_iter()
            .map(|crate_pair| {
                let crate_new = load_pregenerated_rustdoc(&crate_pair, "new");
                let crate_old = load_pregenerated_rustdoc(&crate_pair, "old");
                let indexed_crate_new = VersionedIndexedCrate::new(&crate_new);
                let indexed_crate_old = VersionedIndexedCrate::new(&crate_old);

                let adapter =
                    VersionedRustdocAdapter::new(&indexed_crate_new, Some(&indexed_crate_old))
                        .expect("could not create adapter");
                let results_iter = adapter
                    .run_query(&semver_query.query, semver_query.arguments.clone())
                    .unwrap();
                (
                    format!("./test_crates/{}/", crate_pair),
                    results_iter
                        .map(|res| res.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
                        .collect::<Vec<BTreeMap<_, _>>>(),
                )
            })
            .filter(|(_crate_pair, output)| !output.is_empty())
            .collect();

        // Reorder both vectors of results into a deterministic order that will compensate for
        // nondeterminism in how the results are ordered.
        let sort_individual_outputs = |results: &mut BTreeMap<String, Vec<_>>| {
            let key_func = |elem: &BTreeMap<String, FieldValue>| {
                (
                    elem["span_filename"].as_str().unwrap().to_owned(),
                    elem["span_begin_line"].as_usize().unwrap(),
                )
            };
            for value in results.values_mut() {
                value.sort_unstable_by_key(key_func);
            }
        };
        sort_individual_outputs(&mut expected_results);
        sort_individual_outputs(&mut actual_results);

        if expected_results != actual_results {
            let results_to_string = |name, results| {
                format!(
                    "{}:\n{}",
                    name,
                    ron::ser::to_string_pretty(&results, ron::ser::PrettyConfig::default())
                        .unwrap()
                )
            };
            let messages = vec![
                format!("Query {} produced incorrect output (./src/lints/{}.ron).", &query_name, &query_name),
                results_to_string(format!("Expected output (./test_outputs/{}.output.ron)", &query_name), &expected_results),
                results_to_string("Actual output".to_string(), &actual_results),
                "Note that the individual outputs might have been deliberately reordered. Also, remember about running ./scripts/regenerate_test_rustdocs.sh when needed.".to_string(),
            ];
            panic!("\n{}\n", messages.join("\n\n"));
        }

        let registry = make_handlebars_registry();
        if let Some(template) = semver_query.per_result_error_template {
            assert!(!actual_results.is_empty());

            let flattened_actual_results: Vec<_> = actual_results
                .into_iter()
                .flat_map(|(_key, value)| value)
                .collect();
            for semver_violation_result in flattened_actual_results {
                let pretty_result: BTreeMap<String, TransparentValue> = semver_violation_result
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect();

                registry
                    .render_template(&template, &pretty_result)
                    .with_context(|| "Error instantiating semver query template.")
                    .expect("could not materialize template");
            }
        }
    }
}

macro_rules! add_lints {
    ($($name:ident,)*) => {
        #[cfg(test)]
        mod tests_lints {
            $(
                #[test]
                fn $name() {
                    super::tests::check_query_execution(stringify!($name))
                }
            )*
        }

        fn get_query_text_contents() -> Vec<String> {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(
                    std::fs::read_to_string(
                        format!("./src/lints/{}.ron", stringify!($name))
                    ).unwrap()
                );
            )*
            temp_vec
        }
    }
}

add_lints!(
    auto_trait_impl_removed,
    derive_trait_impl_removed,
    enum_marked_non_exhaustive,
    enum_missing,
    enum_repr_c_removed,
    enum_repr_int_changed,
    enum_repr_int_removed,
    enum_variant_added,
    enum_variant_missing,
    enum_struct_variant_field_missing,
    function_missing,
    function_parameter_count_changed,
    inherent_method_missing,
    method_parameter_count_changed,
    sized_impl_removed,
    struct_marked_non_exhaustive,
    struct_missing,
    struct_pub_field_missing,
    struct_repr_c_removed,
    struct_repr_transparent_removed,
    unit_struct_changed_kind,
    variant_marked_non_exhaustive,
);
