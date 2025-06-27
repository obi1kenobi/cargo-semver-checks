#![no_std]

pub(crate) mod internal {
    // The following items will be relocated to the `internal` module,
    // but will get different flavors of re-exports to ensure the move is non-breaking.

    pub enum RegularReexport {
        Var,
    }

    pub(crate) mod glob_inner {
        pub fn glob_reexport() {}
    }

    pub struct TypedefReexport;

    pub struct TypedefWithGenericsReexport<'a, const N: usize, T> {
        _marker: core::marker::PhantomData<&'a [T; N]>,
    }

    // The following types will also be renamed as part of being moved,
    // but their re-exports will include renames to their previous names,
    // meaning that the move and rename are non-breaking.

    pub fn has_been_renamed_reexport() {}

    pub struct HasBeenRenamedTypedefReexport;

    pub struct HasBeenRenamedTypedefWithGenericsReexport<'a, const N: usize, T> {
        _marker: core::marker::PhantomData<&'a [T; N]>,
    }

    // The following types will get moved, but their typedef re-exports
    // will alter the generics in ways that mean the new typedef isn't equivalent.

    pub struct NonEquivalentReorderedGenerics<'a, const N: usize, T> {
        _marker: core::marker::PhantomData<&'a [T; N]>,
    }

    pub struct NonEquivalentRemovedLifetime<'a, const N: usize, T> {
        _marker: core::marker::PhantomData<&'a [T; N]>,
    }

    pub struct NonEquivalentRemovedConst<'a, const N: usize, T> {
        _marker: core::marker::PhantomData<&'a [T; N]>,
    }

    pub struct NonEquivalentRemovedType<'a, const N: usize, T> {
        _marker: core::marker::PhantomData<&'a [T; N]>,
    }
}

// The following items will be relocated to the `internal` module,
// but will get different flavors of re-exports to ensure the move is non-breaking.

pub use internal::RegularReexport;

pub use internal::glob_inner::*;

pub type TypedefReexport = internal::TypedefReexport;

pub type TypedefWithGenericsReexport<'a, const N: usize, T> =
    internal::TypedefWithGenericsReexport<'a, N, T>;

// The following types will also be renamed as part of being moved,
// but their re-exports will include renames to their previous names,
// meaning that the move and rename are non-breaking.

pub use internal::has_been_renamed_reexport as renamed_reexport;

pub type RenamedTypedefReexport = internal::HasBeenRenamedTypedefReexport;

pub type RenamedTypedefWithGenericsReexport<'a, const N: usize, T> =
    internal::HasBeenRenamedTypedefWithGenericsReexport<'a, N, T>;

// The following types will get moved, but their typedef re-exports
// will alter the generics in ways that mean the new typedef isn't equivalent.

pub type NonEquivalentReorderedGenerics<'a, T, const N: usize> =
    internal::NonEquivalentReorderedGenerics<'a, N, T>;

pub type NonEquivalentRemovedLifetime<const N: usize, T> =
    internal::NonEquivalentRemovedLifetime<'static, N, T>;

pub type NonEquivalentRemovedConst<'a, T> = internal::NonEquivalentRemovedConst<'a, 5, T>;

pub type NonEquivalentRemovedType<'a, const N: usize> =
    internal::NonEquivalentRemovedType<'a, N, i64>;
