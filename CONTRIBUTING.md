# Contributing

- [Making your first contribution](#making-your-first-contribution)
- [Design goals](#design-goals)
- [Running `cargo test` for the first time](#running-cargo-test-for-the-first-time)
- [Adding a new lint](#adding-a-new-lint)
- [Development Environment](#development-environment)

## Making your first contribution

Thanks for taking the time to contribute!
The best starting point is adding a new lint.
[Here is a list](https://github.com/obi1kenobi/cargo-semver-checks/issues?q=is%3Aopen+label%3AE-mentor+label%3AA-lint)
of lints that have all their prerequisites met and are ready to be added,
and which have mentorship available.
Please make use of the mentorship opportunity by asking questions in the relevant GitHub issue!

After choosing a lint to implement, try to identify a related lint that is already implemented
and relies on similar information.
For example, if implementing a lint that uses information about attributes,
find other lints that check attribute information and use them as guides as you write your lint.

Make sure to check the ["Development Environment"](#development-environment) section, especially if you are using Windows.

The ["Adding a new lint"](#adding-a-new-lint) section of this document has a walkthrough for
defining and testing new lints.

Please see the ["Running `cargo test` for the first time"](#running-cargo-test-for-the-first-time)
section to generate the test rustdoc JSON data the tests require. Failing to run this step
will cause `cargo test` failures.

The design of `cargo-semver-checks` is documented in the [Design goals](#design-goals) section.
`cargo-semver-checks` uses the [Trustfall](https://github.com/obi1kenobi/trustfall) query engine,
which in turn uses GraphQL syntax with non-standard semantics.
These extensions were originally developed for a previous project ("GraphQL compiler"),
and have been streamlined and further developed in Trustfall.
Trustfall documentation is unfortunately still minimal and still consists largely of examples,
but most Trustfall query functionality is nearly identical
(down to trivial parameter naming differences) to the query functionality documented in
[the GraphQL compiler query reference](https://graphql-compiler.readthedocs.io/en/latest/language_specification/query_directives.html).

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
[schema](https://github.com/obi1kenobi/trustfall-rustdoc-adapter/blob/rustdoc-v28/src/rustdoc_schema.graphql).
A query playground, including example queries, [is available here](https://play.predr.ag/rustdoc).

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
[support running custom user-specified checks](https://github.com/obi1kenobi/cargo-semver-checks/issues/38)
on top of the same rustdoc JSON + cargo manifest data it uses today.
Checks are configuration, not code: the custom checks would just be a set of files that
`cargo-semver-checks` is configured to run.

Similarly, `cargo-semver-checks` could warn about potentially-undesirable API changes that
may have been done unintentionally, and which could have semver implications without being breaking.
An example is removing the last private field of a `pub struct` that is not `#[non_exhaustive]`:
this would have the side-effect of adding to the public API the ability to construct the struct
with a literal. If this change were published accidentally, undoing the change would be breaking
and would require a new major version. More examples of such useful-but-not-semver checks are
[here](https://github.com/obi1kenobi/cargo-semver-checks/issues/5).

## Running `cargo test` for the first time

Testing this crate requires rustdoc JSON output data, which is too large and variable
to check into git. It has to be generated locally before `cargo test` will succeed,
and will be saved in a `localdata` gitignored directory in the repo root.

To generate this data, please run `./scripts/regenerate_test_rustdocs.sh`.
To use a specific toolchain, like beta or nightly, pass it as
an argument: `./scripts/regenerate_test_rustdocs.sh +nightly`.

## What are those `.snap` or `.snap.new` files generated via `cargo test` 

As part of our overall testing strategy, we use a technique called "snapshot testing."
The tool we use for this ([`insta`](https://insta.rs/docs/)) is user friendly and has mutliple ways to interact with it:

These snapshots are by default written to `.snap.new` files (because `INSTA_UPDATE` explained below defaults to `auto`) if they differ and fail the testcase.
Reviewing them is possible via these options:

1. **With `cargo-insta`**: If you install (or have already installed) the `insta` CLI with
   `cargo install --locked cargo-insta`, you can run `cargo insta review`. Check that the
   new output is what you expect, and accept it in the TUI.
2. **Without `cargo-insta`**:
   From [`insta`s docs](https://insta.rs/docs/quickstart/#tests-without-insta):
   > You can also just use insta directly from cargo test and control it via the `INSTA_UPDATE` environment variable.
   > The default is auto which will write all new snapshots into .snap.new files if no CI is detected so that cargo-insta can pick them up. The following other modes are possible:
   > - `auto`: the default. no for CI environments or new otherwise
   > - `always`: overwrites old snapshot files with new ones unasked
   > - `unseen`: behaves like always for new snapshots and new for others
   > - `new`: write new snapshots into .snap.new files
   > - `no`: does not update snapshot files at all (just runs tests)

   Thus, if you run the following command, you can accept the current snapshots after reviewing the `.snap.new` files.
   ```text
   INSTA_UPDATE=always cargo test
   ```
3. **Manually**: If you can't (or don't want to) use `cargo-insta`, you can verify the snapshot
   file manually.  There should be a file called `test_outputs/<some_path>/<lint_name>.snap.new`.
   Open it, and verify that its contents match what you expected: all expected data is present, and no unexpected data is included. 
   Once you've checked it, remove the `.new` suffix so that the file's new path
   is `test_outputs/<some_path>/<lint_name>.snap`

## Adding a new lint

### Background

Lints are written as queries for the `trustfall` ["query everything" engine](https://github.com/obi1kenobi/trustfall).

Each lint is defined in its own file
in [`src/lints`](https://github.com/obi1kenobi/cargo-semver-checks/tree/main/src/lints).

Lints are tested by running them on a series of test crates defined in
the [`test_crates` directory](https://github.com/obi1kenobi/cargo-semver-checks/tree/main/test_crates).
Each test crate comes in two versions: `old` which represents the semver baseline,
and `new` which represents a semver patch-level update to the crate.

Each `new` crate version is generally expected to trigger one or more lints due to violating semver.
The expected outputs for each of the lints are stored in per-lint files in
the [`test_outputs` directory](https://github.com/obi1kenobi/cargo-semver-checks/tree/main/test_outputs).

### Walkthrough for adding a new lint

First, choose an appropriate name for your lint. We'll refer to it as `<lint_name>`.

We'll use the [`scripts/make_new_lint.sh`](https://github.com/obi1kenobi/cargo-semver-checks/tree/main/scripts/make_new_lint.sh) script to automatically create the necessary file stubs, which you'll then fill in. It will:
- Add a new lint file: `src/lints/<lint_name>.ron`.
- Create a new test crate pair: `test_crates/<lint_name>/old` and `test_crates/<lint_name>/new`.
- Add an empty expected test outputs file: `test_outputs/query_execution/<lint_name>.snap`.
- Register your new lint in the `add_lints!()` macro near the bottom of [`src/query.rs`](https://github.com/obi1kenobi/cargo-semver-checks/tree/main/src/query.rs).

Now it's time to fill in these files!
- Define the lint in `src/lints/<lint_name>.ron`.
- Make sure your lint outputs `span_filename` and `span_begin_line` for it
  to be a valid lint. The pattern we commonly use is:
  ```
  span_: span @optional {
    filename @output
    begin_line @output
  }
  ```
- Demonstrate the semver issue your lint is looking for by adding suitable code in
  the `test_crates/<lint_name>/old` and `test_crates/<lint_name>/new` crates.
- Add code to the test crates that aims to catch for false-positives
  and/or true-but-unintended-positives your query might report.
  For example, a true-but-unintended output would be if a query that looks for
  removal of public fields were to report that a struct was removed.
  Struct removal has its own lint, so there's no reason to also report that
  the removed struct also had its fields removed.
- Re-run [`./scripts/regenerate_test_rustdocs.sh`](https://github.com/obi1kenobi/cargo-semver-checks/tree/main/scripts/regenerate_test_rustdocs.sh)
  to generate rustdoc JSON files for your new test crates.

At this point, everything is wired up to let you test your new lint -- but
the expected outputs file is still empty. That's okay for now!

Run `cargo test` and make sure your new lint's test is running and **is failing**.
If it didn't fail, then your lint didn't report any semver issues in the test crate,
and probably isn't working quite right.
(Did any other lints' tests fail also? See the [troubleshooting item](#troubleshooting-other-lints-tests-failed-too) below.)

For a lint named `enum_struct_variant_field_added`, you'll probably see its test fail with
a message similar to this:
```
---- query::tests_lints::enum_struct_variant_field_added stdout ----
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ Snapshot Summary ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Snapshot file: src/../test_outputs/query_execution/enum_struct_variant_field_added.snap
Snapshot: enum_struct_variant_field_added
Source: src/query.rs:646
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
Expression: &query_execution_results
────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
-old snapshot
+new results
────────────┬───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
    0     0 │ {
    1       │-  "./test_crates/enum_struct_variant_field_added/": []
          1 │+  "./test_crates/enum_no_repr_variant_discriminant_changed/": [
          2 │+    {
          3 │+      "enum_name": String("UnitOnlyBecomesUndefined"),
          4 │+      "field_name": String("a"),
          5 │+      "path": List([
          6 │+        String("enum_no_repr_variant_discriminant_changed"),
          7 │+        String("UnitOnlyBecomesUndefined"),
          8 │+      ]),
          9 │+      "span_begin_line": Uint64(77),
         10 │+      "span_filename": String("src/lib.rs"),
         11 │+      "variant_name": String("Struct"),
         12 │+    },
         13 │+  ],
         14 │+  "./test_crates/enum_struct_field_hidden_from_public_api/": [
         15 │+    {
         16 │+      "enum_name": String("AddedVariantField"),
         17 │+      "field_name": String("y"),
         18 │+      "path": List([
         19 │+        String("enum_struct_field_hidden_from_public_api"),
         20 │+        String("AddedVariantField"),
         21 │+      ]),
         22 │+      "span_begin_line": Uint64(38),
         23 │+      "span_filename": String("src/lib.rs"),
         24 │+      "variant_name": String("StructVariant"),
         25 │+    },
         26 │+  ],
         27 │+  "./test_crates/enum_struct_variant_field_added/": [
         28 │+    {
         29 │+      "enum_name": String("PubEnum"),
         30 │+      "field_name": String("y"),
         31 │+      "path": List([
         32 │+        String("enum_struct_variant_field_added"),
         33 │+        String("PubEnum"),
         34 │+      ]),
         35 │+      "span_begin_line": Uint64(4),
         36 │+      "span_filename": String("src/lib.rs"),
         37 │+      "variant_name": String("Foo"),
         38 │+    },
         39 │+  ],
    2    40 │ }
────────────┴───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────
To update snapshots run `cargo insta review`
Stopped on the first failure. Run `cargo insta test` to run all snapshots.
```

Inspect the generated "actual" output in the `.snap.new` file:
- Does it report the semver issue your lint was supposed to catch? If not, the lint query
  or the test crates' code may need to be tweaked.
- Does it report correct span information? Is the span as specific as possible, for example
  pointing to a struct's field rather than the whole struct if the lint refers to that field?
- Does the output also report any code from test crates other than `test_crates/<lint_name>`?
  If so, ensure the reported code is indeed violating semver and is not being flagged
  by any other lint.

If everything looks okay, either run `cargo insta review` (see 
the [snapshot instructions](#what-are-those-snap-or-snapnew-files-generated-via-cargo-test) 
for context) or manually move `test_outputs/query_execution/<lint_name>.snap.new`
to  `test_outputs/query_execution/<lint_name>.snap`.
Then re-run `cargo test` and make sure everything passes.

Congrats on the new lint!

#### Adding a witness

**Witnesses** are a new, unstable feature of `cargo-semver-checks` that let us create
minimal compile-able examples of potential downstream breakage. They are configured via the
`witness` field in the lint file `src/lints/<lint_name>.ron`:

If it is `None` (or the field is omitted entirely), `cargo-semver-checks` will not be able
to generate witness code for this lint. This can be the right choice, sometimes:
for example, if this lint is a warn- or allow-by-default lint that hints at potential
breakage, but won't cause breaking changes directly. Additionally, if it's not currently
possible to write a Trustfall query that gets the necessary information to generate
witnesses, leave this field as `None`, but leave a `// TODO` comment explaining
what would unblock this lint from being able to generate a witness.

##### Hint templates

When the `witness` field is not `None`, it must have the `hint_template` field. This is a
`handlebars` template that generates a small (1-3 line) human-readable message that
explains the idea of how downstream code would break.

This example code is meant to be small and illustrative, and does not have to pass `cargo check`.
It should give the reader a sense of the kind of breakage in one glance.

For example, for the `function_missing` lint, a witness template may look like this:

```ron
witness: (
  hint_template: r#"{{join "::" path}(...);"#,
),
```

which could render to something like:

```rust
function_missing::will_be_removed_fn(...);
```

This hint will not pass `cargo check` (e.g. the function call arguments are elided),
and that's okay. The hint is a distilled example of breakage, and shouldn't require
additional information beyond what the lint query retrieved.

##### Templating

We use the `handlebars` crate for writing these templates. [More information
about the syntax can be found here](https://handlebarsjs.com/guide/#simple-expressions),
and [here is where `cargo-semver-checks` defines custom helpers
](https://github.com/obi1kenobi/cargo-semver-checks/blob/main/src/templating.rs).

All fields marked with `@output` in the `query` in `<lint_name>.ron` are available
to access with `{{output_name}}` in the `hint_template`, like in the example above.

##### Testing witnesses

When the `witness` field is not `None`, `cargo-semver-checks` tests the witness generation
of the lint similarly to how it tests the `query` itself. After adding a witness for the first
time, run `cargo test` to start generating the snapshots.  The first time you run this test,
it will fail, because there's no expected result to compare to.  Let's make the test pass:

We use `insta` for snapshot testing witness results, so after adding/changing a witness, we need
to update the test outputs.

> [!TIP]
> It may contain output for other test crates — this is not necessarily an error:
> See the [troubleshooting section](#troubleshooting) for more info.

To update the output, please refer to the section 
on [snapshot testing](#what-are-those-snap-or-snapnew-files-generated-via-cargo-test)

Once you've update the test output, run `cargo test` again and the `<lint_name>` test should pass!
**Make sure to commit and push the `test_outputs/witnesses/<lint_name>.snap` into git**;
otherwise the test will fail in CI.

##### Full witness templates

*TODO: @suaviloquence will write this once the feature has been implemented.*

### Troubleshooting

- <details><summary>A valid query must output <code>span_filename</code> and/or <code>span_begin_line</code> (click to expand)</summary>
  
  If your lint fails with an error similar to the following:
  ```
  ---- query::tests_lints::enum_missing stdout ----
  thread 'query::tests_lints::enum_missing' panicked at 'A valid query must output both `span_filename` and `span_begin_line`. See https://github.com/obi1kenobi/cargo-semver-checks/blob/main/CONTRIBUTING.md for how to do this.', src/query.rs:395:26
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
  ```
  
  It likely means that your lint does not specify the `span_filename` and `span_begin_line` of where the error occurs. To fix this, add the following to the part of query that catches the error:
  ```
  span_: span @optional {
    filename @output
    begin_line @output
  }
  ```
  
  </details>
- <details><summary>Other lints' tests failed too (click to expand)</summary>

  This is not always a problem! In process of testing a lint, it's frequently desirable to include
  test code that contains a related semver issue in order to ensure the lint differentiates between
  them.
  
  For example, say one is testing a lint for pub field removals from a struct. Its test crate code
may then include removals of the entire struct, in order to make sure that the lint *does not*
report those. But those struct removals *will* get reported by the lint that looks for semver
violations due to struct removal!
  
  So if you added code to a test crate, and it caused other lints to report new findings, consider:
  - whether your code indeed contains the reported semver issue;
  - whether the same semver issue is being reported only once, and not multiple times
    by different lints,
  - and whether the new reported lint result points to the correct item and span information.
  
  If the answer to all is yes, then everything is fine! Just edit those other lints'
expected output files to include the new items, and you can get back on track.
  </details>

## Development Environment

### Running required automation scripts

While cargo-semver-checks is cross-platform, the development task automation scripts in the scripts
directory require a `bash` shell to run.

Windows users can get a bash + GNU command line environment via WSL or git bash.
Linux and macOS typically have bash installed by default.

### Language server (LSP) integration

It's possible to use a GraphQL language server to
[assist with writing queries](https://github.com/obi1kenobi/trustfall/discussions/679)
by offering schema-aware autocomplete and other functionality.

To make this work, you'll need to point it to a schema file. The schema and Trustfall adapter
`cargo-semver-checks` uses are defined in the `trustfall-rustdoc-adapter` crate:
https://github.com/obi1kenobi/trustfall-rustdoc-adapter
