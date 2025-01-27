// Non-object safe traits
pub trait RefTrait {
    fn by_ref(self) -> Self;
}
