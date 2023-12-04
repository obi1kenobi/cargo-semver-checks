mod MyNonPublicMod {
    pub enum MyEnum {
        A,
    }
}

pub mod MyPublicMod {
    pub enum MyEnum {
        A,
    }
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerStruct {
        pub enum MyEnum {
            A,
        }
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerStruct {
        pub enum MyEnum {
            A,
        }
    }
}

pub enum AliasedAsDocHidden {
    A,
}

pub enum Example {
    A,
}

pub enum PublicEnumHiddenVariant {
    A,
    B,
}

pub enum PublicEnumHiddenStructFieldOnVariant {
    A { a: u8 },
    B,
}

enum PublicEnumThatGoesPrivate {
    A,
}

pub enum PublicEnumDocumentedWithStringHidden {
    A,
}
