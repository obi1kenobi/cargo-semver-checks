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
            let query: SemverQuery = ron::from_str(query_text).unwrap_or_else(|e| {
                panic!(
                    "\
Failed to parse a query: {e}
```ron
{query_text}
```"
                );
            });
            let id_conflict = queries.insert(query.id.clone(), query);
            assert!(id_conflict.is_none(), "{id_conflict:?}");
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
        let path = format!("./localdata/test_data/{crate_pair}/{crate_version}/rustdoc.json");
        load_rustdoc(Path::new(&path))
            .with_context(|| format!("Could not load {path} file, did you forget to run ./scripts/regenerate_test_rustdocs.sh ?"))
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

    type TestOutput = BTreeMap<String, Vec<BTreeMap<String, FieldValue>>>;

    fn pretty_format_output_difference(
        query_name: &str,
        output_name1: String,
        output1: TestOutput,
        output_name2: String,
        output2: TestOutput,
    ) -> String {
        let results_to_string = |name, results| {
            format!(
                "{name}:\n{}",
                ron::ser::to_string_pretty(&results, ron::ser::PrettyConfig::default()).unwrap()
            )
        };
        vec![
            format!("Query {query_name} produced incorrect output (./src/lints/{query_name}.ron)."),
            results_to_string(output_name1, &output1),
            results_to_string(output_name2, &output2),
            "Note that the individual outputs might have been deliberately reordered.".to_string(),
            "Also, remember about running ./scripts/regenerate_test_rustdocs.sh when needed."
                .to_string(),
        ]
        .join("\n\n")
    }

    fn run_query_on_crate_pair(
        semver_query: &SemverQuery,
        crate_pair_name: &String,
        indexed_crate_new: &VersionedIndexedCrate,
        indexed_crate_old: &VersionedIndexedCrate,
    ) -> (String, Vec<BTreeMap<String, FieldValue>>) {
        let adapter = VersionedRustdocAdapter::new(indexed_crate_new, Some(indexed_crate_old))
            .expect("could not create adapter");
        let results_iter = adapter
            .run_query(&semver_query.query, semver_query.arguments.clone())
            .unwrap();
        (
            format!("./test_crates/{crate_pair_name}/"),
            results_iter
                .map(|res| res.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
                .collect::<Vec<BTreeMap<_, _>>>(),
        )
    }

    fn assert_no_false_positives_in_nonchanged_crate(
        query_name: &str,
        semver_query: &SemverQuery,
        indexed_crate: &VersionedIndexedCrate,
        crate_pair_name: &String,
        crate_version: &str,
    ) {
        let (crate_pair_path, output) =
            run_query_on_crate_pair(semver_query, crate_pair_name, indexed_crate, indexed_crate);
        if !output.is_empty() {
            // This `if` statement means that a false positive happened.
            // The query was ran on two identical crates (with the same rustdoc)
            // and it produced a non-empty output, which means that it found issues
            // in a crate pair that definitely has no semver breaks.

            let output_difference = pretty_format_output_difference(
                query_name,
                "Expected output (empty output)".to_string(),
                BTreeMap::new(),
                format!("Actual output ({crate_pair_name}/{crate_version})"),
                BTreeMap::from([(crate_pair_path, output)]),
            );
            panic!("The query produced a non-empty output when it compared two crates with the same rustdoc.\n{output_difference}\n");
        }
    }

    pub(in crate::query) fn check_query_execution(query_name: &str) {
        let query_text = std::fs::read_to_string(format!("./src/lints/{query_name}.ron")).unwrap();
        let semver_query: SemverQuery = ron::from_str(&query_text).unwrap();

        let expected_result_text =
            std::fs::read_to_string(format!("./test_outputs/{query_name}.output.ron"))
            .with_context(|| format!("Could not load test_outputs/{query_name}.output.ron expected-outputs file, did you forget to add it?"))
            .expect("failed to load expected outputs");
        let mut expected_results: TestOutput = ron::from_str(&expected_result_text)
            .expect("could not parse expected outputs as ron format");

        let mut actual_results: TestOutput = get_test_crate_names()
            .into_iter()
            .map(|crate_pair_name| {
                let crate_new = load_pregenerated_rustdoc(&crate_pair_name, "new");
                let crate_old = load_pregenerated_rustdoc(&crate_pair_name, "old");
                let indexed_crate_new = VersionedIndexedCrate::new(&crate_new);
                let indexed_crate_old = VersionedIndexedCrate::new(&crate_old);

                assert_no_false_positives_in_nonchanged_crate(
                    query_name,
                    &semver_query,
                    &indexed_crate_new,
                    &crate_pair_name,
                    "new",
                );
                assert_no_false_positives_in_nonchanged_crate(
                    query_name,
                    &semver_query,
                    &indexed_crate_old,
                    &crate_pair_name,
                    "old",
                );

                run_query_on_crate_pair(
                    &semver_query,
                    &crate_pair_name,
                    &indexed_crate_new,
                    &indexed_crate_old,
                )
            })
            .filter(|(_crate_pair_name, output)| !output.is_empty())
            .collect();

        // Reorder both vectors of results into a deterministic order that will compensate for
        // nondeterminism in how the results are ordered.
        let sort_individual_outputs = |results: &mut TestOutput| {
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
            panic!(
                "\n{}\n",
                pretty_format_output_difference(
                    query_name,
                    format!("Expected output (./test_outputs/{query_name}.output.ron)"),
                    expected_results,
                    "Actual output".to_string(),
                    actual_results
                )
            );
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

        // No way to avoid this lint -- the push() calls are macro-generated.
        #[allow(clippy::vec_init_then_push)]
        fn get_query_text_contents() -> Vec<&'static str> {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(
                    include_str!(concat!("lints/", stringify!($name), ".ron"))
                );
            )*
            temp_vec
        }
    }
}

add_lints!(
    auto_trait_impl_removed,
    constructible_struct_adds_field,
    constructible_struct_adds_private_field,
    derive_trait_impl_removed,
    enum_marked_non_exhaustive,
    enum_missing,
    enum_repr_c_removed,
    enum_repr_int_changed,
    enum_repr_int_removed,
    enum_struct_variant_field_added,
    enum_struct_variant_field_missing,
    enum_variant_added,
    enum_variant_missing,
    function_const_removed,
    function_missing,
    function_parameter_count_changed,
    function_unsafe_added,
    inherent_method_const_removed,
    inherent_method_missing,
    inherent_method_unsafe_added,
    method_parameter_count_changed,
    sized_impl_removed,
    struct_marked_non_exhaustive,
    struct_missing,
    struct_must_use_added,
    struct_pub_field_missing,
    struct_repr_c_removed,
    struct_repr_transparent_removed,
    trait_missing,
    trait_unsafe_added,
    trait_unsafe_removed,
    tuple_struct_to_plain_struct,
    unit_struct_changed_kind,
    variant_marked_non_exhaustive,
);
