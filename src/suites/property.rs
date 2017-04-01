// Property suite
extern crate libc;
use std::ffi::*;
use std::convert::*;
use suites::core::*;
use std::mem;
use rfx::propertyset::*;
use libc::*;

/// Handles are passed to the C plugins, the void * type
/// is C compatible type
pub type OfxPropertySetHandle = * mut c_void;

/// Function signature definition
type PropSetPointerType = extern fn (OfxPropertySetHandle, * const c_char, c_int, * const c_void) -> OfxStatus;
type PropSetStringType = extern fn (OfxPropertySetHandle, * const c_char, c_int, * const c_char) -> OfxStatus;
type PropSetDoubleType = extern fn (OfxPropertySetHandle, * const c_char, c_int, c_double) -> OfxStatus;
type PropSetIntType = extern fn (OfxPropertySetHandle, * const c_char, c_int, c_int) -> OfxStatus;
type PropSetPointerNType = extern fn(OfxPropertySetHandle, * const c_char, c_int,*const * const c_void) -> OfxStatus;
type PropSetIntNType = extern fn(OfxPropertySetHandle, * const c_char, c_int, * const c_int) -> OfxStatus;
type PropSetDoubleNType = extern fn(OfxPropertySetHandle, * const c_char, c_int, * const c_double) -> OfxStatus;
type PropSetStringNType = extern fn(OfxPropertySetHandle, * const c_char, c_int, * const * const c_char) -> OfxStatus;
type PropGetPointerType = extern fn(OfxPropertySetHandle, * const c_char, c_int, *mut * const c_void) -> OfxStatus;
type PropGetStringType = extern fn(OfxPropertySetHandle, * const c_char, c_int, *mut * const c_char) -> OfxStatus;
type PropGetDoubleType = extern fn(OfxPropertySetHandle, * const c_char, c_int, *mut c_double) -> OfxStatus;
type PropGetIntType = extern fn(OfxPropertySetHandle, * const c_char, c_int, *mut c_int) -> OfxStatus;
type PropGetPointerNType = extern fn(OfxPropertySetHandle, * const c_char, c_int, *mut * const c_void) -> OfxStatus;
type PropGetStringNType = extern fn(OfxPropertySetHandle, * const c_char, c_int, *mut * const c_char) -> OfxStatus;
type PropGetDoubleNType = extern fn(OfxPropertySetHandle, * const c_char, c_int, *mut c_double) -> OfxStatus;
type PropGetIntegerNType = extern fn(OfxPropertySetHandle, * const c_char, c_int, *mut c_int) -> OfxStatus;
type PropResetType = extern fn(OfxPropertySetHandle, * const c_char) -> OfxStatus;
type PropGetDimensionType = extern fn(OfxPropertySetHandle, * const c_char, * mut c_int) -> OfxStatus;

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

/// Generic function to insert a property in a property set at a given index
pub extern fn insert_property<T>(properties: OfxPropertySetHandle, 
                         property: * const c_char, 
                         index: c_int, 
                         value: T) -> OfxStatus where PropertyValue: From<T> {
    if !property.is_null() && !properties.is_null() {
        // Convert to unsigned int or raise an error, FIXME: this could go in the wrapper instead
        let uindex = if index >= 0 {index as usize} else {
            error!("negative index in insert property {:?}", property);
            return kOfxStatErrBadIndex;
        }; 
        trace!("setting property {:? } using ptr {:?}", property, properties as * const _);
        let property_set : & mut OfxPropertySet = unsafe { mem::transmute(properties) }; 
        let key_cstr = unsafe { CStr::from_ptr(property) };
        property_set.insert(key_cstr.to_owned(), uindex, value); 
        kOfxStatOK
    } else {
        error!("null properties or property name in insert property");
        kOfxStatErrBadHandle
    }
}

/// Generic function to insert a vector property in a property set
pub extern fn insert_property_multiple<T>(properties:OfxPropertySetHandle, 
                          property: * const c_char, 
                          count: c_int, 
                          pointer: * const T) -> OfxStatus 
        where PropertyValue: From<T>, T: Clone
{
    if !property.is_null() && !properties.is_null() {
        let ucount = if count >= 0 {count as usize} else {
            error!("negative count in insert property multiple {:?}", property);
            return kOfxStatErrBadIndex;
        }; 
        unsafe {
            let key = CStr::from_ptr(property);
            let property_set : & mut OfxPropertySet = mem::transmute(properties); 
            // FIXME: this could be faster by passing a vector instead of calling
            // n times the insert function
            for i in 0..ucount {
                let val : T = (*pointer.offset(i as isize)).clone();
                property_set.insert(key.to_owned(), i, val);
            }
        }
        kOfxStatOK 
    } else {
        error!("null properties or property name in insert property multiple");
        kOfxStatErrBadHandle
    }
}

/// Generic function to retrieve a value from a property set
extern fn get_property<T>(properties: OfxPropertySetHandle,
                         property: * const c_char,
                         index: c_int,
                         dest: * mut T) -> OfxStatus
    where PropertyValue: Into<T>, T: Clone 
{
    if !property.is_null() && !properties.is_null() {
        let uindex = if index >= 0 {index as usize} else {
            error!("negative index in insert property {:?}", property);
            return kOfxStatErrBadIndex;
        }; 
        let key_cstr = unsafe{CStr::from_ptr(property)};
        let key_cstring = key_cstr.to_owned(); // FIXME this is not efficient
        debug!("get_property {:?} on {:?}", key_cstring, properties as *const _);
        let props : & mut OfxPropertySet = unsafe { mem::transmute(properties) }; 
        match props.get(&key_cstring, uindex) {
            Some(prop) => { 
                unsafe {*dest = (*prop).clone().into()};
                kOfxStatOK
            }
            None => {
                error!("could not find key {:?} or index {:?}", key_cstring, index);
                kOfxStatErrBadIndex
            }
        }
    } else {
        error!("null property set or property name in get property");
        kOfxStatErrBadHandle
    }
}

/// Generic function to retrieve a value from a property set
extern fn get_property_multiple<T>(properties: OfxPropertySetHandle,
                         property: * const c_char,
                         count: c_int,
                         dest: * mut T) -> OfxStatus
    where T: From<PropertyValue>, T: Clone 
{
    if !property.is_null() && !properties.is_null() {
        let ucount = if count >= 0 {count as usize} else {
            error!("negative count in get property multiple {:?}", property);
            return kOfxStatErrBadIndex;
        }; 
        unsafe {
            let key_cstr = CStr::from_ptr(property);
            let key_cstring = key_cstr.to_owned(); // FIXME this is not efficient
            let inner_props : * mut OfxPropertySet = mem::transmute(properties);
            for i in 0..ucount {
                match (*inner_props).get(&key_cstring, i) {
                    Some(prop) => {
                        *dest.offset(i as isize) = T::from(prop.clone());
                    }
                    None => {
                        error!("could not find key in get property multiple");
                        return kOfxStatErrBadIndex;
                    }
                }
            }
        }
        kOfxStatOK
    } else {
        error!("null property set or property name in get property");
        kOfxStatErrBadHandle
    }
}

#[allow(unused_variables)]
extern fn prop_reset_func(properties: OfxPropertySetHandle,
                        property: * const c_char ) -> OfxStatus
{
    // TODO : find out what this function is supposed to do
    panic!("code is missing ?");
}

extern fn prop_get_dimension_func(properties: OfxPropertySetHandle,
                           property: * const c_char, 
                           dim: * mut c_int) -> OfxStatus
{
    if property.is_null() || properties.is_null() {
        error!("property set or property name is null in get dimension");
        return kOfxStatErrBadHandle;
    }
    unsafe {
        let key_cstr = CStr::from_ptr(property);
        let key_cstring = key_cstr.to_owned(); // FIXME this is not efficient
        let inner_props : * mut OfxPropertySet = mem::transmute(properties);
        match (*inner_props).dimension(&key_cstring) {
            Some(len) => { 
                *dim = len as c_int; 
                kOfxStatOK
            }
            None => kOfxStatErrBadIndex,
        }
    }
}

pub static OFX_PROPERTY_SUITE_V1 
    : OfxPropertySuiteV1 = 
        OfxPropertySuiteV1 {
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


#[cfg(test)]
use std::ptr;

#[test]
fn ofx_property_empty_set() {
    let prop_set = OfxPropertySet::new();
    let prop_name = CString::new("TestProperty").unwrap().as_ptr();
    let prop_set_ptr = Box::into_raw(prop_set) as * mut c_void;
    let value : i32 = 283839;
    let value_get : * const c_void = unsafe {mem::transmute(&value)};
    unsafe {
        let ret = (OFX_PROPERTY_SUITE_V1.propGetString)(prop_set_ptr, prop_name, 0, mem::transmute(&value_get));
    }
    //TODO assert_eq!(ret, );    
}

#[test]
fn ofx_property_set_and_get_pointer() {
    let prop_set = OfxPropertySet::new();
    let prop_name = CString::new("TestProperty").unwrap().as_ptr();
    let prop_set_ptr = Box::into_raw(prop_set) as * mut c_void;
    let value : i32 = 283839;
    let value_set : * const c_void = unsafe {mem::transmute(&value)};
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
    let prop_set_ptr = Box::into_raw(prop_set) as * mut c_void;
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
    let prop_set_ptr = Box::into_raw(prop_set) as * mut c_void;
    let value_set : c_double = 37677.0;
    let value_get : c_double = 0.0;
    unsafe {
        (OFX_PROPERTY_SUITE_V1.propSetDouble)(prop_set_ptr, prop_name, 0, value_set);
        (OFX_PROPERTY_SUITE_V1.propGetDouble)(prop_set_ptr, prop_name, 0, mem::transmute(&value_get));
    }
    assert_eq!(value_set, value_get); 
}

#[test]
fn ofx_property_set_and_get_string() {
    let prop_set = OfxPropertySet::new();
    let prop_name = CString::new("TestProperty").unwrap().as_ptr();
    let prop_set_ptr = Box::into_raw(prop_set) as * mut c_void;
    let value_set : * const c_char = CString::new("TestPropertyValue").unwrap().as_ptr();;
    let value_get : * mut c_char = ptr::null_mut();
    unsafe {
        (OFX_PROPERTY_SUITE_V1.propSetString)(prop_set_ptr, prop_name, 0, value_set);
        (OFX_PROPERTY_SUITE_V1.propGetString)(prop_set_ptr, prop_name, 0, mem::transmute(&value_get));
    }
    unsafe { assert_eq!(CStr::from_ptr(value_set), CStr::from_ptr(value_get)); }
}

#[test]
fn ofx_property_set_and_get_multiple_int() {
    let prop_set = OfxPropertySet::new();
    let prop_name = CString::new("TestProperty").unwrap().as_ptr();
    let prop_set_ptr = Box::into_raw(prop_set) as * mut c_void;
    let value_set : [i32;3] = [37677, 677, 90];
    let value_get : [i32;3] = [0, 0, 0];
    unsafe {
        (OFX_PROPERTY_SUITE_V1.propSetIntN)(prop_set_ptr, prop_name, 3, &value_set[0]);
        (OFX_PROPERTY_SUITE_V1.propGetIntegerN)(prop_set_ptr, prop_name, 3, mem::transmute(&(value_get[0])));
    }
    assert_eq!(value_set, value_get); 
}

