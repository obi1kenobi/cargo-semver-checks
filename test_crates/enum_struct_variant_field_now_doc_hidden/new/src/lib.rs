#![no_std]

// should not be caught since field was already marked #[doc(hidden)]
pub enum PublicEnum {
    StructVariant {
        #[doc(hidden)]
        x: i64,
        y: u8,
    },
}
//Basic Test case should be caught
pub enum PublicEnumA {
    StructVariant {
        #[doc(hidden)]
        x: i64,
        y: u8,
    },
}
// Non Public Enum should not be affected on adding #[doc(hidden)]
enum NonPublicEnum {
    StructVariant {
        #[doc(hidden)]
        x: i32,
        y: u8,
    },
}
//should not be caught since enum was not a part of public API
#[doc(hidden)]
pub enum PublicEnumB {
    StructVariant {
        #[doc(hidden)]
        x: i32,
        y: i64,
    },
}
