## Basics

Always begin by reading the `CONTRIBUTING.md` file, and all `README.md` files in all directories.

Never under any circumstances propose or apply any change to the `package.version` field
of the top-level `Cargo.toml` file. Never under any cicrcumstances change anything inside
the `.github/` directory, or inside the `funding.json`, `CODEOWNERS`, or any license files.

Read the values of the `Cargo.toml`'s `package.rust-version` and `package.edition` keys,
then always adjust your Rust code to take advantage of the latest functionality supported
by that version and edition. Ensure you do not use functionality that is not compatible with
that version or edition. Do not change the values of these keys unless explicitly requested.

Whenever possible, limit the scope of changes to what was explicitly requested. Only exceed
the explicit request if that's required to fix a broken test or lint, or to make a similar small,
tightly-scoped change that is reasonable to expect in the circumstances.
If you exceed the explicit request, you must always describe how and why you did so
when summarizing the changes.

Always implement everything that was requested, or explicitly explain why that was not possible or
reasonable to accomplish. If you are asked to not implement something, leave a TODO comment
or a `todo!()` Rust expression with an appropriate message.

Always fix problematic tests by fixing the underlying problem, and never do so by deleting the test
or stubbing out or mocking out the functionality so as to make the test suite pass. In general,
avoid mocking as much as possible, preferring to design APIs that can easily and reliably tested
without mocks or complex test harnesses.

Prefer Rust unit tests over integration tests whenever either option is viable.
Always consider what behaviors in your new code are worth testing, and add appropriate tests
making sure to cover all relevant edge cases.

Do not add entries to the `[features]` table in `Cargo.toml` unless explicitly asked to do so.

If working with the file system, prefer `fs_err` APIs over the equivalent `std::fs` ones
whenever possible.

Always prefer `.expect()` and similar methods over `.unwrap()`, making sure to add a descriptive
but concise message that describes the problem.

## Helpful scripts

- `scripts/make_new_lint.sh <lint-name>` will create a new lint file, register the lint, create a corresponding test crate, and a stub snapshot test file.
- `scripts/make_new_test_crate.sh <test-crate>` can be used to add a new test crate without adding a new lint. Use it if checking lint behavior outside of a particular lint is required.
- `scripts/rename_lint.sh <old-name> <new-name>` can be used to rename a lint. The lint names are the same as the name of the lint file without the `.ron` extension, and also the same as the `id` value inside the lint file.

## Testing and linting proposed changes

All proposed changes must be properly formatted, must pass `clippy` without warnings or errors,
and must pass tests. The GitHub workflow at `.github/workflows/ci.yml` is used to scan
all proposed changes and will automatically reject any changes that do not pass.
You are not able to run the `.github/workflows/ci.yml` file, but here follows
a summary of the checks it performs and how to ensure your changes pass.

Before proposing a code change:
- Run `cargo clippy --all-targets --no-deps --fix --allow-dirty --allow-staged -- -D warnings --allow deprecated` to fix any auto-fixable lints. Manually fix anything that couldn't be auto-fixed.
- Run `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items` to ensure the documentation is clean of lint.
- Run `cargo fmt` to format your code.
- Run `cargo test` to the code passes tests.

If adding new test crates, snapshots, and/or new lints, then additionally:
- Ensure `RUSTFLAGS="-A dead_code -A deprecated -A unused -A private_bounds" cargo check --manifest-path=<path-to-new-crate-cargo-toml>` passes without warnings on each new test crate.
- Run `./scripts/regenerate_test_rustdocs.sh` then use `insta` and `cargo-insta` appropriately to update any snapshots. Review the snapshots to ensure they are correct and expected for the code change you propsed.

If you added a new lint and its snapshot file shows no matches, then either your new test crate code is wrong (or missing) or the lint query isn't working as expected. Lint additions that don't match any test cases will not be accepted, so do not ignore the problem: either debug and fix it, or say you got stuck and ask for help.
