use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use trustfall::TransparentValue;

use crate::ReleaseType;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequiredSemverUpdate {
    Major,
    Minor,
}

impl RequiredSemverUpdate {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Major => "major",
            Self::Minor => "minor",
        }
    }
}

impl From<RequiredSemverUpdate> for ReleaseType {
    fn from(value: RequiredSemverUpdate) -> Self {
        match value {
            RequiredSemverUpdate::Major => Self::Major,
            RequiredSemverUpdate::Minor => Self::Minor,
        }
    }
}

/// Kind of semver update.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActualSemverUpdate {
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

impl From<ReleaseType> for ActualSemverUpdate {
    fn from(value: ReleaseType) -> Self {
        match value {
            ReleaseType::Major => Self::Major,
            ReleaseType::Minor => Self::Minor,
            ReleaseType::Patch => Self::Patch,
        }
    }
}

/// A query that can be executed on a pair of rustdoc output files,
/// returning instances of a particular kind of semver violation.
#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemverQuery {
    pub id: String,

    pub(crate) human_readable_name: String,

    pub description: String,

    pub required_update: RequiredSemverUpdate,

    #[serde(default)]
    pub reference: Option<String>,

    #[serde(default)]
    pub reference_link: Option<String>,

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
    pub fn all_queries() -> BTreeMap<String, SemverQuery> {
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
    use trustfall::{FieldValue, TransparentValue};
    use trustfall_rustdoc::{
        load_rustdoc, VersionedCrate, VersionedIndexedCrate, VersionedRustdocAdapter,
    };

    use crate::query::SemverQuery;
    use crate::templating::make_handlebars_registry;

    lazy_static::lazy_static! {
        static ref TEST_CRATE_NAMES: Vec<String> = get_test_crate_names();

        /// Mapping test crate (pair) name -> (old rustdoc, new rustdoc).
        static ref TEST_CRATE_RUSTDOCS: BTreeMap<String, (VersionedCrate, VersionedCrate)> =
            get_test_crate_rustdocs();
    }

    fn get_test_crate_names() -> Vec<String> {
        std::fs::read_dir("./test_crates/")
            .expect("directory test_crates/ not found")
            .map(|dir_entry| dir_entry.expect("failed to list test_crates/"))
            .filter(|dir_entry| {
                // Only return directories inside `test_crates/` that contain
                // an `old/Cargo.toml` file. This works around finicky git + cargo behavior:
                // - Create a git branch, commit a new test case, and generate its rustdoc.
                // - Cargo will then create `Cargo.lock` files for the crate,
                //   which are ignored by git.
                // - Check out another branch, and git won't delete the `Cargo.lock` files
                //   since they aren't tracked. But we don't want to run tests on those crates!
                if !dir_entry
                    .metadata()
                    .expect("failed to retrieve test_crates/* metadata")
                    .is_dir()
                {
                    return false;
                }

                let mut test_crate_cargo_toml = dir_entry.path();
                test_crate_cargo_toml.extend(["old", "Cargo.toml"]);
                test_crate_cargo_toml.as_path().is_file()
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

    fn get_test_crate_rustdocs() -> BTreeMap<String, (VersionedCrate, VersionedCrate)> {
        TEST_CRATE_NAMES
            .iter()
            .map(|crate_pair| {
                let old_rustdoc = load_pregenerated_rustdoc(crate_pair.as_str(), "old");
                let new_rustdoc = load_pregenerated_rustdoc(crate_pair, "new");

                (crate_pair.clone(), (old_rustdoc, new_rustdoc))
            })
            .collect()
    }

    fn load_pregenerated_rustdoc(crate_pair: &str, crate_version: &str) -> VersionedCrate {
        let path = format!("./localdata/test_data/{crate_pair}/{crate_version}/rustdoc.json");
        load_rustdoc(Path::new(&path))
            .with_context(|| format!("Could not load {path} file, did you forget to run ./scripts/regenerate_test_rustdocs.sh ?"))
            .expect("failed to load baseline rustdoc")
    }

    #[test]
    fn all_queries_are_valid() {
        let (_baseline_crate, current_crate) = &TEST_CRATE_RUSTDOCS["template"];
        let indexed_crate = VersionedIndexedCrate::new(current_crate);

        let adapter = VersionedRustdocAdapter::new(&indexed_crate, Some(&indexed_crate))
            .expect("failed to create adapter");
        for semver_query in SemverQuery::all_queries().into_values() {
            let _ = adapter
                .run_query(&semver_query.query, semver_query.arguments)
                .expect("not a valid query");
        }
    }

    #[test]
    fn pub_use_handling() {
        let (_baseline_crate, current_crate) = &TEST_CRATE_RUSTDOCS["pub_use_handling"];
        let current = VersionedIndexedCrate::new(current_crate);

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
            .as_vec_with(|val| val.as_vec_with(FieldValue::as_str))
            .expect("not a Vec<Vec<&str>>");
        actual_paths.sort_unstable();

        let expected_paths = vec![
            vec!["pub_use_handling", "CheckPubUseHandling"],
            vec!["pub_use_handling", "inner", "CheckPubUseHandling"],
        ];
        assert_eq!(expected_paths, actual_paths);
    }

    type TestOutput = BTreeMap<String, Vec<BTreeMap<String, FieldValue>>>;

    fn pretty_format_output_difference(
        query_name: &str,
        output_name1: &'static str,
        output1: TestOutput,
        output_name2: &'static str,
        output2: TestOutput,
    ) -> String {
        let output_ron1 =
            ron::ser::to_string_pretty(&output1, ron::ser::PrettyConfig::default()).unwrap();
        let output_ron2 =
            ron::ser::to_string_pretty(&output2, ron::ser::PrettyConfig::default()).unwrap();
        let diff = similar_asserts::SimpleDiff::from_str(
            &output_ron1,
            &output_ron2,
            output_name1,
            output_name2,
        );
        [
            format!("Query {query_name} produced incorrect output (./src/lints/{query_name}.ron)."),
            diff.to_string(),
            "Remember that result output order matters, and remember to re-run \
            ./scripts/regenerate_test_rustdocs.sh when needed."
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
            let actual_output_name = Box::leak(Box::new(format!(
                "actual ({crate_pair_name}/{crate_version})"
            )));
            let output_difference = pretty_format_output_difference(
                query_name,
                "expected (empty)",
                BTreeMap::new(),
                actual_output_name,
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

        let mut actual_results: TestOutput = TEST_CRATE_NAMES
            .iter()
            .map(|crate_pair_name| {
                let (crate_old, crate_new) = &TEST_CRATE_RUSTDOCS[crate_pair_name];
                let indexed_crate_old = VersionedIndexedCrate::new(crate_old);
                let indexed_crate_new = VersionedIndexedCrate::new(crate_new);

                assert_no_false_positives_in_nonchanged_crate(
                    query_name,
                    &semver_query,
                    &indexed_crate_new,
                    crate_pair_name,
                    "new",
                );
                assert_no_false_positives_in_nonchanged_crate(
                    query_name,
                    &semver_query,
                    &indexed_crate_old,
                    crate_pair_name,
                    "old",
                );

                run_query_on_crate_pair(
                    &semver_query,
                    crate_pair_name,
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
                let filename = elem.get("span_filename").and_then(|value| value.as_str());
                let line = elem.get("span_begin_line");

                match (filename, line) {
                    (Some(filename), Some(line)) => (filename.to_owned(), line.as_usize()),
                    (Some(_filename), _) => panic!("A valid query must output `span_filename`. See https://github.com/obi1kenobi/cargo-semver-checks/blob/main/CONTRIBUTING.md for details."),
                    (_, Some(_line)) => panic!("A valid query must output `span_begin_line`. See https://github.com/obi1kenobi/cargo-semver-checks/blob/main/CONTRIBUTING.md for details."),
                    _ => panic!("A valid query must output both `span_filename` and `span_begin_line`. See https://github.com/obi1kenobi/cargo-semver-checks/blob/main/CONTRIBUTING.md for details."),
                }
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
                    "expected",
                    expected_results,
                    "actual",
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
    ($($name:ident,)+) => {
        #[cfg(test)]
        mod tests_lints {
            $(
                #[test]
                fn $name() {
                    super::tests::check_query_execution(stringify!($name))
                }
            )*
        }

        fn get_query_text_contents() -> Vec<&'static str> {
            vec![
                $(
                    include_str!(concat!("lints/", stringify!($name), ".ron")),
                )*
            ]
        }
    };
    ($($name:ident),*) => {
        compile_error!("Please add a trailing comma after each lint identifier. This ensures our scripts like 'make_new_lint.sh' can safely edit invocations of this macro as needed.");
    }
}

add_lints!(
    auto_trait_impl_removed,
    constructible_struct_adds_field,
    constructible_struct_adds_private_field,
    constructible_struct_changed_type,
    derive_trait_impl_removed,
    enum_marked_non_exhaustive,
    enum_missing,
    enum_must_use_added,
    enum_now_doc_hidden,
    enum_repr_c_removed,
    enum_repr_int_changed,
    enum_repr_int_removed,
    enum_repr_transparent_removed,
    enum_struct_variant_field_added,
    enum_struct_variant_field_missing,
    enum_variant_added,
    enum_variant_missing,
    function_const_removed,
    function_missing,
    function_must_use_added,
    function_now_doc_hidden,
    function_parameter_count_changed,
    function_unsafe_added,
    inherent_method_const_removed,
    inherent_method_missing,
    inherent_method_must_use_added,
    inherent_method_unsafe_added,
    method_parameter_count_changed,
    sized_impl_removed,
    struct_marked_non_exhaustive,
    struct_missing,
    struct_must_use_added,
    struct_now_doc_hidden,
    struct_pub_field_missing,
    struct_repr_c_removed,
    struct_repr_transparent_removed,
    struct_with_pub_fields_changed_type,
    trait_method_missing,
    trait_missing,
    trait_must_use_added,
    trait_now_doc_hidden,
    trait_unsafe_added,
    trait_unsafe_removed,
    tuple_struct_to_plain_struct,
    type_marked_deprecated,
    unit_struct_changed_kind,
    variant_marked_non_exhaustive,
    enum_tuple_variant_field_missing,
    enum_tuple_variant_field_added,
    trait_removed_supertrait,
    pub_module_level_const_missing,
    pub_static_missing,
    trait_removed_associated_type,
    module_missing,
    trait_removed_associated_constant,
    function_changed_abi,
    trait_method_unsafe_added,
    trait_method_unsafe_removed,
    struct_pub_field_now_doc_hidden,
);
