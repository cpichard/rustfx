// TODO dynamic loader
//extern crate ofx_core;
//use ofx_core;
// import openfx modules
mod ofx;
use ofx::bundle::*;
use ofx::plugin::*;
use ofx::core::*;

// declare external crate libc ?
extern crate libc;

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

        //let path = "/home/cyril/develop/hopenfx/src/plugins/HOpenFX.ofx.bundle/Contents/Linux-x86-64/HOpenFX.ofx";
        let path = "/home/cyril/develop/tests/rust/rustfx/OFX/basic.ofx.bundle/Contents/Linux-x86-64/basic.ofx";
        let c_path = CString::new(path).unwrap();
        let c_get_nplugs = CString::new("OfxGetNumberOfPlugins").unwrap();
        let mut plugin = HashMap::new();

        unsafe {
            let plug = dlopen(c_path.as_ptr(), 1); // RTLD_LAZY 
            let nb_plugins = dlsym(plug, c_get_nplugs.as_ptr());
            // Count the number of plugins in the bundle
            //if nb_plugins != std::ptr::null_mut() {
            if !nb_plugins.is_null() {
                // Should we create an OfxBundle object from here ?
                plugin.insert(path, plug);
                let nb_plugins_fun : extern fn () -> c_int = std::mem::transmute(nb_plugins);
                print!("Number of plugins in this bundle: {}\n", nb_plugins_fun());

                // Call get plugin
                //let get_plugin = "OfxGetPlugin";
                let c_get_plugin = CString::new("OfxGetPlugin").unwrap();
                let c_fn_get_plugin = dlsym(plug, c_get_plugin.as_ptr());
                let fn_get_plugin : extern fn (c_uint) -> *const OfxPlugin 
                    = std::mem::transmute(c_fn_get_plugin);
                // TODO iterate on the number of plugins
                let plug0 = fn_get_plugin(0);
                //let plug_api = CStr::from_ptr((*plug0).pluginApi).to_string_lossy();
                //print!("plug api: {}\n", plug_api);
                print!("plug {:?}\n", *plug0);

                // Create an Host
                let mut ofx_host = OfxHost::new();
                let host_ptr : * mut libc::c_void = std::mem::transmute(& mut ofx_host);
                // Pass it to the plugin 
                // TODO: what is the lifetime of host_ptr ?
                ((*plug0).setHost)(host_ptr);




            }
        }

        print!("Found {} plugins\n", plugin.len());

}
