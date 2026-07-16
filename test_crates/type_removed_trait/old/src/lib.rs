#![no_std]

pub trait PubTrait {}

pub struct PubStruct {}
pub enum PubEnum {}
pub union PubUnion {
    f1: usize,
}

impl PubTrait for PubStruct {}
impl PubTrait for PubEnum {}
impl PubTrait for PubUnion {}

pub trait PubGenericTrait<T> {}

pub struct PubGenericStruct {}
pub struct PubGenericEnum {}
pub struct PubGenericUnion {}

impl PubGenericTrait<usize> for PubGenericStruct {}
impl PubGenericTrait<usize> for PubGenericEnum {}
impl PubGenericTrait<usize> for PubGenericUnion {}

pub struct PubGenericBoundStruct {}
pub struct PubGenericBoundEnum {}
pub struct PubGenericBoundUnion {}

impl<T> PubGenericTrait<T> for PubGenericBoundStruct where T: TryInto<usize> {}
impl<T> PubGenericTrait<T> for PubGenericBoundEnum where T: TryInto<usize> {}
impl<T> PubGenericTrait<T> for PubGenericBoundUnion where T: TryInto<usize> {}

pub struct PubLifetimeBoundStruct<'a> {
    p: &'a usize,
}
pub struct PubLifetimeBoundEnum<'a> {
    p: &'a usize,
}
pub struct PubLifetimeBoundUnion<'a> {
    p: &'a usize,
}

impl<'a, T> PubGenericTrait<T> for PubLifetimeBoundStruct<'a> where T: TryInto<usize> {}
impl<'a, T> PubGenericTrait<T> for PubLifetimeBoundEnum<'a> where T: TryInto<usize> {}
impl<'a, T> PubGenericTrait<T> for PubLifetimeBoundUnion<'a> where T: TryInto<usize> {}

// these should not be flagged as the trait is not public

trait PrivateTrait {}

pub struct PrivateTraitStruct {}
pub enum PrivateTraitEnum {}
pub union PrivateTraitUnion {
    f1: usize,
}

impl PrivateTrait for PrivateTraitStruct {}
impl PrivateTrait for PrivateTraitEnum {}
impl PrivateTrait for PrivateTraitUnion {}

pub(crate) trait PubCrateTrait {}

pub struct PubCrateTraitStruct {}
pub enum PubCrateTraitEnum {}
pub union PubCrateTraitUnion {
    f1: usize,
}

impl PubCrateTrait for PubCrateTraitStruct {}
impl PubCrateTrait for PubCrateTraitEnum {}
impl PubCrateTrait for PubCrateTraitUnion {}
