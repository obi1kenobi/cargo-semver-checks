#![no_std]

// Definition names change, but the public aliases below remain the same.
// Variant and field changes must still be reported through those aliases.
mod implementation {
    pub enum OldVariants {
        TupleFieldAdded(u8),
        #[non_exhaustive]
        NonExhaustiveTupleFieldAdded(u8),
        TupleFieldRemoved(u8, u8),
        TupleFieldHidden(u8),
        StructFieldHidden {
            field: u8,
        },
        UnitToTuple,
        TupleToStruct(u8),
        StructToTuple {
            field: u8,
        },
        #[non_exhaustive]
        NonExhaustiveTupleToStruct(u8),
    }

    #[repr(u8)]
    pub enum OldReprDiscriminant {
        Value = 1,
    }

    pub enum OldDiscriminant {
        Value = 1,
    }

    #[non_exhaustive]
    pub enum OldGainsFieldfulVariant {
        Value,
    }

    pub enum OldVariantBecomesNonExhaustive {
        Value,
    }

    // Renaming only the definition must not report any API changes.
    pub enum OldUnchanged {
        Unit,
        Tuple(u8),
        Struct { field: u8 },
    }
}

pub use implementation::OldDiscriminant as Discriminant;
pub use implementation::OldGainsFieldfulVariant as GainsFieldfulVariant;
pub use implementation::OldReprDiscriminant as ReprDiscriminant;
pub use implementation::OldUnchanged as Unchanged;
pub use implementation::OldVariantBecomesNonExhaustive as VariantBecomesNonExhaustive;
pub use implementation::OldVariants as Variants;
