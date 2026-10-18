#![no_std]

// Definition names change, but the public aliases below remain the same.
// Variant and field changes must still be reported through those aliases.
mod implementation {
    pub enum NewVariants {
        TupleFieldAdded(u8, u8),
        #[non_exhaustive]
        NonExhaustiveTupleFieldAdded(u8, u8),
        TupleFieldRemoved(u8),
        TupleFieldHidden(#[doc(hidden)] u8),
        StructFieldHidden {
            #[doc(hidden)]
            field: u8,
        },
        UnitToTuple(u8),
        TupleToStruct {
            field: u8,
        },
        StructToTuple(u8),
        #[non_exhaustive]
        NonExhaustiveTupleToStruct {
            field: u8,
        },
    }

    #[repr(u8)]
    pub enum NewReprDiscriminant {
        Value = 2,
    }

    pub enum NewDiscriminant {
        Value = 2,
    }

    #[non_exhaustive]
    pub enum NewGainsFieldfulVariant {
        Value,
        Added(u8),
    }

    pub enum NewVariantBecomesNonExhaustive {
        #[non_exhaustive]
        Value,
    }

    // Renaming only the definition must not report any API changes.
    pub enum NewUnchanged {
        Unit,
        Tuple(u8),
        Struct { field: u8 },
    }
}

pub use implementation::NewDiscriminant as Discriminant;
pub use implementation::NewGainsFieldfulVariant as GainsFieldfulVariant;
pub use implementation::NewReprDiscriminant as ReprDiscriminant;
pub use implementation::NewUnchanged as Unchanged;
pub use implementation::NewVariantBecomesNonExhaustive as VariantBecomesNonExhaustive;
pub use implementation::NewVariants as Variants;
