// Property suite
extern crate libc;
use std::ffi::*;
use std::convert::*;
use ofx::core::*;
use std::slice;
use std::mem;
use std::ptr;
use rfx::propertyset::*;

/// Handles are passed to the C plugins, the void * type
/// is C compatible type
pub type OfxPropertySetHandle = * mut libc::c_void;

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
type PropGetIntegerNType = extern fn(OfxPropertySetHandle, * const libc::c_char, libc::c_int, *mut libc::c_int) -> OfxStatus;
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
        let property_set : & mut OfxPropertySet = mem::transmute(properties); 
        property_set.insert(key.to_owned(), values);
    }
    // TODO : should return the correct error code 
    kOfxStatOK
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
    where Vec<T>: From<PropertyValue>, T: Clone 
{
    unsafe {
        let key_cstr = CStr::from_ptr(property);
        let key_cstring = key_cstr.to_owned(); // FIXME this is not efficient
        let inner_props : * mut OfxPropertySet = mem::transmute(properties);
        match (*inner_props).get(&key_cstring) {
                
            Some(prop) => {
                let found : Vec<T> = Vec::from(prop.clone());
                ptr::copy(&found[0], dest, count as usize);
            }
            None => panic!("could not find key"),
        }
    }
    kOfxStatOK
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
fn ofx_property_set_and_get_pointer() {
    let prop_set = OfxPropertySet::new();
    let prop_name = CString::new("TestProperty").unwrap().as_ptr();
    let prop_set_ptr = Box::into_raw(prop_set) as * mut libc::c_void;
    let value : i32 = 283839;
    let value_set : * const libc::c_void = unsafe {mem::transmute(&value)};
    let value_get : * mut _ = ptr::null_mut();
    assert!(value_set != value_get);    
    unsafe {
        (OFX_PROPERTY_SUITE_V1.propSetPointer)(prop_set_ptr, prop_name, 0, value_set);
        (OFX_PROPERTY_SUITE_V1.propGetPointer)(prop_set_ptr, prop_name, 0, mem::transmute(&value_get));
    }
    assert_eq!(value_set, value_get);    
}

#[test]
fn ofx_property_set_and_get_int() {
    let prop_set = OfxPropertySet::new();
    let prop_name = CString::new("TestProperty").unwrap().as_ptr();
    let prop_set_ptr = Box::into_raw(prop_set) as * mut libc::c_void;
    let value_set : i32 = 37677;
    let value_get : i32 = 0;
    unsafe {
        (OFX_PROPERTY_SUITE_V1.propSetInt)(prop_set_ptr, prop_name, 0, value_set);
        (OFX_PROPERTY_SUITE_V1.propGetInt)(prop_set_ptr, prop_name, 0, mem::transmute(&value_get));
    }
    assert_eq!(value_set, value_get);    
}

#[test]
fn ofx_property_set_and_get_double() {
    let prop_set = OfxPropertySet::new();
    let prop_name = CString::new("TestProperty").unwrap().as_ptr();
    let prop_set_ptr = Box::into_raw(prop_set) as * mut libc::c_void;
    let value_set : libc::c_double = 37677.0;
    let value_get : libc::c_double = 0.0;
    unsafe {
        (OFX_PROPERTY_SUITE_V1.propSetDouble)(prop_set_ptr, prop_name, 0, value_set);
        (OFX_PROPERTY_SUITE_V1.propGetDouble)(prop_set_ptr, prop_name, 0, mem::transmute(&value_get));
    }
    assert_eq!(value_set, value_get);    
}

#[test]
fn ofx_property_set_and_get_multiple_int() {
    let prop_set = OfxPropertySet::new();
    let prop_name = CString::new("TestProperty").unwrap().as_ptr();
    let prop_set_ptr = Box::into_raw(prop_set) as * mut libc::c_void;
    let value_set : [i32;3] = [37677, 677, 90];
    let value_get : [i32;3] = [0, 0, 0];
    unsafe {
        (OFX_PROPERTY_SUITE_V1.propSetIntN)(prop_set_ptr, prop_name, 3, &value_set[0]);
        (OFX_PROPERTY_SUITE_V1.propGetIntegerN)(prop_set_ptr, prop_name, 3, mem::transmute(&(value_get[0])));
    }
    assert_eq!(value_set, value_get);    
}

