use std::{
    borrow::Cow,
    time::{SystemTime, UNIX_EPOCH},
};

use rand::Rng as _;

use crate::{
    LintLevel, RequiredSemverUpdate,
    data_generation::{
        CacheSettings, CrateDataRequest, DataStorage, GenerationSettings, ProgressCallbacks,
    },
    manifest::Manifest,
};

use super::*;

pub(super) struct TempDir {
    path: PathBuf,
}

impl TempDir {
    pub(super) fn new(prefix: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let suffix: u64 = rand::rng().random();
        let path = std::env::temp_dir().join(format!("{prefix}-{timestamp}-{suffix}"));
        fs::create_dir_all(&path).expect("failed to create temp directory");
        Self { path }
    }

    pub(super) fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

pub(super) fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("failed to create file parent");
    }
    fs::write(path, contents).expect("failed to write test file");
}

pub(super) fn make_local_manifest(
    temp_dir: &TempDir,
    dir_name: &str,
    package_name: &str,
    lib_name: Option<&str>,
    manifest_extra: &str,
    lib_body: &str,
) -> Manifest {
    let crate_dir = temp_dir.path().join(dir_name);
    let manifest_path = crate_dir.join("Cargo.toml");
    let package_name = toml::Value::String(package_name.to_owned()).to_string();
    let lib_section = lib_name
        .map(|name| {
            let name = toml::Value::String(name.to_owned()).to_string();
            format!("\n[lib]\nname = {name}\n")
        })
        .unwrap_or_default();
    write_file(
        &manifest_path,
        &format!(
            "\
[package]
name = {package_name}
version = \"1.2.3\"
edition = \"2024\"
{lib_section}{manifest_extra}"
        ),
    );
    write_file(&crate_dir.join("src/lib.rs"), lib_body);
    Manifest::parse(manifest_path).expect("failed to parse generated manifest")
}

pub(super) fn make_request(manifest: &Manifest, is_baseline: bool) -> CrateDataRequest<'_> {
    make_request_with_settings(manifest, is_baseline, true, &[], None)
}

pub(super) fn make_request_with_settings<'a>(
    manifest: &'a Manifest,
    is_baseline: bool,
    default_features: bool,
    extra_features: &[&'a str],
    build_target: Option<&'a str>,
) -> CrateDataRequest<'a> {
    CrateDataRequest::from_local_project(
        manifest,
        default_features,
        extra_features
            .iter()
            .map(|feature| Cow::Borrowed(*feature))
            .collect(),
        build_target,
        is_baseline,
    )
}

pub(super) fn make_query_result(path: &[&str]) -> BTreeMap<Arc<str>, FieldValue> {
    let mut result = BTreeMap::new();
    result.insert(Arc::<str>::from("path"), path.to_vec().into());
    result
}

pub(super) fn make_semver_query(purpose: WitnessPurpose) -> SemverQuery {
    SemverQuery::from_ron_str(&format!(
        r#"
        (
            id: "witness_test",
            human_readable_name: "witness test",
            description: "witness test",
            required_update: Major,
            lint_level: Deny,
            query: "query {{ Crate {{ name @output }} }}",
            error_message: "witness test",
            witness: Some((
                purpose: {purpose:?},
                hint_template: "hint",
                witness_template: Some("pub fn witness() {{ {{{{join \"::\" path}}}}(); }}"),
            )),
        )
        "#
    ))
    .expect("failed to parse test query")
}

pub(super) fn make_lint_result(
    semver_query: SemverQuery,
    query_results: Vec<BTreeMap<Arc<str>, FieldValue>>,
) -> LintResult {
    LintResult {
        semver_query,
        query_results,
        query_duration: std::time::Duration::ZERO,
        effective_required_update: RequiredSemverUpdate::Major,
        effective_lint_level: LintLevel::Deny,
    }
}

pub(super) fn run_smoke_witness(
    purpose: WitnessPurpose,
    run_consistency_checks: bool,
    baseline_body: &str,
    current_body: &str,
) -> (WitnessRunReport, Vec<LintResult>, TempDir, GlobalConfig) {
    let temp_dir = TempDir::new("cargo-semver-checks-witness");
    let baseline_manifest = make_local_manifest(
        &temp_dir,
        "baseline",
        "demo-package",
        Some("renamed_lib"),
        "",
        baseline_body,
    );
    let current_manifest = make_local_manifest(
        &temp_dir,
        "current",
        "demo-package",
        Some("renamed_lib"),
        "",
        current_body,
    );
    let baseline_request = make_request(&baseline_manifest, true);
    let current_request = make_request(&current_manifest, false);
    let (report, lint_results, config) = run_witness_with_requests(
        purpose,
        run_consistency_checks,
        "demo-package",
        &baseline_request,
        &current_request,
        temp_dir.path().join("target"),
    );

    (report, lint_results, temp_dir, config)
}

pub(super) fn run_witness_with_requests(
    purpose: WitnessPurpose,
    run_consistency_checks: bool,
    crate_name: &str,
    baseline_request: &CrateDataRequest<'_>,
    current_request: &CrateDataRequest<'_>,
    target_dir: PathBuf,
) -> (WitnessRunReport, Vec<LintResult>, GlobalConfig) {
    let witness_data =
        WitnessGenerationData::new(Some(baseline_request), Some(current_request), target_dir);
    let mut lint_results = vec![make_lint_result(
        make_semver_query(purpose),
        vec![make_query_result(&["renamed_lib", "removed"])],
    )];
    let mut config = GlobalConfig::new();
    let witness_generation = WitnessGeneration {
        show_hints: false,
        run_consistency_checks,
    };
    let data_storage = make_test_data_storage(
        baseline_request,
        current_request,
        witness_data.target_dir.as_path(),
    );
    let index_storage = data_storage.create_indexes();
    let adapter = index_storage.create_adapter();

    let report = run_witness_checks(
        &mut config,
        &witness_generation,
        witness_data,
        crate_name,
        &adapter,
        &mut lint_results,
    );

    (report, lint_results, config)
}

struct NoopProgressCallbacks;

impl<'a> ProgressCallbacks<'a> for NoopProgressCallbacks {}

pub(super) fn make_test_data_storage(
    baseline_request: &CrateDataRequest<'_>,
    current_request: &CrateDataRequest<'_>,
    target_dir: &Path,
) -> DataStorage {
    let generation_settings = GenerationSettings {
        pass_through_stderr: false,
        use_color: false,
        deps: false,
    };
    let mut callbacks = NoopProgressCallbacks;
    let baseline_storage = baseline_request
        .resolve(
            target_dir,
            CacheSettings::None,
            generation_settings,
            &mut callbacks,
        )
        .expect("failed to resolve baseline rustdoc data for witness test");
    let current_storage = current_request
        .resolve(
            target_dir,
            CacheSettings::None,
            generation_settings,
            &mut callbacks,
        )
        .expect("failed to resolve current rustdoc data for witness test");
    DataStorage::new(current_storage, baseline_storage)
}
