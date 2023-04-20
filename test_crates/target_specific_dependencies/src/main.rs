fn main() {
    // This crate checks whether the tool correctly detects
    // implicit features defined by target-specific dependencies.
    // https://github.com/obi1kenobi/cargo-semver-checks/issues/369
    #[cfg(not(feature = "async-std"))]
    panic!("the tool should have built the project with this flag");
}
