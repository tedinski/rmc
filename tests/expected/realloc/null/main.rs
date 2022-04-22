// Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Calling realloc with a null pointer fails

use std::alloc::{realloc, Layout};

#[kani::proof]
fn main() {
    unsafe {
        let layout = Layout::array::<i32>(0).unwrap();
        let ptr: *const u8 = std::ptr::null();

        let _ptr = realloc(ptr as *mut u8, layout, 12);
    }
}
