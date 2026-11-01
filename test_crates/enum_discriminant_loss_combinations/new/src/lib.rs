#![no_std]

// Report each independent cause of lost castability once per enum, with its own lint:
// - Drop: enum_discriminants_undefined_drop.
// - Fields: enum_discriminants_undefined_variant_fields.
// - Non-exhaustive variants: enum_discriminants_undefined_non_exhaustive_variant.
// - Explicit non-unit discriminants:
//   enum_discriminants_undefined_non_unit_explicit_discriminant.
// Multiple causes on the same enum or variant must not suppress one another.
// Explicit non-unit discriminants require an integer repr. Those cases use repr(u8);
// existing C and integer representations are preserved from the baseline.

// No discriminant-loss lint: adding another fieldless variant preserves casts.
#[non_exhaustive]
pub enum StillCastable {
    Tuple(),
    Struct {},
    Added,
}

// Reports Drop.
#[non_exhaustive]
pub enum DropOnly {
    Tuple(),
    Struct {},
}

impl Drop for DropOnly {
    fn drop(&mut self) {}
}

// Reports fields.
#[non_exhaustive]
pub enum FieldsOnly {
    Tuple(),
    Struct {},
    Fields(u8),
}

// Reports a non-exhaustive variant.
#[non_exhaustive]
pub enum NonExhaustiveOnly {
    Tuple(),
    Struct {},
    #[non_exhaustive]
    Added,
}

// Reports explicit non-unit discriminants.
#[repr(u8)]
#[non_exhaustive]
pub enum ExplicitOnly {
    Tuple(),
    Struct {},
    Explicit() = 2,
}

// Reports Drop and fields.
#[non_exhaustive]
pub enum DropAndFields {
    Tuple(),
    Struct {},
    Fields { value: u8 },
}

impl Drop for DropAndFields {
    fn drop(&mut self) {}
}

// Reports Drop and a non-exhaustive variant.
#[non_exhaustive]
pub enum DropAndNonExhaustive {
    Tuple(),
    Struct {},
    #[non_exhaustive]
    Added,
}

impl Drop for DropAndNonExhaustive {
    fn drop(&mut self) {}
}

// Reports Drop and explicit non-unit discriminants.
#[repr(u8)]
#[non_exhaustive]
pub enum DropAndExplicit {
    Tuple(),
    Struct {},
    Explicit {} = 2,
}

impl Drop for DropAndExplicit {
    fn drop(&mut self) {}
}

// Reports fields and non-exhaustiveness for the same added variant.
#[repr(C)]
#[non_exhaustive]
pub enum FieldsAndNonExhaustive {
    Tuple(),
    Struct {},
    #[non_exhaustive]
    Fields(u8),
}

// Reports fields and explicit non-unit discriminants independently.
#[repr(u8)]
#[non_exhaustive]
pub enum FieldsAndExplicit {
    Tuple(),
    Struct {},
    Fields(u8),
    Explicit {} = 3,
}

// Reports an explicit non-unit discriminant and non-exhaustiveness
// for the same added variant.
#[repr(u8)]
#[non_exhaustive]
pub enum NonExhaustiveAndExplicit {
    Tuple(),
    Struct {},
    #[non_exhaustive]
    Explicit() = 2,
}

// Reports Drop, fields, and a non-exhaustive variant.
#[repr(u8)]
#[non_exhaustive]
pub enum DropFieldsAndNonExhaustive {
    Tuple(),
    Struct {},
    Fields {
        value: u8,
    },
    #[non_exhaustive]
    Added,
}

impl Drop for DropFieldsAndNonExhaustive {
    fn drop(&mut self) {}
}

// Reports Drop, fields, and explicit non-unit discriminants independently.
#[repr(u8)]
#[non_exhaustive]
pub enum DropFieldsAndExplicit {
    Tuple(),
    Struct {},
    Fields(u8),
    Explicit() = 3,
}

impl Drop for DropFieldsAndExplicit {
    fn drop(&mut self) {}
}

// Reports Drop, explicit non-unit discriminants, and a non-exhaustive variant.
#[repr(u8)]
#[non_exhaustive]
pub enum DropNonExhaustiveAndExplicit {
    Tuple(),
    Struct {},
    #[non_exhaustive]
    Added,
    Explicit {} = 3,
}

impl Drop for DropNonExhaustiveAndExplicit {
    fn drop(&mut self) {}
}

// Reports fields, explicit non-unit discriminants,
// and non-exhaustiveness independently.
#[repr(u8)]
#[non_exhaustive]
pub enum FieldsNonExhaustiveAndExplicit {
    Tuple(),
    Struct {},
    Fields {
        value: u8,
    },
    #[non_exhaustive]
    Added,
    Explicit() = 4,
}

// Reports all four causes independently: Drop, fields,
// explicit non-unit discriminants, and a non-exhaustive variant.
#[repr(u8)]
#[non_exhaustive]
pub enum AllCauses {
    Tuple(),
    Struct {},
    Fields(u8),
    #[non_exhaustive]
    Added,
    Explicit {} = 4,
    Unit = 5,
}

impl Drop for AllCauses {
    fn drop(&mut self) {}
}

// Reports fields and explicit non-unit discriminants independently,
// even though both causes affect the same tuple variant.
#[repr(u8)]
#[non_exhaustive]
pub enum FieldsAndExplicitSameTupleVariant {
    Tuple(),
    Struct {},
    Added(u8) = 2,
}

// Reports all four causes independently, including fields, an explicit discriminant,
// and non-exhaustiveness on the same struct variant.
#[repr(u8)]
#[non_exhaustive]
pub enum AllCausesSameStructVariant {
    Tuple(),
    Struct {},
    #[non_exhaustive]
    Added {
        value: u8,
    } = 2,
}

impl Drop for AllCausesSameStructVariant {
    fn drop(&mut self) {}
}
