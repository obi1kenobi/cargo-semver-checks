pub trait SuperTrait {}
pub trait SuperTrait2 {}
pub trait GenericSuperTrait<T> {}

pub trait RemovingSingleTrait {}
pub trait RemovingOneTraitOfMultiple : SuperTrait  {}
pub trait RemovingOneWithGenerics<T> : SuperTrait  {}
pub trait RemovingOneWithGenericsOnTheSuperTrait<T> : SuperTrait  {}
pub trait RemovingTraitAndLifetime<'a> : SuperTrait {}
pub trait RemovingMultiple {}

pub trait CorrectTrait : SuperTrait {}
pub trait CorrectTraitMultipleSuperTraits : SuperTrait + SuperTrait2 {}
pub trait CorrectTraitRemovingLifetime<'a> : SuperTrait {}
pub trait CorrectTraitChangingTheGenericTypeBreaking : GenericSuperTrait<String> {}
pub trait CorrectTraitChangingTheGenericTypeNonBreaking : GenericSuperTrait<i64> {}
pub trait NotPresentOnPreviousVersion {}
