// Parameter suite
extern crate libc;
use libc::*;
use std::ffi::CStr;
use rfx::paramset::*;
use suites::property::*;
use suites::core::*;
use std::mem::*;

/// Rust <-> C suites for parameters

pub type OfxParamSetHandle = *mut c_void;
pub type OfxParamHandle = *mut c_void;

// Types of OfxParameterSuite functions
// type ParamDefineType = extern fn (OfxParamSetHandle, * const c_char, * const c_char, * mut OfxPropertySetHandle) -> OfxStatus;
type ParamGetHandleType = extern "C" fn(OfxParamSetHandle,
                                        *const c_char,
                                        *mut OfxParamHandle,
                                        *mut OfxPropertySetHandle)
                                        -> OfxStatus;
type ParamSetGetPropertySetType = extern "C" fn(OfxParamHandle, *mut OfxPropertySetHandle)
                                                -> OfxStatus;
pub type ParamGetValueType = unsafe extern "C" fn(OfxParamHandle, ...) -> OfxStatus;
pub type ParamGetValueAtTimeType = unsafe extern "C" fn(OfxParamHandle, OfxTime, ...) -> OfxStatus;
pub type ParamGetDerivativeType = unsafe extern "C" fn(OfxParamHandle, OfxTime, ...) -> OfxStatus;
pub type ParamGetIntegralType = unsafe extern "C" fn(OfxParamHandle, OfxTime, OfxTime, ...)
                                                     -> OfxStatus;
pub type ParamSetValueType = unsafe extern "C" fn(OfxParamHandle, ...) -> OfxStatus;
pub type ParamSetValueAtTimeType = unsafe extern "C" fn(OfxParamHandle, OfxTime, ...) -> OfxStatus;
pub type ParamGetNumKeysType = extern "C" fn(OfxParamHandle, *mut libc::c_int) -> OfxStatus;
pub type ParamGetKeyTimeType = extern "C" fn(OfxParamHandle, libc::c_uint, *mut OfxTime) -> OfxStatus;
pub type ParamGetKeyIndexType = extern "C" fn(OfxParamHandle,
                                              OfxTime,
                                              libc::c_int,
                                              *mut libc::c_int)
                                              -> OfxStatus;
pub type ParamDeleteKeyType = extern "C" fn(OfxParamHandle, OfxTime) -> OfxStatus;
pub type ParamDeleteAllKeysType = extern "C" fn(OfxParamHandle) -> OfxStatus;
pub type ParamCopyType = extern "C" fn(OfxParamHandle, OfxParamHandle, OfxTime, *const OfxRangeD)
                                       -> OfxStatus;
pub type ParamEditBeginType = extern "C" fn(OfxParamSetHandle, *const libc::c_char) -> OfxStatus;
pub type ParamEditEndType = extern "C" fn(OfxParamSetHandle) -> OfxStatus;


/// Parameter definition, caller in describe in context
/// Arguments
/// pset_ptr:  handle to the parameter set descriptor that will hold this parameter
/// p_type: type of the parameter to create, one of the kOfxParamType #defines
/// p_name: unique name of the parameter
/// props: if not null, a pointer to the parameter descriptor's property set will be placed here.
///
/// TODO: handle case where the type is unknown or unsupported
/// kOfxStatErrUnknown - if the type is unknown
/// kOfxStatErrUnsupported - if the type is known but unsupported
#[no_mangle]
pub extern "C" fn param_define(pset_ptr: OfxParamSetHandle,
                           p_type: *const c_char,
                           p_name: *const c_char,
                           props: *mut OfxPropertySetHandle)
                           -> OfxStatus {
    // Test pointers nullness
    if pset_ptr.is_null() || p_type.is_null() || p_name.is_null() {
        error!("null pointer passed to param define");
        return kOfxStatErrBadHandle;
    }

    // Create the parameter and returns an error if there was already one
    // NOTE that if the type is unknown, the code will return None as if
    // a parameter was created.
    let mut paramset: &mut OfxParameterSet = unsafe { transmute(pset_ptr) };
    let p_type_str = unsafe { CStr::from_ptr(p_type) }.to_str().unwrap();
    let p_name_str = unsafe { CStr::from_ptr(p_name) }.to_owned();
    if let Some(previous_param) = paramset.create_param(p_type_str, p_name_str) {
        return kOfxStatErrExists;
    }

    // Get the newly created parameter
    let p_name_str = unsafe { CStr::from_ptr(p_name) }.to_owned();
    let found_param = unsafe { paramset.get_param(transmute(&p_name_str)) };
    if let Some(param) = found_param {
        if !props.is_null() {
            unsafe {
                *props = param.properties();
                trace!("param_define props is {:?}", *props);
            }
        }
        kOfxStatOK
    } else {
        kOfxStatErrUnsupported // Type unknown unsupported || no memory left
    }
}

#[no_mangle]
pub extern "C" fn param_get_nb_component(handle: *mut c_void) -> u32 {
    let p_obj = handle as *mut OfxParam;
    // TODO move to paramset
    unsafe {
        match *p_obj {
            OfxParam::Int1(_) => 1,
            OfxParam::Int2(_) => 2,
            OfxParam::Int3(_) => 3,
            OfxParam::Double1(_) => 1,
            OfxParam::Double2(_) => 2,
            OfxParam::Double3(_) => 3,
            OfxParam::RGB(_) => 3,
            OfxParam::RGBA(_) => 4,
            OfxParam::String(_, _) => 1,
            OfxParam::Boolean(_, _) => 1,
            OfxParam::Choice(_) => 1,
            OfxParam::Custom(_) => 1,
            OfxParam::PushButton(_) => 1,
            OfxParam::Group(_) => 1,
            OfxParam::Page(_) => 1,
        }
    }
}


/// This is the actual function which change the values
/// its called from the C code
///
#[no_mangle]
pub extern "C" fn param_set_components(handle: *mut c_void, data: *mut c_void) {
    let param = handle as *mut OfxParam;
    if param.is_null() {
        error!("unable to set parameter value, got null pointer");
    } else {
        unsafe { (*param).set_raw_data(data) };
    }
}

#[no_mangle]
pub extern "C" fn param_get_components(handle: *mut c_void, data: *mut c_void) {
    let param = handle as *mut OfxParam;
    if param.is_null() {
        error!("unable to get parameter value, got null pointer");
    } else {
        unsafe { (*param).get_raw_data(data) };
    }
}

/// This function is used in the C code to differentiate between
/// integer, float, and string
#[no_mangle]
pub extern "C" fn param_get_type(handle: *mut c_void) -> u32 {
    let p_obj = handle as *mut OfxParam;
    unsafe {
        match *p_obj {
            OfxParam::Int1(_) => 0,
            OfxParam::Int2(_) => 0,
            OfxParam::Int3(_) => 0,
            OfxParam::Double1(_) => 1,
            OfxParam::Double2(_) => 1,
            OfxParam::Double3(_) => 1,
            OfxParam::RGB(_) => 1,
            OfxParam::RGBA(_) => 1,
            OfxParam::String(_, _) => 2,
            OfxParam::Boolean(_, _) => 0,
            OfxParam::Choice(_) => 0,
            OfxParam::Custom(_) => 0,
            OfxParam::PushButton(_) => 0,
            OfxParam::Group(_) => 0,
            OfxParam::Page(_) => 0,
        }
    }
}

// TODO: return a pointer on a parameter
// This is not in the rust spirit
extern "C" fn param_get_handle(pset_ptr: OfxParamSetHandle,
                               name: *const c_char,
                               handle: *mut OfxParamHandle,
                               props: *mut OfxPropertySetHandle)
                               -> OfxStatus {
    if pset_ptr.is_null() {
        error!("param_get_handle parameter set handle is null");
        return kOfxStatErrBadHandle;
    }
    trace!("paramGetHandle {:?} in {:?}", name, pset_ptr);
    unsafe {
        let mut paramset: &mut OfxParameterSet = transmute(pset_ptr);
        let param_and_prop = paramset.param_and_prop_ptr(name);
        *handle = param_and_prop.0;
        if !props.is_null() {
            // TODO : set properties
            // param : & OfxParam = transmute(handle);
            warn!("setting prop {:?}", param_and_prop.1);
            *props = param_and_prop.1;
        }
    }
    kOfxStatOK
}

extern "C" fn paramset_get_property_set(handle: OfxParamHandle,
                                        pset: *mut OfxPropertySetHandle)
                                        -> OfxStatus {
    kOfxStatOK
}

extern "C" fn param_get_num_keys(handle: OfxParamHandle, nb_keys: *mut libc::c_int) -> OfxStatus {
    unsafe {
        *nb_keys = 0;
    }
    kOfxStatOK
}

extern "C" fn param_get_key_time(handle: OfxParamHandle,
                                 n_keytime: libc::c_uint,
                                 time: *mut OfxTime)
                                 -> OfxStatus {
    kOfxStatOK
}

extern "C" fn param_get_key_index(handle: OfxParamHandle,
                                  time: OfxTime,
                                  direction: libc::c_int,
                                  index: *mut libc::c_int)
                                  -> OfxStatus {
    kOfxStatOK
}

extern "C" fn param_delete_key(handle: OfxParamHandle, time: OfxTime) -> OfxStatus {
    kOfxStatOK
}

extern "C" fn param_delete_all_keys(handle: OfxParamHandle) -> OfxStatus {
    kOfxStatOK
}

extern "C" fn param_copy(param_to: OfxParamHandle,
                         param_from: OfxParamHandle,
                         dst_offset: OfxTime,
                         frame_range: *const OfxRangeD)
                         -> OfxStatus {
    kOfxStatOK
}

extern "C" fn param_edit_begin(handle: OfxParamSetHandle, name: *const libc::c_char) -> OfxStatus {
    kOfxStatOK
}

extern "C" fn param_edit_end(handle: OfxParamSetHandle) -> OfxStatus {
    kOfxStatOK
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxParameterSuiteV1 {
    paramDefine: extern "C" fn(OfxParamSetHandle,
                               *const c_char,
                               *const c_char,
                               *mut OfxPropertySetHandle)
                               -> OfxStatus,
    paramGetHandle: ParamGetHandleType,
    paramSetGetPropertySet: ParamSetGetPropertySetType,
    paramGetValue: ParamGetValueType,
    paramGetValueAtTime: ParamGetValueAtTimeType,
    paramGetDerivative: ParamGetDerivativeType,
    paramGetIntegral: ParamGetIntegralType,
    paramSetValue: ParamSetValueType,
    paramSetValueAtTime: ParamSetValueAtTimeType,
    paramGetNumKeys: ParamGetNumKeysType,
    paramGetKeyTime: ParamGetKeyTimeType,
    paramGetKeyIndex: ParamGetKeyIndexType,
    paramDeleteKey: ParamDeleteKeyType,
    paramDeleteAllKeys: ParamDeleteAllKeysType,
    paramCopy: ParamCopyType,
    paramEditBegin: ParamEditBeginType,
    paramEditEnd: ParamEditEndType,
}


/// We had to write C suites cause rust doesn't handle varargs.
extern "C" {
    fn param_get_value(param_set: OfxParamHandle, ...) -> OfxStatus;
    fn param_get_value_at_time(param_set: OfxParamHandle, time: OfxTime, ...) -> OfxStatus;
    fn param_get_derivative(param_set: OfxParamHandle, time: OfxTime, ...) -> OfxStatus;
    fn param_get_integral(param_set: OfxParamHandle,
                          time1: OfxTime,
                          time2: OfxTime,
                          ...)
                          -> OfxStatus;
    fn param_set_value(param_set: OfxParamHandle, ...) -> OfxStatus;
    fn param_set_value_at_time(param_set: OfxParamHandle, time: OfxTime, ...) -> OfxStatus;
}

pub static OFX_PARAMETER_SUITE_V1: OfxParameterSuiteV1 = OfxParameterSuiteV1 {
    paramDefine: param_define,
    paramGetHandle: param_get_handle,
    paramSetGetPropertySet: paramset_get_property_set,
    paramGetValue: param_get_value,
    paramGetValueAtTime: param_get_value_at_time,
    paramGetDerivative: param_get_derivative,
    paramGetIntegral: param_get_integral,
    paramSetValue: param_set_value,
    paramSetValueAtTime: param_set_value_at_time,
    paramGetNumKeys: param_get_num_keys,
    paramGetKeyTime: param_get_key_time,
    paramGetKeyIndex: param_get_key_index,
    paramDeleteKey: param_delete_key,
    paramDeleteAllKeys: param_delete_all_keys,
    paramCopy: param_copy,
    paramEditBegin: param_edit_begin,
    paramEditEnd: param_edit_end,
};

#[cfg(test)]
use std::ffi::CString;
#[cfg(test)]
use std::ptr;
#[cfg(test)]
fn init_parameter_test() -> (*mut OfxParameterSet, CString, CString) {
    let p_set = Box::into_raw(OfxParameterSet::new());
    let p_type = CString::new("OfxParamTypeInteger").unwrap();
    let p_name = CString::new("TestIntParam").unwrap();
    (p_set, p_type, p_name)
}

#[cfg(test)]
unsafe fn param_initialization_test(param: *mut OfxParam) {
    match *param {
        OfxParam::Int1(ref k) => assert!(k.default == KeyFramedParameter::default().default),
        OfxParam::Int2(ref k) => assert!(k.default == KeyFramedParameter::default().default),
        _ => assert!(false),            
    }
}

#[test]
fn create_parameter_set() {
    let (p_set, p_type, p_name) = init_parameter_test();
    // call image effect to create a paramset.
    let ret = unsafe {
        (OFX_PARAMETER_SUITE_V1.paramDefine)(transmute(p_set),
                                             p_type.as_ptr(),
                                             p_name.as_ptr(),
                                             ptr::null_mut())
    };
    assert!(ret == kOfxStatOK);
    let mut p_handle: OfxParamHandle = ptr::null_mut();
    let ret2 = unsafe {
        (OFX_PARAMETER_SUITE_V1.paramGetHandle)(transmute(p_set),
                                                p_name.as_ptr(),
                                                &mut p_handle,
                                                ptr::null_mut())
    };
    assert!(p_handle != ptr::null_mut());
    assert!(ret2 == kOfxStatOK);
}

#[test]
fn create_and_set_parameter() {
    let (p_set, p_type, p_name) = init_parameter_test();
    unsafe {
        let ret = (OFX_PARAMETER_SUITE_V1.paramDefine)(transmute(p_set),
                                                       p_type.as_ptr(),
                                                       p_name.as_ptr(),
                                                       ptr::null_mut());
        let mut p_handle: OfxParamHandle = ptr::null_mut();
        let ret2 = (OFX_PARAMETER_SUITE_V1.paramGetHandle)(transmute(p_set),
                                                           p_name.as_ptr(),
                                                           &mut p_handle,
                                                           ptr::null_mut());
        let p_obj = p_handle as *mut OfxParam;
        param_initialization_test(p_obj);
        let ret3 = (OFX_PARAMETER_SUITE_V1.paramSetValue)(p_handle, 257 as i32);
        match *p_obj {
            OfxParam::Int1(ref p) => assert!(p.default == 257),
            _ => assert!(false),            
        }
    }
}

#[test]
fn create_and_set_parameter_2d() {
    let p_set = Box::into_raw(OfxParameterSet::new());
    let p_type = CString::new("OfxParamTypeInteger2D").unwrap();
    let p_name = CString::new("TestInt2DParam").unwrap();
    unsafe {
        let ret = (OFX_PARAMETER_SUITE_V1.paramDefine)(transmute(p_set),
                                                       p_type.as_ptr(),
                                                       p_name.as_ptr(),
                                                       ptr::null_mut());
        let mut p_handle: OfxParamHandle = ptr::null_mut();
        let ret2 = (OFX_PARAMETER_SUITE_V1.paramGetHandle)(transmute(p_set),
                                                           p_name.as_ptr(),
                                                           &mut p_handle,
                                                           ptr::null_mut());
        // Check init value is zero
        let p_obj = p_handle as *mut OfxParam;
        param_initialization_test(p_obj);
        // Set value
        let ret3 = (OFX_PARAMETER_SUITE_V1.paramSetValue)(p_handle, 257 as i32, 67 as i32);
        // Check new value
        match *p_obj {
            OfxParam::Int2(ref param) => assert!(param.default.0 == 257 && param.default.1 == 67),
            _ => assert!(false),            
        }

        let a: i32 = 0;
        let b: i32 = 0;

        let ret4 = (OFX_PARAMETER_SUITE_V1.paramGetValue)(p_handle, &a, &b);
        assert!(a == 257);
        assert!(b == 67)
    }
}
