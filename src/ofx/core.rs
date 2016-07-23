extern crate libc;
//use libc::*;
use ofx::property::*;
use ofx::imageeffect::*;
use std::mem;
use std::ffi::*;

pub type OfxStatus = i32;
pub const kOfxStatOK : OfxStatus = 0;
pub const kOfxStatFailed : OfxStatus = 1;
pub const kOfxStatErrFatal : OfxStatus =2;
pub const kOfxStatErrBadHandle : OfxStatus = 9;
pub const kOfxStatReplyDefault : OfxStatus = 14;

pub type OfxTime = f64;

//#[repr(C)]
//pub struct OfxRectI {
//    x1: i32,    
//    y1: i32,    
//    x2: i32,    
//    Y2: i32,    
//}

#[repr(C)]
pub struct OfxRectD {
    x1: f64,    
    y1: f64,    
    x2: f64,    
    y2: f64,    
}

/// FIXME: could we store the following string as c_char instead of str ? 
#[allow(non_upper_case_globals)]
pub const kOfxGetNumberOfPlugins : & 'static str = "OfxGetNumberOfPlugins";
#[allow(non_upper_case_globals)]
pub const kOfxGetPlugin : & 'static str = "OfxGetPlugin";

pub type FetchSuiteType = extern fn (OfxPropertySetHandle, * const libc::c_char, libc::c_int) -> * mut libc::c_void;

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxHost {
  pub host: OfxPropertySetHandle,
  pub fetchSuite: FetchSuiteType,
}

//impl Drop for OfxHost {
//    
//    fn drop( & mut self ) {
//        println!("Dropping host");    
//    }    
//}
//const kOfxPropertySuite : &'static str = "OfxPropertySuite";

// FIXME : allocate suites only once and return pointers to them
#[allow(unused_variables)]
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
        _ => {
            //panic!("suite {} not implemented", suite_str) 
            unsafe {mem::transmute(& OFX_IMAGE_EFFECT_SUITE_V1)}
        }
    }
}

impl OfxHost {
    pub fn new(properties: Box<OfxPropertySet>) -> Box<OfxHost> {
        //let properties = OfxPropertySet::new();
        let host_props = Box::into_raw(properties);
        trace!("properties for host set ptr is {:?}", host_props as * const _);
        Box::new(OfxHost { host: host_props as * mut libc::c_void, fetchSuite: fetch_suite })
        //Box::new(OfxHost { host: 0 as * mut libc::c_void, fetchSuite: fetch_suite })
    }
}

