// Parameter suite
extern crate libc;
use ofx::property::*;
use ofx::core::*;

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxParamSetStruct {
    
}

pub type OfxParamSetHandle = * mut libc::c_void; 
pub type OfxParamHandle = * mut libc::c_void;

// Types of OfxParameterSuite functions
//type ParamDefineType = extern fn (OfxParamSetHandle, * const libc::c_char, * const libc::c_char, * mut OfxPropertySetHandle) -> OfxStatus;
//type ParamGetHandleType = extern fn (OfxParamSetHandle, * const libc::c_char, * mut OfxParamHandle, * mut OfxPropertySetHandle)-> OfxStatus;
//type ParamSetGetPropertySet = extern fn (OfxParamSetHandle, * mut OfxPropertySetHandle)-> OfxStatus;
pub type ParamGetValueType = unsafe extern fn (OfxParamSetHandle, ...) -> OfxStatus;


#[allow(non_snake_case)]
pub struct OfxParameterSuiteV1 {
    //paramDefine : ParamDefineType,
    //paramGetHandle: ParamGetHandleType, 
    pub paramGetValue: ParamGetValueType,
}

#[link(name = "ofxhelpers")]
extern { 
    fn param_get_value (param_set: OfxParamSetHandle, ...) -> OfxStatus;
}

pub static OFX_PARAMETER_SUITE_V1 : OfxParameterSuiteV1 = OfxParameterSuiteV1 {
    paramGetValue : param_get_value,    
};

