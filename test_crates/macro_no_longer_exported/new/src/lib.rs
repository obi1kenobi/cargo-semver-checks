// No longer exported but not hidden - should trigger
macro_rules! example_macro {
    () => {
        println!("Hello from macro!");
    };
}

// No longer exported but is hidden - should NOT trigger (caught by different lint)
#[doc(hidden)]
macro_rules! will_be_hidden {
    () => {
        println!("Will become hidden");
    };
}

// Now exported - should not trigger
#[macro_export]
macro_rules! internal_macro {
    () => {
        println!("Internal macro");
    };
}
