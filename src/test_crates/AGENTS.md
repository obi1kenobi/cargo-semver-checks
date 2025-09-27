# Test crate authoring guide

## Core layout
- Model each lint scenario as a `<lint_name>/{old,new}` pair whose directory name matches the crate name. The manifests on both sides should share the same `[package]` metadata—`publish = false`, identical `name`, `version`, and `edition`—so that only the semantic API delta differs between versions (see `associated_items_hidden_from_public_api/old|new/Cargo.toml`).
- Commit a `Cargo.lock` file for both `old/` and `new/` crates. Even the minimal fixtures do this to keep rustdoc JSON generation deterministic and snapshots stable.
- Keep edits tightly scoped: avoid changing unrelated items so each diff demonstrates just the behavior the lint targets. Crates like `move_item_and_reexport` isolate the API move they exercise instead of mixing in extra churn.

## Source file conventions
- Default to `#![no_std]` at the top of `src/lib.rs` unless the scenario genuinely needs `std`. This keeps fixtures lightweight (e.g., `union_missing/old/src/lib.rs`).
- Co-locate expectations with the code under test. Inline comments or doc comments such as "should be reported" / "shouldn't be reported" appear next to the relevant items so future authors can see which lines are intended to trigger the lint (for example, `struct_must_use_removed/old/src/lib.rs`).
- Each fixture should demonstrate both true positives and guard rails against false positives. Use patterns like private modules, `#[doc(hidden)]`, sealed traits, or other non-public visibility to prove removals that *look* similar do **not** trip the lint (see `union_missing/old/src/lib.rs`).

## Features and configuration
- When exercising feature-sensitive behavior, mirror the change in both `Cargo.toml` and the accompanying `#[cfg]` usage so the old/new delta clearly expresses the scenario. The `feature_not_enabled_by_default` pair is a good template for documenting how feature defaults evolve.
- If a fixture must adjust lint severities to focus on a specific tool-path, encode that under `[package.metadata.cargo-semver-checks.lints]` (as in `features_simple/new/Cargo.toml`). This prevents unrelated warnings from obscuring the behavior under test.

## Manifest-driven fixtures
- Scenarios that primarily test manifest handling (like lint configuration overrides) belong under `test_crates/manifest_tests/`. That subdirectory skips rustdoc regeneration; see its `README.md` for the rationale before adding new cases.
