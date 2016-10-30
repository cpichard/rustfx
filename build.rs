extern crate gcc;

fn main() {
    gcc::compile_library("libofxhelpers.a", &["src/bindings/ofxhelpers.c"]);
}

