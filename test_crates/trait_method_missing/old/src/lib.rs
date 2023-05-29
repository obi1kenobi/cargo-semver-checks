pub trait RemovedTraitMethod {
    fn fooA();
}

// This trait gets removed completely so a missing trait method should not be reported.
pub trait RemovedTraitWithMethod {
    fn fooA();
}

// This trait is private. Its removal is not breaking and should not be reported.
trait PrivateTrait {
    fn fooB();
}
