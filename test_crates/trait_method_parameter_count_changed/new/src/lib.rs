#![no_std]

pub trait Example {
    fn gain_a_parameter(x: i64);

    fn gain_a_parameter_with_receiver(&self, x: i64);

    fn lose_a_parameter();

    fn lose_a_parameter_with_receiver(&self);

    // This is breaking for unsealed traits, because the impl has to match.
    fn parameter_becomes_receiver(&self);

    // This is breaking for both unsealed traits (impl has to match) and in general
    // if the method is callable, because calling via the receiver doesn't work anymore.
    fn receiver_becomes_parameter(x: &Self);
}

mod private {
    pub trait Sealed {}
}

pub trait SealedExample {
    // This is breaking for unsealed traits, because the impl has to match.
    // This trait is sealed, so this shouldn't get flagged.
    fn parameter_becomes_receiver(&self);

    // This is breaking for both unsealed traits (impl has to match) and in general
    // if the method is callable, because calling via the receiver doesn't work anymore.
    // It's still breaking here.
    fn receiver_becomes_parameter(x: &Self);
}
