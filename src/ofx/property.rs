// Property suite
extern crate libc;
use std::collections::HashMap;
use std::ffi::{CString, CStr};

use std::convert::*;
use ofx::core::*;
use std::slice;
use std::mem;
use ofx::propertyvalue::*;


/// Properties are stored in a HashMap for now
#[repr(C)]
pub struct OfxPropertySet {
    pub props: Box<HashMap<CString, PropertyValue>>,
}

/// Handles are passed to the C plugins, the void * type
/// is C compatible type
pub type OfxPropertySetHandle = * mut libc::c_void;

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


/// Function signature definition
type PropSetPointerType = extern fn (OfxPropertySetHandle, * const libc::c_char, libc::c_int, * const libc::c_void) -> OfxStatus;
type PropSetStringType = extern fn (OfxPropertySetHandle, * const libc::c_char, libc::c_int, * const libc::c_char) -> OfxStatus;
type PropSetDoubleType = extern fn (OfxPropertySetHandle, * const libc::c_char, libc::c_int, libc::c_double) -> OfxStatus;
type PropSetIntType = extern fn (OfxPropertySetHandle, * const libc::c_char, libc::c_int, libc::c_int) -> OfxStatus;
type PropSetPointerNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int,*const * const libc::c_void) -> OfxStatus;
type PropSetIntNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, * const libc::c_int) -> OfxStatus;
type PropSetDoubleNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, * const libc::c_double) -> OfxStatus;
type PropSetStringNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, * const * const libc::c_char) -> OfxStatus;
type PropGetPointerType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut * const libc::c_void) -> OfxStatus;
type PropGetStringType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut * const libc::c_char) -> OfxStatus;
type PropGetDoubleType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut libc::c_double) -> OfxStatus;
type PropGetIntType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut libc::c_int) -> OfxStatus;
type PropGetPointerNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut * const libc::c_void) -> OfxStatus;
type PropGetStringNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut * const libc::c_char) -> OfxStatus;
type PropGetDoubleNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut libc::c_double) -> OfxStatus;
type PropGetIntegerNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut libc::c_double) -> OfxStatus;
type PropResetType = extern fn(OfxPropertySetHandle, * const libc::c_char) -> OfxStatus;
type PropGetDimensionType = extern fn(OfxPropertySetHandle, * const libc::c_char, * mut libc::c_int) -> OfxStatus;

// ffi
#[repr(C)]
#[allow(non_snake_case)]
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
#[allow(unused_variables)]
pub extern fn insert_property<T>(properties: OfxPropertySetHandle, 
                         property: * const libc::c_char, 
                         index: libc::c_int, 
                         value: T) -> OfxStatus where PropertyValue: From<T> {
    trace!("setting property {:? } using ptr {:?}", property, properties as * const _);
    let property_set : & mut OfxPropertySet = unsafe { mem::transmute(properties) }; 
    let key_cstr = unsafe { CStr::from_ptr(property) };
    let key_cstring = key_cstr.to_owned();
    match property_set.insert(key_cstring, value) {
        Some(prop) => warn!("replacing a previous value"),
        None => unsafe {warn!("new property {}", CStr::from_ptr(property).to_str().unwrap())},
    }
    kOfxStatOK
}

/// Generic function to insert a vector property in a property set
pub extern fn insert_property_multiple<T>(properties:OfxPropertySetHandle, 
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
        //(*properties).insert(key.to_owned(), PropertyValue::from(values));
        let inner_props : * mut OfxPropertySet = mem::transmute(properties);
        //(*inner_props).insert(key.to_str().unwrap(), values);
    }
    // TODO : should return the correct error code 
    return 0;
}

/// Generic function to retrieve a value from a property set
#[allow(unused_variables)]
extern fn get_property<T>(properties: OfxPropertySetHandle,
                         property: * const libc::c_char,
                         index: libc::c_int,
                         dest: * mut T) -> OfxStatus
    where PropertyValue: Into<T>, T: Clone 
{
    if !property.is_null() && !properties.is_null() {
        let key_cstr = unsafe{CStr::from_ptr(property)};
        let key_cstring = key_cstr.to_owned(); // FIXME this is not efficient
        debug!("get_property {:?} on {:?}", key_cstring, properties as *const _);
        let props : & mut OfxPropertySet = unsafe { mem::transmute(properties) }; 
        match props.get(&key_cstring) {
            Some(prop) => unsafe {*dest = (*prop).clone().into()},
            None => error!("could not find key {:?}", key_cstring),
        }
    }
    kOfxStatOK

}

/// Generic function to retrieve a value from a property set
#[allow(unused_variables)]
extern fn get_property_multiple<T>(properties: OfxPropertySetHandle,
                         property: * const libc::c_char,
                         count: libc::c_int,
                         dest: * mut T) -> OfxStatus
    where PropertyValue: Into<Vec<T>>, T: Clone 
{
    //unsafe {
    //    let key_cstr = CStr::from_ptr(property);
    //    let key_cstring = key_cstr.to_owned(); // FIXME this is not efficient
    //    let inner_props : * mut OfxPropertySet = mem::transmute(properties);
    //    match (*inner_props).get(&key_cstring) {
    //        Some(prop) => panic!("not implemented"),
    //        None => panic!("could not find multiple key"),
    //    }
    //}
    panic!("code is missing ?");
}

#[allow(unused_variables)]
extern fn prop_reset_func(properties: OfxPropertySetHandle,
                        property: * const libc::c_char ) -> OfxStatus
{
    // TODO : find out what this function is supposed to do
    panic!("code is missing ?");
}
#[allow(unused_variables)]
extern fn prop_get_dimension_func(properties: OfxPropertySetHandle,
                           property: * const libc::c_char, 
                           dim: * mut libc::c_int) -> OfxStatus
{
    // TODO : find out what this function is supposed to do
    panic!("code is missing ?");
}

pub static OFX_PROPERTY_SUITE_V1 : OfxPropertySuiteV1 = OfxPropertySuiteV1 {
            propSetPointer: insert_property,
            propSetString: insert_property,
            propSetDouble: insert_property,
            propSetInt: insert_property,
            propSetPointerN: insert_property_multiple,
            propSetStringN: insert_property_multiple,
            propSetDoubleN: insert_property_multiple,
            propSetIntN: insert_property_multiple,
            propGetPointer: get_property,
            propGetString: get_property,
            propGetDouble: get_property,
            propGetInt: get_property,
            propGetPointerN: get_property_multiple,
            propGetStringN: get_property_multiple,
            propGetDoubleN: get_property_multiple,
            propGetIntegerN: get_property_multiple,
            propReset: prop_reset_func,
            propGetDimension: prop_get_dimension_func
};


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
    let value = 9299.0 as f64;
    properties.insert(key.clone(), value);
    let value_wrapper = PropertyValue::Double(value);
    assert_eq!(properties.get(&key), Some(&value_wrapper));
}

#[test]
fn test_property_empty() {
    let mut properties = OfxPropertySet::new();
    let key = CString::new("Test").unwrap();
    assert_eq!(properties.get(&key), None);
}


// TODO : test override value with type change; shouldn't happen
