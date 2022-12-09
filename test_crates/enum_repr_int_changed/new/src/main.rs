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

#[repr(usize)]
pub enum IsizeToUsizeEnum {
    Bar,
    Baz,
}

// The following enums have *removals* of repr(i*) and repr(u*),
// not changes to another repr(i*) or repr(u*).
// They should not be reported by this rule, because they have their own rule.

pub enum U8Enum {
    Bar,
    Baz,
}

pub enum I32Enum {
    Bar,
    Baz,
}

pub enum IsizeEnum {
    Bar,
    Baz,
}

pub enum UsizeEnum {
    Bar,
    Baz,
}
