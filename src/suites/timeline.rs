
extern crate libc;
use libc::*;
use suites::core::*;

// TODO: code timeline suite 

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxTimelineSuiteV1 {
    getTime: extern "C" fn(*mut c_void, *mut c_double) -> OfxStatus,
    gotoTime: extern "C" fn(*mut c_void, c_double) -> OfxStatus,
    getTimeBounds: extern "C" fn(*mut c_void, *mut c_double, *mut c_double) -> OfxStatus,
}

pub extern "C" fn get_time(handle: *mut c_void, time: *mut c_double) -> OfxStatus {

    kOfxStatOK
}

pub extern "C" fn goto_time(handle: *mut c_void, time: c_double) -> OfxStatus {
    kOfxStatOK
}

pub extern "C" fn get_time_bounds(handle: *mut c_void,
                                  t1: *mut c_double,
                                  t2: *mut c_double)
                                  -> OfxStatus {
    kOfxStatOK
}

pub static OFX_TIMELINE_SUITE_V1: OfxTimelineSuiteV1 = OfxTimelineSuiteV1 {
    getTime: get_time,
    gotoTime: goto_time,
    getTimeBounds: get_time_bounds,
};
