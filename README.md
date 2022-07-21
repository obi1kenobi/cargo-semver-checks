# cargo-semver-checks
Scan your Rust crate for semver violations.

Queries `rustdoc`-generated crate documentation using the `trustfall`
["query everything" engine](https://github.com/obi1kenobi/trustfall).
Each query looks for a particular kind of semver violation, such as:
- public struct was removed
- public enum's variant was removed
- public struct became non-exhaustive
- public enum has a new variant, but wasn't non-exhaustive in the last version

```
cargo install cargo-semver-checks

# Check whether it's safe to release the new version:
cargo semver-checks check-release --current <new-rustdoc-json> --baseline <previous-rustdoc-json>

# Or, via a GitHub Action (see .github/workflows/ci.yml in this repo):
steps:
- uses: actions/checkout@v3
  with:
    fetch-depth: 0
- name: Check semver
  uses: obi1kenobi/cargo-semver-checks-action@v1
- name: Publish to crates.io
  run: # your `cargo publish` code here

# To generate rustdoc JSON data for your crate, use:
cargo +nightly rustdoc -- -Zunstable-options --output-format json
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
  by running `cargo +nightly rustdoc -- -Zunstable-options --output-format json`.
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
