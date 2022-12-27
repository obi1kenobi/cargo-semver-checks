// Basic case, should be reported.
pub struct TupleToPlainStructPublicFields(pub i32, pub usize, pub String);

// This struct is not externally constructible due to a private field and should not be reported.
pub struct TupleToPlainStructPrivateFields(i32, usize, String);

// This struct is explicitly #[non_exhaustive] and should not be reported.
#[non_exhaustive]
pub struct TupleToPlainStructNonExhaustive(pub i32, pub usize, pub String);

// Even though this struct has no fields, changing it to plain struct is a breaking change.
pub struct TupleToPlainStructEmpty();

// This struct is not publicly-visible, so it should not be reported.
struct TupleToPlainStructPrivate(pub i32, pub usize, pub String);

// Tuple struct changing to unit struct is a different kind of breaking change.
pub struct TupleToUnitStruct();
