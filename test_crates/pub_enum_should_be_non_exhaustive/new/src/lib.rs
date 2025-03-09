#![no_std]

pub enum RegularEnum {
    Variant1,
    Variant2,
}

pub enum ExhaustiveEnum {
    Variant1,
    Variant2,
}

#[non_exhaustive]
pub enum NonExhaustiveEnum {
    VariantA,
    VariantB,
}
