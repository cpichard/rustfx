
extern crate libc;
use libc::*;
use suites::core::*;


/*#include "ofxMessage.h"
typedef struct OfxMessageSuiteV2 {
  OfxStatus (*message)(void *handle,
		       const char *messageType,
		       const char *messageId,
		       const char *format,
		       ...);
  OfxStatus (*setPersistentMessage)(void *handle,
                                    const char *messageType,
                                    const char *messageId,
                                    const char *format,
                                    ...);
  OfxStatus (*clearPersistentMessage)(void *handle);
} OfxMessageSuiteV2;
*/

// In C
extern { 
    fn c_message(handle: * mut c_void, message_type: * const c_char, message_id: * const c_char, format: * const c_char, ...) -> OfxStatus;
    fn c_set_persistent_message(handle: * mut c_void, message_type: * const c_char, message_id: * const c_char, format: * const c_char, ...) -> OfxStatus;
}

pub extern fn clear_persistent_message(handle: * mut c_void ) -> OfxStatus {kOfxStatOK}


#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxMessageSuiteV2 {
    message: unsafe extern fn (* mut c_void, * const c_char, * const c_char, * const c_char, ...) -> OfxStatus,
    setPersistentMessage: unsafe extern fn (* mut c_void, * const c_char, * const c_char, * const c_char, ...) -> OfxStatus,
    clearPersistentMessage: extern fn ( * mut c_void ) -> OfxStatus,
}


pub static OFX_MESSAGE_SUITE_V2
    : OfxMessageSuiteV2 
        = OfxMessageSuiteV2 {
            message: c_message,
            setPersistentMessage: c_set_persistent_message,
            clearPersistentMessage: clear_persistent_message,
        };

