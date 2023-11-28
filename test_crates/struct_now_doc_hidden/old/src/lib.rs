mod MyNonPublicMod {
    pub struct MyStruct;
}

pub mod MyPublicMod {
    pub struct MyStruct;
}

pub struct Example;

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
