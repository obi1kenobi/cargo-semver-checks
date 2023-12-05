mod MyNonPublicMod {
    pub struct MyStruct;
}

pub mod MyPublicMod {
    pub struct MyStruct;
}

pub struct AliasedAsDocHidden;

pub struct Example;

pub struct PublicStructHiddenField {
    pub my_field: i8,
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerMod {
        pub struct MyStruct;
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerMod {
        pub struct MyStruct;
    }
}

pub struct PublicStructThatGoesPrivate;

pub struct PublicStructDocumentedWithStringHidden;
