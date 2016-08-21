extern crate libc;
use libc::*;
use std::mem::*;
use std::ffi::*;

///
/// This code crashes the compiler
///

// Store string as a unsigned char buffer, like in ofx
pub const CONST_C_STR1: &'static [u8] = b"test1\0";


// Convert from C char pointer to rust unsigned char buffer
fn pack<'a>(value: * const c_char) -> &'a[u8] {
    let from_client = unsafe {CStr::from_ptr(value)};
    from_client.to_bytes_with_nul()
}

fn compare_extern(value: * const c_char) {
    match pack(value){
        CONST_C_STR1 => println!("test1"),
        a => println!("{:?} {:?}", CONST_C_STR1, a),
    }
}

fn compare_intern(value : * const c_char) {
    println!("{:?}", value); 
}

fn main() {
    let test = CString::new("test1").unwrap();
    compare_extern(test.as_ptr());

    let r = unsafe{CStr::from_bytes_with_nul_unchecked(CONST_C_STR1).as_ptr()}; 
    compare_intern(r);
}
