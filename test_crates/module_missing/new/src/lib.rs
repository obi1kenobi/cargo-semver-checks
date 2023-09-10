// Removing this, but no warning should happen:
// it isn't public.
//
// mod a {}

mod b {
    // Removing this, but no warning should happen:
    // it isn't visible.
    // pub mod b {}
}

pub mod bb {
    // Removing this should cause a warning.
    //
    // pub mod will_remove {}
}

// Making this private should cause a warning
pub(crate) mod will_make_private {
    mod e {}
}

// Adding a module shouldn't cause problems.
pub mod new_module {}
