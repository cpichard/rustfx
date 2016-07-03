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

// Structure to store the bundles and the different plugins ?
pub struct PluginList {
    bundles: Vec<Bundle>,
    plugin_names: HashMap<String, usize>,
}

impl PluginList {
    pub fn from_bundles(bundle_list: Vec<Bundle>) -> PluginList {
        let mapping = PluginList::create_mapping(&bundle_list);
        PluginList {bundles: bundle_list, plugin_names: mapping}
    }
    
    /// Create a mapping between the bundle and the plugins they contain
    fn create_mapping(bundle_list: &Vec<Bundle>) -> HashMap<String, usize> {
        let mut names_bundles_map = HashMap::new();
        for (ind, bundle) in bundle_list.iter().enumerate() {
            for plugin_name in bundle.plugin_names() {
                names_bundles_map.insert(plugin_name, ind);
            }
        }
        names_bundles_map
    }
}

/// A bundle stores plugins
pub struct Bundle {
    path: path:: PathBuf,
    nb_plugins: u32, // TODO : double check type
    dll_handle: *mut c_void,
    get_plugin: extern fn (c_uint) -> *const OfxPlugin,
}

impl Bundle {

    /// Create a Bundle from a directory
    fn init_from_path(dir: io::Result<DirEntry>) -> io::Result<Bundle> {
        let bundle_root = dir.unwrap().path();
        let dll_path = Bundle::get_dll_path(&bundle_root);
        unsafe {
            println!("Loading {:?}", bundle_root);
            // Open the dynamic library
            let plug_dll : *mut c_void = dlopen(dll_path, 1); 
            if plug_dll.is_null() {
                let error_message = format!("unable to dlopen {:?} reason is: {:?}", 
                                    CStr::from_ptr(dll_path),
                                    CStr::from_ptr(dlerror()));
                let custom_error = io::Error::new(io::ErrorKind::Other, error_message);
                return Err(custom_error)
            }
            // Look for the function that returns the number of plugins 
            let c_nb_plugins_fun = dlsym(plug_dll, from_str(ofx_get_number_of_plugins));
            if c_nb_plugins_fun.is_null() {
                let error_message = format!("unable to find function {}", 
                                                ofx_get_number_of_plugins);
                let custom_error = io::Error::new(io::ErrorKind::Other, error_message);
                return Err(custom_error)
            }
            let nb_plugins 
                : extern fn () -> c_uint 
                    = mem::transmute(c_nb_plugins_fun);

            // Look for the function that returns a structure describing the plugin
            let c_get_plugin_fun = dlsym(plug_dll, from_str(ofx_get_plugin));
            if c_get_plugin_fun.is_null() {
                let error_message = format!("unable to find function {}", 
                                                    ofx_get_plugin);
                let custom_error = io::Error::new(io::ErrorKind::Other, error_message);
                return Err(custom_error)
            }
            let get_plugin_fun 
                : extern fn (c_uint) -> *const OfxPlugin 
                    = mem::transmute(c_get_plugin_fun);
            // Everything went fine, so we return a new bundle
            return Ok(Bundle {
                    path: bundle_root, 
                    nb_plugins: nb_plugins(), 
                    dll_handle: plug_dll,
                    get_plugin: get_plugin_fun});
        }
    }

    /// Returns the path of the dynamic library given the bundle root path 
    fn get_dll_path(bundle_root: & PathBuf) -> * const c_char {
        let dll_name = bundle_root.file_stem().unwrap();
        let dll_path = if cfg!(target_os = "macos") { 
            bundle_root.join("Contents/MacOS-x86-64")
                       .join(dll_name)
        } else if cfg!(target_os = "linux") {
            bundle_root.join("Contents/Linux-x86-64")
                       .join(dll_name)
        } else {
            panic!("this application does not work on this operating system");
        };
        from_str(dll_path.to_str().unwrap())
    }

    fn plugin_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        // List plugin names of the bundle
        for index in 0..(self.nb_plugins) {
            let plugin = (self.get_plugin)(index);
            if ! plugin.is_null() {
                let ptr_wrap = unsafe {CStr::from_ptr((*plugin).pluginIdentifier)};
                let plug_name = ptr_wrap.to_str().unwrap().to_string();
                names.push(plug_name);
            }
        }
        names
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
                // FIXME LOG println!("Found '{}'", path.display());
                paths.push(path);
            }   
        }

        None => {
            //FIXME LOG println!("environment variable OFXPLUGINS not set");
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

/// Returns a list of found bundles in the bundle_paths
pub fn init_bundles(bundle_paths: Vec<PathBuf>) -> Vec<Bundle> {

    let mut bundles : Vec<Bundle> = Vec::new();

    for path in bundle_paths {
        match path.as_path().read_dir() {
            Ok(entries) => { 
                for d_entry in entries {
                    if is_ofx_bundle(&d_entry) {
                        match Bundle::init_from_path(d_entry) {
                            Ok(bundle) => { 
                                bundles.push(bundle)
                            }
                            Err(k) => {
                                println!("error '{}'", k);
                            }
                        }
                    } else {
                        println!("warning: folder {:?} is not an ofx bundle", 
                                    d_entry.unwrap().path());
                    }
                }
            }
            Err(k) => {
                println!("{:?}: {}", path, k);
            }
        }
    }
    
    bundles
}

fn from_str(s:& str) -> * const c_char {
    //CStr::from_bytes_with_nul(s.as_bytes()).unwrap().as_ptr()
    CString::new(s).unwrap().as_ptr()
}

