
extern crate libc;
use libc::*;
use ofx::core::*;

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxTimelineSuiteV1 {
    getTime: extern fn (* mut c_void, * mut c_double) -> OfxStatus,
    gotoTime: extern fn (* mut c_void, c_double) -> OfxStatus,
    getTimeBounds: extern fn (* mut c_void, * mut c_double, * mut c_double) -> OfxStatus,
}

pub extern fn get_time(handle: * mut c_void, time: * mut c_double) -> OfxStatus {

    kOfxStatOK    
}

pub extern fn goto_time(handle: * mut c_void, time: c_double) -> OfxStatus {
    kOfxStatOK
}

pub extern fn get_time_bounds(handle: * mut c_void, t1: * mut c_double, t2: * mut c_double) -> OfxStatus {
    kOfxStatOK
}

pub static OFX_TIMELINE_SUITE_V1 
    : OfxTimelineSuiteV1 = 
        OfxTimelineSuiteV1 {
            getTime : get_time,
            gotoTime : goto_time,
            getTimeBounds : get_time_bounds,    
        };
