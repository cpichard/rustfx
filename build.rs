use std::process::Command;
use std::env;
use std::path::Path;

extern crate gcc;

fn main() {
    gcc::compile_library("libofxhelpers.a", &["src/ofx/ofxhelpers.c"]);
}
//fn main() {
//    let out_dir = env::var("OUT_DIR").unwrap();
//	println!("out dir is {}", out_dir);
//
//    Command::new("gcc").args(&["src/ofx/ofxhelpers.c", "-c", "-fPIC", "-o"])
//                       .arg(&format!("{}/ofxhelpers.o", out_dir))
//                       .status().unwrap();
//
//
//    Command::new("ar").args(&["crus", "libofxhelpers.a", "ofxhelpers.o"])
//                      .current_dir(&Path::new(&out_dir))
//                      .status().unwrap();
//
//    println!("cargo:rustc-link-search=native={}", out_dir);
//    println!("cargo:rustc-link-lib=static=ofxhelpers");
//}

