#![no_std]

pub trait Base {
    type Assoc;

    fn base(&self) -> Self::Assoc;
}

/// This `where Self: Base<Assoc = u8>` bound is equivalent to a colon
/// supertrait, so it should not trigger `trait_removed_supertrait`.
pub trait ColonToWhere
where
    Self: Base<Assoc = u8>,
{
}

/// This colon supertrait is equivalent to `where Self: Base<Assoc = u8>`, so it
/// should not trigger `trait_added_supertrait`.
pub trait WhereToColon: Base<Assoc = u8> {}
