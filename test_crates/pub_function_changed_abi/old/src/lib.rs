pub extern "C" fn c_function_becomes_rust() -> () {}

pub extern "C" fn c_function_becomes_rust_implicitly() -> () {}

pub extern "Rust" fn rust_function_becomes_c() -> () {}

pub fn implicit_rust_function_becomes_c() -> () {}

pub extern fn implicit_c_function_becomes_rust() -> () {}

pub extern "Rust" fn rust_function_remains_rust() -> () {}

pub extern "C" fn c_function_remains_c() -> () {}
