extern crate libc;
use libc::{c_void, c_char, c_double};
use ofx::core::*;

// handle is 
extern fn progress_start(effect_instance:* mut c_void, label: * const c_char) -> OfxStatus {
    // TODO check there is no ongoing progress for this instance
    kOfxStatOK
}

extern fn progress_update(effect_instance:* mut c_void, progress: c_double) -> OfxStatus {
    kOfxStatOK
}  

extern fn progress_end(effect_instance: * mut c_void) -> OfxStatus {
    kOfxStatOK
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxProgressSuiteV1 {
  progressStart: extern fn (* mut c_void, * const c_char) -> OfxStatus,
  progressUpdate: extern fn (* mut c_void, c_double) -> OfxStatus,
  progressEnd: extern fn (* mut c_void) -> OfxStatus,
}


pub static OFX_PROGRESS_SUITE_V1
    : OfxProgressSuiteV1 
        = OfxProgressSuiteV1 {
            progressStart: progress_start,
            progressUpdate: progress_update,
            progressEnd: progress_end,
};
