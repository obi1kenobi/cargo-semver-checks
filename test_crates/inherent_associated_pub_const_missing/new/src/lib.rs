pub struct StructA {}
impl StructA {
    pub const PublicConstantB: i32 = 0;
    #[doc(hidden)]
    pub const PublicConstantE: i32 = 0;
    pub const PublicConstantRenamedA: i32 = 0;
}
struct StructB {}
impl StructB {
    #[doc(hidden)]
    pub const PublicConstantD: i32 = 0;
    pub const PublicConstantRenamedB: i32 = 0;
}
#[doc(hidden)]
pub struct StructC {}
impl StructC {}
#[doc(hidden)]
pub struct StructD {}
impl StructD {
    pub const PublicConstantG: i32 = 0;
}
pub struct StructE {}
#[doc(hidden)]
impl StructE {
    pub const PublicConstantI: i32 = 0;
}
