pub(crate) mod internal {
    /// This type will be removed, which is non-breaking:
    /// even though it's `pub`, it isn't actually externally visible
    /// since it's in a `pub(crate)` module and not publicly re-exported.
    pub struct NotPublicAndWillDisappear;
}

// The following items will be relocated to the `internal` module,
// but will get different flavors of re-exports to ensure the move is non-breaking.

pub enum RegularReexport {
    Var,
}

pub fn glob_reexport() {}

pub struct TypedefReexport;

pub struct TypedefWithGenericsReexport<'a, const N: usize, T> {
    _marker: std::marker::PhantomData<&'a [T; N]>,
}

// The following types will also be renamed as part of being moved,
// but their re-exports will include renames to their previous names,
// meaning that the move and rename are non-breaking.

pub fn renamed_reexport() {}

pub struct RenamedTypedefReexport;

pub struct RenamedTypedefWithGenericsReexport<'a, const N: usize, T> {
    _marker: std::marker::PhantomData<&'a [T; N]>,
}

// The following types will get moved, but their typedef re-exports
// will alter the generics in ways that mean the new typedef isn't equivalent.

pub struct NonEquivalentReorderedGenerics<'a, const N: usize, T> {
    _marker: std::marker::PhantomData<&'a [T; N]>,
}

pub struct NonEquivalentRemovedLifetime<'a, const N: usize, T> {
    _marker: std::marker::PhantomData<&'a [T; N]>,
}

pub struct NonEquivalentRemovedConst<'a, const N: usize, T> {
    _marker: std::marker::PhantomData<&'a [T; N]>,
}

pub struct NonEquivalentRemovedType<'a, const N: usize, T> {
    _marker: std::marker::PhantomData<&'a [T; N]>,
}
