// Parameter suite
extern crate libc;
use libc::*;
use ofx::property::*;
use ofx::core::*;
use std::ffi::*;
use std::mem::*;
use std::ptr;
use std::collections::HashMap;
use std::vec::Vec;
use rfx::propertyset::*;
#[repr(C)]
#[allow(non_snake_case)]

/*
integers, 1, 2 and 3 dimensional
doubles, 1, 2 and 3 dimensional
colour, RGB and RGB + Alpha
booleans
choice
string
custom
push button
group
page
parametric
*/
#[derive(Default)]
pub struct KeyFramedParameter<T: Default> {
    properties: Box<OfxPropertySet>,
    default: T,
    keys: Vec<(OfxTime, T)>, 
}

/// Stores a parameter, 
/// FIXME: rust data type could be named rfxParameter to differentiate between ofx and rfx
/// Find other field to add, like time, associated properties and so on
/// ADD PropertySet
/// ADD KeyFramedValues
pub enum OfxParam {
	Int1(KeyFramedParameter<i32>),
	Int2(KeyFramedParameter<(i32, i32)>),
	Int3(i32, i32, i32),
	Double1(f64),
	Double2(f64, f64),
	Double3(f64, f64, f64),
	RGB(f64, f64, f64),
	RGBA(f64, f64, f64, f64),
    String(CString),
	Boolean(bool),
	Choice(i32), // FIXME: create the correct data needed
    Custom(i32),
	PushButton(i32),// FIXME: create the correct data needed
	Group(i32),// FIXME: create the correct data needed
	Page(i32),// FIXME: create the correct data needed
	//Parametric(i32),// FIXME: create the correct data needed
}

impl OfxParam {
    pub fn properties(& self) -> * mut c_void {
        match *self {
            OfxParam::Int1(ref val) => unsafe{transmute(&val.properties)},
            OfxParam::Int2(ref val) => unsafe{transmute(&val.properties)},
            _ => ptr::null_mut(),
        }
    }    
}

// Holds parameters. There is one OfxParameterSet per OfxImageEffect
pub struct OfxParameterSet {
	data: HashMap<CString, OfxParam>,
}

impl OfxParameterSet {
	pub fn new() -> Box<Self> {
		let pset = OfxParameterSet {
			data: HashMap::new(),	
		};
		Box::new(pset)
	}

    // FIXME: define would better be in the ofx function instead, to keep OfxParameterSet less
    // dependent in ofx
	pub fn define(& mut self, p_type: *const c_char, p_name: * const c_char) {
        let p_type_str = unsafe { CStr::from_ptr(p_type) }.to_str().unwrap();
        let p_name_str = unsafe { CStr::from_ptr(p_name) }.to_owned();
        trace!("defining parameter {:?} {:?}", p_name_str, p_type_str);
        match p_type_str {
            "OfxParamTypeInteger" => self.data.insert(p_name_str, OfxParam::Int1(KeyFramedParameter::default())), 
            "OfxParamTypeDouble" => self.data.insert(p_name_str, OfxParam::Double1(0.0)), 
            "OfxParamTypeBoolean" => self.data.insert(p_name_str, OfxParam::Boolean(true)), 
            "OfxParamTypeChoice" => self.data.insert(p_name_str, OfxParam::Choice(0)), 
            "OfxParamTypeRGBA" => self.data.insert(p_name_str, OfxParam::RGBA(0.0, 0.0, 0.0, 0.0)), 
            "OfxParamTypeRGB" => self.data.insert(p_name_str, OfxParam::RGB(0.0, 0.0, 0.0)), 
            "OfxParamTypeDouble2D" => self.data.insert(p_name_str, OfxParam::Double2(0.0, 0.0)), 
            "OfxParamTypeInteger2D" => self.data.insert(p_name_str, OfxParam::Int2(KeyFramedParameter::default())), 
            "OfxParamTypeDouble3D" => self.data.insert(p_name_str, OfxParam::Double3(0.0, 0.0, 0.0)), 
            "OfxParamTypeInteger3D" => self.data.insert(p_name_str, OfxParam::Int3(0, 0, 0)), 
            "OfxParamTypeString" => self.data.insert(p_name_str, OfxParam::String(CString::new("").unwrap())),
            "OfxParamTypeCustom" => self.data.insert(p_name_str, OfxParam::Custom(0)),
            "OfxParamTypeGroup" => self.data.insert(p_name_str, OfxParam::Group(0)),
            "OfxParamTypePage" => self.data.insert(p_name_str, OfxParam::Page(0)),
            "OfxParamTypePushButton" => self.data.insert(p_name_str, OfxParam::PushButton(0)),
            _ => panic!("unknown parameter type"),
        };
        
	}
   
    pub fn get_handle_and_prop(& mut self, p_name: * const c_char) -> (* mut c_void, * mut c_void) {
        let p_name_str = unsafe { CStr::from_ptr(p_name) }.to_owned();
        unsafe {
            match self.data.get(&p_name_str) {
                Some(param) => return (transmute(param), param.properties()) ,
                None => return (ptr::null_mut(), ptr::null_mut()), 
            }
        }
    }
}

pub type OfxParamSetHandle = * mut c_void; 
pub type OfxParamHandle = * mut c_void;

// Types of OfxParameterSuite functions
//type ParamDefineType = extern fn (OfxParamSetHandle, * const c_char, * const c_char, * mut OfxPropertySetHandle) -> OfxStatus;
type ParamGetHandleType = extern fn (OfxParamSetHandle, * const c_char, * mut OfxParamHandle, * mut OfxPropertySetHandle)-> OfxStatus;
type ParamSetGetPropertySetType = extern fn (OfxParamHandle, * mut OfxPropertySetHandle)-> OfxStatus;
pub type ParamGetValueType = unsafe extern fn (OfxParamHandle, ...) -> OfxStatus;
pub type ParamGetValueAtTimeType = unsafe extern fn (OfxParamHandle, OfxTime, ...) -> OfxStatus;
pub type ParamGetDerivativeType =  unsafe extern fn (OfxParamHandle, OfxTime, ...) -> OfxStatus;
pub type ParamGetIntegralType = unsafe extern fn (OfxParamHandle, OfxTime, OfxTime, ...) -> OfxStatus;
pub type ParamSetValueType = unsafe extern fn (OfxParamHandle, ...) -> OfxStatus;
pub type ParamSetValueAtTimeType = unsafe extern fn (OfxParamHandle, OfxTime, ...) -> OfxStatus;
pub type ParamGetNumKeysType = extern fn (OfxParamHandle, * mut libc::c_int) -> OfxStatus;
pub type ParamGetKeyTimeType = extern fn (OfxParamHandle, libc::c_uint, * mut OfxTime) -> OfxStatus;
pub type ParamGetKeyIndexType = extern fn (OfxParamHandle, OfxTime, libc::c_int, * mut libc::c_int) -> OfxStatus;
pub type ParamDeleteKeyType = extern fn (OfxParamHandle, OfxTime) -> OfxStatus;
pub type ParamDeleteAllKeysType = extern fn (OfxParamHandle) -> OfxStatus;
pub type ParamCopyType = extern fn (OfxParamHandle, OfxParamHandle, OfxTime, * const OfxRangeD) -> OfxStatus;
pub type ParamEditBeginType = extern fn (OfxParamSetHandle, * const libc::c_char) -> OfxStatus;
pub type ParamEditEndType = extern fn (OfxParamSetHandle) -> OfxStatus;


/*
   Parameter definition, caller in decribe in context
   Arguments
   pset_ptr:  handle to the parameter set descriptor that will hold this parameter
   p_type: type of the parameter to create, one of the kOfxParamType #defines
   P_name: unique name of the parameter
   props: if not null, a pointer to the parameter descriptor's property set will be placed here.
 */
extern fn param_define(pset_ptr: OfxParamSetHandle, p_type: *const c_char, p_name: * const c_char, props: * mut OfxPropertySetHandle) -> OfxStatus {
    let mut param_set : & mut OfxParameterSet = unsafe {transmute(pset_ptr)};
    trace!("================>param define");
    param_set.define(p_type, p_name);
    kOfxStatOK
}

#[no_mangle]
pub extern fn param_get_nb_component(handle: * mut c_void) -> u32 {
    let p_obj = handle as * mut OfxParam;
    unsafe {
        match *p_obj {
            OfxParam::Int1(_) => 1,
            OfxParam::Int2(_) => 2,
            OfxParam::Int3(_, _, _) => 3,
            OfxParam::Double1(_) => 1,
            OfxParam::Double2(_, _) => 2,
            OfxParam::Double3(_, _, _) => 3,
            OfxParam::RGB(_, _, _) => 3,
            OfxParam::RGBA(_, _, _, _) => 4,
            OfxParam::String(_) => 1,
            OfxParam::Boolean(bool) => 1,
            OfxParam::Choice(_) => 1,
            OfxParam::Custom(_) => 1,
            OfxParam::PushButton(_) => 1,
            OfxParam::Group(_) => 1,
            OfxParam::Page(_) => 1,
        }
    }
}

#[no_mangle]
pub extern fn param_set_components(handle: * mut c_void, data: * mut c_void) {
    let p_obj = handle as * mut OfxParam;
    unsafe {
        match *p_obj {
            OfxParam::Int1(ref mut param) => {
                let value : * mut i32 = data as * mut i32; 
                param.default = *value;
            }
            OfxParam::Int2(ref mut param) => {
                let value : * mut i32 = data as * mut i32; 
                param.default = (*value, *value.offset(1));
                //*p_obj = OfxParam::Int2(*value, *value.offset(1));
            }
            //OfxParam::Int3(_, _, _) => 3,
            //OfxParam::Double1(_) => 1,
            //OfxParam::Double2(_, _) => 2,
            //OfxParam::Double3(_, _, _) => 3,
            //OfxParam::RGB(_, _, _) => 3,
            //OfxParam::RGBA(_, _, _, _) => 4,
            //OfxParam::String(_) => 1,
            //OfxParam::Boolean(bool) => 1,
            //OfxParam::Choice(_) => 1,
            //OfxParam::Custom(_) => 1,
            //OfxParam::PushButton(_) => 1,
            //OfxParam::Group(_) => 1,
            //OfxParam::Page(_) => 1,
            _ => panic!("not implemented yet"),
        }
    }
    
}

#[no_mangle]
pub extern fn param_get_components(handle: * mut c_void, data: * mut c_void) {
    unsafe {
        let p_obj = handle as * mut OfxParam;
        match *p_obj {
            OfxParam::Int1(ref param) => {
                let value : * mut i32 = data as * mut i32; 
                *value = param.default;
            }
            OfxParam::Int2(ref param) => {
                let value : * mut i32 = data as * mut i32; 
                *value = param.default.0;
                *value.offset(1) = param.default.1;
            }
            //OfxParam::Int3(_, _, _) => 3,
            //OfxParam::Double1(_) => 1,
            //OfxParam::Double2(_, _) => 2,
            //OfxParam::Double3(_, _, _) => 3,
            //OfxParam::RGB(_, _, _) => 3,
            //OfxParam::RGBA(_, _, _, _) => 4,
            //OfxParam::String(_) => 1,
            //OfxParam::Boolean(bool) => 1,
            //OfxParam::Choice(_) => 1,
            //OfxParam::Custom(_) => 1,
            //OfxParam::PushButton(_) => 1,
            //OfxParam::Group(_) => 1,
            //OfxParam::Page(_) => 1,
            _ => panic!("not implemented yet"),
        }
    }
    
}

#[no_mangle]
pub extern fn param_get_type(handle: * mut c_void) -> u32 {
    let p_obj = handle as * mut OfxParam;
    unsafe {
        match *p_obj {
            OfxParam::Int1(_) => 0,
            OfxParam::Int2(_) => 0,
            OfxParam::Int3(_, _, _) => 0,
            OfxParam::Double1(_) => 1,
            OfxParam::Double2(_, _) => 1,
            OfxParam::Double3(_, _, _) => 1,
            OfxParam::RGB(_, _, _) => 1,
            OfxParam::RGBA(_, _, _, _) => 1,
            OfxParam::String(_) => 2,
            OfxParam::Boolean(bool) => 0,
            OfxParam::Choice(_) => 0,
            OfxParam::Custom(_) => 0,
            OfxParam::PushButton(_) => 0,
            OfxParam::Group(_) => 0,
            OfxParam::Page(_) => 0,
        }
    }
}

// TODO: return a pointer on a parameter
extern fn param_get_handle(pset_ptr: OfxParamSetHandle, name: * const c_char, handle: * mut OfxParamHandle, props: * mut OfxPropertySetHandle)-> OfxStatus {
    if pset_ptr.is_null() {
        error!("param_get_handle parameter set handle is null");
        return kOfxStatErrBadHandle;
    }
    trace!("paramGetHandle {:?} in {:?}", name, pset_ptr);
    unsafe {
        let mut param_set : & mut OfxParameterSet = transmute(pset_ptr);
        let handle_and_prop = param_set.get_handle_and_prop(name);
        *handle = handle_and_prop.0;
        if !props.is_null() {
            // TODO : set properties
            //param : & OfxParam = transmute(handle);
            warn!("setting prop {:?}", handle_and_prop.1);
            *props = handle_and_prop.1;
        }
    }
    kOfxStatOK
}

extern fn param_set_get_property_set(handle: OfxParamHandle, pset: * mut OfxPropertySetHandle) -> OfxStatus {
    kOfxStatOK
}

extern fn param_get_num_keys(handle: OfxParamHandle, nb_keys: * mut libc::c_int) -> OfxStatus {
    unsafe {
        *nb_keys = 0;
    }
    kOfxStatOK    
}

extern fn param_get_key_time(handle: OfxParamHandle, n_keytime: libc::c_uint, time:* mut OfxTime) -> OfxStatus {
    kOfxStatOK
}

extern fn param_get_key_index(handle: OfxParamHandle, time: OfxTime, direction: libc::c_int, index:* mut libc::c_int) -> OfxStatus {
    kOfxStatOK
}

extern fn param_delete_key(handle: OfxParamHandle, time: OfxTime) -> OfxStatus {
    kOfxStatOK
}

extern fn param_delete_all_keys(handle: OfxParamHandle) -> OfxStatus {
    kOfxStatOK
}

extern fn param_copy(param_to: OfxParamHandle, param_from: OfxParamHandle, dst_offset: OfxTime, frame_range: * const OfxRangeD) -> OfxStatus {
    kOfxStatOK
}

extern fn param_edit_begin(handle: OfxParamSetHandle, name: * const libc::c_char) -> OfxStatus {
    kOfxStatOK
}

extern fn param_edit_end(handle: OfxParamSetHandle) -> OfxStatus {
    kOfxStatOK
}
#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxParameterSuiteV1 {
    paramDefine : extern fn (OfxParamSetHandle, * const c_char, * const c_char, * mut OfxPropertySetHandle) -> OfxStatus,
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


/// We had to write C bindings cause rust doesn't handle varargs.
#[link(name = "ofxhelpers")]
extern { 
    fn param_get_value (param_set: OfxParamHandle, ...) -> OfxStatus;
    fn param_get_value_at_time (param_set: OfxParamHandle, time: OfxTime, ...) -> OfxStatus;
    fn param_get_derivative (param_set: OfxParamHandle, time: OfxTime, ...) -> OfxStatus;
    fn param_get_integral (param_set: OfxParamHandle, time1: OfxTime, time2: OfxTime, ...) -> OfxStatus;
    fn param_set_value (param_set: OfxParamHandle, ...) -> OfxStatus;
    fn param_set_value_at_time (param_set: OfxParamHandle, time: OfxTime, ...) -> OfxStatus;
}

pub static OFX_PARAMETER_SUITE_V1 : OfxParameterSuiteV1 = OfxParameterSuiteV1 {
    paramDefine: param_define,
    paramGetHandle: param_get_handle,
    paramSetGetPropertySet: param_set_get_property_set,
    paramGetValue : param_get_value,    
    paramGetValueAtTime : param_get_value_at_time,    
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
fn init_parameter_test() -> (* mut OfxParameterSet, CString, CString) {
    let p_set = Box::into_raw(OfxParameterSet::new());
    let p_type = CString::new("OfxParamTypeInteger").unwrap();
    let p_name = CString::new("TestIntParam").unwrap();
    (p_set, p_type, p_name)
}

#[cfg(test)]
unsafe fn param_initialization_test(param: * mut OfxParam) {
    match *param {
        OfxParam::Int1(ref k) => assert!(k.default==KeyFramedParameter::default().default),
        OfxParam::Int2(ref k) => assert!(k.default==KeyFramedParameter::default().default),
        _ => assert!(false),            
    }
}

#[test]
fn create_parameter_set() {
    let (p_set, p_type, p_name) = init_parameter_test();
    // call image effect to create a paramset.
    let ret = unsafe{(OFX_PARAMETER_SUITE_V1.paramDefine)(transmute(p_set), p_type.as_ptr(), p_name.as_ptr(), ptr::null_mut())};
    assert!(ret == kOfxStatOK);
    let mut p_handle : OfxParamHandle = ptr::null_mut();
    let ret2 = unsafe {(OFX_PARAMETER_SUITE_V1.paramGetHandle)(transmute(p_set), p_name.as_ptr(), &mut p_handle, ptr::null_mut())};
    assert!(p_handle != ptr::null_mut());
    assert!(ret2 == kOfxStatOK);
}

#[test]
fn create_and_set_parameter( ) {
    let (p_set, p_type, p_name) = init_parameter_test();
    unsafe {
        let ret = (OFX_PARAMETER_SUITE_V1.paramDefine)(transmute(p_set), p_type.as_ptr(), p_name.as_ptr(), ptr::null_mut());
        let mut p_handle : OfxParamHandle = ptr::null_mut();
        let ret2 = (OFX_PARAMETER_SUITE_V1.paramGetHandle)(transmute(p_set), p_name.as_ptr(), &mut p_handle, ptr::null_mut());
        let p_obj = p_handle as * mut OfxParam;
        param_initialization_test(p_obj);
        let ret3 = (OFX_PARAMETER_SUITE_V1.paramSetValue)(p_handle, 257 as i32);
        match *p_obj {
            OfxParam::Int1(ref p) => {
                assert!(p.default==257)
            }
            _ => assert!(false),            
        }
    }
}

#[test]
fn create_and_set_parameter_2d( ) {
    let p_set = Box::into_raw(OfxParameterSet::new());
    let p_type = CString::new("OfxParamTypeInteger2D").unwrap();
    let p_name = CString::new("TestInt2DParam").unwrap();
    unsafe {
        let ret = (OFX_PARAMETER_SUITE_V1.paramDefine)(transmute(p_set), p_type.as_ptr(), p_name.as_ptr(), ptr::null_mut());
        let mut p_handle : OfxParamHandle = ptr::null_mut();
        let ret2 = (OFX_PARAMETER_SUITE_V1.paramGetHandle)(transmute(p_set), p_name.as_ptr(), &mut p_handle, ptr::null_mut());
        // Check init value is zero
        let p_obj = p_handle as * mut OfxParam;
        param_initialization_test(p_obj);
        // Set value
        let ret3 = (OFX_PARAMETER_SUITE_V1.paramSetValue)(p_handle, 257 as i32, 67 as i32);
        // Check new value
        match *p_obj {
            OfxParam::Int2(ref param) => {
                assert!(param.default.0==257 && param.default.1==67)
            }
            _ => assert!(false),            
        }

        let a : i32 = 0;
        let b : i32 = 0;

        let ret4 = (OFX_PARAMETER_SUITE_V1.paramGetValue)(p_handle, &a , &b);
        assert!(a==257);
        assert!(b==67)
    }
}


