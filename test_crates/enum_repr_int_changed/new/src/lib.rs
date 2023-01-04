#[repr(u16)]
pub enum U8ToU16Enum {
    Bar,
    Baz,
}

#[repr(i8)]
pub enum I32ToI8Enum {
    Bar,
    Baz,
}

#[repr(u32)]
pub enum I32ToU32Enum {
    Bar,
    Baz,
}

#[repr(u16, C)]
pub enum U8CToU16CEnum {
    Bar,
    Baz,
}

#[repr(C, u16)]
pub enum CU8ToCU16Enum {
    Bar,
    Baz,
}

#[repr(u16)]
#[repr(C)]
pub enum U8CToSeparateU16CEnum {
    Bar,
    Baz,
}


// The following enums have changes that can be breaking on some systems.
// Since there are no guarantees on what particular system a given crate
// might run on, this is a breaking change.

#[repr(usize)]
pub enum U64ToUsizeEnum {
    Bar,
    Baz,
}

#[repr(u64)]
pub enum UsizeToU64Enum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum I64ToIsizeEnum {
    Bar,
    Baz,
}

#[repr(i64)]
pub enum IsizeToI64Enum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum UsizeToIsizeEnum {
    Bar,
    Baz,
}

#[repr(usize)]
pub enum IsizeToUsizeEnum {
    Bar,
    Baz,
}


// The following enums have *rearrangements* of repr(*), potentially
// splitting singular repr(*) into multiple, smaller repr(*) or merging
// repr(*) into larger repr(*).
// They should not be reported by this rule, because they are legal.

#[repr(u8, C)]
pub enum SeparateU8CToU8CEnum {
    Bar,
    Baz,
}

#[repr(u8)]
#[repr(C)]
pub enum U8CToSeparateU8CEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum SeparateCU8ToCU8Enum {
    Bar,
    Baz,
}

#[repr(C)]
#[repr(u8)]
pub enum CU8ToSeparateCU8Enum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum SeparateU8CToCU8Enum {
    Bar,
    Baz,
}

#[repr(u8, C)]
pub enum SeparateCU8ToU8CEnum {
    Bar,
    Baz,
}

#[repr(u8, C)]
pub enum CU8ToU8CEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum U8CToCU8Enum {
    Bar,
    Baz,
}

#[repr(u8)]
#[repr(C)]
pub enum SeparateCU8ToSeparateU8CEnum {
    Bar,
    Baz,
}
