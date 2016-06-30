//use libc;
use libc::*;
use ofx::core::*;
use ofx::plugin::*;
use std::collections::HashMap;
use std::env;
use std::ffi::*;
use std::fs::DirEntry;
use std::io;
use std::mem;
use std::path::PathBuf;
use std::ptr;
use std::str;
use std::path;

/// A bundle stores plugins
pub struct Bundle {
    path: path:: PathBuf,
    nb_plugins: i32, // TODO : double check type
    dll_handle: *mut c_void,
    get_plugin: extern fn (c_uint) -> *const OfxPlugin,
    //main_entry:
    //plugin_names: Vec<str>, // for fast retrieval
    // Might have open and close bundle functions
}

impl Bundle {

    fn init_from_path(dir: io::Result<DirEntry>) -> io::Result<Bundle> {
        let bundle_root = dir.unwrap().path();
        let dll_path = Bundle::get_dll_path(&bundle_root);
        let c_dll_path = from_str(dll_path.to_str().unwrap());
        println!("Loading {:?}", dll_path);
        unsafe {
            let plug_dll : *mut c_void = dlopen(c_dll_path, 1); 
            if plug_dll.is_null() {
                let custom_error = io::Error::new(io::ErrorKind::Other, "Unable to dlopen");
                return Err(custom_error)
            }
            let c_nb_plugins_fun = dlsym(plug_dll, from_str(ofx_get_number_of_plugins));
            if c_nb_plugins_fun.is_null() {
                let custom_error = io::Error::new(io::ErrorKind::Other, "Unable to find get number
                                                  of plugins");
                return Err(custom_error)
            }
            let nb_plugins : extern fn () -> c_int 
                = mem::transmute(c_nb_plugins_fun);
            let c_get_plugin_fun = dlsym(plug_dll, from_str(ofx_get_plugin));
            let get_plugin_fun : extern fn (c_uint) -> *const OfxPlugin 
                = mem::transmute(c_get_plugin_fun);
            return Ok(Bundle {
                    path: bundle_root, 
                    nb_plugins: nb_plugins(), 
                    dll_handle: plug_dll,
                    get_plugin: get_plugin_fun});
        }
    }
    
    fn get_dll_path(bundle_root: & PathBuf) -> PathBuf {
        let dll_name = bundle_root.file_stem().unwrap();
        // TODO: use cfg! for multiplatform
        let dll_path = bundle_root.join("Contents/MacOS-x86-64")
                                  .join(dll_name);
    
        dll_path 
    }

}


impl Drop for Bundle {

    // Close dynamic library if it was opened
    fn drop(&mut self) {
        println!("Closing dynamic library!");
        if ! self.dll_handle.is_null() {
            unsafe {
                dlclose(self.dll_handle);
            }
            println!("Closed");
        }
    }

}

/// Get bundle paths from the OFXPLUGINS env variable
pub fn get_bundle_paths() -> Vec<PathBuf> {
    let mut paths : Vec<PathBuf> = Vec::new();
    match env::var_os("OFXPLUGINS") {
        Some(inline_paths) => {
            for path in env::split_paths(&inline_paths) {
                println!("Found '{}'", path.display());
                paths.push(path);
            }   
        }

        None => {
            //println!("environment variable OFXPLUGINS not set");
            paths.push(PathBuf::from("/usr/local/OFX"));
        }
    }

    paths
}

/// Returns true if the given dir follows the ofx bundle specification
fn is_ofx_bundle(dir: & io::Result<DirEntry>) -> bool {
    match *dir {
        Ok(ref entry) => {
            // Should end with "bundle" and be a directory
            // We should be able to test if the pathbuffer endswith ofx.bundle 
            // in encoded in standard ascii, not necessarily in utf8
            if entry.file_name().to_str().unwrap().ends_with(".ofx.bundle") {
                return true;
            } else {
                return false;
            }
        }
        Err(_) => {
            return false;
        }
    }
}

// Look for the dynamic library
//fn read_bundle(dir: io::Result<DirEntry>) -> io::Result<Bundle> {
//    //let custom_error = io::Error::new(io::ErrorKind::Other, "oh no!");
//    //Err(custom_error)
//    // DO STUFF
//    // Find dynamic library containing the plugin, on linux it should stay in
//    let dir_name = dir.unwrap().path();
//    let bundle_name = dir_name.file_stem().unwrap();
//    let ofx_path = dir_name.parent().unwrap()
//                           .join("Contents/MacOS-x86-64")
//                           .join(bundle_name)
//                           .join(".ofx");
//    // TODO : use cfg! as described in the doc for the different platforms
//    //dir/Contents/Linux-x86/bundle_name".ofx"
//    //dir/Contents/Linux-x86-64
//    //dir/Contents/MacOS-x86-64
//
//    Bundle::init_from_path(ofx_path)
//    //let c_ofx_path = from_str(ofx_path.to_str().unwrap());
//    //unsafe {
//    //    // FIXME: find a way to make sure the dll is correctly closed even if 
//    //    // there is an exception 
//    //    let plug_dll : *mut c_void = dlopen(c_ofx_path, 1);  // 1 == RTLD_LAZY 
//    //    let c_nb_plugins_func = dlsym(plug_dll, from_str(ofx_get_number_of_plugins));
//
//    //    // Count the number of plugins in the bundle
//    //    if !c_nb_plugins_func.is_null() {
//    //        dlclose(plug_dll);
//    //        return Ok(Bundle {path: ofx_path, nb_plugins:1, dll_handle: ptr::null_mut()});
//    //    } else {
//    //        let error_func_not_found = io::Error::new(io::ErrorKind::Other,
//    //            "unable to find OfxGetNumberOfPlugins function");
//    //        return Err(error_func_not_found);
//    //    }
//    //}
//}

pub fn get_plugin_list(bundle_paths: Vec<PathBuf>) -> HashMap<String, Bundle> {

    let mut plugins : HashMap<String, Bundle> = HashMap::new();
    plugins

}

/// Returns a list of found bundles in the bundle_paths
pub fn init_bundles(bundle_paths: Vec<PathBuf>) -> Vec<Bundle> {

    let mut bundles : Vec<Bundle> = Vec::new();

    for path in bundle_paths {
        match path.as_path().read_dir() {
            Ok(entries) => { 
                for d_entry in entries {
                    // TODO create bundle from here
                    // Like match OfxBundle::new(d_entry) {
                    //}
                    if is_ofx_bundle(&d_entry) {
                        //println!("dir '{:?}' has bundle", path.as_path());
                        //println!("entry '{:?}'", entry.path());
                        // TODO read nb_plugins
                        match Bundle::init_from_path(d_entry) {
                            Ok(bundle) => { 
                                bundles.push(bundle)
                            }
                            Err(k) => {
                                println!("error '{}'", k);
                            }
                        }
                    }
                }
            }
            Err(k) => {
                println!("error '{}'", k);
                // TODO : put in error
            }
        }
    }
    
    bundles
}

fn from_str(s:& str) -> * const c_char {
    //CStr::from_bytes_with_nul(s.as_bytes()).unwrap().as_ptr()
    CString::new(s).unwrap().as_ptr()
}
//const array: * const c_char = unsafe { mem::transmute("Rust".as_ptr()) };
//const cc_get_nplugs : * const c_char = CString::new("OfxGetNumberOfPlugins").unwrap().as_ptr();


pub fn load_plugin_test() {
    //let path = "/home/cyril/develop/hopenfx/src/plugins/HOpenFX.ofx.bundle/Contents/Linux-x86-64/HOpenFX.ofx";
    //let path = "/home/cyril/develop/tests/rust/rustfx/OFX/basic.ofx.bundle/Contents/Linux-x86-64/basic.ofx";
    let path = "/Users/cyril/Installs/openfx-1.4/Examples/Basic/basic.ofx.bundle/Contents/MacOs/basic.ofx";
    let c_path = CString::new(path).unwrap().as_ptr();
    unsafe {
        let plug_dll = dlopen(c_path, 1); // RTLD_LAZY 
        let c_nb_plugins_fun = dlsym(plug_dll, from_str(ofx_get_number_of_plugins));

        // Count the number of plugins in the bundle
        if !c_nb_plugins_fun.is_null() {
            // Should we create an OfxBundle object from here ?
            let nb_plugins : extern fn () -> c_int = mem::transmute(c_nb_plugins_fun);
            print!("Number of plugins in this bundle: {}\n", nb_plugins());

            // Call get plugin
            let c_get_plugin_fun = dlsym(plug_dll, from_str(ofx_get_plugin));
            let get_plugin : extern fn (c_uint) -> *const OfxPlugin 
                = mem::transmute(c_get_plugin_fun);
            // TODO iterate on the number of plugins
            let plug0 = get_plugin(0);
            //let plug_api = CStr::from_ptr((*plug0).pluginApi).to_string_lossy();
            //print!("plug api: {}\n", plug_api);
            print!("plug {:?}\n", *plug0);

            // Create an Host
            let mut ofx_host = OfxHost::new();
            let host_ptr : * mut c_void = mem::transmute(& mut ofx_host);
            // Pass it to the plugin 
            // TODO: what is the lifetime of host_ptr ?
            ((*plug0).setHost)(host_ptr);

            // Call init functions
            let ofx_action_load = CString::new("OfxActionLoad").unwrap();
            ((*plug0).mainEntry)(ofx_action_load.as_ptr(), ptr::null_mut(), ptr::null_mut(), ptr::null_mut());

        }
    }
}


