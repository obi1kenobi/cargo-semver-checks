#[cfg(not(feature = "unit_struct_changed_kind"))]
pub struct UnitStructToPlain;

#[cfg(feature = "unit_struct_changed_kind")]
pub struct UnitStructToPlain {}

#[cfg(not(feature = "unit_struct_changed_kind"))]
#[non_exhaustive]
pub struct NonExhaustiveUnitStructToPlain;

#[cfg(feature = "unit_struct_changed_kind")]
#[non_exhaustive]
pub struct NonExhaustiveUnitStructToPlain {}
