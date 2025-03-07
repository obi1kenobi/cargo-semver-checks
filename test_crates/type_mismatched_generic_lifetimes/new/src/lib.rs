#![no_std]

#![allow(dead_code)]

pub struct Example<'a, 'b, 'c>(&'a i64, &'b i64, &'c i64);

pub enum Either<'a, 'b, 'c> {
    Left(&'a i64),
    Right(&'b u64),
    Mid(&'c isize),
}

pub union OneOrTheOther<'a, 'b, 'c> {
    left: &'a i64,
    right: &'b u64,
    center: &'c isize,
}

// Renaming lifetimes while leaving them semantically identical is not breaking.
// AFAIK there's no way to refer to a lifetime's defined name while using the type.
pub struct RenamedLifetimes<'c, 'a>(&'c i64, &'a i64);

// Attempting to specify lifetime parameters that don't exist is breaking.
// This should be reported.
pub struct NotGenericAnymore(&'static str);

// This is breaking too and should be reported. The witness is just wrapping the type
// in another type, at which point all generics must be specified.
/// ```rust,compile_fail
/// struct Witness(type_mismatched_generic_lifetimes::BecameGeneric);
/// ```
pub struct BecameGeneric<'a>(&'static str, &'a str);
