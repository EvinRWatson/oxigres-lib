#[path = "./lib.rs"]
mod lib;

use crate::lib::*;
use std::ffi::CString;
use std::os::raw::c_char;

// main is only used for testing purposes in the console, it is not used in the actual library
fn main() {
    let mut input: String = String::new();

    println!("Input Query:");

    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let response_ptr: *mut c_char = query_database(CString::new(input).unwrap().into_raw());

    let response_converted: CString = unsafe { CString::from_raw(response_ptr as *mut c_char) };

    println!("Response:\n{}", response_converted.to_str().unwrap());
}