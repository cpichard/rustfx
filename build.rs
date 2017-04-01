extern crate gcc;

fn main() {
    //gcc::compile_library("libparam.a", &["src/suites/ofxhelpers.c"]);
    gcc::Config::new()
        .file("src/suites/param.c")
        .file("src/suites/message.c")
        .file("src/suites/core.c")
        .flag("-std=c99")
        .compile("libofxc.a");
}

