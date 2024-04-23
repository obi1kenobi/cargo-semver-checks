use cargo_semver_checks::{ActualSemverUpdate, Check, GlobalConfig, ReleaseType, Rustdoc};

#[test]
fn major_required_bump_if_breaking_change() {
    let current = Rustdoc::from_root("test_crates/trait_missing/old/");
    let baseline = Rustdoc::from_root("test_crates/trait_missing/new/");
    let mut config = GlobalConfig::new();
    let mut check = Check::new(current);
    let check = check.set_baseline(baseline);
    let report = check.check_release(&mut config).unwrap();
    assert!(!report.success());
    let (_crate_name, crate_report) = report.crate_reports().iter().next().unwrap();
    let required_bump = crate_report.required_bump().unwrap();
    assert_eq!(required_bump, ReleaseType::Major);
    assert_eq!(crate_report.detected_bump(), ActualSemverUpdate::NotChanged);
}

#[test]
fn major_required_bump_if_breaking_change_and_major_bump_detected() {
    let current = Rustdoc::from_root("test_crates/trait_missing_with_major_bump/old/");
    let baseline = Rustdoc::from_root("test_crates/trait_missing_with_major_bump/new/");
    let mut check = Check::new(current);
    let check = check.set_baseline(baseline);
    let report = check.check_release(&mut GlobalConfig::new()).unwrap();
    // semver is successful because the new crate has a major bump version
    assert!(report.success());
    let (_crate_name, crate_report) = report.crate_reports().iter().next().unwrap();
    let required_bump = crate_report.required_bump();
    assert_eq!(required_bump, None);
    assert_eq!(crate_report.detected_bump(), ActualSemverUpdate::Major);
}
