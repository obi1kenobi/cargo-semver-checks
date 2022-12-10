#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8)]
pub enum U8ToU16Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u16)]
pub enum U8ToU16Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(i32)]
pub enum I32ToI8Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(i8)]
pub enum I32ToI8Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(i32)]
pub enum I32ToU32Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u32)]
pub enum I32ToU32Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(isize)]
pub enum IsizeToUsizeEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(usize)]
pub enum IsizeToUsizeEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8, C)]
pub enum U8CToU16CEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u16, C)]
pub enum U8CToU16CEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(C, u8)]
pub enum CU8ToCU16Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C, u16)]
pub enum CU8ToCU16Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8, C)]
pub enum U8CToSeparateU16CEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u16)]
#[repr(C)]
pub enum U8CToSeparateU16CEnum {
    Bar,
    Baz,
}

// The following enums have *rearrangements* of repr(*), potentially
// splitting singular repr(*) into multiple, smaller repr(*) or merging
// repr(*) into larger repr(*).
// They should not be reported by this rule, because they are legal.

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8)]
#[repr(C)]
pub enum SeparateU8CToU8CEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u8, C)]
pub enum SeparateU8CToU8CEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8, C)]
pub enum U8CToSeparateU8CEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u8)]
#[repr(C)]
pub enum U8CToSeparateU8CEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(C)]
#[repr(u8)]
pub enum SeparateCU8ToCU8Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C, u8)]
pub enum SeparateCU8ToCU8Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(C, u8)]
pub enum CU8ToSeparateCU8Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C)]
#[repr(u8)]
pub enum CU8ToSeparateCU8Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8)]
#[repr(C)]
pub enum SeparateU8CToCU8Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C, u8)]
pub enum SeparateU8CToCU8Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(C)]
#[repr(u8)]
pub enum SeparateCU8ToU8CEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u8, C)]
pub enum SeparateCU8ToU8CEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(C, u8)]
pub enum CU8ToU8CEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u8, C)]
pub enum CU8ToU8CEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8, C)]
pub enum U8CToCU8Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C, u8)]
pub enum U8CToCU8Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(C)]
#[repr(u8)]
pub enum SeparateCU8ToSeparateU8CEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(u8)]
#[repr(C)]
pub enum SeparateCU8ToSeparateU8CEnum {
    Bar,
    Baz,
}

// The following enums have *removals* of repr(i*) and repr(u*),
// not changes to another repr(i*) or repr(u*).
// They should not be reported by this rule, because they have their own rule.

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8)]
pub enum U8EnumToEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
pub enum U8EnumToEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(i32)]
pub enum I32EnumToEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
pub enum I32EnumToEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(isize)]
pub enum IsizeEnumToEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
pub enum IsizeEnumToEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(usize)]
pub enum UsizeEnumToEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
pub enum UsizeEnumToEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8, C)]
pub enum U8CEnumToCEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C)]
pub enum U8CEnumToCEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8)]
#[repr(C)]
pub enum SeparateU8CEnumToCEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C)]
pub enum SeparateU8CEnumToCEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(C, u8)]
pub enum CU8EnumToCEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C)]
pub enum CU8EnumToCEnum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(C)]
#[repr(u8)]
pub enum SeparateCU8EnumToCEnum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
#[repr(C)]
pub enum SeparateCU8EnumToCEnum {
    Bar,
    Baz,
}
