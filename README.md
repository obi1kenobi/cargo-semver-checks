# cargo-semver-checks

Lint your crate API changes for semver violations.

- [Quick Start](#quick-start)
- [FAQ](#faq)
- [Contributing](https://github.com/obi1kenobi/cargo-semver-check/blob/main/CONTRIBUTING.md)

## Quick Start

```console
$ cargo install cargo-semver-checks --locked

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

### Does the crate I'm checking have to be published on crates.io?

No, it does not have to be published anywhere. You'll just need to use a flag to help
`cargo-semver-checks` locate the version to use as a baseline for semver-checking.

By default, `cargo-semver-checks` uses crates.io to look up the previous version of the crate,
which is used as the baseline for semver-checking the current version of the crate.
The following flags can be used to explicitly specify a baseline instead:
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

Custom registries are not currently supported
([#160](https://github.com/obi1kenobi/cargo-semver-check/issues/160)), so crates published on
registries other than crates.io should use one of the other approaches of generating the baseline.

### Does `cargo-semver-checks` have false positives?

"False positive" means that `cargo-semver-checks` reported a semver violation incorrectly.
A design goal of `cargo-semver-checks` is to not have false positives.
If they do occur, they are considered bugs.

When `cargo-semver-checks` reports a semver violation, it should always point to a specific
file and approximate line number where the specified issue occurs; failure to specify
a file and line number is also considered a bug.

If you think `cargo-semver-checks` might have a false-positive but you aren't sure, please
[open an issue](https://github.com/obi1kenobi/cargo-semver-checks/issues/new?assignees=&labels=C-bug&template=bug_report.yml).
Semver in Rust has [many non-obvious and tricky edge cases](https://predr.ag/blog/toward-fearless-cargo-update/),
especially [in the presence of macros](https://github.com/obi1kenobi/cargo-semver-checks/issues/167).
We'd be happy to look into it together with you to determine if it's a false positive or not.

### Will `cargo-semver-checks` catch every semver violation?

No, it will not — not yet!
There are many ways to break semver, and `cargo-semver-checks`
[doesn't yet have lints for all of them](https://github.com/obi1kenobi/cargo-semver-check/issues/5).
New lints are added frequently, and
[we'd be happy to mentor you](https://github.com/obi1kenobi/cargo-semver-check/blob/main/CONTRIBUTING.md)
if you'd like to contribute new lints!

Append `--verbose` when semver-checking your crate to see the full list of performed semver checks.

Here are some example areas where `cargo-semver-checks` currently will not catch semver violations:
- breaking type changes, for example in the type of a field or function parameter
- breaking changes in generics or lifetimes
- breaking changes that exist when only a subset of all crate features are activated

### Why `cargo-semver-checks` instead of ...?

[rust semverver](https://github.com/rust-lang/rust-semverver) builds on top of
rustc internals to build rlib's and compare their metadata.  This strips the
code down to the basics for identifying changes.  However, is tightly coupled
to specific nightly compiler versions and [takes work to stay in
sync](https://github.com/rust-lang/rust-semverver/search?q=Rustup+to&type=commits).

[cargo breaking](https://github.com/iomentum/cargo-breaking) effectively runs
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
