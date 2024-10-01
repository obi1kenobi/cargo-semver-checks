/// Should trigger the function_must_use_added lint, which is configured
/// as allow (priority -1) and major (priority 0)
// + #[must_use]
pub fn function() {}

/// Should trigger the enum_must_use_added lint, which is configured as
/// warn and major from the `must_use_added` lint group
// + #[must_use]
pub enum Enum {}

/// Should trigger the struct_must_use_added lint, which is configured as deny with priority 1,
/// but overriden in the `must_use_added` with priority 0.
// + #[must_use]
pub struct Struct;
