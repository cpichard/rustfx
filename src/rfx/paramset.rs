
extern crate libc;
use libc::*;
use bindings::core::*;
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
#[derive(Default)]
pub struct KeyFramedParameter<T: Default> {
    pub properties: Box<OfxPropertySet>, // TODO: remove pub, move function in this module
    pub default: T,
    pub keys: Vec<(OfxTime, T)>,
}

/// Stores a parameter,
/// FIXME: rust data type could be named rfxParameter to differentiate between ofx and rfx
/// Find other field to add, like time, associated properties and so on
/// ADD PropertySet
/// ADD KeyFramedValues
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
    Choice(i32), // FIXME: create the correct data needed
    Custom(i32), // FIXME: create the correct associated data
    PushButton(i32), // FIXME: create the correct data needed
    Group(i32), // FIXME: create the correct data needed
    Page(i32), /* FIXME: create the correct data needed
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
            // TODO String, Boolean, Choice, ...
            _ => {
                error!("no property for this type of parameter, returning null");
                ptr::null_mut()
            }
        }
    }

    pub fn set_raw_data(&mut self, data: *mut c_void) {
        unsafe {
            match *self {
                OfxParam::Int1(ref mut param) => {
                    let value: *mut i32 = data as *mut i32;
                    param.default = *value;
                }
                OfxParam::Int2(ref mut param) => {
                    let value: *mut i32 = data as *mut i32;
                    param.default = (*value, *value.offset(1));
                    // *p_obj = OfxParam::Int2(*value, *value.offset(1));
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
    }
    pub fn get_raw_data(&self, data: *mut c_void) {
        unsafe {
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
}

// Holds parameters. There is one OfxParameterSet per OfxImageEffect
pub struct OfxParameterSet {
    pub data: HashMap<CString, OfxParam>,
}

impl OfxParameterSet {
    pub fn new() -> Box<Self> {
        let pset = OfxParameterSet { data: HashMap::new() };
        Box::new(pset)
    }

    /// This function is used by the bindings to get a pointer to a parameter value
    /// and be able to change it
    pub fn get_handle_and_prop(&mut self, p_name: *const c_char) -> (*mut c_void, *mut c_void) {
        let p_name_str = unsafe { CStr::from_ptr(p_name) }.to_owned();
        unsafe {
            match self.data.get(&p_name_str) {
                Some(param) => return (transmute(param), param.properties()),
                None => return (ptr::null_mut(), ptr::null_mut()), 
            }
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
            "OfxParamTypeChoice" => self.data.insert(param_name, OfxParam::Choice(0)), 
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
            "OfxParamTypeCustom" => self.data.insert(param_name, OfxParam::Custom(0)),
            "OfxParamTypeGroup" => self.data.insert(param_name, OfxParam::Group(0)),
            "OfxParamTypePage" => self.data.insert(param_name, OfxParam::Page(0)),
            "OfxParamTypePushButton" => self.data.insert(param_name, OfxParam::PushButton(0)),
            _ => None, 
        }
    }

    /// Find the parameter or panic
    pub fn get_param(&mut self, param_name: &CString) -> Option<&OfxParam> {
        self.data.get(param_name)
    }

    /// Change default value of a parameter
    pub fn set_int1(&mut self, param_name: &CString, value: i32) {
        // TODO: handle unwrap
        let mut stored_value = self.data.get_mut(param_name).unwrap();
        unsafe {
            stored_value.set_raw_data(transmute(&value));
        }
    }

    pub fn get_int1(&mut self, param_name: &CString) -> i32 {
        let mut param = self.data.get_mut(param_name).unwrap();
        let value: i32 = i32::default();
        unsafe {
            param.get_raw_data(transmute(&value));
        }
        value
    }

    pub fn set_int2(&mut self, param_name: &CString, v1: i32, v2: i32) {
        // TODO: handle unwrap
        let mut stored_value = self.data.get_mut(param_name).unwrap();
        /// MOuahahahaahha... unsafe everywhere
        let value: [i32; 2] = [v1, v2];
        unsafe {
            stored_value.set_raw_data(transmute(&value));
        }
    }

    pub fn get_int2(&mut self, param_name: &CString) -> [i32;2] {
        let mut param = self.data.get_mut(param_name).unwrap();
        let value: [i32; 2] = [0, 0];
        unsafe {
            param.get_raw_data(transmute(&value));
        }
        value
    }
}





// TESTS !!!
//
#[test]
fn set_get_int1() {
    let mut paramset = OfxParameterSet::new();
    let p_name = CString::new("test_parameter").unwrap();
    paramset.create_param("OfxParamTypeInteger", p_name);

    let p_name = CString::new("test_parameter").unwrap();
    paramset.set_int1(&p_name, 3);

    assert!(paramset.get_int1(&p_name) == 3);
}

// TODO return error message when the type is not correct

#[test]
fn set_get_int2() {
    let mut paramset = OfxParameterSet::new();
    let p_name = CString::new("test_parameter").unwrap();
    paramset.create_param("OfxParamTypeInteger2D", p_name);

    let p_name = CString::new("test_parameter").unwrap();
    paramset.set_int2(&p_name, 3, 1);
    let ints = paramset.get_int2(&p_name);

    println!("{:?}", ints);

    assert!(ints[0] == 3 && ints[1] == 1);
}



