#![no_std]

pub trait Base {
    type Assoc;

    fn base(&self) -> Self::Assoc;
}

/// Moving this supertrait bound to `where Self: Base<Assoc = u8>` is not a
/// breaking change, so it should not trigger `trait_removed_supertrait`.
pub trait ColonToWhere: Base<Assoc = u8> {}

/// Moving this `where Self: Base<Assoc = u8>` bound to colon supertrait syntax
/// is not a breaking change, so it should not trigger `trait_added_supertrait`.
pub trait WhereToColon
where
    Self: Base<Assoc = u8>,
{
}
