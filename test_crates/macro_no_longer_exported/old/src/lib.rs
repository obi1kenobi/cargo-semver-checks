#[macro_export]
macro_rules! example_macro {
    () => {
        println!("Hello from macro!");
    };
}

#[macro_export]
macro_rules! will_be_hidden {
    () => {
        println!("Will become hidden");
    };
}

// Internal macro - should not trigger when made public
macro_rules! internal_macro {
    () => {
        println!("Internal macro");
    };
}

mod foo {
    // Public macro. Exported even though it's in a private module,
    // because of the `#[macro_export]`.
    #[macro_export]
    macro_rules! some_macro {
        () => {}
    }
}

mod bar {
    // Private macro by the same name.
    macro_rules! some_macro {
        () => {}
    }
}
