#![no_std]
#![feature(portable_simd)]
// Test Cases where #[doc(hidden)] is neither added or removed
pub struct StructA {}

impl StructA {
    pub const PublicConstantB: i32 = 0;
    // Should Be caught on renaming
    pub const PublicConstantRenamedA: i32 = 0;
}

struct StructB {}

impl StructB {
    // Should not be caught since StructB is not pub
    pub const PublicConstantRenamedB: i32 = 0;
}

#[doc(hidden)]
pub struct StructC {}

impl StructC {
    // Should not be caught on removing or renaming since the struct is #[doc(hidden)]
    pub const PublicConstantRenamedC: i32 = 0;
}

pub struct StructD {}

#[doc(hidden)]
impl StructD {
    pub const PublicConstantE: i32 = 0;
}

// Test Cases where #[doc(hidden)] is added
pub struct StructE {}

#[doc(hidden)]
impl StructE {
    pub const PublicConstantH: i32 = 0;
}

#[doc(hidden)]
pub struct StructF {}

impl StructF {
    pub const PublicConstantJ: i32 = 0;
}

// Test cases where #[doc(hidden)] is removed
pub struct DocHiddenStruct {}

impl DocHiddenStruct {
    pub const PublicConstantL: i32 = 0;
}

impl DocHiddenStruct {
    pub const PublicConstantO: i32 = 0;
}

// Test for when an inherent const is implemented as trait's const
pub trait TraitA {
    const N_A: i64 = 0;
}

pub struct ExampleA;

impl TraitA for ExampleA {}

// Test for when an inherent const is implemented as trait's const but given a value in impl block
pub trait TraitB {
    const N_B: i64;
}

pub struct ExampleB;

impl TraitB for ExampleB {
    const N_B: i64 = 0;
}

// Test for when an inherent const is implemented as trait's const,
// but the `impl Trait` block is not public API. In this case, we report the lint
// since falling back to the trait implementation would require using non-public API.
pub trait TraitC {
    const N_C: i64;
}

pub struct ExampleC;

#[doc(hidden)]
impl TraitC for ExampleC {
    const N_C: i64 = 0;
}

pub struct Limits;

impl Limits {
    const MAX: usize = 8;

    // Removing `pub` makes this a removal even though it also gains `#[doc(hidden)]`.
    #[doc(hidden)]
    const DEFAULT: usize = 4;
}

mod fallback {
    pub trait DefaultLimits {
        const MAX: usize = 13;
    }
}

pub use fallback::DefaultLimits as _;

pub struct LimitsWithTrait;

impl fallback::DefaultLimits for LimitsWithTrait {}

impl core::simd::Swizzle<2> for LimitsWithTrait {
    const INDEX: [usize; 2] = [1, 0];
}

trait PrivateLimits {
    const STEP: usize;
}

impl PrivateLimits for LimitsWithTrait {
    const STEP: usize = 2;
}

impl LimitsWithTrait {
    // A downstream glob import brings the anonymously re-exported trait into scope,
    // so `LimitsWithTrait::MAX` resolves to its default of 13.
    const MAX: usize = 8;
    // With `Swizzle` in scope, `LimitsWithTrait::INDEX` resolves to `[1, 0]`.
    // The lint must find `INDEX` on the impl because rustdoc omits the trait declaration.
    const INDEX: [usize; 2] = [0, 1];
    // The private trait cannot preserve `LimitsWithTrait::STEP` for downstream callers.
    pub(crate) const STEP: usize = 1;
}

pub struct HiddenFallback;

#[doc(hidden)]
impl core::simd::Swizzle<2> for HiddenFallback {
    const INDEX: [usize; 2] = [1, 0];
}

impl HiddenFallback {
    // `Swizzle` provides `INDEX` only through a doc-hidden impl.
    const INDEX: [usize; 2] = [0, 1];
}
