// Wrapper for property values
extern crate libc;
use std::convert::*;

/// Container for a property value
#[derive(Debug, PartialEq, Clone)]
pub enum PropertyValue {
    Pointer (* const libc::c_void),
    Integer (libc::c_int),
    Double (f64), // TODO: double check 
    String (* const libc::c_char),
    PointerN(Vec<* const libc::c_void>),
    StringN(Vec<* const libc::c_char>),
    DoubleN(Vec<f64>),
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
