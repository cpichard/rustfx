use libc;
use bindings::property::*;
use rfx::propertyset::*;
use std::collections::HashMap;
use bindings::param::*;
use bindings::core::*;
use std::mem;
use std::ffi::{CString, CStr};
use std::ops::DerefMut;

// NOTE: ImageEffectStruct is used for:
// PluginInstance
// PluginDescriptor
// It's like a Node
#[derive(Clone)]
pub struct OfxImageEffectStruct {
    props: *mut OfxPropertySet,
    params: *mut OfxParameterSet,
    clips: HashMap<CString, Box<OfxImageClip>>,
}

//impl Clone for OfxImageEffectStruct {
//    fn clone(&self) -> OfxImageEffectStruct { 
//        // TODO: copy values
//        OfxImageEffectStruct {
//            props: self.props.clone(), 
//            params: self.params.clone(),
//            clips: self.clips.clone(), // TODO: it seems inefficient at firts sight; check perf ! 
//        }
//    }
//}

pub type OfxImageEffectHandle = *mut libc::c_void;


impl OfxImageEffectStruct {
    pub fn new() -> Self {
        OfxImageEffectStruct {
            props: Box::into_raw(OfxPropertySet::new()),
            params: Box::into_raw(OfxParameterSet::new()),
            clips: HashMap::new(),
        }
    }
}

pub struct OfxImageMemoryStruct {
   // TODO stuff for image memory 
}

pub type OfxImageMemoryHandle = *mut libc::c_void;

#[derive(Clone)]
pub struct OfxImageClip {
    // TODO move ImageClip where it belongs and fill with relevant code
    props: Box<OfxPropertySet>,
}

pub type OfxImageClipHandle = *mut libc::c_void;

impl OfxImageClip {
    pub fn new() -> Self {
        OfxImageClip { props: OfxPropertySet::new() }
    }
}


// OfxImageEffectSuite function types here for clarity
pub type GetParamSetType = extern "C" fn(OfxImageEffectHandle, *mut OfxParamSetHandle) -> OfxStatus;
pub type ClipDefineType = extern "C" fn(OfxImageEffectHandle,
                                        *const libc::c_char,
                                        *mut OfxPropertySetHandle)
                                        -> OfxStatus;
pub type ClipGetHandleType = extern "C" fn(OfxImageEffectHandle,
                                           *const libc::c_char,
                                           *mut OfxImageClipHandle,
                                           *mut OfxPropertySetHandle)
                                           -> OfxStatus;
pub type ClipGetPropertySetType = extern "C" fn(OfxImageClipHandle, *mut OfxPropertySetHandle)
                                                -> OfxStatus;
pub type ClipGetImageType = extern "C" fn(OfxImageClipHandle,
                                          OfxTime,
                                          *const OfxRectD,
                                          *mut OfxPropertySetHandle)
                                          -> OfxStatus;
pub type ClipReleaseImageType = extern "C" fn(OfxPropertySetHandle) -> OfxStatus;
pub type ClipGetRegionOfDefinitionType = extern "C" fn(OfxImageClipHandle, OfxTime, *mut OfxRectD)
                                                       -> OfxStatus;
pub type AbortType = extern "C" fn(OfxImageEffectHandle) -> i32;
pub type ImageMemoryAllocType = extern "C" fn(OfxImageEffectHandle,
                                              libc::size_t,
                                              *mut OfxImageMemoryHandle)
                                              -> OfxStatus;
pub type ImageMemoryFreeType = extern "C" fn(OfxImageMemoryHandle) -> OfxStatus;
pub type ImageMemoryLockType = extern "C" fn(OfxImageMemoryHandle, *mut *mut libc::c_void)
                                             -> OfxStatus;
pub type ImageMemoryUnlockType = extern "C" fn(OfxImageMemoryHandle) -> OfxStatus;

extern "C" fn get_property_set(image_effect_ptr: OfxImageEffectHandle,
                               prop_handle: *mut OfxPropertySetHandle)
                               -> OfxStatus {
    if !image_effect_ptr.is_null() {
        let image_effect: &mut OfxImageEffectStruct = unsafe { mem::transmute(image_effect_ptr) };
        unsafe { *prop_handle = image_effect.props as *mut libc::c_void };
        unsafe {
            trace!("getPropertySet setting props {:?}",
                   *prop_handle as *const _)
        };
        kOfxStatOK
    } else {
        kOfxStatErrBadHandle
    }
}

extern "C" fn get_param_set(image_effect_ptr: OfxImageEffectHandle,
                            params: *mut OfxParamSetHandle)
                            -> OfxStatus {
    if !image_effect_ptr.is_null() && !params.is_null() {
        let image_effect: &OfxImageEffectStruct = unsafe { mem::transmute(image_effect_ptr) };
        unsafe { *params = mem::transmute(image_effect.params) };
        unsafe { trace!("getParameterSet {:?}", *params as *const _) };
        return kOfxStatOK;
    }
    kOfxStatErrBadHandle
}

/// This function is used by a plugin to define a clip to a host,
/// the returned property set is used to describe various aspects of the clip to the host.
/// Note that this does not create a clip instance.
/// Arguments
///
/// handle - ImageEffect
/// name - unique name of the clip to define
/// propertySet - a property handle for the clip descriptor will be returned here
/// the property handle returned by this function is purely for definition purposes only

extern "C" fn clip_define(handle: OfxImageEffectHandle,
                          name: *const libc::c_char,
                          props: *mut OfxPropertySetHandle)
                          -> OfxStatus {
    // We need to store a property per clip names per ImageEffectHandle
    if handle.is_null() {
        panic!("null image effect handle passed in clipDefine function");
    }
    let image_effect: &mut OfxImageEffectStruct = unsafe { mem::transmute(handle) };

    // TODO: check if name is valid, and is not a null pointer
    let key: CString = unsafe { CStr::from_ptr(name).to_owned() };
    let mut clip = OfxImageClip::new();
    unsafe { *props = mem::transmute(clip.props.deref_mut()) };
    unsafe {trace!("defining clip {:?} in {:?}", key, image_effect as * const _)};
    image_effect.clips.insert(key, Box::new(clip));
    kOfxStatOK
}

/// Get the propery handle of the named input clip in the given instance
/// Arguments
/// imageEffect - an instance handle to the plugin
/// name - name of the clip, previously used in a clip define call
/// clip - where to return the clip
/// propertySet - if not null, the descriptor handle for a parameter's property set will be placed here.
///
/// The propertySet will have the same value as would be returned by OfxImageEffectSuiteV1::clipGetPropertySet
///
/// This return a clip handle for the given instance, note that this will m not be the same as the clip handle returned by clipDefine and will be distanct to clip handles in any other instance of the plugin.
///
extern "C" fn clip_get_handle(handle: OfxImageEffectHandle,
                              name: *const libc::c_char,
                              clip_handle: *mut OfxImageClipHandle,
                              props: *mut OfxPropertySetHandle)
                              -> OfxStatus {
    if handle.is_null() {
        panic!("null image effect handle passed in clipDefine function");
    }
    let image_effect: &mut OfxImageEffectStruct = unsafe { mem::transmute(handle) };

    // TODO: check if name is valid, and is not a null pointer
    let key: CString = unsafe { CStr::from_ptr(name).to_owned() };
    let clip_found = image_effect.clips.get_mut(&key);
    // Get
    match clip_found {
        Some(mut clip) => {
            trace!("clip found");
            if !props.is_null() {
                unsafe { *props = mem::transmute(clip.props.deref_mut()) };
            };

            if !clip_handle.is_null() {
                // TODO : return a new clip handle associated to Image effect ?
                unsafe { *clip_handle = mem::transmute(clip.deref_mut()) };
            };
            kOfxStatOK
        }
        None => {
            let key: CString = unsafe { CStr::from_ptr(name).to_owned() };
            unsafe{trace!("clip {:?} not found in {:?}", key, handle as * const _ )};
            panic!("unable to find clip");
            kOfxStatErrBadHandle
        }
    }
}

extern "C" fn clip_get_property_set(handle: OfxImageClipHandle,
                                    props: *mut OfxPropertySetHandle)
                                    -> OfxStatus {
    error!("WILL PANIC: clip_get_property_set not implemented");
    panic!("unimplemented")
}

extern "C" fn clip_get_image(handle: OfxImageClipHandle,
                             time: OfxTime,
                             region: *const OfxRectD,
                             props: *mut OfxPropertySetHandle)
                             -> OfxStatus {
    error!("WILL PANIC: clip_get_image not implemented");
    panic!("unimplemented")
}
extern "C" fn clip_release_image(handle: OfxPropertySetHandle) -> OfxStatus {
    error!("WILL PANIC: clip_release_image not implemented");
    panic!("unimplemented")
}
extern "C" fn clip_get_region_of_definition(handle: OfxImageClipHandle,
                                            time: OfxTime,
                                            rod: *mut OfxRectD)
                                            -> OfxStatus {
    error!("WILL PANIC: clip_get_region_of_definition not implemented");
    panic!("unimplemented")
}
extern "C" fn abort(handle: OfxImageEffectHandle) -> i32 {
    error!("WILL PANIC: abort not implemented");
    panic!("unimplemented")
}
extern "C" fn image_memory_alloc(handle: OfxImageEffectHandle,
                                 size: libc::size_t,
                                 mem: *mut OfxImageMemoryHandle)
                                 -> OfxStatus {
    panic!("unimplemented")
}
extern "C" fn image_memory_free(handle: OfxImageMemoryHandle) -> OfxStatus {
    panic!("unimplemented")
}
extern "C" fn image_memory_lock(handle: OfxImageMemoryHandle,
                                lock: *mut *mut libc::c_void)
                                -> OfxStatus {
    panic!("unimplemented")
}
extern "C" fn image_memory_unlock(handle: OfxImageMemoryHandle) -> OfxStatus {
    panic!("unimplemented")
}
// pub type ClipGetPropertySetTypeI = typeof(clip_get_property_set);

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxImageEffectSuiteV1 {
    // Parameters and properties
    getPropertySet: extern "C" fn(OfxImageEffectHandle, *mut OfxPropertySetHandle) -> OfxStatus,
    getParamSet: GetParamSetType,
    // Clips
    clipDefine: ClipDefineType,
    clipGetHandle: ClipGetHandleType,
    clipGetPropertySet: ClipGetPropertySetType,
    clipGetImage: ClipGetImageType,
    clipReleaseImage: ClipReleaseImageType,
    clipGetRegionOfDefinition: ClipGetRegionOfDefinitionType,
    // Running
    abort: AbortType,
    // Image Memory
    imageMemoryAlloc: ImageMemoryAllocType,
    imageMemoryFree: ImageMemoryFreeType,
    imageMemoryLock: ImageMemoryLockType,
    imageMemoryUnlock: ImageMemoryUnlockType,
}


pub static OFX_IMAGE_EFFECT_SUITE_V1: OfxImageEffectSuiteV1 = OfxImageEffectSuiteV1 {
    getPropertySet: get_property_set,
    getParamSet: get_param_set,
    clipDefine: clip_define,
    clipGetHandle: clip_get_handle,
    clipGetPropertySet: clip_get_property_set,
    clipGetImage: clip_get_image,
    clipReleaseImage: clip_release_image,
    clipGetRegionOfDefinition: clip_get_region_of_definition,
    abort: abort,
    imageMemoryAlloc: image_memory_alloc,
    imageMemoryFree: image_memory_free,
    imageMemoryLock: image_memory_lock,
    imageMemoryUnlock: image_memory_unlock,
};
