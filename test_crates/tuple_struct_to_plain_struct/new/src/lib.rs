// Basic case, should be reported.
pub struct TupleToPlainStructPublicFields {
    pub a: i32,
    pub b: usize,
    pub c: String,
}

// This struct is not externally constructible due to a private field and should not be reported.
pub struct TupleToPlainStructPrivateFields {
    a: i32,
    b: usize,
    c: String,
}

// This struct is explicitly #[non_exhaustive] and should not be reported.
#[non_exhaustive]
pub struct TupleToPlainStructNonExhaustive {
    pub a: i32,
    pub b: usize,
    pub c: String,
}

// Even though this struct has no fields, changing it to plain struct is a breaking change.
pub struct TupleToPlainStructEmpty {}

// This struct is not publicly-visible, so it should not be reported.
struct TupleToPlainStructPrivate {
    pub a: i32,
    pub b: usize,
    pub c: String,
}

// Tuple struct changing to unit struct is a different kind of breaking change.
pub struct TupleToUnitStruct;

// Plain struct changing to tuple struct is a different kind of breaking change.
pub struct PlainToTupleStruct(pub i32, pub usize, pub String);

// Struct becoming non_exhaustive should take priority, and not get reported
#[non_exhaustive]
pub struct TupleToNonExhaustivePlainStruct {
    pub a: i32,
    pub b: usize,
    pub c: String,
}
