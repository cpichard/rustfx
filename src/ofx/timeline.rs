
extern crate libc;
use libc::*;
use ofx::core::*;

pub struct OfxTimelineSuiteV1 {
    getTime: extern fn (* mut c_void, * mut c_double) -> OfxStatus,
    gotoTime: extern fn (* mut c_void, c_double) -> OfxStatus,
    getTimeBounds: extern fn (* mut c_void, * mut c_double, * mut c_double) -> OfxStatus,
}


//pub static OFX_TIMELINE_SUITE_V1 
//    : OfxTimelineSuiteV1 = 
//        OfxTimelineSuiteV1 {
//            getTime = get_time,
//            gotoTime = goto_time,
//            getTimeBounds = get_time_bounds,    
//        }
