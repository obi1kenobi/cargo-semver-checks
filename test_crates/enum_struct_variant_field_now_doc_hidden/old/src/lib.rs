#![no_std]

pub enum PublicEnum {
    StructVariant {
        #[doc(hidden)]
        x: i64,
        y: u8,
    },
}
pub enum PublicEnumA {
    StructVariant { x: i64, y: u8 },
}
enum NonPublicEnum {
    StructVariant { x: i32, y: u8 },
}
#[doc(hidden)]
pub enum PublicEnumB {
    StructVariant { x: i32, y: i64 },
}
