#![no_std]

mod abi_only {
    #[unsafe(no_mangle)]
    pub extern "C" fn parameter_count_changed(a: i32, b: i32) {
        let _ = (a, b);
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn return_value_removed() {
        // now returns unit
    }

    #[unsafe(no_mangle)]
    pub extern "C-unwind" fn abi_now_unwind() {}

    #[unsafe(no_mangle)]
    pub extern "C" fn abi_no_longer_unwind() {}
}
