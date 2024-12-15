// No longer exported but not hidden - should trigger
macro_rules! example_macro {
    () => {
        println!("Hello from macro!");
    };
}

// No longer exported but is hidden - should NOT trigger (caught by different lint)
#[doc(hidden)]
macro_rules! will_be_hidden_and_not_exported {
    () => {
        println!("Will become hidden and not exported");
    };
}

// Now exported - should not trigger
#[macro_export]
macro_rules! internal_macro {
    () => {
        println!("Internal macro");
    };
}

pub mod foo {
    // Private macro, which is not exported despite being in a public module.
    // Macros require `#[macro_export]` or they aren't visible outside their crate.
    //
    // This is a breaking change.
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
