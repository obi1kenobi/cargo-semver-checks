#[derive(Clone)]
pub struct DebugFoo;

#[derive(Clone)]
pub enum CopyBar {
    Var
}

#[derive(PartialEq)]
pub struct EqFoo;

// The following is not a semver issue: it's not breaking to replace
// a derived impl with a hand-impl of the same trait.

#[derive(PartialEq)]
pub struct ManualEqFoo;

impl Eq for ManualEqFoo {}
