// Wrapper for property values
extern crate libc;
use std::convert::*;
use std::collections::HashMap;
use std::ffi::{CString, CStr};
use ofx::core::*;
use std::slice;
use std::mem;

/// Properties are stored in a HashMap for now
pub struct OfxPropertySet {
    props: Box<HashMap<CString, PropertyValue>>,
}

impl OfxPropertySet {
    
    pub fn new () -> Box<Self> {
        let prop_set = OfxPropertySet {
            props: Box::new(HashMap::new()),
        };
        Box::new(prop_set)
    }
    
    pub fn insert<K, T>(& mut self, key: K, value: T) -> Option<PropertyValue> 
        where PropertyValue: From<T>, K : Into<Vec<u8>>
    {
        let key_cstring = CString::new(key).unwrap();
        self.props.insert(key_cstring, PropertyValue::from(value))
    } 

    pub fn get(& mut self, key: &CString) -> Option<&PropertyValue> {
        trace!("property set {:?} queried", self as * const _);    
        debug!("in function get, getting {:?}", key);
        debug!("self.hashmap queried {:?}", & self.props as * const _);
        self.props.get(key)
    }
}

/// Container for a property value
#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    Pointer (* const libc::c_void),
    Integer (libc::c_int),
    Double (libc::c_double), // TODO: double check it shouldn't be a float
    String (* const libc::c_char),
    PointerN(Vec<* const libc::c_void>),
    StringN(Vec<* const libc::c_char>),
    DoubleN(Vec<libc::c_double>),
    IntegerN(Vec<libc::c_int>),
}

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
        match value {
            PropertyValue::String(val) => val,
            _ => panic!("wrong type: String"),
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

#[test]
fn test_property_set_and_get_integer() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    let value = 9299 as i32;
    properties.insert(key.clone(), value);
    let value_wrapper = PropertyValue::Integer(value);
    assert_eq!(properties.get(&key), Some(&value_wrapper));
}

#[test]
fn test_property_set_and_get_floating() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    let value = 9299.0 as libc::c_double;
    properties.insert(key.clone(), value);
    let value_wrapper = PropertyValue::Double(value);
    assert_eq!(properties.get(&key), Some(&value_wrapper));
}

#[test]
fn test_property_set_and_get_string() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    let value = CString::new("test").unwrap();
    properties.insert(key.clone(), value.as_ptr());
    let value_wrapper = PropertyValue::String(value.as_ptr());
    assert_eq!(properties.get(&key), Some(&value_wrapper));
}

#[test]
fn test_property_set_and_get_multiple_integer() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("TestMultiple").unwrap();
    let value : Vec<libc::c_int>= vec![0,1,2]; 
    properties.insert(key.clone(), value.clone());
    let value_wrapper = PropertyValue::IntegerN(value);
    assert_eq!(properties.get(&key), Some(&value_wrapper));
}

#[test]
fn test_property_empty() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    assert_eq!(properties.get(&key), None);
}
