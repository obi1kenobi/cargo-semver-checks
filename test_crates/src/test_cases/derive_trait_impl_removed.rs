#[cfg(not(feature = "derive_trait_impl_removed"))]
#[derive(Debug, Clone)]
pub struct DebugFoo;

#[cfg(feature = "derive_trait_impl_removed")]
#[derive(Clone)]
pub struct DebugFoo;

#[cfg(not(feature = "derive_trait_impl_removed"))]
#[derive(Clone, Copy)]
pub enum CopyBar {
    Var
}

#[cfg(feature = "derive_trait_impl_removed")]
#[derive(Clone)]
pub enum CopyBar {
    Var
}

#[cfg(not(feature = "derive_trait_impl_removed"))]
#[derive(PartialEq, Eq)]
pub struct EqFoo;

#[cfg(feature = "derive_trait_impl_removed")]
#[derive(PartialEq)]
pub struct EqFoo;

// The following is not a semver issue: it's not breaking to replace
// a derived impl with a hand-impl of the same trait.

#[cfg(not(feature = "derive_trait_impl_removed"))]
#[derive(PartialEq, Eq)]
pub struct ManualEqFoo;

#[cfg(feature = "derive_trait_impl_removed")]
#[derive(PartialEq)]
pub struct ManualEqFoo;

#[cfg(feature = "derive_trait_impl_removed")]
impl Eq for ManualEqFoo {}
