pub trait RemovedTraitMethod {
    fn fooA();
}

// This trait is private. Its removal is not breaking and should not be reported.
trait PrivateTrait {
    fn fooB();
}
