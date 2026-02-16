# Lint authoring guidelines

Each `*.ron` file in this directory specifies a lint, as a serialized Rust `SemverQuery` value.
Lints are always named in `snake_case`.

To create a new lint, run `scripts/make_new_lint.sh <lint-name>` from the repo root.

## File & metadata structure
Name each file `<lint_id>.ron`; keep the `id` field identical to the filename.

Begin each `SemverQuery` with the metadata block in this order:
1. `id`, `human_readable_name`, and optional `//` context comments.
2. `description` as a concise complete sentence.
3. Severity settings: `required_update`, `lint_level`.
4. Optional references: `reference_link` (URL string) and `reference: Some(r#"..."#)` for (concise) prose.

When deciding what reference link to use, follow this order:
1. If [the Rust SemVer reference](https://doc.rust-lang.org/cargo/reference/semver.html) has a relevant section, use that.
2. If [the Rust language reference](https://doc.rust-lang.org/reference/) has a section describing the cause of the breakage (e.g. the attribute that causes the breakage), use that.
3. If there is an issue on [the `cargo-semver-checks` issue tracker](https://github.com/obi1kenobi/cargo-semver-checks/issues), or a public writeup from a reputable source (such as on https://predr.ag/blog/ ) that explains the breakage or explains another reason the lint is desirable, use that.
4. Otherwise, set `reference_link` to `None`.

Strings that contain Handlebars helpers or newlines should use raw string syntax (`r#"..."#`).

After these metadata fields, produce the remaining fields in the following order: `query`, `arguments`, `error_message`, `per_error_result_template`, `witness`.

## Trustfall query conventions
The `query` field specifies a Trustfall query to run.
Trustfall is the query engine found here: https://github.com/obi1kenobi/trustfall
Its documentation can be found here: https://github.com/obi1kenobi/trustfall/tree/main/docs/docs

The schema for the query is defined in the https://github.com/obi1kenobi/trustfall-rustdoc-adapter project, in its `src/rustdoc_schema.graphql`.

Every query is a raw string literal formatted as:
```graphql
{
    CrateDiff {
        baseline {
            # contents here
        }
        current {
            # contents here
        }
    }
}
```
The `baseline` and `current` edges might come in either order, depending on each query's needs.

Follow these rules when drafting queries:
- List scalar & list of scalar properties before edges.
- Separate logical groups with a blank line.
- Lints looking for item removals find the item in `baseline` then assert its non-existence in `current`. Lints for additions do the inverse. Lints where something merely changed could be written in either order: always prefer to check the less likely scenario first, to make queries more efficient.
- Use `@tag` to capture data in the query and `%tag` inside `@filter` to reuse it anywhere after the `@tag`.
- Use `@fold @transform(op: "count")` to assert presence/absence (`= $zero`, `> $zero`, etc.) and to deduplicate results as needed.
- Multiple inline fragments (`... on X`) cannot be stacked at the same level in a single scope. Either split the logic into separate scopes/folds, or use `__typename` plus a `@filter` for the expected type combination.
- If a line with multiple directives gets longer than about 100 characters, put each directive on its own line, indented one 4-space step with all directives vertically aligned with each other.
- Gate results to the public API before following edges: combine `visibility_limit = $public` with `public_api` or `public_api_eligible` as appropriate. Only omit a gate when an inline `#` comment explains why.
- Alias reusable edges with a trailing underscore (`span_`, `abi_`, …) so templates can rely on fields like `span_filename`. An alias applied to an edge acts as a prefix for all output names inside that's edge subtree.
- When span data is unavailable, emit an explicit ordering key (for example, re-outputting a name) to keep deterministic result ordering. See the `feature_missing` lint for an example.
- When multiple spans are emitted, the span that appears in the diagnostic should be aliased with `span_` while the other spans should have an appropriate prefix describing their nature: `baseline_span_`, `current_span_`, `non_matching_span_`, etc.
- Inline query comments use `#` and align with surrounding code; Rust-side (RON file) comments use `//`.

## Arguments & reusable constants
- Declare every `$parameter` inside the `arguments` map.
- Reuse shared constants such as `"public"`, `true`, or `0` instead of inlining literals directly in the query.
- Prefer to reuse values in queries, for example using `= $true` and `!= $true` instead of defining separate `$true` and `$false` arguments.

## Messaging & witnesses
- `error_message` should concisely describe the change and why it matters to downstream users.
- `per_result_error_template` is a Handlebars string that references the query outputs (including span data when available). Keep it concise aiming to fit the output on one terminal line. It often uses helpers like such as `{{join "::" path}}` or `{{lowercase enum_kind}}`. Keep the formatting consistent with existing lints.
- Provide a `witness` hint when possible, using the same templating conventions. If a witness is not yet viable (manifest-only lint, unresolved witness design, etc.), set `witness: None` and explain the omission with comments or an issue link.

## Duplicate suppression & scope hygiene
- Filter out items that other lints already cover (for example, doc-hidden or `#[non_exhaustive]` changes) so diagnostics do not overlap. Document unusual exclusions inline.
- Ensure queries only surface public, importable items to avoid false positives from private or unreachable APIs.
- Items can be importable (and have data for the `importable_path` edge) even if they aren't public API because of `#[doc(hidden)]`. If a lint should target only public API items, a `public_api` filter for `true` should be used inside `importable_path`. Make sure to test such lints with `#[doc(hidden)]` items.
- Avoid negative test cases where the old/new code is identical; those are redundant since we already test old-old and new-new comparisons to ensure zero lint results.

## Meta comments

If at any point you feel it's best to deviate from these rules, you must ALWAYS point out and explain the reason for the deviation in an adjacent comment.
