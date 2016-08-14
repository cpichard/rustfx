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

// TODO: finish to write all status
pub type OfxStatus = i32;
pub const kOfxStatOK : OfxStatus = 0;
pub const kOfxStatFailed : OfxStatus = 1;
pub const kOfxStatErrFatal : OfxStatus = 2;
pub const kOfxStatErrMemory : OfxStatus = 8;
pub const kOfxStatErrBadHandle : OfxStatus = 9;
pub const kOfxStatErrBadIndex : OfxStatus = 10;
pub const kOfxStatReplyDefault : OfxStatus = 14;

/// The time is represented as double
pub type OfxTime = f64;

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
pub const kOfxGetNumberOfPlugins : & 'static str = "OfxGetNumberOfPlugins";
pub const kOfxGetPlugin : & 'static str = "OfxGetPlugin";
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
    let suite_cstr = unsafe {CStr::from_ptr(suite_name)};
    let suite_str = suite_cstr.to_str().unwrap();
    match suite_str {
        "OfxPropertySuite" => {
            unsafe {mem::transmute(& OFX_PROPERTY_SUITE_V1)}
        }
        "OfxImageEffectSuite" => {
            unsafe {mem::transmute(& OFX_IMAGE_EFFECT_SUITE_V1)}
        }
        "OfxParameterSuite" => {
            unsafe {mem::transmute(& OFX_PARAMETER_SUITE_V1)}
        }
        "OfxProgressSuite" => {
            unsafe {mem::transmute(& OFX_PROGRESS_SUITE_V1)}
        }
        "OfxMemorySuite" => {
            unsafe {mem::transmute(& OFX_MEMORY_SUITE_V1)}
        }
        "OfxMultiThreadSuite" => {
            unsafe {mem::transmute(& OFX_MULTITHREAD_SUITE_V1)}
        }
        "OfxInteractSuite" => {
            unsafe {mem::transmute(& OFX_INTERACT_SUITE_V1)}
        } 
        "OfxMessageSuite" => {
            unsafe {mem::transmute(& OFX_MESSAGE_SUITE_V2)}
        }
        _ => {
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

