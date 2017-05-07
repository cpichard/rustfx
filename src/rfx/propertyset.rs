// Wrapper for property values
extern crate libc;
use std::convert::*;
use std::collections::HashMap;
use std::ffi::{CString, CStr};
use libc::*;
use suites::core::*;

/// Container for a property value
/// A property value can be either Pointer, Integer, Double, String or Undefined
#[derive(Debug, Clone)]
pub enum PropertyValue {
    Pointer(*const c_void),
    Integer(c_int),
    Double(c_double),
    String(*const c_char),
    Undefined,
}

// TODO: drop for *const c_char

impl PartialEq for PropertyValue {
    fn eq(&self, other: &Self) -> bool {
        if let PropertyValue::String(ref lhs) = *self {
            if let PropertyValue::String(ref rhs) = *other {
                // TODO test for null value
                return unsafe { CStr::from_ptr(*lhs) == CStr::from_ptr(*rhs) };
            } else {
                return false;
            }
        } else if let PropertyValue::Integer(ref lhs) = *self {
            if let PropertyValue::Integer(ref rhs) = *other {
                return lhs == rhs;
            } else {
                return false;
            }
        } else if let PropertyValue::Double(ref lhs) = *self {
            if let PropertyValue::Double(ref rhs) = *other {
                return lhs == rhs;
            } else {
                return false;
            }
        } else if let PropertyValue::Pointer(ref lhs) = *self {
            if let PropertyValue::Pointer(ref rhs) = *other {
                return lhs == rhs;
            } else {
                return false;
            }
        } else {
            false
        }
    }
}

pub trait ToOfxProperty<T> {
    fn from_ref(&self) -> T;
}

/// Properties are stored in a HashMap.
/// For each key we store a vector of properties
#[derive(Clone, Debug)]
pub struct OfxPropertySet {
    props: HashMap<CString, Vec<PropertyValue>>,
}

impl OfxPropertySet {
    /// Create a new boxed property set
    pub fn new() -> Box<Self> {
        let prop_set = OfxPropertySet { props: HashMap::new() };
        Box::new(prop_set)
    }

    /// Insert a value at (key, index)
    pub fn insert<K, T>(&mut self, key: K, index: usize, value: T)
        where PropertyValue: From<T>,
              K: Into<Vec<u8>>
    {
        // Look for property
        let key_cstring = CString::new(key).unwrap();
        let mut properties = self.props.entry(key_cstring).or_insert(Vec::with_capacity(8));
        // Resize if index is out of bounds
        if index >= properties.len() {
            properties.resize(index + 1, PropertyValue::Undefined);
        }
        match properties.get_mut(index) {
            Some(stored) => *stored = PropertyValue::from(value),
            None => panic!("unable to find value at index"),
        };
    }

    /// Get a property value at (key, index)
    pub fn get(&mut self, key: &CString, index: usize) -> Option<&PropertyValue> {
        match self.props.get(key) {
            Some(prop_vector) => prop_vector.get(index),
            None => None,
        }
    }

    /// Returns the number of properties for the key
    pub fn dimension(&mut self, key: &CString) -> Option<usize> {
        match self.props.get(key) {
            Some(prop_vector) => Some(prop_vector.len()),
            None => None,
        }
    }
}

impl Default for Box<OfxPropertySet> {
    fn default() -> Box<OfxPropertySet> {
        OfxPropertySet::new()
    }
}

/// Conversion from a string slice to a PropertyValue
impl<'a> From<&'a str> for PropertyValue {
    fn from(value: &'a str) -> Self {
        let cstr = CString::new(value).unwrap();
        PropertyValue::String(cstr.into_raw())
    }
}

/// Functions to convert to PropertyValues
impl From<*const c_void> for PropertyValue {
    fn from(value: *const c_void) -> Self {
        PropertyValue::Pointer(value)
    }
}

///
impl From<*const c_char> for PropertyValue {
    fn from(value: *const c_char) -> Self {
        let c_str = unsafe { CStr::from_ptr(value) };
        PropertyValue::String(c_str.to_owned().into_raw())
    }
}

impl<'a> From<OfxKeyword<'a>> for PropertyValue {
    fn from(value: OfxKeyword) -> Self {
        PropertyValue::String(value.into())
    }
}

impl From<c_double> for PropertyValue {
    fn from(value: c_double) -> Self {
        PropertyValue::Double(value)
    }
}

impl From<c_int> for PropertyValue {
    fn from(value: c_int) -> PropertyValue {
        PropertyValue::Integer(value)
    }
}

/// ToOfxProperty trait
impl ToOfxProperty<*const c_void> for PropertyValue {
    fn from_ref(&self) -> *const c_void {
        match *self {
            PropertyValue::Pointer(val) => val,
            _ => panic!("wrong type Pointer"),
        }
    }
}
impl ToOfxProperty<c_int> for PropertyValue {
    fn from_ref(&self) -> c_int {
        match *self {
            PropertyValue::Integer(val) => val,
            _ => panic!("wrong type Integer"),
        }
    }
}

impl ToOfxProperty<c_double> for PropertyValue {
    fn from_ref(&self) -> c_double {
        match *self {
            PropertyValue::Double(val) => val,
            _ => panic!("wrong type Double"),
        }
    }
}

impl ToOfxProperty<*const c_char> for PropertyValue {
    fn from_ref(&self) -> *const c_char {
        match self {
            &PropertyValue::String(ref val) => *val,
            _ => panic!("wrong type String"),
        }
    }
}

pub fn properties_ptr(props: Box<OfxPropertySet>) -> *mut c_void {
    Box::into_raw(props) as *mut c_void
}


#[test]
fn test_property_set_and_get_integer() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    let value_0 = 9299 as i32;
    properties.insert(key.clone(), 0, value_0);
    let value_wrapper = PropertyValue::Integer(value_0);
    assert_eq!(properties.get(&key, 0), Some(&value_wrapper));
}

#[test]
fn test_property_set_and_get_floating() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    let value = 9299.0 as c_double;
    properties.insert(key.clone(), 0, value);
    let value_wrapper = PropertyValue::Double(value);
    assert_eq!(properties.get(&key, 0), Some(&value_wrapper));
}

#[test]
fn test_property_set_and_get_string() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    let value = CString::new("test").unwrap();
    properties.insert(key.clone(), 0, value.as_ptr());
    let value_wrapper = PropertyValue::String(value.into_raw());
    assert_eq!(properties.get(&key, 0), Some(&value_wrapper));
}

#[cfg(test)]
pub fn clone_keyword_test<'a>(value: &'a [u8]) -> CString {
    let mut v: Vec<u8> = Vec::with_capacity(value.len());
    unsafe {
        v.set_len(value.len());
    }
    v.clone_from_slice(value);
    v.pop(); // removes \0
    unsafe { CString::from_vec_unchecked(v) }
}


#[cfg(test)]
pub fn kw_to_cstring_test<'a>(value: &'a [u8]) -> CString {
    let mut v: Vec<u8> = Vec::with_capacity(value.len());
    unsafe {
        v.set_len(value.len());
    }
    v.clone_from_slice(value);
    v.pop(); // removes \0
    unsafe { CString::from_vec_unchecked(v) }
}

#[test]
fn test_property_set_and_get_c_char() {
    let mut properties = OfxPropertySet::new();
    let uchar_buffer_key: &'static [u8] = b"uchar_buffer_key\0";
    let uchar_buffer_value: &'static [u8] = b"uchar_buffer_value\0";
    let key = kw_to_cstring_test(uchar_buffer_key);
    let new_value = clone_keyword_test(uchar_buffer_value);
    properties.insert(key, 0, new_value.as_ptr());
    let value_wrapper =
        PropertyValue::String(CString::new("uchar_buffer_value").unwrap().into_raw());
    let key = kw_to_cstring_test(uchar_buffer_key);
    assert_eq!(properties.get(&key, 0), Some(&value_wrapper));
}

#[test]
fn test_property_set_and_get_c_char_clone() {
    let mut properties = OfxPropertySet::new();
    let uchar_buffer_key: &'static [u8] = b"uchar_buffer_key\0";
    let uchar_buffer_value: &'static [u8] = b"uchar_buffer_value\0";
    let key = kw_to_cstring_test(uchar_buffer_key);
    let new_value = clone_keyword_test(uchar_buffer_value);
    properties.insert(key, 0, new_value.as_ptr());
    let mut properties_cloned = properties.clone();
    let value_wrapper =
        PropertyValue::String(CString::new("uchar_buffer_value").unwrap().into_raw());
    let key = kw_to_cstring_test(uchar_buffer_key);
    assert_eq!(properties_cloned.get(&key, 0), Some(&value_wrapper));
}


#[test]
fn test_property_set_and_get_multiple_integer() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    let value_0 = 9299 as i32;
    properties.insert(key.clone(), 0, value_0);
    let value_wrapper = PropertyValue::Integer(value_0);
    assert_eq!(properties.get(&key, 0), Some(&value_wrapper));

    let value_1 = 91909 as i32;
    properties.insert(key.clone(), 1, value_1);
    let value_wrapper = PropertyValue::Integer(value_1);
    assert_eq!(properties.get(&key, 1), Some(&value_wrapper));

    let value_10 = 9190389 as i32;
    properties.insert(key.clone(), 10, value_10);
    let value_wrapper = PropertyValue::Integer(value_10);
    assert_eq!(properties.get(&key, 10), Some(&value_wrapper));
}

#[test]
fn test_property_empty() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    assert_eq!(properties.get(&key, 0), None);
}
