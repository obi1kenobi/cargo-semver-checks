mod a {
}

mod b {
    pub mod b {
    }
}

pub mod bb {
    pub mod will_remove {
    }
}

pub mod will_make_private {
    mod e {
    }
}
