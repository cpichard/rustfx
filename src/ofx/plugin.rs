extern crate libc;
///! Module plugin
use ofx::core::*;

pub type OfxPluginEntryPoint = 
     extern fn (*const libc::c_char, *const libc::c_void, *mut libc::c_void, *mut libc::c_void) 
        -> OfxStatus;
//
// OfxCore bindings
//
#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct OfxPlugin {
  // Defines the type of the plug-in, this will tell the host what the plug-in 
  // does. e.g.: an image effects plug-in would be a "OfxImageEffectPlugin"
  pub pluginApi: *const libc::c_char,

  //Defines the version of the pluginApi that this plug-in implements */
  pub apiVersion: libc::c_int,

  // String that uniquely labels the plug-in among all plug-ins that implement an API.
  //    It need not necessarily be human sensible, however the preference is to use reverse
  //    internet domain name of the developer, followed by a '.' then by a name that represents
  //    the plug-in.. It must be a legal ASCII string and have no whitespace in the
  //    name and no non printing chars.
  //    For example "uk.co.somesoftwarehouse.myPlugin"
  pub pluginIdentifier: *const libc::c_char,

  // Major version of this plug-in, this gets incremented when 
  // backwards compatibility is broken.
  pub pluginVersionMajor: libc::c_uint,

  // Minor version of this plug-in, this gets incremented when software 
  // is changed, but does not break backwards compatibility.
  pub pluginVersionMinor: libc::c_uint,

  // Function the host uses to connect the plug-in to the host's api fetcher
  pub setHost: extern fn (*mut libc::c_void) -> libc::c_void,

  // Main entry point for plug-ins
  pub mainEntry : OfxPluginEntryPoint,
}

