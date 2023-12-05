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

#[doc(hidden)]
pub mod MyTopLevelDocHiddenMod {
    pub enum MyEnumThatIsNowDocHidden {
        A,
    }
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerMod {
        pub enum MyEnum {
            A,
        }
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerMod {
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
