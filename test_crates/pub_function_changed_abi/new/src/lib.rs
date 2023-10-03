pub extern "Rust" fn c_function_becomes_rust() -> () {}

pub fn c_function_becomes_rust_implicitly() -> () {}

pub extern "C" fn rust_function_becomes_c() -> () {}

pub extern "C" fn implicit_rust_function_becomes_c() -> () {}

pub extern "Rust" fn implicit_c_function_becomes_rust() -> () {}

pub extern "Rust" fn rust_function_remains_rust() -> () {}

pub extern "C" fn c_function_remains_c() -> () {}

pub extern "C" fn system_function_becomes_c() -> () {}

extern "Rust" fn private_c_function_becomes_rust() -> () {}
