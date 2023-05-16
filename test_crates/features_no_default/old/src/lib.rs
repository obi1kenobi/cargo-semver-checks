#[cfg(feature = "A")]
pub fn function_previously_depending_on_A() {}

#[cfg(feature = "B")]
pub fn function_depending_on_B() {}
