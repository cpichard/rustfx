
extern crate libc;

use ofx::property::*;

// import everything from libc ?
use libc::*;
use std::mem;

pub type OfxStatus = i32;
// OfxCore bindings

pub type FetchSuiteType = extern fn (* mut OfxPropertySet, * const libc::c_char, libc::c_int)-> * mut libc::c_void;

#[repr(C)]
pub struct OfxHost {
  pub host: * mut OfxPropertySet,
  pub fetchSuite: FetchSuiteType,
}

extern fn fetchSuiteFunc(host:* mut OfxPropertySet, 
                  suite_name: * const libc::c_char,
                  suite_version: libc::c_int) -> * mut libc::c_void {
    let mut prop_suite = OfxPropertySuiteV1::new();
    unsafe {
        let suite_ptr : * mut libc::c_void = mem::transmute(& mut prop_suite);
        suite_ptr
    }
}

impl OfxHost {
    
    pub fn new() -> OfxHost {
        let mut properties = OfxPropertySet::new(); 
        unsafe {
            let host_ptr : * mut OfxPropertySet = mem::transmute(& mut properties);
            OfxHost { host: host_ptr, fetchSuite: fetchSuiteFunc }
        }
    }
}
