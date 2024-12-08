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
