pub struct StructA {}

impl StructA {
    pub const PublicConstantA: i32 = 0;
   
    // #[doc(hidden)]
    pub const DocHiddenConstantA: i32 = 0;
}

struct StructB {}

impl StructB {
    pub const PublicConstantA: i32 = 0;
   
    // #[doc(hidden)]
    pub const DocHiddenConstantA: i32 = 0;
}

pub enum EnumA {
    U32,
}

impl EnumA {
    pub const PublicConstantA: u32 = 0;
   
    // #[doc(hidden)]
    pub const DocHiddenConstantA: u32 = 0;
}

pub union UnionA {
    u1: u32,
    u2: u32,
}

impl UnionA {
    pub const PublicConstantA: u32 = 0;
   
    // #[doc(hidden)]
    pub const DocHiddenConstantA: u32 = 0;
}
