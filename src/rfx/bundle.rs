use libc::*;
use bindings::core::*;
use bindings::plugin::*;
use std::env;
use std::ffi::*;
use std::fs::DirEntry;
use std::io;
use std::mem;
use std::path::PathBuf;
use std::str;
use std::ptr;

/// A bundle stores plugins
#[derive(Debug)]
pub struct Bundle {
    dll_path: PathBuf,
    dll_handle: *mut c_void,
    pub nb_plugins: u32, // TODO : double check type
    c_get_plugin: extern "C" fn(c_uint) -> *const OfxPlugin,
}

impl Bundle {
    // Returns a reference of the plugin returned by the library
    pub fn get_plugin(&self, nb: c_uint) -> &mut OfxPlugin {
        let plugin_ptr = (self.c_get_plugin)(nb);
        if !plugin_ptr.is_null() {
            unsafe { mem::transmute(plugin_ptr) }
        } else {
            panic!("plugin pointer is null");
        }
    }
    /// Returns a list of found bundles in the bundle_paths
    pub fn from_paths(bundle_paths: Vec<PathBuf>) -> Vec<Bundle> {

        let mut bundles: Vec<Bundle> = Vec::new();

        for path in bundle_paths {
            match path.as_path().read_dir() {
                Ok(entries) => {
                    for d_entry in entries {
                        if is_ofx_bundle(&d_entry) {
                            match Bundle::create_from_path(d_entry) {
                                Ok(bundle) => bundles.push(bundle),
                                Err(k) => error!("{}", k),
                            }
                        } else {
                            warn!("folder {:?} is not an ofx bundle", d_entry.unwrap().path());
                        }
                    }
                }
                Err(k) => error!("{:?}: {}", path, k), 
            }
        }

        bundles
    }

    /// Create a Bundle from a directory
    fn create_from_path(dir: io::Result<DirEntry>) -> io::Result<Bundle> {
        let bundle_root = dir.unwrap().path();
        let dll_path = Bundle::get_dll_path(&bundle_root);
        debug!("Loading {:?}", &bundle_root);
        // Open the dynamic library
        let c_dll_path = from_str(dll_path.to_str().unwrap());
        unsafe {
            let plug_dll: *mut c_void = dlopen(c_dll_path.as_ptr(), 1);
            if plug_dll.is_null() {
                let error_message = CStr::from_ptr(dlerror()).to_str().unwrap();
                let custom_error = io::Error::new(io::ErrorKind::Other, error_message);
                return Err(custom_error);
            }
            // Look for the function that returns the number of plugins
            let c_nb_plugins_fun = dlsym(plug_dll, from_str(kOfxGetNumberOfPlugins).as_ptr());
            if c_nb_plugins_fun.is_null() {
                let error_message = format!("unable to find function {}", kOfxGetNumberOfPlugins);
                let custom_error = io::Error::new(io::ErrorKind::Other, error_message);
                return Err(custom_error);
            }
            let nb_plugins: extern "C" fn() -> c_uint = mem::transmute(c_nb_plugins_fun);

            // Look for the function that returns a structure describing the plugin
            let c_get_plugin_fun = dlsym(plug_dll, from_str(kOfxGetPlugin).as_ptr());
            if c_get_plugin_fun.is_null() {
                let error_message = format!("unable to find function {}", kOfxGetPlugin);
                let custom_error = io::Error::new(io::ErrorKind::Other, error_message);
                return Err(custom_error);
            }

            // Everything went fine, so we return a new bundle
            Ok(Bundle {
                dll_path: dll_path,
                dll_handle: plug_dll,
                nb_plugins: nb_plugins(),
                c_get_plugin: mem::transmute(c_get_plugin_fun),
            })
        }
    }

    /// Returns the path of the dynamic library given the bundle root path
    fn get_dll_path(bundle_root: &PathBuf) -> PathBuf {
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
        dll_path
    }
}

impl Drop for Bundle {
    // Close dynamic library if it was opened
    fn drop(&mut self) {
        trace!("Closing dynamic library {:?}", self.dll_path);
        if !self.dll_handle.is_null() {
            unsafe {
                dlclose(self.dll_handle);
                self.dll_handle = ptr::null_mut();
            }
        }
    }
}

/// Get bundle paths from the OFXPLUGINS env variable
pub fn default_bundle_paths() -> Vec<PathBuf> {
    let mut paths: Vec<PathBuf> = Vec::new();
    match env::var_os("OFXPLUGINS") {
        Some(inline_paths) => {
            for path in env::split_paths(&inline_paths) {
                debug!("add plugin path {:?}", path);
                paths.push(path);
            }
        }

        None => {
            warn!("environment variable OFXPLUGINS not set, looking for plugins in /usr/local/OFX");
            paths.push(PathBuf::from("/usr/local/OFX"));
        }
    }
    paths
}

/// Returns true if the given dir follows the ofx bundle specification
fn is_ofx_bundle(dir: &io::Result<DirEntry>) -> bool {
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


// TODO: this should be used in a lot of places, so move to a common module
// This function causes returns dangling pointers
fn from_str(s: &str) -> CString {
    // TODO: What is the lifetime of the returned pointer ?
    CString::new(s).unwrap()
}
