#![no_std]

mod implementation {
    pub enum SolePath {
        Tuple(u8),
        Struct { field: u8 },
    }

    pub enum PreservedPath {
        Tuple(u8),
        Struct { field: u8 },
    }

    pub enum ReplacedPaths {
        Tuple(u8),
        Struct { field: u8 },
    }
}

pub use implementation::SolePath;

pub use implementation::PreservedPath;
pub use implementation::PreservedPath as RetainedAlias;

pub use implementation::ReplacedPaths;
pub use implementation::ReplacedPaths as OldAlias;
