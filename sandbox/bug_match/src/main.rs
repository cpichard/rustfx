use std::ffi::*;

#[derive(PartialEq, Eq)]
pub struct MyStruct<'a>(&'a [u8]);

pub const CONSTANT: MyStruct<'static> =
    MyStruct(b"ConstantString\0");

fn main() {
    let cstr = CString::new("ConstantString").unwrap();
    match MyStruct(cstr.to_bytes_with_nul()) {
        //ref v @ MyStruct(_) if *v == CONSTANT => println!("compilation error here"),
        CONSTANT => println!("compilation error here"),
        _ => println!("not found"),
    }
}
