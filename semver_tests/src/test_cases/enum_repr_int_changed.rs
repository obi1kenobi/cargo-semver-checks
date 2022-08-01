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

// The following enums have *removals* of repr(i*) and repr(u*),
// not changes to another repr(i*) or repr(u*).
// They should not be reported by this rule, because they have their own rule.

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(u8)]
pub enum U8Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
pub enum U8Enum {
    Bar,
    Baz,
}

#[cfg(not(feature = "enum_repr_int_changed"))]
#[repr(i32)]
pub enum I32Enum {
    Bar,
    Baz,
}

#[cfg(feature = "enum_repr_int_changed")]
pub enum I32Enum {
    Bar,
    Baz,
}
