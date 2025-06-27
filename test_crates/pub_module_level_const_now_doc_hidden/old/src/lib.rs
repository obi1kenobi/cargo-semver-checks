#![no_std]

pub const GlobalConstantA: i32 = 0;
pub const GlobalConstantB: i32 = 0;

mod PrivateModule {
    pub const PrivateModuleConstantA: i32 = 0;
    pub const PrivateModuleConstantB: i32 = 0;
    const PrivateModuleConstantC: i32 = 0;
}
pub mod PublicModule {
    pub const PublicModuleConstantA: i32 = 0;
    pub const PublicModuleConstantB: i32 = 0;
    const PublicModuleConstantC: i32 = 0;
}
