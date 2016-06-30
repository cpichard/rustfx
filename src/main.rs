// TODO dynamic loader
//extern crate ofx_core;
//use ofx_core;
// import openfx modules
// declare external crate libc ?
extern crate libc;
mod ofx;
use ofx::plugin::*;
use ofx::core::*;

mod engine;
use engine::*;

mod bundle;
use bundle::*;

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
    // Initialization
    let bundle_paths = get_bundle_paths();

    // Not sure we need to init bundles, but we need to list the available plugins by name
    // though also we need to filter the ones that are compatible with our host
    let mut bundles = init_bundles(bundle_paths);

    //let plugins = get_plugin_list(bundle_paths);
    //let plugins = get_plugins(bundle_paths);

    // So 
    let engine = Engine::new();

    load_plugin_test();

    //let read_plugin = plugins.create_instance(ofxhost, "readexr");
    //let write_plugin = plugins.create_instance(ofxhost, "writeexr");

    // Load and parse scene description
    // We need a structure to store a project
    // Look for read plugin
    // Look for write plugin
    // Connect them
    // and render

    //print!("Found {} plugins\n", plugin.len());
}
