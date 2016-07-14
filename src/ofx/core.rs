extern crate libc;
use libc::*;
use ofx::property::*;
use ofx::imageeffect::*;
use std::mem;
use std::ffi::*;
pub type OfxStatus = i32;

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
    Y2: f64,    
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

//const kOfxPropertySuite : &'static str = "OfxPropertySuite";

// FIXME : allocate suites only once and return pointers to them
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
            let mut suite = OfxPropertySuiteV1::new();
            unsafe {mem::transmute(& mut suite)}
        }
        "OfxImageEffectSuite" => {
            let mut suite = OfxImageEffectSuiteV1::new();
            unsafe {mem::transmute(& mut suite)}
        }
        _ => {
            panic!("suite {} not implemented", suite_str) 
        }
    }
}
// TODO: this should be used in a lot of places, so move to a common module
fn from_str(s: & str) -> * const c_char {
    // TODO: What is the lifetime of the returned pointer ? 
    CString::new(s).unwrap().as_ptr()
}


impl OfxHost {
    pub fn new() -> OfxHost {
        let mut properties = OfxPropertySet::new(); 
        OfxHost::describeCapabilities(& mut properties);
        unsafe {
            let host_ptr : * mut OfxPropertySet = mem::transmute(& mut properties);
            OfxHost { host: host_ptr, fetchSuite: fetch_suite }
        }
    }

    // TODO describe
    fn describeCapabilities(props : & mut OfxPropertySet) {
        //props.insert(CString::new("TOTO").unwrap(), PropertyValue::from(10));
        //set_property(props, from_str("Toto"), 0, 10); 
        //set_property(props, from_str("Toto1"), 0, from_str("test")); 
        //props.set("", "");
        props.insert("TOTO", 10);
    }
}
