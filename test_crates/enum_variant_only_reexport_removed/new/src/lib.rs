#![no_std]

mod private_mod {
    pub enum NonImportableEnum {
        MissingReexport,
        StillReexported,
    }
}

pub use private_mod::NonImportableEnum::StillReexported;
