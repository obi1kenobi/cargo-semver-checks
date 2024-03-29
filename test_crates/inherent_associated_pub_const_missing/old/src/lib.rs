pub struct StructA {}
impl StructA {
    //Basic Test case should be caught
    pub const PublicConstantA: i32 = 0;
    pub const PublicConstantB: i32 = 0;
    //Should not be caught on marking #[doc(hidden)]
    pub const PublicConstantE: i32 = 0;
    // Should Be caught on renaming
    pub const PublicConstantRenameA: i32 = 0;
    //Should not be caught on removing since its not pub
    const ConstantA: i32 = 0;
}
struct StructB {}
impl StructB {
    //should not be caught since StructB is not pub
    pub const PublicConstantC: i32 = 0;
    pub const PublicConstantD: i32 = 0;
    pub const PublicConstantRenameB: i32 = 0;
    const ConstantB: i32 = 0;
}
#[doc(hidden)]
pub struct StructC {}
impl StructC {
    // should not be caught on removing since the struct #[doc(hidden)]
    pub const PublicConstantF: i32 = 0;
}
//should not be caught by this lint on marking #[doc(hidden)]
pub struct StructD {}
impl StructD {
    pub const PublicConstantG: i32 = 0;
    pub const PublicConstantH: i32 = 0;
}
pub struct StructE {}
//this impl block will be #[doc(hidden)]
impl StructE {
    pub const PublicConstantI: i32 = 0;
    //this will be removed
    pub const PublicConstantJ: i32 = 0;
}
