# Contributing

## Design goals

In short:
- Checks should be configuration, not code.
- That helps us ensure we don't have to trade off ergonomics versus maintainability.

To make a semver-checker that is pleasant to use (and therefore gets widely adopted),
we have to go beyond being merely "technically correct" when reporting problems.

For example, say the tool has discovered that a `pub struct` no longer implements some trait:
this is a breaking change and semver requires a major version bump. It's technically correct to
state this fact and move on, but it's more helpful to have contextually-appropriate advice and
reference links based on whether the trait in question is:
- an auto-trait like `Send`, `Sync`, or `Sized`
- a trait that is usually added via `#[derive(...)]`, like `Debug` or `Clone`
- a built-in trait that is usually not derived, like `From`
- one of the crate's own traits

If all our semver checks were written imperatively, it would have been difficult to reuse code
and optimizations across different checks. This would have incentivized having a single overarching
"trait is missing" check with a ton of special cases, i.e. complex code with a maintainability
hazard.

Checks should be configuration, not code, and that's what `cargo-semver-checks` does.
It uses a datasource-agnostic query engine called
[Trustfall](https://github.com/obi1kenobi/trustfall) to allow writing semver checks as
[declarative strongly-typed queries](https://twitter.com/PredragGruevski/status/1550135974499438592)
over a
[schema](https://github.com/obi1kenobi/cargo-semver-check/blob/main/src/rustdoc_schema.graphql).
Adding a new semver check is as simple as adding a new file that specifies the query to run and
metadata like the error message to display in case the query finds any results (errors).

This has several advantages:
- **It's easy to write more checks or specialize existing ones.** Just duplicate an existing
  query file and edit it to your liking. The strongly-typed query language doesn't prevent
  logic errors (neither does Rust 😅), but like Rust it has a strong tendency
  to "work correctly as soon as it compiles."
- **Fast performance without complex code.** Trustfall enables efficient lazy evaluation of queries
  without any cloning of rustdoc JSON data and without `unsafe`. The obvious way to write queries
  is also the fast way.
- **Optimizations are decoupled from queries.** When a new optimization (e.g. some caching)
  is added to  `cargo-semver-checks` (or even Trustfall itself), all queries automatically
  benefit from it without needing any changes.

### Future functionality made easier by the query-based approach

In principle, `cargo-semver-checks` could be extended to
[support running custom user-specified checks](https://github.com/obi1kenobi/cargo-semver-check/issues/38)
on top of the same rustdoc JSON + cargo manifest data it uses today.
Checks are configuration, not code: the custom checks would just be a set of files that
`cargo-semver-checks` is configured to run.

Similarly, `cargo-semver-checks` could warn about potentially-undesirable API changes that
may have been done unintentionally, and which could have semver implications without being breaking.
An example is removing the last private field of a `pub struct` that is not `#[non_exhaustive]`:
this would have the side-effect of adding to the public API the ability to construct the struct
with a literal. If this change were published accidentally, undoing the change would be breaking
and would require a new major version. More examples of such useful-but-not-semver checks are
[here](https://github.com/obi1kenobi/cargo-semver-check/issues/5).

## Running `cargo test` in this crate for the first time

Testing this crate requires rustdoc JSON output data, which is too large and variable
to check into git. It has to be generated locally before `cargo test` will succeed,
and will be saved in a `localdata` gitignored directory in the repo root.

To generate this data, please run `./scripts/regenerate_test_rustdocs.sh`.

## Adding a new lint

Lints are written as queries for the  `trustfall` ["query everything" engine](https://github.com/obi1kenobi/trustfall).

Checklist:
- Choose an appropriate name for your query. We'll refer to it as `<query_name>`.
- Add the query file: `src/queries/<query_name>.ron`.
- Add a `<query-name>` feature to `semver_tests/Cargo.toml`.
- Add a `<query-name>.rs` file in `semver_tests/src/test_cases`.
- Add code to that file that demonstrates that semver issue: write the "baseline" first,
  and then use `#[cfg(feature = <query_name>)]` and `#[cfg(not(feature = <query_name>))]` as
  necessary to alter that baseline into a shape that causes the semver issue
  your query looks for.
- Add test code for false-positives and/or true-but-unintended-positives your query might report.
  For example, a true-but-unintended output would be if a query that looks for
  removal of public fields were to report that a struct was removed. This is unintended
  since it would overwhelm the user with errors, instead of having a separate query that
  specifically reports the removal of the struct rather than all its fields separately.
- Add `<query_name>` to the list of features that need rustdoc data
  in `scripts/regenerate_test_rustdocs.sh`.
- Add the outputs you expect your query to produce over your test case in
  a new file: `src/test_data/<query_name>.output.run`.
- Add `<query_name>` to the list of queries tested by the `query_execution_tests!()`
  macro near the bottom of `src/adapter.rs`.
- Re-run `./scripts/regenerate_test_rustdocs.sh` to generate the new rustdoc JSON file.
- Run `cargo test` and ensure your new test appears in the test list and runs correctly.
- Add an `include_str!("queries/<query_name>.ron"),` line to `SemverQuery::all_queries()`
  in the `src/query.rs` file, to ensure your query is enabled for use in query runs.
- Whew! You're done. Thanks for your contribution.
- If you have the energy, please try to simplify this process by removing and
  automating some of these steps.
