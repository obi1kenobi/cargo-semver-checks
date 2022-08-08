# cargo-semver-checks
Scan your Rust crate for semver violations.

Queries `rustdoc`-generated crate documentation using the `trustfall`
["query everything" engine](https://github.com/obi1kenobi/trustfall).
Each query looks for a particular kind of semver violation, such as:
- public struct was removed
- public enum's variant was removed
- public struct is no longer `Send/Sync/Debug/Clone` etc.
- public enum has a new variant, but wasn't non-exhaustive in the last version

```
cargo install cargo-semver-checks

# Check whether it's safe to release the new version:
cargo semver-checks check-release --current <new-rustdoc-json> --baseline <previous-rustdoc-json>

# Use as a GitHub Action (used in .github/workflows/ci.yml in this repo):
- name: Check semver
  uses: obi1kenobi/cargo-semver-checks-action@v1
- name: Publish to crates.io
  run: # your `cargo publish` code here

# To generate rustdoc JSON data for your crate, use:
cargo +nightly rustdoc --all-features -- --document-private-items -Zunstable-options --output-format json
```

Each failing check references specific items in the Cargo SemVer reference
or other reference pages, as appropriate. It also includes the item name
and file location that are the cause of the problem, as well as a link
to the implementation of that query in the current version of the tool:
![image](https://user-images.githubusercontent.com/2348618/180127698-240e4bed-5581-4cbd-9f47-038affbc4a3e.png)

This crate is functional and capable of catching many semver violations.
However, it won't catch every kind of semver issue, and its performance on massive crates
(X00,000 lines+) has not been optimized. If you run into any problems, please open an issue!

## Using `cargo-semver-checks` to check your crate

The easiest way to use this crate is via
[the corresponding GitHub Action](https://github.com/obi1kenobi/cargo-semver-checks-action)
that will automatically do all the steps for you.

If you'd like to perform those steps manually, here they are:
- Perform a `git checkout` of your crate's last published version*,
  which will represent your semver baseline.
- Generate `rustdoc` documentation in JSON format for the crate's last published version
  by running `cargo +nightly rustdoc --all-features -- --document-private-items -Zunstable-options --output-format json`.
- The above command will generate a file named `doc/<your-crate-name>.json` in your crate's
  build target directory. Copy this file somewhere else -- otherwise it will be overwritten
  by the next steps.
- Switch to the version of your crate that you'd like to check.
- Repeat the `cargo rustdoc` command above, and note
  the newly-generated `doc/<your-crate-name>.json` file in your build target directory.
- Run `cargo semver-checks check-release --current <new-rustdoc> --baseline <previous-rustdoc>`.
  This step will run multiple queries that look for particular kinds of semver violations,
  and report violations they find.

*: Specifically, we want the largest published version number that is smaller than the
   version that we are preparing to publish. The distinction matters if, say, you've already
   published v1.2.2 and v1.3.0, and you need to backport some fixes and release v1.2.3:
   1.2.2 would be your baseline, and you'd compare 1.2.3 -> 1.2.2 and not 1.2.3 -> 1.3.0.

## Comparison to other semver tools

Broadly, there are two approaches for semver-checkers: compiler-based like `rust-semverver`
or `cargo-breaking`, and rustdoc-based like `cargo-semver-checks`, `cargo-crate-api`,
`cargo-public-api`, and others.

Compiler-based approaches interface directly with `rustc`, whereas rustdoc-based approaches
use the JSON output of `rustdoc`.

Interfacing with `rustc` directly allows compiler-based tools to wield the full power of
the Rust compiler, so they can check for many more kinds of semver violations. However, `rustc`
internals are not stable and not meant to be used as a library, making
[maintainability difficult](https://github.com/iomentum/cargo-breaking/issues/40#issuecomment-1009000397)
for compiler-based approaches. They typically require running the checks on nightly Rust,
and sometimes even mandate a *single specific nightly* version: `rust-semverver` as of this writing
needs `nightly-2022-08-03`. As `rustc` internals change,
[constant work is required](https://github.com/rust-lang/rust-semverver/search?q=Rustup+to&type=commits)
to keep compiler-based semver tools up to date and working. It's not clear that there is a path
toward minimizing this maintenance burden, or a path toward getting these tools running
on stable Rust.

Meanwhile, rustdoc-based tools are limited by the information contained in the rustdoc JSON output,
but have a clearer stable Rust story: the rustdoc JSON generation itself is unstable
(though working with a much wider range of nightlies, and on
[a path toward stabilization](https://github.com/rust-lang/rust/issues/76578)),
but the checking of the JSON files can be done on stable Rust. There is interest in
[hosting rustdoc JSON](https://github.com/rust-lang/docs.rs/issues/1285) on `docs.rs` meaning
that semver-checking could one day download the baseline rustdoc JSON file instead of generating it.
Also, generally speaking, inspecting JSON data is likely going to be faster than full compilation.

### Comparing `cargo-semver-checks` vs `cargo-crate-api` and `cargo-public-api`

TL;DR: `cargo-semver-checks` is a linter, not
[a differ](https://github.com/Enselic/cargo-public-api#diff-the-public-api).
You probably want both a linter and a differ in your toolbox, just like you'd want
both a screwdriver and a wrench.

`cargo-crate-api` and `cargo-public-api` output
[a diff ("what changed") of your public API](https://github.com/Enselic/cargo-public-api#diff-the-public-api).
This diff is primarily useful for humans instead of in CI: just because a diff is non-empty
doesn't mean you've violated semver. These tools are most useful when generating a changelog,
or when manually inspecting the summary of changes happening in the public API.

`cargo-semver-checks` is meant to be clippy-like and is especially useful in CI:
- [outputs specific issues it finds](https://twitter.com/PredragGruevski/status/1556333347571470336/photo/1),
  rather than a diff you have to read and understand
- no false positives: if it reports a problem, you know it's real
- fast runtime
- clear and actionable error messages that cite relevant sections of the Rust and cargo books

### Why build `cargo-semver-checks` instead of adding a linter to an existing tool?

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

## Naming note

This crate was intended to be published under the name `cargo-semver-check`, and may indeed one
day be published under that name. Due to
[an unfortunate mishap](https://github.com/rust-lang/crates.io/issues/728#issuecomment-118276095),
it remains `cargo-semver-checks` for the time being.

The `cargo_semver_check` name is reserved on crates.io but all its versions
are intentionally yanked. Please use the `cargo-semver-checks` crate instead.

## Running `cargo test` in this crate for the first time

Testing this crate requires rustdoc JSON output data, which is too large and variable
to check into git. It has to be generated locally before `cargo test` will succeed,
and will be saved in a `localdata` gitignored directory in the repo root.

To generate this data, please run `./scripts/regenerate_test_rustdocs.sh`.

## Adding a new semver query
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

## Appendix

Notes:
- If using it on a massive codebase (multiple hundreds of thousands of lines of Rust),
  the queries may be a bit slow: there is some `O(n^2)` scaling for `n` items in a few places that
  I haven't had time to optimize down to `O(n)` yet. Apologies! I have temporarily prioritized
  features over speed, and the runtime will improve significantly with a small amount of extra work.
- **No false positives**: Currently, all queries report constructive proof of semver violations:
  there are no false positives. They always list a file name and line number for the baseline item
  that could not be found in the new code.
- **There are false negatives**: This tool is a work-in-progress, and cannot check all kinds of
  semver violations yet. Just because it doesn't find any semver issues doesn't mean
  they don't exist.
