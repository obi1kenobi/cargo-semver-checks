#![no_std]

mod abi_only {
    #[unsafe(no_mangle)]
    pub extern "C" fn parameter_count_changed(a: i32) {
        let _ = a;
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn return_value_removed() -> i32 {
        5
    }

    #[unsafe(no_mangle)]
    pub extern "C" fn abi_now_unwind() {}

    #[unsafe(no_mangle)]
    pub extern "C-unwind" fn abi_no_longer_unwind() {}
}
