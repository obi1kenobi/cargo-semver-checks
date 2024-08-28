#![feature(associated_type_defaults)]

mod sealed {
    pub(crate) trait Sealed {}
}

pub trait WillLoseDefault {
    type Foo;
}

pub trait WillLoseDefaultSealed: sealed::Sealed {
    type Foo;
}

pub trait Unchanged {
    type Foo = bool;
}
pub trait UnchangedSealed: sealed::Sealed {
    type Foo = bool;
}

pub trait UnchangedNoDefault {
    type Foo;
}
pub trait UnchangedNoDefaultSealed: sealed::Sealed {
    type Foo;
}
