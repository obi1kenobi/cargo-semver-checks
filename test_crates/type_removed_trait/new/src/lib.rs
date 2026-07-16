#![no_std]

pub trait PubTrait {}

pub struct PubStruct {}
pub enum PubEnum {}
pub union PubUnion {
    f1: usize,
}

pub trait PubGenericTrait<T> {}

pub struct PubGenericStruct {}
pub struct PubGenericEnum {}
pub struct PubGenericUnion {}

pub struct PubGenericBoundStruct {}
pub struct PubGenericBoundEnum {}
pub struct PubGenericBoundUnion {}

pub struct PubLifetimeBoundStruct<'a> {
    p: &'a usize,
}
pub struct PubLifetimeBoundEnum<'a> {
    p: &'a usize,
}
pub struct PubLifetimeBoundUnion<'a> {
    p: &'a usize,
}

// these should not be flagged as the trait is not public

trait PrivateTrait {}

pub struct PrivateTraitStruct {}
pub enum PrivateTraitEnum {}
pub union PrivateTraitUnion {
    f1: usize,
}

pub(crate) trait PubCrateTrait {}

pub struct PubCrateTraitStruct {}
pub enum PubCrateTraitEnum {}
pub union PubCrateTraitUnion {
    f1: usize,
}
