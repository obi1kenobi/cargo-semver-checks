#![no_std]

// Only callers using Partial<u8> see new deprecation warnings, changes to unused-result
// warnings, and the loss of reset and LIMIT from the supported API.
// TODO: Report the six changes on u8 once impl targets can be matched.
pub struct Partial<T>(pub T);
impl Partial<u8> {
    #[must_use]
    #[deprecated]
    pub fn check(&self) -> bool {
        true
    }
    pub fn finish(&self) -> u8 {
        0
    }
    #[doc(hidden)]
    pub fn reset(&self) {}
    #[deprecated]
    pub const DEFAULT: u8 = 0;
    #[doc(hidden)]
    pub const LIMIT: u8 = 1;
}
impl Partial<u16> {
    pub fn check(&self) -> bool {
        true
    }
    #[must_use]
    pub fn finish(&self) -> u8 {
        0
    }
    pub fn reset(&self) {}
    pub const DEFAULT: u8 = 0;
    pub const LIMIT: u8 = 1;
}

// The attributes on Converges<u8> now match those on Converges<u16>. Comparing names
// alone can point the diagnostic at an unchanged u16 declaration.
// TODO: Report the six changes on u8 once impl targets can be matched.
pub struct Converges<T>(pub T);
impl Converges<u8> {
    #[must_use]
    #[deprecated]
    pub fn check(&self) -> bool {
        true
    }
    pub fn finish(&self) -> u8 {
        0
    }
    #[doc(hidden)]
    pub fn reset(&self) {}
    #[deprecated]
    pub const DEFAULT: u8 = 0;
    #[doc(hidden)]
    pub const LIMIT: u8 = 1;
}
impl Converges<u16> {
    #[must_use]
    #[deprecated]
    pub fn check(&self) -> bool {
        true
    }
    pub fn finish(&self) -> u8 {
        0
    }
    #[doc(hidden)]
    pub fn reset(&self) {}
    #[deprecated]
    pub const DEFAULT: u8 = 0;
    #[doc(hidden)]
    pub const LIMIT: u8 = 1;
}

// Removing the impl breaks calls on Removed<u8>. Each member on Removed<u16>
// keeps the attributes it had before the removal.
pub struct Removed<T>(pub T);
impl Removed<u16> {
    #[must_use]
    #[deprecated]
    pub fn check(&self) -> bool {
        true
    }
    pub fn finish(&self) -> u8 {
        0
    }
    #[doc(hidden)]
    pub fn reset(&self) {}
    #[deprecated]
    pub const DEFAULT: u8 = 0;
    #[doc(hidden)]
    pub const LIMIT: u8 = 1;
}

// The sole impl moves from Moved<u8> to Moved<u16>. Calls on Moved<u8> lose
// their targets, while calls on Moved<u16> gain entirely new members.
pub struct Moved<T>(pub T);
impl Moved<u16> {
    #[must_use]
    #[deprecated]
    pub fn check(&self) -> bool {
        true
    }
    pub fn finish(&self) -> u8 {
        0
    }
    #[doc(hidden)]
    pub fn reset(&self) {}
    #[deprecated]
    pub const DEFAULT: u8 = 0;
    #[doc(hidden)]
    pub const LIMIT: u8 = 1;
}

// A downstream alias declared as `type Old = GainsParameter;` now denotes
// GainsParameter<0>. Only GainsParameter<1> provides the new members.
pub struct GainsParameter<const N: usize = 0>;
impl GainsParameter<1> {
    #[must_use]
    #[deprecated]
    pub fn check(&self) -> bool {
        true
    }
    pub fn finish(&self) -> u8 {
        0
    }
    #[doc(hidden)]
    pub fn reset(&self) {}
    #[deprecated]
    pub const DEFAULT: u8 = 0;
    #[doc(hidden)]
    pub const LIMIT: u8 = 1;
}

// The old default LosesParameter<0> had no such members, and callers that named
// LosesParameter<1> can no longer name their type.
pub struct LosesParameter;
impl LosesParameter {
    #[must_use]
    #[deprecated]
    pub fn check(&self) -> bool {
        true
    }
    pub fn finish(&self) -> u8 {
        0
    }
    #[doc(hidden)]
    pub fn reset(&self) {}
    #[deprecated]
    pub const DEFAULT: u8 = 0;
    #[doc(hidden)]
    pub const LIMIT: u8 = 1;
}

pub struct Control;
impl Control {
    // Deprecated items still belong to the supported API when hidden. Report the
    // deprecations of check and DEFAULT as well as the new #[must_use] on check.
    #[doc(hidden)]
    #[deprecated]
    #[must_use]
    pub fn check(&self) -> Result<(), u8> {
        Ok(())
    }

    pub fn finish(&self) -> u8 {
        0
    }

    #[doc(hidden)]
    pub fn reset(&self) {}
    #[doc(hidden)]
    #[deprecated]
    pub const DEFAULT: u8 = 0;
    #[doc(hidden)]
    pub const LIMIT: u8 = 1;

    // Callers still get the same kinds of warnings with the revised messages.
    #[must_use = "inspect the message"]
    #[deprecated = "inspect the status"]
    pub fn message(&self) -> u8 {
        2
    }
    #[deprecated = "legacy format"]
    pub const MESSAGE: u8 = 2;
}
