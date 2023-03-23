mod internal {
    pub struct NewName;

    pub enum NewEnum {
        Foo,
    }

    pub trait NewTrait {}

    pub fn new_fn() {}
}

pub use internal::{NewName, NewEnum, NewTrait, new_fn};

// Re-export the items under their old names,
// so that this isn't a breaking change.
pub use NewName as OldName;
pub use NewEnum as OldEnum;
pub use NewTrait as OldTrait;
pub use new_fn as old_fn;
