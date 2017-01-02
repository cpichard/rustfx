extern crate gcc;

fn main() {
    //gcc::compile_library("libofxhelpers.a", &["src/bindings/ofxhelpers.c"]);
    gcc::Config::new().file("src/bindings/ofxhelpers.c").flag("-std=c99").compile("libofxhelpers.a");
}

