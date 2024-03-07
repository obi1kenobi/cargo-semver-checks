//Adding #[doc(hidden)] should not affect since the enum is not a part of public API
enum NonPublicEnum {
    TupleVariant(#[doc(hidden)] i32,u8),
}
// basic test case(adding #[doc(hidden)] to public enum tuple variant)
pub enum PulicEnumA {
    TupleVariantWithPublicField(#[doc(hidden)] u8,i32),
}
// Should not affect since there the field was already hidden
pub enum PulicEnumB {
    TupleVariantWithPublicFieldHidden(#[doc(hidden)] i64, u8),
}
// should not affect since the public enum was not a part of Public API 
#[doc(hidden)]
pub enum PulicEnumC{
    TupleVariantWithPublicField(#[doc(hidden)] u8,i64),
}