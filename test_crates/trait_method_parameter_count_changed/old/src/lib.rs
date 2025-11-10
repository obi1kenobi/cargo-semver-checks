#![no_std]

pub trait Example {
    fn gain_a_parameter();

    fn gain_a_parameter_with_receiver(&self);

    fn lose_a_parameter(a: i64);

    fn lose_a_parameter_with_receiver(&self, a: i64);

    // This is breaking for unsealed traits, because the impl has to match.
    fn parameter_becomes_receiver(x: &Self);

    // This is breaking for both unsealed traits (impl has to match) and in general
    // if the method is callable, because calling via the receiver doesn't work anymore.
    fn receiver_becomes_parameter(&self);
}

mod private {
    pub trait Sealed {}
}

pub trait SealedExample: private::Sealed {
    // This is breaking for unsealed traits, because the impl has to match.
    fn parameter_becomes_receiver(x: &Self);

    // This is breaking for both unsealed traits (impl has to match) and in general
    // if the method is callable, because calling via the receiver doesn't work anymore.
    // It's still breaking here.
    fn receiver_becomes_parameter(&self);
}
