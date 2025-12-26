#![no_std]

mod private_mod {
    pub enum NonImportableEnum {
        /// Testing: <https://doc.rust-lang.org/cargo/reference/semver.html#item-remove>
        MissingReexport,
        StillReexported,
    }
}

pub use private_mod::NonImportableEnum::MissingReexport;
pub use private_mod::NonImportableEnum::StillReexported;
