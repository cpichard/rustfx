
extern crate libc;
use libc::*;
use bindings::property::*;
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

    // FIXME: define would better be in the ofx function instead, to keep OfxParameterSet less
    // dependent in ofx
    pub fn define(&mut self, p_type: *const c_char, p_name: *const c_char) -> &OfxParam {
        let p_type_str = unsafe { CStr::from_ptr(p_type) }.to_str().unwrap();
        let p_name_str = unsafe { CStr::from_ptr(p_name) }.to_owned();
        trace!("defining parameter {:?} {:?}", p_name_str, p_type_str);
        // FIXME: use statics
        match p_type_str {
            "OfxParamTypeInteger" => {
                self.data.insert(p_name_str, OfxParam::Int1(KeyFramedParameter::default()))
            } 
            "OfxParamTypeDouble" => {
                self.data.insert(p_name_str, OfxParam::Double1(KeyFramedParameter::default()))
            } 
            "OfxParamTypeBoolean" => {
                self.data.insert(p_name_str, OfxParam::Boolean(true, Box::default()))
            } 
            "OfxParamTypeChoice" => self.data.insert(p_name_str, OfxParam::Choice(0)), 
            "OfxParamTypeRGBA" => {
                self.data.insert(p_name_str, OfxParam::RGBA(KeyFramedParameter::default()))
            } 
            "OfxParamTypeRGB" => {
                self.data.insert(p_name_str, OfxParam::RGB(KeyFramedParameter::default()))
            } 
            "OfxParamTypeDouble2D" => {
                self.data.insert(p_name_str, OfxParam::Double2(KeyFramedParameter::default()))
            } 
            "OfxParamTypeInteger2D" => {
                self.data.insert(p_name_str, OfxParam::Int2(KeyFramedParameter::default()))
            } 
            "OfxParamTypeDouble3D" => {
                self.data.insert(p_name_str, OfxParam::Double3(KeyFramedParameter::default()))
            } 
            "OfxParamTypeInteger3D" => {
                self.data.insert(p_name_str, OfxParam::Int3(KeyFramedParameter::default()))
            } 
            "OfxParamTypeString" => {
                self.data.insert(p_name_str,
                                 OfxParam::String(CString::new("").unwrap(), Box::default()))
            }
            "OfxParamTypeCustom" => self.data.insert(p_name_str, OfxParam::Custom(0)),
            "OfxParamTypeGroup" => self.data.insert(p_name_str, OfxParam::Group(0)),
            "OfxParamTypePage" => self.data.insert(p_name_str, OfxParam::Page(0)),
            "OfxParamTypePushButton" => self.data.insert(p_name_str, OfxParam::PushButton(0)),
            _ => panic!("unknown parameter type"),
        };
        let p_name_str = unsafe { CStr::from_ptr(p_name) }.to_owned();
        self.data.get(&p_name_str).unwrap()
    }

    pub fn get_handle_and_prop(&mut self, p_name: *const c_char) -> (*mut c_void, *mut c_void) {
        let p_name_str = unsafe { CStr::from_ptr(p_name) }.to_owned();
        unsafe {
            match self.data.get(&p_name_str) {
                Some(param) => return (transmute(param), param.properties()),
                None => return (ptr::null_mut(), ptr::null_mut()), 
            }
        }
    }
}
