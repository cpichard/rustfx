extern crate libc;
use libc::*;
use ofx::core::*;

// Handle is the effect instance
extern "C" fn memory_alloc(handle: *mut libc::c_void,
                           nbytes: libc::size_t,
                           allocated: *mut *mut libc::c_void)
                           -> OfxStatus {

    if handle.is_null() {
        return kOfxStatErrMemory;
    }

    unsafe {
        *allocated = malloc(nbytes);
        if (*allocated).is_null() {
            return kOfxStatErrMemory;
        } else {
            return kOfxStatOK;
        }
    }
}

extern "C" fn memory_free(allocated: *mut libc::c_void) -> OfxStatus {
    if allocated.is_null() {
        return kOfxStatErrBadHandle;
    } else {
        unsafe {
            free(allocated);
        }
    }
    kOfxStatOK
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxMemorySuiteV1 {
    pub memoryAlloc: extern "C" fn(*mut libc::c_void, libc::size_t, *mut *mut libc::c_void)
                                   -> OfxStatus,
    pub memoryFree: extern "C" fn(*mut libc::c_void) -> OfxStatus,
}

pub static OFX_MEMORY_SUITE_V1: OfxMemorySuiteV1 = OfxMemorySuiteV1 {
    memoryAlloc: memory_alloc,
    memoryFree: memory_free,
};
