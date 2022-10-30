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

## FAQ

### What Rust versions does `cargo-semver-checks` support?

`cargo-semver-checks` uses the rustdoc tool to analyze the crate's API.
Rustdoc's JSON output format isn't stable, and can have breaking changes in new Rust versions.

When each `cargo-semver-checks` version is released, it will at minimum include support
for the then-current stable and beta Rust versions. It may, but is not guaranteed to,
additionally support some nightly Rust versions.

[The GitHub Action](https://github.com/obi1kenobi/cargo-semver-checks-action) uses
the most recent versions of both `cargo-semver-checks` and stable Rust,
so it should be unaffected. Users using `cargo-semver-checks` in other ways
are encouraged to update `cargo-semver-checks` when updating Rust versions
to ensure continued compatibility.

### Does `cargo-semver-checks` have false positives or false negatives?

"False positive" means that `cargo-semver-checks` reported a semver violation incorrectly.
This should never happen, and is considered a bug.

"False negative" means that `cargo-semver-checks` failed to report a semver violation.
There are many ways to break semver, and `cargo-semver-checks`
[doesn't yet have lints for all of them](https://github.com/obi1kenobi/cargo-semver-check/issues/5).
We offer mentorship for
[contributing new lints](https://github.com/obi1kenobi/cargo-semver-check/issues?q=is%3Aopen+is%3Aissue+label%3AA-lint+label%3AE-help-wanted).
Please check out
[Contributing](https://github.com/obi1kenobi/cargo-semver-check/blob/main/CONTRIBUTING.md)
for details.

### Does the crate I'm checking have to be published on crates.io?

When checking semver, `cargo-semver-checks` by default looks up the previous version of the crate
on crates.io and uses it to generate the baseline against which it will compare the current version.

`cargo-semver-checks` has flags specifying other ways to generate a baseline as well:
```
--baseline-version <X.Y.Z>
    Version from registry to lookup for a baseline

--baseline-rev <REV>
    Git revision to lookup for a baseline

--baseline-root <MANIFEST_ROOT>
    Directory containing baseline crate source

--baseline-rustdoc <JSON_PATH>
    The rustdoc json file to use as a semver baseline
```

If your crate is not published on crates.io, you can still use `cargo-semver-checks`!
Looking up the baseline version from the registry won't work (custom registries are not
currently supported: [#160](https://github.com/obi1kenobi/cargo-semver-check/issues/160)),
but you can still use any other other ways of generating the semver baseline.

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

### Why is scanning massive crates slow?

Crates that are ~100,000+ lines of Rust may currently experience scan times
of a several minutes due to some missing optimizations.

For implementation convenience, there's some `O(n^2)` scaling for `n` items in a few places.
If your crate is negatively affected by this,
[please let us know](https://github.com/obi1kenobi/cargo-semver-check/issues/new?assignees=&labels=C-bug&template=bug_report.yml)
so we can best prioritize optimization versus feature work.

### Why is it sometimes `cargo-semver-check` and `cargo-semver-checks`?

This crate was intended to be published under the name `cargo-semver-check`, and may indeed one
day be published under that name. Due to
[an unfortunate mishap](https://github.com/rust-lang/crates.io/issues/728#issuecomment-118276095),
it remains `cargo-semver-checks` for the time being.

The `cargo_semver_check` name is reserved on crates.io but all its versions
are intentionally yanked. Please use the `cargo-semver-checks` crate instead.

### License

Available under the Apache License (Version 2.0) or the MIT license, at your option.

Copyright 2022-present Predrag Gruevski and Project Contributors.
The present date is determined by the timestamp of the most recent commit in the repository.
Project Contributors are all authors and committers of commits in the repository.
