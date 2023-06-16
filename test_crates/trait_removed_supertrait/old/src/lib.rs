pub trait SuperTrait {}
pub trait SuperTrait2 {}
pub trait GenericSuperTrait<T> {}

// Lint should warn us about
pub trait RemovingSingleTrait : SuperTrait {}
pub trait RemovingOneTraitOfMultiple : SuperTrait + SuperTrait2 {}
pub trait RemovingOneWithGenerics<T> : SuperTrait + SuperTrait2 {}
pub trait RemovingOneWithGenericsOnTheSuperTrait<T> : SuperTrait + GenericSuperTrait<T> {}
pub trait RemovingTraitAndLifetime<'a> : SuperTrait + SuperTrait2 + 'a {}
pub trait RemovingMultiple: SuperTrait + SuperTrait2 {}

// Lint should ignore
pub trait CorrectTrait : SuperTrait {}
pub trait CorrectTraitMultipleSuperTraits : SuperTrait + SuperTrait2 {}
pub trait CorrectTraitRemovingLifetime<'a> : SuperTrait + 'a {}
pub trait CorrectTraitChangingTheGenericTypeBreaking : GenericSuperTrait<i64> {}
pub trait CorrectTraitChangingTheGenericTypeNonBreaking<T> : GenericSuperTrait<T> where T: Into<i64> {}
