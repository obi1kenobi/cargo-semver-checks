#![no_std]

pub extern "C" fn unwind_function_becomes_non_unwind() {}

pub extern "C-unwind" fn non_unwind_function_becomes_unwind() {}

pub extern "C" fn non_unwind_function_unchanged() {}

pub extern "C-unwind" fn unwind_function_unchanged() {}
