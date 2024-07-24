pub const GlobalConstantA: i32 = 0;
//Should be caught on adding #[doc(hidden)]
#[doc(hidden)]
pub const GlobalConstantB: i32 = 0;

mod PrivateModule {
    pub const PrivateModuleConstantA: i32 = 0;
    //Should not be caught since its a private module
    #[doc(hidden)]
    pub const PrivateModuleConstantB: i32 = 0;
    #[doc(hidden)]
    const PrivateModuleConstantC: i32 = 0;
}
pub mod PublicModule {
    pub const PublicModuleConstantA: i32 = 0;
    //Should be caught on adding #[doc(hidden)] since its a public constant in a public module
    #[doc(hidden)]
    pub const PublicModuleConstantB: i32 = 0;
    //Should not be caught since its not a public constant
    #[doc(hidden)]
    const PublicModuleConstantC: i32 = 0;
}
