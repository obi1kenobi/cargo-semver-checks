# cargo-semver-checks

Lint your crate API changes for semver violations.

- [Quick Start](#quick-start)
- [FAQ](#faq)
- [Contributing](https://github.com/obi1kenobi/cargo-semver-check/blob/main/CONTRIBUTING.md)

## Quick Start

```console
$ cargo install cargo-semver-checks

# Check whether it's safe to release the new version:
$ cargo semver-checks check-release
```

Or use as a [GitHub Action](https://github.com/obi1kenobi/cargo-semver-checks-action) (used in .github/workflows/ci.yml in this repo):
```yaml
- name: Check semver
  uses: obi1kenobi/cargo-semver-checks-action@v1
```

![image](https://user-images.githubusercontent.com/2348618/180127698-240e4bed-5581-4cbd-9f47-038affbc4a3e.png)

Each failing check references specific items in the
[Cargo SemVer reference](https://doc.rust-lang.org/cargo/reference/semver.html)
or other reference pages, as appropriate. It also includes the item name
and file location that are the cause of the problem, as well as a link
to the implementation of that query in the current version of the tool.

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

## FAQ

### Why `cargo-semver-checks` instead of ...?

[rust semverver](https://github.com/rust-lang/rust-semverver) builds on top of
rustc internals to build rlib's and compare their metadata.  This strips the
code down to the basics for identifying changes.  However, is tightly coupled
to specific nightly compiler versions and [takes work to stay in
sync](https://github.com/rust-lang/rust-semverver/search?q=Rustup+to&type=commits).

[cargo breaking](https://github.com/iomentum/cargo-breakinga) effectively runs
`cargo expand` and re-parses the code using
[`syn`](https://crates.io/crates/syn) which requires re-implementing large
swaths of rust's semantics to then lint the API for changes.
This is limited to the feature and target the crate was compiled for.

`cargo-semver-checks` sources its data from rustdoc's json output.  While the
json output format is unstable, the rate of change is fairly low, reducing the
churn in keeping up.  The lints are also written as queries for `trustfall`
["query everything" engine](https://github.com/obi1kenobi/trustfall), reducing
the work for creating and maintaining them.  Because of the extra data that
rustdoc includes, some level of feature/target awareness might be able to be
introduced.

There is interest in
[hosting rustdoc JSON](https://github.com/rust-lang/docs.rs/issues/1285) on `docs.rs` meaning
that semver-checking could one day download the baseline rustdoc JSON file instead of generating it.
Also, generally speaking, inspecting JSON data is likely going to be faster than full compilation.

[`cargo-public-api`](https://crates.io/crates/cargo-public-api) uses rustdoc,
like `cargo-semver-checks`, but focuses more on API diffing (showing which
items has changed) and not API linting (explaining why they have changed and
providing control over what counts).

### Why is it sometimes `cargo-semver-check` and `cargo-semver-checks`?

This crate was intended to be published under the name `cargo-semver-check`, and may indeed one
day be published under that name. Due to
[an unfortunate mishap](https://github.com/rust-lang/crates.io/issues/728#issuecomment-118276095),
it remains `cargo-semver-checks` for the time being.

The `cargo_semver_check` name is reserved on crates.io but all its versions
are intentionally yanked. Please use the `cargo-semver-checks` crate instead.

### License

Available under the Apache License (Version 2.0) or the MIT license, at your option.

Copyright 2022-present Predrag Gruevski
The present date is determined by the timestamp of the most recent commit in the repository.
