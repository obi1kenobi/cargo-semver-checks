pub static GlobalStaticA: i32 = 0;
//Should be caught on adding #[doc(hidden)]
#[doc(hidden)]
pub static GlobalStaticB: i32 = 0;

mod PrivateModule {
    pub static PrivateModuleStaticA: i32 = 0;
    //Should not be caught since its in a private module
    #[doc(hidden)]
    pub static PrivateModuleStaticB: i32 = 0;
    #[doc(hidden)]
    static PrivateModuleStaticC: i32 = 0;
}
pub mod PublicModule {
    pub static PublicModuleStaticA: i32 = 0;
    //Should be caught on adding #[doc(hidden)] since its a public static in a public module
    #[doc(hidden)]
    pub static PublicModuleStaticB: i32 = 0;
    //Should not be caught since its not a public static
    #[doc(hidden)]
    static PublicModuleStaticC: i32 = 0;
}
