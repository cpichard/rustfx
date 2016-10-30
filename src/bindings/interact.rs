
extern crate libc;
use libc::*;
use bindings::core::*;
use bindings::property::*;

/*
typedef struct OfxInteractSuiteV1 {	
  OfxStatus (*interactSwapBuffers)(OfxInteractHandle interactInstance);
  OfxStatus (*interactRedraw)(OfxInteractHandle interactInstance);
  OfxStatus (*interactGetPropertySet)(OfxInteractHandle interactInstance,
				      OfxPropertySetHandle *property);
} OfxInteractSuiteV1;
*/
pub type OfxInteractHandle = * mut c_void;

pub extern fn interact_swap_buffers(instance: OfxInteractHandle) -> OfxStatus { kOfxStatOK }
pub extern fn interact_redraw(instance: OfxInteractHandle) -> OfxStatus { kOfxStatOK }
pub extern fn interact_get_propertyset(instance: OfxInteractHandle, props: * mut OfxPropertySetHandle) -> OfxStatus { kOfxStatOK }

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxInteractSuiteV1 {
    interactSwapBuffers: extern fn (OfxInteractHandle) -> OfxStatus,
    interactRedraw: extern fn (OfxInteractHandle) -> OfxStatus,    
    interactGetPropertySet: extern fn (OfxInteractHandle, * mut OfxPropertySetHandle) -> OfxStatus,
}

pub static OFX_INTERACT_SUITE_V1 
    : OfxInteractSuiteV1 =
        OfxInteractSuiteV1 {
            interactSwapBuffers: interact_swap_buffers,
            interactRedraw: interact_redraw,
            interactGetPropertySet: interact_get_propertyset,
        };
