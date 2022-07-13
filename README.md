# cargo-semver-checks
Scan your Rust crate for semver violations.

Queries `rustdoc`-generated crate documentation using the `trustfall`
["query everything" engine](https://github.com/obi1kenobi/trustfall).
Each query looks for a particular kind of semver violation, such as:
- public struct was removed
- public plain struct's public field was removed
- public enum was removed
- public enum's variant was removed

```
cargo install cargo-semver-checks

cargo semver-checks diff-files --current <new-rustdoc-json> --baseline <previous-rustdoc-json>

# To have rustdoc generate JSON output, use:
cargo +nightly rustdoc -- -Zunstable-options --output-format json
```

This crate is a work-in-progress. It can catch some semver violations, and will miss many more.
Its queries and adapter implementation have not been optimized for runtime,
and will currently exhibit `O(n^2)` runtime growth on large codebases.
See the notes in the section below for details.

## Using `cargo-semver-checks` to check your crate

Steps:
- Perform a `git checkout` of your crate's last published version,
  which will represent your semver baseline.
- Generate `rustdoc` documentation in JSON format for the crate's last published version
  by running `cargo +nightly rustdoc -- -Zunstable-options --output-format json`.
- The above command will generate a file named `doc/<your-crate-name>.json` in your crate's
  build target directory. Copy this file somewhere else -- otherwise it will be overwritten
  by the next steps.
- Switch to the version of your crate that you'd like to check.
- Repeat the `cargo rustdoc` command above, and note
  the newly-generated `doc/<your-crate-name>.json` file in your build target directory.
- Run `cargo semver-checks diff-files --current <new-rustdoc> --baseline <previous-rustdoc>`.
  This step will run multiple queries that look for particular kinds of semver violations,
  and report violations they find.

Notes:
- Only 5 violations per category are reported for now.
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
