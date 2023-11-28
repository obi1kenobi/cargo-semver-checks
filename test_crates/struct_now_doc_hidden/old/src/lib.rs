mod MyNonPublicMod {
    pub struct MyStruct;
}

pub mod MyPublicMod {
    pub struct MyStruct;
}

pub struct MispelledDocHidden;

pub struct Example;

pub struct PublicStructHiddenField {
    pub my_field: i8,
}

mod MyNestedNonPublicMod {
    pub mod PublicInnerStruct {
        pub struct MyStruct;
    }
}

pub mod MyNestedPublicMod {
    pub mod PublicInnerStruct {
        pub struct MyStruct;
    }
}

pub struct PublicStructThatGoesPrivate;

pub struct PublicStructDocumentedWithStringHidden;
