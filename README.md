<p align="center">
  <img alt="semver-checks" src="./brand/banner.png" width="800" />
</p>

# cargo-semver-checks

Lint your crate API changes for semver violations.

- [Quick Start](#quick-start)
- [FAQ](#faq)
- [Configuration](#configuration)
- [Contributing](https://github.com/obi1kenobi/cargo-semver-checks/blob/main/CONTRIBUTING.md)

## Quick Start

```sh
$ cargo install cargo-semver-checks --locked

# Check whether it's safe to release the new version:
$ cargo semver-checks
```

Or use as a [GitHub Action](https://github.com/obi1kenobi/cargo-semver-checks-action) (used in .github/workflows/ci.yml in this repo):

```yaml
- name: Check semver
  uses: obi1kenobi/cargo-semver-checks-action@v2
```

![image](https://user-images.githubusercontent.com/2348618/180127698-240e4bed-5581-4cbd-9f47-038affbc4a3e.png)

Each failing check references specific items in the
[Cargo SemVer reference](https://doc.rust-lang.org/cargo/reference/semver.html)
or other reference pages, as appropriate. It also includes the item name
and file location that are the cause of the problem, as well as a link
to the implementation of that query in the current version of the tool.

## FAQ
- [What Rust versions does `cargo-semver-checks` support?](#what-rust-versions-does-cargo-semver-checks-support)
- [Can I use `cargo-semver-checks` with `nightly` Rust?](#can-i-use-cargo-semver-checks-with-nightly-rust)
- [What if my project needs stronger guarantees around supported Rust versions?](#what-if-my-project-needs-stronger-guarantees-around-supported-rust-versions)
- [Does the crate I'm checking have to be published on crates.io?](#does-the-crate-im-checking-have-to-be-published-on-cratesio)
- [What features does `cargo-semver-checks` enable in the tested crates?](#what-features-does-cargo-semver-checks-enable-in-the-tested-crates)
- [Does `cargo-semver-checks` have false positives?](#does-cargo-semver-checks-have-false-positives)
- [Will `cargo-semver-checks` catch every semver violation?](#will-cargo-semver-checks-catch-every-semver-violation)
- [If I really want a new feature to be implemented, can I sponsor its development?](#if-i-really-want-a-new-feature-to-be-implemented-can-i-sponsor-its-development)
- [How is `cargo-semver-checks` similar to and different from other tools?](#how-is-cargo-semver-checks-similar-to-and-different-from-other-tools)
- [Why is it sometimes `cargo-semver-check` and `cargo-semver-checks`?](#why-is-it-sometimes-cargo-semver-check-and-cargo-semver-checks)
- [What is the MSRV policy with respect to semver?](#what-is-the-msrv-policy-with-respect-to-semver)
- [Can I configure individual lints?](#can-i-configure-individual-lints)

### What Rust versions does `cargo-semver-checks` support?

`cargo-semver-checks` uses the rustdoc tool to analyze the crate's API.
Rustdoc's JSON output format isn't stable, and can have breaking changes in new Rust versions.

When each `cargo-semver-checks` version is released, it will at minimum include support
for the then-current stable and beta Rust versions. It may, but is not guaranteed to,
additionally support some nightly Rust versions.

[The GitHub Action](https://github.com/obi1kenobi/cargo-semver-checks-action) by default uses
the most recent versions of both `cargo-semver-checks` and stable Rust,
so it should be unaffected. Users using `cargo-semver-checks` in other ways
are encouraged to update `cargo-semver-checks` when updating Rust versions
to ensure continued compatibility.

### Can I use `cargo-semver-checks` with `nightly` Rust?

Support for `nightly` Rust versions is on a best-effort basis.
It will work _often, but not always_. If you _must_ use `nightly`,
it's strongly recommended to pin to a specific `nightly` version to avoid broken workflows.

`cargo-semver-checks` relies on the rustdoc JSON format, which is unstable and changes often.
After a new rustdoc JSON format version gets shipped in `nightly`, it usually takes
several days to several weeks for it to be supported in a new `cargo-semver-checks`,
during which time it is not possible to use `cargo-semver-checks` with those `nightly` versions.

It's also possible that support for some `nightly` versions may be dropped even while older
stable versions are still supported. This usually happens when a rustdoc format gets superseded by
a newer version before becoming part of any stable Rust. In that case, we may drop support for
that format to conserve maintenance bandwidth and speed up compile times.
For example, `cargo-semver-checks` v0.24
[supported](https://github.com/obi1kenobi/cargo-semver-checks/blob/93baddc2a65a5055117ca670fe156dc761403fa5/Cargo.toml#L18) rustdoc formats v24, v26, and v27, but did not support the nightly-only v25 format.

### What if my project needs stronger guarantees around supported Rust versions?

If you'd like extended support for older Rust versions,
or an SLA on supporting new `nightly` releases, we're happy to offer those _on a commercial basis_.
It could be in the form of a formal support contract,
or something as simple as discussing expectations over email
and setting up a recurring GitHub sponsorship for an agreed-upon amount.

Please reach out at the email in the
[Cargo.toml](https://github.com/obi1kenobi/cargo-semver-checks/blob/93baddc2a65a5055117ca670fe156dc761403fa5/Cargo.toml#L5)
and let us know about what projects this is for and what their needs are.

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
([#160](https://github.com/obi1kenobi/cargo-semver-checks/issues/160)), so crates published on
registries other than crates.io should use one of the other approaches of generating the baseline.

### What features does `cargo-semver-checks` enable in the tested crates?

By default, checking is done on all features except features named `unstable`, `nightly`, `bench`, `no_std`, or ones with prefix `_`, `unstable-`, or `unstable_`, as such names are commonly used for private or unstable features.

This behaviour can be overriden. Checked feature set can be changed to:

- _all_ the features, selected with `--all-features`,
- only the crate's default features, selected with `--default-features`,
- empty set, selected with `--only-explicit-features`.

Additionally, features can be enabled one-by-one, using flags `--features`, `--baseline-features` and `--current-features`.

For example, consider crate [serde](https://github.com/serde-rs/serde), with the following features (per v1.0.163):

- `std` - the crate's only default feature,
- `alloc`, `derive`, `rc` - optional features,
- `unstable` - a feature that possibly breaks semver.

| used flags                                     | selected feature set                       | explanation                                                        |
| ---------------------------------------------- | ------------------------------------------ | ------------------------------------------------------------------ |
| none                                           | `std`, `alloc`, `derive`, `rc`             | Feature `unstable` is excluded by the default heuristic.           |
| `--features unstable`                          | `std`, `alloc`, `derive`, `rc`, `unstable` | The flag explicitly adds `unstable` to the heuristic's selections. |
| `--all-features`                               | `std`, `alloc`, `derive`, `rc`, `unstable` | All the features are used, disabling the default heuristic.        |
| `--default-features`                           | `std`                                      | The crate has only one default feature.                            |
| `--default-features --features derive`         | `std`, `derive`                            | Feature `derive` is used along with crate's default features.      |
| `--only-explicit-features`                     | none                                       | No explicit features are passed.                                   |
| `--only-explicit-features --features unstable` | `unstable`                                 | All features can be added explicitly, regardless of their name.    |

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
[doesn't yet have lints for all of them](https://github.com/obi1kenobi/cargo-semver-checks/issues/5).
New lints are added frequently, and
[we'd be happy to mentor you](https://github.com/obi1kenobi/cargo-semver-checks/blob/main/CONTRIBUTING.md)
if you'd like to contribute new lints!

Append `--verbose` when semver-checking your crate to see the full list of performed semver checks.

Here are some example areas where `cargo-semver-checks` currently will not catch semver violations:

- breaking type changes, for example in the type of a field or function parameter
- breaking changes in generics or lifetimes
- breaking changes that exist when only a subset of all crate features are activated

### If I really want a new feature to be implemented, can I sponsor its development?

Depending on the feature, possibly yes!

Please reach out to us _ahead of time_
[over email](https://github.com/obi1kenobi/cargo-semver-checks/blob/93baddc2a65a5055117ca670fe156dc761403fa5/Cargo.toml#L5)
to discuss the scope of the feature and sponsorship.

It's possible that the feature might be deemed out of scope, too complex to build or maintain,
or otherwise unsuitable. In such cases we'd like to let you know that _before_ you've sent us money,
since [there are no refunds on GitHub Sponsors](https://docs.github.com/en/billing/managing-billing-for-github-sponsors/downgrading-a-sponsorship#:~:text=When%20you%20downgrade%20or%20cancel,for%20payments%20for%20GitHub%20Sponsors.).

If the feature is viable and the work involved in building is commensurate to
the sponsorship amount, we'd be happy to build it.
At your option, we'd also be happy to give you a shout-out for sponsoring the feature when
it is announced in the release notes.

### How is `cargo-semver-checks` similar to and different from other tools?

[rust semverver](https://github.com/rust-lang/rust-semverver) builds on top of
rustc internals to build rlib's and compare their metadata. This strips the
code down to the basics for identifying changes. However, is tightly coupled
to specific nightly compiler versions and [takes work to stay in
sync](https://github.com/rust-lang/rust-semverver/search?q=Rustup+to&type=commits).
As of April 17, 2023, it appears to be
[deprecated and no longer maintained](https://github.com/rust-lang/rust-semverver/pull/390).

[cargo breaking](https://github.com/iomentum/cargo-breaking) effectively runs
`cargo expand` and re-parses the code using
[`syn`](https://crates.io/crates/syn) which requires re-implementing large
swaths of rust's semantics to then lint the API for changes.
This is limited to the feature and target the crate was compiled for.
As of November 22, 2022, it appears to be
[archived and no longer maintained](https://github.com/iomentum/cargo-breaking).

`cargo-semver-checks` sources its data from rustdoc's json output. While the
json output format is unstable, the rate of change is fairly low, reducing the
churn in keeping up. The lints are also written as queries for `trustfall`
["query everything" engine](https://github.com/obi1kenobi/trustfall), reducing
the work for creating and maintaining them. Because of the extra data that
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
[an unfortunate mishap](https://github.com/rust-lang/crates.io/issues/728#issuecomment-1182760954),
it remains `cargo-semver-checks` for the time being.

The `cargo_semver_check` name is reserved on crates.io but all its versions
are intentionally yanked. Please use the `cargo-semver-checks` crate instead.

### What is the MSRV policy with respect to semver?

MSRV bumps are _not_ considered major changes.

`cargo-semver-checks` has two Rust version bounds, since it depends on Rust
both at compile-time and at runtime:

- The MSRV for _compiling_ `cargo-semver-checks` (_"compile MSRV"_) is currently Rust 1.70.
  This is primarily determined by our dependencies' MSRVs.
- The MSRV for _checking crates_ (_"runtime MSRV"_) is currently Rust 1.71.
  This is determined based on the rustdoc JSON format versions and
  known bugs in older rustdoc versions.

As much as practically possible, changes to the _runtime MSRV_ will come in bumps
of the _middle_ number in the version, e.g. `0.24.1 -> 0.25.0` or `1.2.3 -> 1.3.0`.

Changes to the _compile MSRV_ may happen in any kind of version bump.
As much as practically possible, we'll aim to make them
simultaneously with _runtime MSRV_ bumps.

### Can I configure individual lints?

Yes! See [lint-level configuration](#lint-level-configuration)

## Configuration

### Lint-level configuration

Sometimes, it is useful to be able to control and enable/disable individual lints that `cargo-semver-checks` runs.  On the `main` branch, `cargo-semver-checks` has this functionality.  First, a refresher on some definitions:

- **check/lint** - `cargo-semver-checks` runs many checks between versions of a crate's public API, which each check for one specific semver-breaking behavior.  For example, the `function_missing` lint is triggered when a function in the crate's public API is removed in a new version.
- **lint level** - For each of these checks, it is now possible to configure the level of severity for when this check occurs.  `deny` means it is a hard error when the check finds breaking behavior, which will require a version bump to resolve.  `warn` still runs the check and reports findings, but it is only a warning if it triggers and does not require (but it does suggest) a version bump.  `allow` means that the breaking behavior in this check is allowed in the code, so the check doesn't need to be run.
- **required (semver) update** - what version bump this check should require (for `deny`-level) or suggest (for `warn`) when the check is triggered.  For example, `function_missing` is `major` by default, so if a public function is removed between versions, the version needs a major version bump (e.g., 1.2.3 to 2.0.0 or 0.5.2 to 0.6.0).  This can be configured to `major` or `minor` (1.2.3 to 1.3.0 or 0.5.2 to 0.5.3).


To configure the level and/or required update for a lint, first find its id.  This will be in `snake_case` and is reported on error/warning or found as the file name in the [lints folder](https://github.com/obi1kenobi/cargo-semver-checks/tree/main/src/lints).  Let's use `function_missing` as an example.

Then, determine what the new lint level and/or required version update should be.  For our example, let's set the level to `warn` and the required update to `minor`.

In the `Cargo.toml` manifest for the package we're checking, add the `cargo-semver-checks` lints table:

```toml
# ...
[package.metadata.cargo-semver-checks.lints]
function_missing = { level = "warn", required-update = "minor" }
```

To configure other lints, just add them as additional entries to that table.  Note that it is not required to configure *both* lint level and required version update, and you can use the shorthand `lint_id = "level"` to configure just the lint level:

```toml
[package.metadata.cargo-semver-checks.lints]
function_missing = { level = "warn", required-update = "minor" }
trait_now_doc_hidden = "allow" # shorthand to set just lint level to `allow`
enum_variant_added = { level = "deny" } # leaves required-update as the default
function_changed_abi = { required-update = "minor" } # leaves level as default
```

This table should be placed in the current/new `Cargo.toml` manifest, not the baseline/old manifest.  (this defaults to `./Cargo.toml` in the directory that `cargo-semver-checks` was invoked on, and can be configured to a different path with the `--manifest-path` CLI option).

#### Configuring a workspace

It may also be helpful to configure these checks at the workspace level for multiple different crates in the same Cargo workspace. To do this, in the workspace root `Cargo.toml`, add a similar configuration table:

```toml
[workspace.metadata.cargo-semver-checks.lints]
function_missing = { level = "warn", required-update = "minor" }
```

Note that it is `workspace.metadata` and not `package.metadata`.  Then, to opt-in to these for each package that is being semver-checked, one of these keys to the *package* Cargo.toml:

```toml
[lints]
workspace = true
```

or

```toml
[package.metadata.cargo-semver-checks.lints]
workspace = true
```

Either of these is acceptable to indicate to read the `[workspace.metadata.cargo-semver-checks.lints]` table for this package.  Using the `lints.workspace` key can be helpful to indicate this behavior for other linters (e.g., cargo and clippy), but will cause a cargo error if it is set and there is no `[workspace.lints]` table in the workspace Cargo.toml.  To solve this, we added the `package.metadata.cargo-semver-checks.lints.workspace` key for compatibility. Note that setting `workspace = false` is not valid configuration for either of these keys.  To indicate not to read workspace configuration for a crate, simply omit both keys entirely from the package's Cargo.toml.

#### Overriding workspace configuration

When `workspace = true` is set, it is possible to override individual lint configuration lines in a package.  For example, if we have in the *workspace* Cargo.toml:

```toml
[workspace.metadata.cargo-semver-checks.lints]
function_missing = { level = "warn", required-update = "minor" }
trait_now_doc_hidden = "warn"
```

and in the *package* Cargo.toml:

```toml
[package.metadata.cargo-semver-checks.lints]
workspace = true
function_missing = "deny"
trait_now_doc_hidden = { required-update = "major" }
```

Fields set in the package configuration override the workspace configuration.  Thus, the lint level for `function_missing` will be overridden to `deny`, and the workspace default `required-update` of `minor` will be used because it is not configured in the package.  Similarly, the lint level for `trait_now_doc_hidden` will be the workspace's `warn`, but the required update will be overridden in the package to a `major` version bump.

#### Limitations

Currently, the configuration can only be read from the new version of the checked crate's Cargo.toml.  If this manifest isn't used (for example, when using the `--current-rustdoc` to use a preexisting rustdoc output instead, it is not possible to configure lints.  Configuration through other means (e.g., CLI flags) could be added in the future if there is a strong demand for them.

### Visual Design

Logo by [NUMI](https://github.com/numi-hq/open-design):

[<img src="https://raw.githubusercontent.com/numi-hq/open-design/main/assets/numi-lockup.png" alt="NUMI Logo" style="width: 200px;"/>](https://numi.tech/?ref=semver-checks)

### License

Available under the Apache License (Version 2.0) or the MIT license, at your option.

Copyright 2022-present Predrag Gruevski and Project Contributors.
The present date is determined by the timestamp of the most recent commit in the repository.
Project Contributors are all authors and committers of commits in the repository.
