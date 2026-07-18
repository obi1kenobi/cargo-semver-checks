#![no_std]

pub trait PubTrait {}

pub struct PubStruct {}
pub enum PubEnum {}
pub union PubUnion {
    f1: usize,
}

pub trait PubGenericTrait<T> {}

pub struct PubGenericStruct {}
pub enum PubGenericEnum {}
pub union PubGenericUnion {
    f1: usize,
}

pub struct PubGenericBoundStruct {}
pub enum PubGenericBoundEnum {}
pub union PubGenericBoundUnion {
    f1: usize,
}

pub struct PubLifetimeBoundStruct<'a> {
    p: &'a usize,
}
pub enum PubLifetimeBoundEnum<'a> {
    Data(&'a usize),
}
pub union PubLifetimeBoundUnion<'a> {
    p: &'a usize,
}

// these should not be flagged because the impl for T
// has not changed even though the lifetime has

pub struct LifetimeToStaticStruct {
    p: &'static usize,
}
pub enum LifetimeToStaticEnum {
    Data(&'static usize),
}
pub union LifetimeToStaticUnion {
    p: &'static usize,
}

impl<'a> PubGenericTrait<usize> for LifetimeToStaticStruct {}
impl<'a> PubGenericTrait<usize> for LifetimeToStaticEnum {}
impl<'a> PubGenericTrait<usize> for LifetimeToStaticUnion {}

pub struct LifetimeToNonStaticStruct<'a> {
    p: &'a usize,
}
pub enum LifetimeToNonStaticEnum<'a> {
    Data(&'a usize),
}
pub union LifetimeToNonStaticUnion<'a> {
    p: &'a usize,
}

impl<'a> PubGenericTrait<usize> for LifetimeToNonStaticStruct<'a> {}
impl<'a> PubGenericTrait<usize> for LifetimeToNonStaticEnum<'a> {}
impl<'a> PubGenericTrait<usize> for LifetimeToNonStaticUnion<'a> {}

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

impl<'a, T> LifetimeGenericTrait<'a, T> for LifetimeGenericStruct<'static, T> {}
impl<'a, T> LifetimeGenericTrait<'a, T> for LifetimeGenericEnum<'static, T> {}
impl<'a, T> LifetimeGenericTrait<'a, T> for LifetimeGenericUnion<'static, T> {}

// going from manual to automatic should not get flagged

#[derive(Eq, PartialEq)]
pub struct ManualToDeriveStruct {}
#[derive(Eq, PartialEq)]
pub enum ManualToDeriveEnum {}

#[derive(PartialEq)]
pub struct DeriveToManualStruct {}
#[derive(PartialEq)]
pub enum DeriveToManualEnum {}

impl Eq for DeriveToManualStruct {}
impl Eq for DeriveToManualEnum {}

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
