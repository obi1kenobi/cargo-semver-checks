#![allow(dead_code)]

pub struct Example<'a, 'b>(&'a i64, &'b i64);

#[non_exhaustive]
pub enum Either<'a, 'b> {
    Left(&'a i64),
    Right(&'b u64),
}

pub union OneOrTheOther<'a, 'b> {
    left: &'a i64,
    right: &'b u64,
}

// Renaming lifetimes while leaving them semantically identical is not breaking.
// AFAIK there's no way to refer to a lifetime's defined name while using the type.
pub struct RenamedLifetimes<'a, 'b>(&'a i64, &'b i64);

// Attempting to specify lifetime parameters that don't exist is breaking.
// This should be reported.
pub struct NotGenericAnymore<'a>(&'a str);

// This is breaking too and should be reported. The witness is just wrapping the type
// in another type, at which point all generics must be specified.
/// ```rust
/// struct Witness(type_mismatched_generic_lifetimes::BecameGeneric);
/// ```
pub struct BecameGeneric(String);
