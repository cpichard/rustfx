extern crate libc;
use libc::*;
use ofx::property::*;
use rfx::propertyset::*;
use ofx::imageeffect::*;
use ofx::progress::*;
use ofx::param::*;
use ofx::memory::*;
use ofx::thread::*;
use ofx::interact::*;
use ofx::message::*;
use std::mem;
use std::ffi::*;
use std::ptr;

// We include all the static constants which were translated 
// from the openfx include files.
include!("constants.rs");

pub const kOfxGetNumberOfPlugins : & 'static str = "OfxGetNumberOfPlugins";
pub const kOfxGetPlugin : & 'static str = "OfxGetPlugin";

/// Utility to convert from a const char pointer received from the client 
/// to a u8 buffer
pub fn to_keyword<'a>(value: * const c_char) -> &'a[u8] {
    let from_client = unsafe {CStr::from_ptr(value)};
    from_client.to_bytes_with_nul()
}

pub fn clone_keyword<'a>(value: &'a[u8]) -> * const c_char {
    let mut v :Vec<u8> = Vec::with_capacity(value.len());
    unsafe {v.set_len(value.len());}
    v.clone_from_slice(value);
    unsafe {CString::from_vec_unchecked(v).as_ptr()}
}

///
pub fn from_keyword<'a>(value: &'a[u8]) -> * const c_char {
    unsafe{CStr::from_bytes_with_nul_unchecked(value).as_ptr()}
}

//pub to_pointer<Box<T>>(value: Box<T>) -> * mut c_void {
//    
//}

/// The time is represented as double
pub type OfxTime = f64;
pub type OfxStatus = i32;

#[repr(C)]
pub struct OfxRectI {
    x1: i32,    
    y1: i32,    
    x2: i32,    
    y2: i32,    
}

#[repr(C)]
pub struct OfxRectD {
    x1: f64,    
    y1: f64,    
    x2: f64,    
    y2: f64,    
}

#[repr(C)]
pub struct OfxRangeD {
    min: f64,    
    max: f64,
}

/// FIXME: could we store the following string as c_char instead of str ? 
//pub const kOfxGetNumberOfPlugins : & 'static str = "OfxGetNumberOfPlugins";
//pub const kOfxGetPlugin : & 'static str = "OfxGetPlugin";
//const kOfxPropertySuite : &'static str = "OfxPropertySuite";

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxHost {
    pub host: OfxPropertySetHandle,
    pub fetchSuite: extern fn (OfxPropertySetHandle, * const c_char, c_int) -> * mut c_void,
}

//impl Drop for OfxHost {
//    
//    fn drop( & mut self ) {
//        println!("Dropping host");    
//    }    
//}

/// returns the static suite or null if the suite wasn't found
#[allow(unused_variables)] // FIXME => handle the version of the returned suite
extern fn fetch_suite(host: OfxPropertySetHandle, 
                      suite_name: * const libc::c_char,
                      suite_version: libc::c_int) -> * mut libc::c_void {
    if suite_name.is_null() {
        panic!("the plugin asked for a null suite");
    }
    //let suite_cstr = unsafe {CStr::from_ptr(suite_name)};
    //let suite_str = suite_cstr.to_str().unwrap();
    //let suite_str = to_keyword(suite_name);
    match to_keyword(suite_name) {
        kOfxPropertySuite => {
            unsafe {mem::transmute(& OFX_PROPERTY_SUITE_V1)}
        }
        kOfxImageEffectSuite => {
            unsafe {mem::transmute(& OFX_IMAGE_EFFECT_SUITE_V1)}
        }
        kOfxParameterSuite => {
            unsafe {mem::transmute(& OFX_PARAMETER_SUITE_V1)}
        }
        kOfxProgressSuite => {
            unsafe {mem::transmute(& OFX_PROGRESS_SUITE_V1)}
        }
        kOfxMemorySuite => {
            unsafe {mem::transmute(& OFX_MEMORY_SUITE_V1)}
        }
        kOfxMultiThreadSuite => {
            unsafe {mem::transmute(& OFX_MULTITHREAD_SUITE_V1)}
        }
        kOfxInteractSuite => {
            unsafe {mem::transmute(& OFX_INTERACT_SUITE_V1)}
        } 
        kOfxMessageSuite => {
            unsafe {mem::transmute(& OFX_MESSAGE_SUITE_V2)}
        }
        _ => {
            let suite_cstr = unsafe {CStr::from_ptr(suite_name)};
            let suite_str = suite_cstr.to_str().unwrap();
            error!("plugin is asking for an unimplemented suite : {}", suite_str);
            ptr::null_mut() as * mut c_void
        }
    }
}

impl OfxHost {
    pub fn new(properties: Box<OfxPropertySet>) -> Box<OfxHost> {
        let host_props = Box::into_raw(properties);
        trace!("properties for host set ptr is {:?}", host_props as * const _);
        Box::new(OfxHost { host: host_props as * mut libc::c_void, fetchSuite: fetch_suite })
        //Box::new(OfxHost { host: 0 as * mut libc::c_void, fetchSuite: fetch_suite })
    }
}

#[cfg(test)]
#[link(name = "ofxhelpers")] 
extern {
    fn c_test_host(host: * mut c_void) -> c_int;    
}

#[test]
fn test_ofx_host() {
    let props = OfxPropertySet::new();
    let host = OfxHost::new(props);
    let g = unsafe {c_test_host(mem::transmute(Box::into_raw(host)))};
    assert!(g == 0);    
}

