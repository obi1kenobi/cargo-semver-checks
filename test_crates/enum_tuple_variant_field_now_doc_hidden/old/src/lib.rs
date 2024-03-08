enum NonPublicEnum {
    TupleVariant(i32, u8),
}
pub enum PulicEnumA {
    TupleVariantWithPublicField(u8, i32),
}
pub enum PulicEnumB {
    TupleVariantWithPublicFieldHidden(#[doc(hidden)] i64, u8),
}
#[doc(hidden)]
pub enum PulicEnumC {
    TupleVariantWithPublicField(u8, i64),
}
