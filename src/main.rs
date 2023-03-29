extern crate libc;

use std::ffi::CString;
use libc::{strlen, strcmp};

fn main() {
    let c_string_1 = CString::new("Hello, world!").expect("CString::new failed");
    let length = unsafe { strlen(c_string_1.as_ptr()) };
    println!("Length: {}", length);
    let c_string_2 = CString::new("Hello, world!").expect("CString::new failed");
    let result: i32 = unsafe { strcmp(c_string_1.as_ptr(), c_string_2.as_ptr()) };
    // returns 0 if equal.
    println!("Result: {}", result);
}
