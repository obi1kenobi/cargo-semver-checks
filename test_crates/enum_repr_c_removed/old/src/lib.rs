#[repr(C)]
pub enum CEnumToEnum {
    Bar,
}

#[repr(u8, C)]
pub enum U8CEnumToU8Enum {
    Bar,
    Baz,
}

#[repr(u8)]
#[repr(C)]
pub enum SeparateU8CEnumToU8Enum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CU8EnumToU8Enum {
    Bar,
    Baz,
}

#[repr(C)]
#[repr(u8)]
pub enum SeparateCU8EnumToU8Enum {
    Bar,
    Baz,
}


// The following enums have *rearrangements* of repr(C), potentially
// splitting singular repr(*) into multiple, smaller repr(*) or merging
// repr(*) into larger repr(*).
// They should not be reported by this rule, because they are legal.

#[repr(u8)]
#[repr(C)]
pub enum SeparateU8CToU8CEnum {
    Bar,
    Baz,
}

#[repr(u8, C)]
pub enum U8CToSeparateU8CEnum {
    Bar,
    Baz,
}

#[repr(C)]
#[repr(u8)]
pub enum SeparateCU8ToCU8Enum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CU8ToSeparateCU8Enum {
    Bar,
    Baz,
}

#[repr(u8)]
#[repr(C)]
pub enum SeparateU8CToCU8Enum {
    Bar,
    Baz,
}

#[repr(C)]
#[repr(u8)]
pub enum SeparateCU8ToU8CEnum {
    Bar,
    Baz,
}

#[repr(C, u8)]
pub enum CU8ToU8CEnum {
    Bar,
    Baz,
}

#[repr(u8, C)]
pub enum U8CToCU8Enum {
    Bar,
    Baz,
}

#[repr(C)]
#[repr(u8)]
pub enum SeparateCU8ToSeparateU8CEnum {
    Bar,
    Baz,
}
