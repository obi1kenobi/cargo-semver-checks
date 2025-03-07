#![no_std]

// this line will be removed in the `new` version in order to trigger
// the `function_missing` lint, so we can test that we can override
// its lint level and required version bump in the [package.metadata]
// and [workspace.metadata].
pub fn function_missing() {}

// similarly, this line will be commented out in the `new` version so we
// can trigger and test configuration of the `module_missing` lint.
pub mod module_missing {}
