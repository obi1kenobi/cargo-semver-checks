#![no_std]

pub struct Reader<T>(pub T);

impl Reader<u8> {
    // `Reader::<u8>::changed(&reader)` still works, but `reader.changed()` no longer does.
    // TODO: Report this receiver loss once the adapter distinguishes this impl from the u16 impl.
    pub fn changed(_: &Self) {}

    // Both methods keep their receivers when one becomes doc-hidden and the other becomes private.
    #[doc(hidden)]
    pub fn hidden(&self) {}
    fn private(&self) {}
}

impl Reader<u16> {
    // These functions already existed. In particular, the removal of `Reader<u8>::removed`
    // must not make the unchanged u16 function look like it lost a receiver.
    pub fn changed(_: &Self) {}
    pub fn removed(_: &Self) {}
    pub fn hidden(_: &Self) {}
    pub fn private(_: &Self) {}

    // Each release has only one `moved`, but the two declarations apply to different types.
    pub fn moved(_: &Self) {}
}

// Callers using `GainsParameter` now get `GainsParameter<0>`. Only `GainsParameter<1>` gets this function.
pub struct GainsParameter<const N: usize = 0>;
impl GainsParameter<1> {
    pub fn read(_: &Self) {}
}

// The old default instantiation never had a `read` method. The receiver belonged to `LosesParameter<1>`.
pub struct LosesParameter;
impl LosesParameter {
    pub fn read(_: &Self) {}
}

pub trait Read {
    fn changed(_: &Self) {}
    fn hidden(_: &Self) {}
    fn from_trait(_: &Self) {}
    fn preserved(&self) {}
    fn public_const(_: &Self) {}
    fn private_const(_: &Self) {}
}

pub trait OtherRead {
    fn from_trait(_: &Self) {}
}

pub struct Lookup;
impl Read for Lookup {}
impl OtherRead for Lookup {}

impl Lookup {
    // `value.changed()` breaks. `Lookup::changed(&value)` still calls the inherent function,
    // so the diagnostic should point here even with `Read` in scope.
    pub fn changed(_: &Self) {}

    // Both call forms still select this method. Only its API visibility changed.
    #[doc(hidden)]
    pub fn hidden(&self) {}

    // Downstream, importing either trait on its own keeps `Lookup::from_trait(&value)`
    // working. Dot syntax breaks. Cite both trait declarations, which callers can select
    // by importing `Read` or `OtherRead`.
    fn from_trait(&self) {}

    // `Lookup::preserved(&value)` uses this function, while `value.preserved()`
    // still finds the method from `Read` when the trait is in scope.
    pub fn preserved(_: &Self) {}

    // Even with `Read` in scope, `Lookup::public_const` resolves to this constant.
    // A doc-hidden constant still takes precedence over the trait function.
    #[allow(non_upper_case_globals)]
    #[doc(hidden)]
    pub const public_const: u8 = 0;

    // A private constant lets downstream `Lookup::private_const(&value)` reach the trait function.
    #[allow(non_upper_case_globals)]
    const private_const: u8 = 0;
}
