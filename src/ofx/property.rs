// Property suite
extern crate libc;
use std::collections::HashMap;
use std::ffi::{CString, CStr};

use std::convert::*;
use ofx::core::*;
use std::slice;

/// Container for a property value
#[derive(Debug, PartialEq, Clone)]
enum PropertyValue {
    Pointer (* const libc::c_void),
    Integer (libc::c_int),
    Double (f64), // TODO: double check 
    String (* const libc::c_char),
    PointerN(Vec<* const libc::c_void>),
    StringN(Vec<* const libc::c_char>),
    DoubleN(Vec<f64>),
    IntegerN(Vec<libc::c_int>),
}

/// Properties are stored in a HashMap for now
pub type OfxPropertySet = HashMap<CString, PropertyValue>;

/// Functions to convert to PropertyValues
impl From<* const libc::c_void> for PropertyValue {
    fn from(value: * const libc::c_void) -> Self {
        PropertyValue::Pointer(value)
    }
}

///
impl From<PropertyValue> for * const libc::c_void {
    fn from(value: PropertyValue) -> Self {
        match value {
            PropertyValue::Pointer(p) => p,
            _ => panic!("wrong type: Pointer"),
        } 
    }
}

///
impl From<* const libc::c_char> for PropertyValue {
    fn from(value: * const libc::c_char) -> Self {
            PropertyValue::String(value)
    }
}

///
impl From<PropertyValue> for * const libc::c_char {
    fn from(value: PropertyValue) -> Self {
        unsafe {
            match value {
                PropertyValue::String(val) => val,
                _ => panic!("wrong type: String"),
            }
        }
    }
}

impl From<libc::c_double> for PropertyValue {
    fn from(value: libc::c_double) -> Self {
        PropertyValue::Double(value)
    }
}

impl From<PropertyValue> for libc::c_double {
    fn from(value: PropertyValue) -> Self {
        match value {
            PropertyValue::Double(value) => value,
            _ => panic!("wrong type Double"),
        }
    }
}

impl From<libc::c_int> for PropertyValue {
    fn from(value: libc::c_int) -> PropertyValue {
        PropertyValue::Integer(value)
    }
}

impl From<PropertyValue> for libc::c_int {
    fn from(value: PropertyValue) -> Self {
        match value {
            PropertyValue::Integer(val) => val,
            _ => panic!("wrong type Integer"),
        }
    }
}

impl From< Vec<* const libc::c_void> > for PropertyValue {
    fn from(values: Vec<* const libc::c_void>) -> Self {
        PropertyValue::PointerN(values)
    }
}

impl From< Vec<libc::c_int> > for PropertyValue {
    fn from(values: Vec<libc::c_int>) -> Self {
        PropertyValue::IntegerN(values)
    }
}

impl From< Vec<libc::c_double> > for PropertyValue {
    fn from(values: Vec<libc::c_double>) -> Self {
        PropertyValue::DoubleN(values)
    }
}

impl From< Vec<* const libc::c_char> > for PropertyValue {
    fn from(values: Vec<* const libc::c_char>) -> Self {
        PropertyValue::StringN(values)
    }
}

impl From<PropertyValue> for Vec<* const libc::c_void> {
    fn from(values: PropertyValue) -> Self {
        match values {
            PropertyValue::PointerN(val) => val,
            _ => panic!("wrong type PointerN"),
        }
    }
}

impl From<PropertyValue> for Vec<* const libc::c_char> {
    fn from(values: PropertyValue) -> Self {
        match values {
            PropertyValue::StringN(val) => val,
            _ => panic!("wrong type StringN"),
        }
    }
}

impl From<PropertyValue> for Vec<libc::c_double> {
    fn from(values: PropertyValue) -> Self {
        match values {
            PropertyValue::DoubleN(val) => val,
            _ => panic!("wrong type DoubleN"),
        }
    }
}

impl From<PropertyValue> for Vec<libc::c_int> {
    fn from(values: PropertyValue) -> Self {
        match values {
            PropertyValue::IntegerN(val) => val,
            _ => panic!("wrong type IntegerN"),
        }
    }
}

/// Function signature definition
type PropSetPointerType = extern fn (* mut OfxPropertySet, * const libc::c_char, libc::c_int, * const libc::c_void) -> OfxStatus;
type PropSetStringType = extern fn (* mut OfxPropertySet, * const libc::c_char, libc::c_int, * const libc::c_char) -> OfxStatus;
type PropSetDoubleType = extern fn (* mut OfxPropertySet, * const libc::c_char, libc::c_int, libc::c_double) -> OfxStatus;
type PropSetIntType = extern fn (* mut OfxPropertySet, * const libc::c_char, libc::c_int, libc::c_int) -> OfxStatus;
type PropSetPointerNType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int,*const * const libc::c_void) -> OfxStatus;
type PropSetIntNType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, * const libc::c_int) -> OfxStatus;
type PropSetDoubleNType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, * const libc::c_double) -> OfxStatus;
type PropSetStringNType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, * const * const libc::c_char) -> OfxStatus;
type PropGetPointerType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, *mut * const libc::c_void) -> OfxStatus;
type PropGetStringType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, *mut * const libc::c_char) -> OfxStatus;
type PropGetDoubleType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, *mut libc::c_double) -> OfxStatus;
type PropGetIntType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, *mut libc::c_int) -> OfxStatus;
type PropGetPointerNType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, *mut * const libc::c_void) -> OfxStatus;
type PropGetStringNType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, *mut * const libc::c_char) -> OfxStatus;
type PropGetDoubleNType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, *mut libc::c_double) -> OfxStatus;
type PropGetIntegerNType = extern fn(* mut OfxPropertySet, * const libc::c_char, libc::c_int, *mut libc::c_double) -> OfxStatus;
type PropResetType = extern fn(* mut OfxPropertySet, * const libc::c_char) -> OfxStatus;
type PropGetDimensionType = extern fn(* mut OfxPropertySet, * const libc::c_char, * mut libc::c_int) -> OfxStatus;
// ffi
#[repr(C)]
pub struct OfxPropertySuiteV1 {
    propSetPointer : PropSetPointerType,
    propSetString: PropSetStringType,
    propSetDouble : PropSetDoubleType,
    propSetInt : PropSetIntType,
    propSetPointerN: PropSetPointerNType,
    propSetStringN: PropSetStringNType,
    propSetDoubleN: PropSetDoubleNType,
    propSetIntN: PropSetIntNType,
    propGetPointer: PropGetPointerType,
    propGetString: PropGetStringType,
    propGetDouble: PropGetDoubleType,
    propGetInt: PropGetIntType,
    propGetPointerN: PropGetPointerNType,
    propGetStringN: PropGetStringNType,
    propGetDoubleN: PropGetDoubleNType,
    propGetIntegerN: PropGetIntegerNType,
    propReset: PropResetType,
    propGetDimension: PropGetDimensionType,
}

/// Generic function to insert a property in a property set
extern fn propSetFunc<T>(properties:* mut OfxPropertySet, 
                         property: * const libc::c_char, 
                         index: libc::c_int, 
                         value: T) -> OfxStatus where PropertyValue: From<T> {
    unsafe {
        let key = CStr::from_ptr(property);
        (*properties).insert(key.to_owned(), PropertyValue::from(value));
    }
    // TODO : should return if the insert was effective
    return 0;
}

/// Generic function to insert a vector property in a property set
extern fn propSetFuncN<T>(properties:* mut OfxPropertySet, 
                          property: * const libc::c_char, 
                          count: libc::c_int, 
                          pointer: * const T) -> OfxStatus 
        where PropertyValue: From<Vec<T>>, T: Clone
{
    unsafe {
        let rawparts : &[T] = slice::from_raw_parts(pointer, count as usize); 
        let mut values : Vec<T> = Vec::new();
        values.extend_from_slice(rawparts);
        let key = CStr::from_ptr(property);
        (*properties).insert(key.to_owned(), PropertyValue::from(values));
    }
    // TODO : should return the correct error code 
    return 0;
}

/// Generic function to retrieve a value from a property set
extern fn propGetFunc<T>(properties: * mut OfxPropertySet,
                         property: * const libc::c_char,
                         index: libc::c_int,
                         dest: * mut T) -> OfxStatus
    where PropertyValue: Into<T>, T: Clone 
{
    unsafe {
        let key = CStr::from_ptr(property);
        match (*properties).get(key) {
            Some(prop) => *dest = (*prop).clone().into(),
            _ => panic!("could not find key"),
        }
    }
    0 
}

/// Generic function to retrieve a value from a property set
extern fn propGetFuncN<T>(properties: * mut OfxPropertySet,
                         property: * const libc::c_char,
                         count: libc::c_int,
                         dest: * mut T) -> OfxStatus
    where PropertyValue: Into<Vec<T>>, T: Clone 
{
    unsafe {
        let key = CStr::from_ptr(property);
        match (*properties).get(key) {
            Some(prop) => panic!("not implemented"),
            _ => panic!("could not find key"),
        }
    }
    0 
}

extern fn propResetFunc(properties: * mut OfxPropertySet,
                        property: * const libc::c_char ) -> OfxStatus
{
    // TODO : find out what this function is supposed to do
    0
}

extern fn propGetDimensionFunc(properties: * mut OfxPropertySet,
                           property: * const libc::c_char, 
                           dim: * mut libc::c_int) -> OfxStatus
{
    // TODO : find out what this function is supposed to do
    0
}



impl OfxPropertySuiteV1 {

    pub fn new() -> OfxPropertySuiteV1 {
        OfxPropertySuiteV1 {
            propSetPointer: propSetFunc,
            propSetString: propSetFunc,
            propSetDouble: propSetFunc,
            propSetInt: propSetFunc,
            propSetPointerN: propSetFuncN,
            propSetStringN: propSetFuncN,
            propSetDoubleN: propSetFuncN,
            propSetIntN: propSetFuncN,
            propGetPointer: propGetFunc,
            propGetString: propGetFunc,
            propGetDouble: propGetFunc,
            propGetInt: propGetFunc,
            propGetPointerN: propGetFuncN,
            propGetStringN: propGetFuncN,
            propGetDoubleN: propGetFuncN,
            propGetIntegerN: propGetFuncN,
            propReset: propResetFunc,
            propGetDimension: propGetDimensionFunc
        }
    }
}


//
// TODO define OfxPropertySetHandle 
// pub type OfxPropertySetHandle = &OfxPropertySet;

#[test]
fn test_property_set() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    let value = PropertyValue::Integer(9299);
    properties.insert(key.clone(), value.clone());
    assert!(properties.contains_key(&key));
    assert_eq!(properties.get(&key), Some(&value));
}

// TODO : test override value with type change; shouldn't happen
