pub struct StructA {}
impl StructA {
    //Basic Test case should be caught
    pub const PublicConstantA: i32 = 0;
    pub const PublicConstantB: i32 = 0;
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
