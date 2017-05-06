extern crate libc;
use libc::*;
use suites::core::*;
use std::ffi::*;
use std::mem::*;
use std::ptr;
use std::collections::HashMap;
use std::vec::Vec;
use rfx::propertyset::*;
use std::ops::Deref;
#[repr(C)]
#[allow(non_snake_case)]
// integers, 1, 2 and 3 dimensional
// doubles, 1, 2 and 3 dimensional
// colour, RGB and RGB + Alpha
// booleans
// choice
// string
// custom
// push button
// group
// page
// parametric
//
#[derive(Default, Clone, Debug)]
pub struct KeyFramedParameter<T: Default + Clone> {
    pub properties: Box<OfxPropertySet>, // TODO: remove pub, move function in this module
    pub default: T,
    pub keys: Vec<(OfxTime, T)>,
}

/// Stores a parameter,
/// FIXME: rust data type could be named rfxParameter to differentiate between ofx and rfx
/// Find other field to add, like time, associated properties and so on
#[derive(Clone, Debug)]
pub enum OfxParam {
    Int1(KeyFramedParameter<i32>),
    Int2(KeyFramedParameter<(i32, i32)>),
    Int3(KeyFramedParameter<(i32, i32, i32)>),
    Double1(KeyFramedParameter<f64>),
    Double2(KeyFramedParameter<(f64, f64)>),
    Double3(KeyFramedParameter<(f64, f64, f64)>),
    RGB(KeyFramedParameter<(f64, f64, f64)>),
    RGBA(KeyFramedParameter<(f64, f64, f64, f64)>),
    String(CString, Box<OfxPropertySet>),
    Boolean(bool, Box<OfxPropertySet>),
    Choice(i32, Box<OfxPropertySet>), // FIXME: create the correct data needed
    Custom(i32, Box<OfxPropertySet>), // FIXME: create the correct associated data
    PushButton(i32, Box<OfxPropertySet>), // FIXME: create the correct data needed
    Group(i32, Box<OfxPropertySet>), // FIXME: create the correct data needed
    Page(i32, Box<OfxPropertySet>), /* FIXME: create the correct data needed
                                     * Parametric(i32),  // FIXME: create the correct data needed */
}

impl OfxParam {
    pub fn properties(&self) -> *mut c_void {
        match *self {
            OfxParam::Int1(ref val) => unsafe { transmute(val.properties.deref()) },
            OfxParam::Int2(ref val) => unsafe { transmute(val.properties.deref()) },
            OfxParam::Int3(ref val) => unsafe { transmute(val.properties.deref()) },
            OfxParam::Double1(ref val) => unsafe { transmute(val.properties.deref()) },
            OfxParam::Double2(ref val) => unsafe { transmute(val.properties.deref()) },
            OfxParam::Double3(ref val) => unsafe { transmute(val.properties.deref()) },
            OfxParam::RGB(ref val) => unsafe { transmute(val.properties.deref()) },
            OfxParam::RGBA(ref val) => unsafe { transmute(val.properties.deref()) },
            OfxParam::String(_, ref propertyset) => unsafe { transmute(propertyset.deref()) },
            OfxParam::Boolean(_, ref propertyset) => unsafe { transmute(propertyset.deref()) },
            OfxParam::Custom(_, ref propertyset) => unsafe { transmute(propertyset.deref()) },
            OfxParam::Group(_, ref propertyset) => unsafe { transmute(propertyset.deref()) },
            OfxParam::Page(_, ref propertyset) => unsafe { transmute(propertyset.deref()) },
            // TODO Choice, ...
            ref param @ _ => {
                error!("property for {:?} is not implemented, returning null",
                       param);
                ptr::null_mut()
            }
        }
    }

    pub fn int_set(&mut self, value: &[i32]) {
        match *self {
            OfxParam::Int1(ref mut param) => {
                param.default = value[0];
            }
            OfxParam::Int2(ref mut param) => {
                param.default = (value[0], value[1]);
            }
            OfxParam::Int3(ref mut param) => {
                param.default = (value[0], value[1], value[2]);
            }
            _ => panic!("int_set: setting an int to a non int parameter"),
        }
    }

    pub fn int1_set(&mut self, value: i32) {
        // Should test for type here ??
        unsafe {
            self.set_raw_data(transmute(&value));
        }
    }

    pub fn int1_get(&mut self) -> i32 {
        // Should test for type here ??
        let value: i32 = i32::default();
        unsafe {
            self.get_raw_data(transmute(&value));
        }
        value
    }

    pub fn int2_set(&mut self, v1: i32, v2: i32) {
        let value: [i32; 2] = [v1, v2];
        unsafe {
            self.set_raw_data(transmute(&value));
        }
    }

    pub fn int2_get(&mut self) -> [i32;2] {
        let value: [i32; 2] = [0, 0];
        unsafe {
            self.get_raw_data(transmute(&value));
        }
        value
    }

    pub unsafe fn set_raw_data(&mut self, data: *mut c_void) {
        match *self {
            OfxParam::Int1(ref mut param) => {
                let value: *mut i32 = data as *mut i32;
                param.default = *value;
            }
            OfxParam::Int2(ref mut param) => {
                let value: *mut i32 = data as *mut i32;
                param.default = (*value, *value.offset(1));
            }
            // OfxParam::Int3(_, _, _) => 3,
            // OfxParam::Double1(_) => 1,
            // OfxParam::Double2(_, _) => 2,
            // OfxParam::Double3(_, _, _) => 3,
            // OfxParam::RGB(_, _, _) => 3,
            // OfxParam::RGBA(_, _, _, _) => 4,
            // OfxParam::String(_) => 1,
            // OfxParam::Boolean(bool) => 1,
            // OfxParam::Choice(_) => 1,
            // OfxParam::Custom(_) => 1,
            // OfxParam::PushButton(_) => 1,
            // OfxParam::Group(_) => 1,
            // OfxParam::Page(_) => 1,
            _ => panic!("param set components for this type is not implemented yet"),
        }
    }
    pub unsafe fn get_raw_data(&self, data: *mut c_void) {
        match *self {
            OfxParam::Int1(ref param) => {
                let value: *mut i32 = data as *mut i32;
                *value = param.default;
            }
            OfxParam::Int2(ref param) => {
                let value: *mut i32 = data as *mut i32;
                *value = param.default.0;
                *value.offset(1) = param.default.1;
            }
            // TODO implement other case when the overall structure is less
            // likely to change
            // OfxParam::Int3(_, _, _) => 3,
            // OfxParam::Double1(_) => 1,
            // OfxParam::Double2(_, _) => 2,
            // OfxParam::Double3(_, _, _) => 3,
            // OfxParam::RGB(_, _, _) => 3,
            // OfxParam::RGBA(_, _, _, _) => 4,
            // OfxParam::String(_) => 1,
            // OfxParam::Boolean(bool) => 1,
            // OfxParam::Choice(_) => 1,
            // OfxParam::Custom(_) => 1,
            // OfxParam::PushButton(_) => 1,
            // OfxParam::Group(_) => 1,
            // OfxParam::Page(_) => 1,
            _ => panic!("param get component for this type is not implemented yet"),
        }
    }
}

// Holds parameters. There is one OfxParameterSet per OfxImageEffect
#[derive(Clone, Debug)]
pub struct OfxParameterSet {
    pub data: HashMap<CString, OfxParam>,
    pub propertyset: Box<OfxPropertySet>,
}

impl OfxParameterSet {
    /// Create a new ParameterSet on the heap
    pub fn new() -> Box<Self> {
        let paramset = OfxParameterSet {
            data: HashMap::new(),
            propertyset: Box::default(),
        };
        Box::new(paramset)
    }

    /// This function is used by the suites to get a pointer to the
    /// parameter and property sets, they used as an handle
    pub unsafe fn param_and_prop_ptr(&mut self,
                                     c_param_name: *const c_char)
                                     -> (*mut c_void, *mut c_void) {
        let param_name_str = CStr::from_ptr(c_param_name).to_owned();
        match self.data.get(&param_name_str) {
            Some(param) => return (transmute(param), param.properties()),
            None => return (ptr::null_mut(), ptr::null_mut()), 
        }
    }

    pub fn create_param(&mut self, type_str: &str, param_name_arg: CString) -> Option<OfxParam> {
        let param_name = param_name_arg.clone();
        match type_str {
            "OfxParamTypeInteger" => {
                self.data.insert(param_name, OfxParam::Int1(KeyFramedParameter::default()))
            } 
            "OfxParamTypeDouble" => {
                self.data.insert(param_name, OfxParam::Double1(KeyFramedParameter::default()))
            } 
            "OfxParamTypeBoolean" => {
                self.data.insert(param_name, OfxParam::Boolean(true, Box::default()))
            } 
            "OfxParamTypeChoice" => {
                self.data.insert(param_name, OfxParam::Choice(0, Box::default()))
            } 
            "OfxParamTypeRGBA" => {
                self.data.insert(param_name, OfxParam::RGBA(KeyFramedParameter::default()))
            } 
            "OfxParamTypeRGB" => {
                self.data.insert(param_name, OfxParam::RGB(KeyFramedParameter::default()))
            } 
            "OfxParamTypeDouble2D" => {
                self.data.insert(param_name, OfxParam::Double2(KeyFramedParameter::default()))
            } 
            "OfxParamTypeInteger2D" => {
                self.data.insert(param_name, OfxParam::Int2(KeyFramedParameter::default()))
            } 
            "OfxParamTypeDouble3D" => {
                self.data.insert(param_name, OfxParam::Double3(KeyFramedParameter::default()))
            } 
            "OfxParamTypeInteger3D" => {
                self.data.insert(param_name, OfxParam::Int3(KeyFramedParameter::default()))
            } 
            "OfxParamTypeString" => {
                self.data.insert(param_name,
                                 OfxParam::String(CString::new("").unwrap(), Box::default()))
            }
            "OfxParamTypeCustom" => {
                self.data.insert(param_name, OfxParam::Custom(0, Box::default()))
            }
            "OfxParamTypeGroup" => self.data.insert(param_name, OfxParam::Group(0, Box::default())),
            "OfxParamTypePage" => self.data.insert(param_name, OfxParam::Page(0, Box::default())),
            "OfxParamTypePushButton" => {
                self.data.insert(param_name, OfxParam::PushButton(0, Box::default()))
            }
            _ => None, 
        }
    }

    /// Find the parameter or nothing
    // TODO: get_param -> param_get
    pub fn param_get(&mut self, param_name: &CString) -> Option<&mut OfxParam> {
        self.data.get_mut(param_name)
    }
}


// TESTS !!!
//
#[test]
fn set_and_get_int1() {
    let mut paramset = OfxParameterSet::new();
    let param_name = CString::new("test_parameter").unwrap();
    paramset.create_param("OfxParamTypeInteger", param_name.clone());

    let mut param = paramset.param_get(&param_name).unwrap();
    param.int1_set(3);
    assert!(param.int1_get() == 3);
}

#[test]
fn set_and_get_int2() {
    let mut paramset = OfxParameterSet::new();
    let param_name = CString::new("test_parameter").unwrap();
    paramset.create_param("OfxParamTypeInteger2D", param_name.clone());

    let mut param = paramset.param_get(&param_name).unwrap();
    param.int2_set(3, 400);
    assert_eq!(param.int2_get(), [3, 400]);
    param.int2_set(6789, -90);
    assert_eq!(param.int2_get(), [6789, -90]);
}


// TODO test parameter properties
