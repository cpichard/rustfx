// TODO dynamic loader
//extern crate ofx_core;
//use ofx_core;
// import openfx modules
// declare external crate libc ?
extern crate libc;
mod ofx;
use ofx::bundle::*;
use ofx::plugin::*;
use ofx::core::*;
mod host;
use host::*;

// import everything from libc ?
use libc::*;
use std::ffi::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::DirEntry;
use std::io::Result;
use std::str;

fn main() {
    // Get env OFX and list all the plugins specified in the path
    // split using :
    let bundle_paths = get_bundle_paths();
    let bundles = init_bundles(bundle_paths);
    load_plugin_test();
    //print!("Found {} plugins\n", plugin.len());
}
