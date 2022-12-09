#[repr(u8)]
pub enum U8ToU16Enum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32ToI8Enum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32ToU32Enum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum IsizeToUsizeEnum {
    Bar,
    Baz,
}

// The following enums have *removals* of repr(i*) and repr(u*),
// not changes to another repr(i*) or repr(u*).
// They should not be reported by this rule, because they have their own rule.

#[repr(u8)]
pub enum U8Enum {
    Bar,
    Baz,
}

#[repr(i32)]
pub enum I32Enum {
    Bar,
    Baz,
}

#[repr(isize)]
pub enum IsizeEnum {
    Bar,
    Baz,
}

#[repr(usize)]
pub enum UsizeEnum {
    Bar,
    Baz,
}
