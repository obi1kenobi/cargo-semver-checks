use std::{collections::BTreeMap, sync::Arc};

use ron::extensions::Extensions;
use serde::{Deserialize, Serialize};
use trustfall::TransparentValue;

use crate::ReleaseType;

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RequiredSemverUpdate {
    #[serde(alias = "minor")]
    Minor,
    #[serde(alias = "major")]
    Major,
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

/// The level of intensity of the error when a lint occurs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum LintLevel {
    /// If this lint occurs, do nothing.
    #[serde(alias = "allow")]
    Allow,
    /// If this lint occurs, print a warning.
    #[serde(alias = "warn")]
    Warn,
    /// If this lint occurs, raise an error.
    #[serde(alias = "deny")]
    Deny,
}

impl LintLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            LintLevel::Allow => "allow",
            LintLevel::Warn => "warn",
            LintLevel::Deny => "deny",
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

    /// The default lint level for when this lint occurs.
    pub lint_level: LintLevel,

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
        for (id, query_text) in get_queries() {
            let mut deserializer = ron::Deserializer::from_str_with_options(
                query_text,
                ron::Options::default().with_default_extension(Extensions::IMPLICIT_SOME),
            )
            .expect("Failed to construct deserializer.");

            let query = Self::deserialize(&mut deserializer).unwrap_or_else(|e| {
                panic!(
                    "\
Failed to parse a query: {e}
```ron
{query_text}
```"
                );
            });
            assert_eq!(id, query.id, "Query id must match file name");
            let id_conflict = queries.insert(query.id.clone(), query);
            assert!(id_conflict.is_none(), "{id_conflict:?}");
        }

        queries
    }
}

/// Configured values for a [`SemverQuery`] that differ from the lint's defaults.
#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct QueryOverride {
    /// The required version bump for this lint; see [`SemverQuery`].`required_update`.
    ///
    /// If this is `None`, use the query's default `required_update` when calculating
    /// the effective required version bump.
    #[serde(default)]
    pub required_update: Option<RequiredSemverUpdate>,

    /// The lint level for this lint; see [`SemverQuery`].`lint_level`.
    ///
    /// If this is `None`, use the query's default `lint_level` when calculating
    /// the effective lint level.
    #[serde(default)]
    pub lint_level: Option<LintLevel>,
}

/// A mapping of lint ids to configured values that override that lint's defaults.
pub type OverrideMap = BTreeMap<String, QueryOverride>;

/// Stores a stack of [`OverrideMap`] references such that items towards the top of
/// the stack (later in the backing `Vec`) have *higher* precedence and override items lower in the stack.
/// That is, when an override is set and not `None` for a given lint in multiple maps in the stack, the value
/// at the top of the stack will be used to calculate the effective lint level or required version update.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct OverrideStack(Vec<Arc<OverrideMap>>);

impl OverrideStack {
    /// Creates a new, empty [`OverrideStack`] instance.
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Inserts the given element at the top of the stack.
    ///
    /// The inserted overrides will take precedence over any lower item in the stack,
    /// if both maps have a not-`None` entry for a given lint.
    pub fn push(&mut self, item: Arc<OverrideMap>) {
        self.0.push(item);
    }

    /// Calculates the *effective* lint level of this query, by searching for an override
    /// mapped to this query's id from the top of the stack first, returning the query's default
    /// lint level if not overridden.
    #[must_use]
    pub fn effective_lint_level(&self, query: &SemverQuery) -> LintLevel {
        self.0
            .iter()
            .rev()
            .find_map(|x| x.get(&query.id).and_then(|y| y.lint_level))
            .unwrap_or(query.lint_level)
    }

    /// Calculates the *effective* required version bump of this query, by searching for an override
    /// mapped to this query's id from the top of the stack first, returning the query's default
    /// required version bump if not overridden.
    #[must_use]
    pub fn effective_required_update(&self, query: &SemverQuery) -> RequiredSemverUpdate {
        self.0
            .iter()
            .rev()
            .find_map(|x| x.get(&query.id).and_then(|y| y.required_update))
            .unwrap_or(query.required_update)
    }
}

impl From<Vec<Arc<OverrideMap>>> for OverrideStack {
    fn from(value: Vec<Arc<OverrideMap>>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use std::collections::BTreeSet;
    use std::path::PathBuf;
    use std::sync::{Arc, OnceLock};
    use std::{collections::BTreeMap, path::Path};

    use anyhow::Context;
    use trustfall::{FieldValue, TransparentValue};
    use trustfall_rustdoc::{
        load_rustdoc, VersionedCrate, VersionedIndexedCrate, VersionedRustdocAdapter,
    };

    use crate::query::{
        LintLevel, OverrideStack, QueryOverride, RequiredSemverUpdate, SemverQuery,
    };
    use crate::templating::make_handlebars_registry;

    static TEST_CRATE_NAMES: OnceLock<Vec<String>> = OnceLock::new();

    /// Mapping test crate (pair) name -> (old rustdoc, new rustdoc).
    static TEST_CRATE_RUSTDOCS: OnceLock<BTreeMap<String, (VersionedCrate, VersionedCrate)>> =
        OnceLock::new();

    fn get_test_crate_names() -> &'static [String] {
        TEST_CRATE_NAMES.get_or_init(initialize_test_crate_names)
    }

    fn get_test_crate_rustdocs(test_crate: &str) -> &'static (VersionedCrate, VersionedCrate) {
        &TEST_CRATE_RUSTDOCS.get_or_init(initialize_test_crate_rustdocs)[test_crate]
    }

    fn initialize_test_crate_names() -> Vec<String> {
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

    fn initialize_test_crate_rustdocs() -> BTreeMap<String, (VersionedCrate, VersionedCrate)> {
        get_test_crate_names()
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
        let (_baseline_crate, current_crate) = get_test_crate_rustdocs("template");
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
        let (_baseline_crate, current_crate) = get_test_crate_rustdocs("pub_use_handling");
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

        let mut actual_results: TestOutput = get_test_crate_names()
            .iter()
            .map(|crate_pair_name| {
                let (crate_old, crate_new) = get_test_crate_rustdocs(crate_pair_name);
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

    /// Helper function to construct a blank query with a given id, lint level, and required
    /// version bump.
    #[must_use]
    fn make_blank_query(
        id: String,
        lint_level: LintLevel,
        required_update: RequiredSemverUpdate,
    ) -> SemverQuery {
        SemverQuery {
            id,
            lint_level,
            required_update,
            human_readable_name: String::new(),
            description: String::new(),
            reference: None,
            reference_link: None,
            query: String::new(),
            arguments: BTreeMap::new(),
            error_message: String::new(),
            per_result_error_template: None,
        }
    }

    #[test]
    fn test_overrides() {
        let mut stack = OverrideStack::new();
        stack.push(Arc::new(
            [
                (
                    "query1".into(),
                    QueryOverride {
                        lint_level: Some(LintLevel::Allow),
                        required_update: Some(RequiredSemverUpdate::Minor),
                    },
                ),
                (
                    "query2".into(),
                    QueryOverride {
                        lint_level: None,
                        required_update: Some(RequiredSemverUpdate::Minor),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        ));

        let q1 = make_blank_query(
            "query1".into(),
            LintLevel::Deny,
            RequiredSemverUpdate::Major,
        );
        let q2 = make_blank_query(
            "query2".into(),
            LintLevel::Warn,
            RequiredSemverUpdate::Major,
        );

        // Should pick overridden values.
        assert_eq!(stack.effective_lint_level(&q1), LintLevel::Allow);
        assert_eq!(
            stack.effective_required_update(&q1),
            RequiredSemverUpdate::Minor
        );

        // Should pick overridden value for semver and fall back to default lint level
        // which is not overridden
        assert_eq!(stack.effective_lint_level(&q2), LintLevel::Warn);
        assert_eq!(
            stack.effective_required_update(&q2),
            RequiredSemverUpdate::Minor
        );
    }

    #[test]
    fn test_override_precedence() {
        let mut stack = OverrideStack::new();
        stack.push(Arc::new(
            [
                (
                    "query1".into(),
                    QueryOverride {
                        lint_level: Some(LintLevel::Allow),
                        required_update: Some(RequiredSemverUpdate::Minor),
                    },
                ),
                (
                    "query2".into(),
                    QueryOverride {
                        lint_level: None,
                        required_update: Some(RequiredSemverUpdate::Minor),
                    },
                ),
            ]
            .into_iter()
            .collect(),
        ));

        stack.push(Arc::new(
            [(
                "query1".into(),
                QueryOverride {
                    required_update: None,
                    lint_level: Some(LintLevel::Warn),
                },
            )]
            .into_iter()
            .collect(),
        ));

        let q1 = make_blank_query(
            "query1".into(),
            LintLevel::Deny,
            RequiredSemverUpdate::Major,
        );
        let q2 = make_blank_query(
            "query2".into(),
            LintLevel::Warn,
            RequiredSemverUpdate::Major,
        );

        // Should choose overridden value at the top of the stack
        assert_eq!(stack.effective_lint_level(&q1), LintLevel::Warn);
        // Should fall back to a configured value lower in the stack because
        // top is not set.
        assert_eq!(
            stack.effective_required_update(&q1),
            RequiredSemverUpdate::Minor
        );

        // Should pick overridden value for semver and fall back to default lint level
        // which is not overridden
        assert_eq!(stack.effective_lint_level(&q2), LintLevel::Warn);
        assert_eq!(
            stack.effective_required_update(&q2),
            RequiredSemverUpdate::Minor
        );
    }

    pub(super) fn check_all_lint_files_are_used_in_add_lints(added_lints: &[&str]) {
        let mut lints_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        lints_dir.push("src");
        lints_dir.push("lints");

        let expected_lints: BTreeSet<_> = added_lints.iter().copied().collect();
        let mut missing_lints: BTreeSet<String> = Default::default();

        let dir_contents =
            fs_err::read_dir(lints_dir).expect("failed to read 'src/lints' directory");
        for file in dir_contents {
            let file = file.expect("failed to examine file");
            let path = file.path();

            // Check if we found a `*.ron` file. If so, that's a lint.
            if path.extension().map(|x| x.to_string_lossy()) == Some(Cow::Borrowed("ron")) {
                let stem = path
                    .file_stem()
                    .map(|x| x.to_string_lossy())
                    .expect("failed to get file name as utf-8");

                // Check if the lint was added using our `add_lints!()` macro.
                // If not, that's an error.
                if !expected_lints.contains(stem.as_ref()) {
                    missing_lints.insert(stem.to_string());
                }
            }
        }

        assert!(
            missing_lints.is_empty(),
            "some lints in 'src/lints/' haven't been registered using the `add_lints!()` macro, \
            so they won't be part of cargo-semver-checks: {missing_lints:?}"
        )
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

            #[test]
            fn all_lint_files_are_used_in_add_lints() {
                let added_lints = [
                    $(
                        stringify!($name),
                    )*
                ];

                super::tests::check_all_lint_files_are_used_in_add_lints(&added_lints);
            }
        }

        fn get_queries() -> Vec<(&'static str, &'static str)> {
            vec![
                $(
                    (
                        stringify!($name),
                        include_str!(concat!("lints/", stringify!($name), ".ron")),
                    ),
                )*
            ]
        }
    };
    ($($name:ident),*) => {
        compile_error!("Please add a trailing comma after each lint identifier. This ensures our scripts like 'make_new_lint.sh' can safely edit invocations of this macro as needed.");
    }
}

// The following add_lints! invocation is programmatically edited by scripts/make_new_lint.sh
// If you must manually edit it, be sure to read the "Requirements" comments in that script first
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
    enum_repr_int_changed,
    enum_repr_int_removed,
    enum_repr_transparent_removed,
    enum_struct_variant_field_added,
    enum_struct_variant_field_missing,
    enum_struct_variant_field_now_doc_hidden,
    enum_tuple_variant_changed_kind,
    enum_tuple_variant_field_added,
    enum_tuple_variant_field_missing,
    enum_tuple_variant_field_now_doc_hidden,
    enum_unit_variant_changed_kind,
    enum_variant_added,
    enum_variant_discriminant_changed,
    enum_variant_marked_non_exhaustive,
    enum_variant_missing,
    exported_function_changed_abi,
    function_abi_no_longer_unwind,
    function_changed_abi,
    function_const_removed,
    function_export_name_changed,
    function_missing,
    function_must_use_added,
    function_now_doc_hidden,
    function_parameter_count_changed,
    function_unsafe_added,
    inherent_associated_const_now_doc_hidden,
    inherent_associated_pub_const_missing,
    inherent_method_const_removed,
    inherent_method_missing,
    inherent_method_must_use_added,
    inherent_method_now_doc_hidden,
    inherent_method_unsafe_added,
    method_parameter_count_changed,
    module_missing,
    pub_module_level_const_missing,
    pub_module_level_const_now_doc_hidden,
    pub_static_missing,
    pub_static_mut_now_immutable,
    pub_static_now_doc_hidden,
    repr_c_removed,
    repr_packed_added,
    repr_packed_removed,
    sized_impl_removed,
    struct_marked_non_exhaustive,
    struct_missing,
    struct_must_use_added,
    struct_now_doc_hidden,
    struct_pub_field_missing,
    struct_pub_field_now_doc_hidden,
    struct_repr_transparent_removed,
    struct_with_pub_fields_changed_type,
    trait_associated_const_added,
    trait_associated_const_default_removed,
    trait_associated_const_now_doc_hidden,
    trait_associated_type_added,
    trait_associated_type_default_removed,
    trait_associated_type_now_doc_hidden,
    trait_method_default_impl_removed,
    trait_method_added,
    trait_method_missing,
    trait_method_now_doc_hidden,
    trait_method_unsafe_added,
    trait_method_unsafe_removed,
    trait_missing,
    trait_must_use_added,
    trait_newly_sealed,
    trait_no_longer_object_safe,
    trait_now_doc_hidden,
    trait_removed_associated_constant,
    trait_removed_associated_type,
    trait_removed_supertrait,
    trait_unsafe_added,
    trait_unsafe_removed,
    tuple_struct_to_plain_struct,
    type_marked_deprecated,
    union_field_missing,
    union_missing,
    union_must_use_added,
    union_now_doc_hidden,
    union_pub_field_now_doc_hidden,
    unit_struct_changed_kind,
);
