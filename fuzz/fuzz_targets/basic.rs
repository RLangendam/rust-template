#![no_main]

use std::convert::TryInto;
use libfuzzer_sys::fuzz_target;
use my_project_lib::add;

fuzz_target!(|data: &[u8]| {
    // If we have at least 16 bytes, interpret the first 16 as two u64 values
    // and pass them into the library's `add` API after narrowing to usize.
    if data.len() >= 16 {
        let a_bytes: [u8; 8] = data[0..8].try_into().unwrap();
        let b_bytes: [u8; 8] = data[8..16].try_into().unwrap();
        let a = u64::from_le_bytes(a_bytes) as usize;
        let b = u64::from_le_bytes(b_bytes) as usize;

        // Call into the library under test. Keep the call simple; the fuzzer
        // will mutate input and explore behaviors.
        let _ = add(a, b);
    }
});
