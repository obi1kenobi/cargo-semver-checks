# Rustdoc schema – full type and edge list

This is a flattened reference from `trustfall-rustdoc-adapter`’s `rustdoc_schema.graphql`.  
**Source:** `~/.cargo/registry/src/.../trustfall-rustdoc-adapter-57.0.4/src/rustdoc_schema.graphql`  
(or https://github.com/obi1kenobi/trustfall-rustdoc-adapter/blob/rustdoc-v57/src/rustdoc_schema.graphql)

---

## Root

| Type | Edges (fields) |
|------|-----------------|
| **RootSchemaQuery** | `Crate`, `CrateDiff` |
| **CrateDiff** | `current` (Crate!), `baseline` (Crate) |

---

## Crate

| Type | Edges (fields) |
|------|-----------------|
| **Crate** | `root`, `crate_version`, `includes_private`, `format_version`, `root_module`, `item`, `ffi_exported_function`, `feature`, `default_feature` |

---

## Item (interface) – base for all items

| Type | Edges (fields) |
|------|-----------------|
| **Item** | `id`, `crate_id`, `name`, `docs`, `attrs`, `doc_hidden`, `deprecated`, `public_api_eligible`, `visibility_limit`, `attribute`, `span` |

---

## GenericItem (interface) – Item + generics

| Type | Edges (fields) |
|------|-----------------|
| **GenericItem** | (all Item) + `generic_parameter` |

---

## Importable (interface) – Item + paths

| Type | Edges (fields) |
|------|-----------------|
| **Importable** | (all Item) + `importable_path`, `canonical_path` |

---

## ImplOwner (interface) – Item + Importable + GenericItem + impls

| Type | Edges (fields) |
|------|-----------------|
| **ImplOwner** | (all above) + `impl`, `inherent_impl` |

---

## Concrete types (vertices)

| Type | Edges (fields) |
|------|-----------------|
| **Module** | Item + Importable + `is_stripped`, `item` |
| **Struct** | Item + Importable + ImplOwner + GenericItem + `struct_type`, `fields_stripped`, `impl`, `inherent_impl`, `generic_parameter`, `field` |
| **StructField** | Item + `position`, `span`, `attribute`, `raw_type` |
| **Enum** | Item + Importable + ImplOwner + GenericItem + `variants_stripped`, `impl`, `inherent_impl`, `generic_parameter`, `variant` |
| **Variant** (interface) | Item + Importable + `position`, `field`, `discriminant` |
| **PlainVariant** | Item + Importable + Variant |
| **TupleVariant** | Item + Importable + Variant |
| **StructVariant** | Item + Importable + Variant |
| **Union** | Item + Importable + ImplOwner + GenericItem + `fields_stripped`, `impl`, `inherent_impl`, `generic_parameter`, `field` |
| **Discriminant** | `value` |
| **Span** | `filename`, `begin_line`, `begin_column`, `end_line`, `end_column` |
| **Impl** | Item + GenericItem + `unsafe`, `negative`, `synthetic`, `generic_parameter`, `implemented_trait`, `method`, `associated_constant` |
| **Trait** | Item + Importable + GenericItem + `unsafe`, `object_safe`, `dyn_compatible`, `unconditionally_sealed`, `sealed`, `public_api_sealed`, `generic_parameter`, `method`, `supertrait`, `associated_type`, `associated_constant` |
| **ImportablePath** | `visibility_limit`, `doc_hidden`, `deprecated`, `public_api`, `path` |
| **Path** | `path` |
| **FunctionLike** (interface) | `const`, `unsafe`, `async`, `has_body`, `signature`, `parameter`, `return_value`, `abi` |
| **ExportableFunction** (interface) | Item + FunctionLike + `export_name` |
| **FunctionParameter** | `name` |
| **FunctionAbi** | `name`, `raw_name`, `unwind` |
| **RequiredTargetFeature** | `name`, `explicit`, `globally_enabled`, `valid_for_current_target` |
| **Function** | Item + FunctionLike + Importable + ExportableFunction + GenericItem + `importable_path`, `canonical_path`, `generic_parameter`, `requires_feature` |
| **Method** | Item + FunctionLike + GenericItem + `export_name`, `generic_parameter`, `receiver`, `requires_feature` |
| **Receiver** | `by_value`, `by_reference`, `by_mut_reference`, `kind` |
| **ReturnValue** | `is_unit` |
| **GlobalValue** (interface) | Item + Importable |
| **Constant** | Item + Importable + GlobalValue + `expr`, `value`, `is_literal` |
| **Static** | Item + Importable + GlobalValue + `mutable`, `unsafe`, `export_name` |
| **Attribute** | `raw_attribute`, `is_inner`, `content` |
| **AttributeMetaItem** | `raw_item`, `base`, `assigned_item`, `argument` |
| **ImplementedTrait** | `bare_name`, `name`, `instantiated_name`, `trait_id`, `trait` |
| **AssociatedType** | Item + `has_default` |
| **AssociatedConstant** | Item + `default` |
| **Macro** | Item + Importable |
| **ProcMacro** (interface) | Item + Importable |
| **FunctionLikeProcMacro** | Item + Importable + ProcMacro |
| **AttributeProcMacro** | Item + Importable + ProcMacro |
| **DeriveProcMacro** | Item + Importable + ProcMacro + `helper_attribute` |
| **DeriveMacroHelperAttribute** | `name` |
| **GenericParameter** (interface) | `name`, `position` |
| **GenericTypeParameter** | GenericParameter + `has_default`, `synthetic`, `type_bound`, `maybe_sized` |
| **GenericLifetimeParameter** | GenericParameter |
| **GenericConstParameter** | GenericParameter + `has_default` |
| **RawType** (interface) | `name` |
| **ResolvedPathType** | RawType |
| **Feature** | `name`, `directly_enables`, `transitively_enables` |

---

## Quick edge index (where to go from each type)

- **CrateDiff** → `baseline` (Crate), `current` (Crate)
- **Crate** → `root_module` (Module), `item` ([Item]), `ffi_exported_function`, `feature`, `default_feature`
- **Module** → `item` ([Item]), `importable_path`, `canonical_path`, `span`, `attribute`
- **Struct / Enum / Union** (ImplOwner) → `importable_path`, `canonical_path`, `impl`, **`inherent_impl`** ([Impl]), `generic_parameter`, `field` or `variant`
- **Impl** → **`method`** ([Method]), `associated_constant`, `implemented_trait`, `generic_parameter`, `span`, `attribute`
- **Method** → `receiver`, `parameter`, `return_value`, `abi`, `span`, `attribute`, `generic_parameter`, `requires_feature` + scalars: **`const`**, `unsafe`, `name`, `signature`, etc.
- **ImportablePath** → scalar **`path`** ([String!]), `public_api`, `visibility_limit`, `doc_hidden`, `deprecated`
- **Span** → `filename`, `begin_line`, `end_line`, `begin_column`, `end_column`

---

## Directives (Trustfall)

- `@filter(op: String!, value: [String!])`
- `@tag(name: String)`
- `@output(name: String)`
- `@optional`
- `@recurse(depth: Int!)`
- `@fold`

---

*Generated from rustdoc_schema.graphql for cargo-semver-checks.*
