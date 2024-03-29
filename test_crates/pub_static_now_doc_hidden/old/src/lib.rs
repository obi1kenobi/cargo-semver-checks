pub static GlobalStaticA: i32 = 0;
pub static GlobalStaticB: i32 = 0;

mod PrivateModule {
    pub static PrivateModuleStaticA: i32 = 0;
    pub static PrivateModuleStaticB: i32 = 0;
    static PrivateModuleStaticC: i32 = 0;
}
pub mod PublicModule {
    pub static PublicModuleStaticA: i32 = 0;
    pub static PublicModuleStaticB: i32 = 0;
    static PublicModuleStaticC: i32 = 0;
}
