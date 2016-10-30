extern crate libc;
use std::ffi::*;
use libc::*;
use bindings::core::*;

//
// OfxCore bindings
//
#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct OfxPlugin {
  // Defines the type of the plug-in, this will tell the host what the plug-in 
  // does. e.g.: an image effects plug-in would be a "OfxImageEffectPlugin"
  pub pluginApi: *const c_char,

  //Defines the version of the pluginApi that this plug-in implements */
  pub apiVersion: c_int,

  // String that uniquely labels the plug-in among all plug-ins that implement an API.
  //    It need not necessarily be human sensible, however the preference is to use reverse
  //    internet domain name of the developer, followed by a '.' then by a name that represents
  //    the plug-in.. It must be a legal ASCII string and have no whitespace in the
  //    name and no non printing chars.
  //    For example "uk.co.somesoftwarehouse.myPlugin"
  pub pluginIdentifier: *const c_char,

  // Major version of this plug-in, this gets incremented when 
  // backwards compatibility is broken.
  pub pluginVersionMajor: c_uint,

  // Minor version of this plug-in, this gets incremented when software 
  // is changed, but does not break backwards compatibility.
  pub pluginVersionMinor: c_uint,

  // Function the host uses to connect the plug-in to the host's api fetcher
  pub setHost: extern fn (*mut c_void) -> c_void,

  // Main entry point for plug-ins
  pub mainEntry: extern fn (*const c_char, *const c_void, *mut c_void, *mut c_void) -> OfxStatus,
}

/// accessor functions
impl OfxPlugin {
    pub fn identifier(&self) -> String {
        let ptr_wrap = unsafe {CStr::from_ptr(self.pluginIdentifier)};
        ptr_wrap.to_str().unwrap().to_string()
    }    
}


