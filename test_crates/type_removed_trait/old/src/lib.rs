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
pub enum PubGenericEnum {}
pub union PubGenericUnion {
    f1: usize,
}

impl PubGenericTrait<usize> for PubGenericStruct {}
impl PubGenericTrait<usize> for PubGenericEnum {}
impl PubGenericTrait<usize> for PubGenericUnion {}

pub struct PubGenericBoundStruct {}
pub enum PubGenericBoundEnum {}
pub union PubGenericBoundUnion {
    f1: usize,
}

impl<T> PubGenericTrait<T> for PubGenericBoundStruct where T: TryInto<usize> {}
impl<T> PubGenericTrait<T> for PubGenericBoundEnum where T: TryInto<usize> {}
impl<T> PubGenericTrait<T> for PubGenericBoundUnion where T: TryInto<usize> {}

pub struct PubLifetimeBoundStruct<'a> {
    p: &'a usize,
}
pub enum PubLifetimeBoundEnum<'a> {
    Data(&'a usize),
}
pub union PubLifetimeBoundUnion<'a> {
    p: &'a usize,
}

impl<'a, T> PubGenericTrait<T> for PubLifetimeBoundStruct<'a> where T: TryInto<usize> {}
impl<'a, T> PubGenericTrait<T> for PubLifetimeBoundEnum<'a> where T: TryInto<usize> {}
impl<'a, T> PubGenericTrait<T> for PubLifetimeBoundUnion<'a> where T: TryInto<usize> {}

// these should not be flagged because the impl for T
// has not changed even though the lifetime has

pub struct LifetimeToStaticStruct<'a> {
    p: &'a usize,
}
pub enum LifetimeToStaticEnum<'a> {
    Data(&'a usize),
}
pub union LifetimeToStaticUnion<'a> {
    p: &'a usize,
}

impl<'a> PubGenericTrait<usize> for LifetimeToStaticStruct<'a> {}
impl<'a> PubGenericTrait<usize> for LifetimeToStaticEnum<'a> {}
impl<'a> PubGenericTrait<usize> for LifetimeToStaticUnion<'a> {}

pub struct LifetimeToNonStaticStruct {
    p: &'static usize,
}
pub enum LifetimeToNonStaticEnum {
    Data(&'static usize),
}
pub union LifetimeToNonStaticUnion {
    p: &'static usize,
}

impl PubGenericTrait<usize> for LifetimeToNonStaticStruct {}
impl PubGenericTrait<usize> for LifetimeToNonStaticEnum {}
impl PubGenericTrait<usize> for LifetimeToNonStaticUnion {}

// only a lifetime change; don't flag
pub trait LifetimeGenericTrait<'a, T> {}

pub struct LifetimeGenericStruct<'a, T> {
    p: &'a T,
}
pub enum LifetimeGenericEnum<'a, T> {
    Data(&'a T),
}
pub union LifetimeGenericUnion<'a, T> {
    p: &'a T,
}

impl<'a, T> LifetimeGenericTrait<'a, T> for LifetimeGenericStruct<'a, T> {}
impl<'a, T> LifetimeGenericTrait<'a, T> for LifetimeGenericEnum<'a, T> {}
impl<'a, T> LifetimeGenericTrait<'a, T> for LifetimeGenericUnion<'a, T> {}

// going from manual to automatic should not get flagged

#[derive(PartialEq)]
pub struct ManualToDeriveStruct {}
#[derive(PartialEq)]
pub enum ManualToDeriveEnum {}

impl Eq for ManualToDeriveStruct {}
impl Eq for ManualToDeriveEnum {}

#[derive(Eq, PartialEq)]
pub struct DeriveToManualStruct {}
#[derive(Eq, PartialEq)]
pub enum DeriveToManualEnum {}

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
