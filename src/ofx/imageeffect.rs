use libc;
use ofx::property::*;
use ofx::param::*;
use ofx::core::*;
use std::mem;
// This structure is private to the module, the pointer is public
pub struct OfxImageEffectStruct {
// TODO stuff for image effect    
    props: * mut OfxPropertySet,    
}
//pub type OfxImageEffectHandle = * mut OfxImageEffectStruct;
pub type OfxImageEffectHandle = * mut libc::c_void;


impl OfxImageEffectStruct {
    
    pub fn new() -> Self {
        OfxImageEffectStruct {
            props: Box::into_raw(OfxPropertySet::new()),
        }
    }
}

pub struct OfxImageMemoryStruct {
   // TODO stuff for image memory 
}
pub type OfxImageMemoryHandle = * mut OfxImageMemoryStruct;


pub struct OfxImageClip {
// TODO move ImageClip where it belongs and fill with relevant code
}
pub type OfxImageClipHandle = * mut OfxImageClip;

// OfxImageEffectSuite function types here for clarity
pub type GetPropertySetType = extern fn (OfxImageEffectHandle, * mut OfxPropertySetHandle) -> OfxStatus;
pub type GetParamSetType = extern fn (OfxImageEffectHandle, * mut OfxParamSetHandle) -> OfxStatus;
pub type ClipDefineType = extern fn (OfxImageEffectHandle, * const libc::c_char, * mut OfxPropertySetHandle ) -> OfxStatus;
pub type ClipGetHandleType = extern fn (OfxImageEffectHandle, 
                                * const libc::c_char,  
                                * mut OfxImageClipHandle, 
                                * mut OfxPropertySetHandle) -> OfxStatus;
pub type ClipGetPropertySetType = extern fn (OfxImageClipHandle, * mut OfxPropertySetHandle) -> OfxStatus;
pub type ClipGetImageType = extern fn (OfxImageClipHandle, OfxTime, * const OfxRectD, * mut OfxPropertySetHandle) -> OfxStatus;
pub type ClipReleaseImageType = extern fn (OfxPropertySetHandle) -> OfxStatus;
pub type ClipGetRegionOfDefinitionType = extern fn (OfxImageClipHandle, OfxTime, * mut OfxRectD) -> OfxStatus;
pub type AbortType = extern fn (OfxImageEffectHandle) -> i32;
pub type ImageMemoryAllocType = extern fn (OfxImageEffectHandle, libc::size_t, * mut OfxImageMemoryHandle) -> OfxStatus;
pub type ImageMemoryFreeType = extern fn (OfxImageMemoryHandle) -> OfxStatus;
pub type ImageMemoryLockType = extern fn (OfxImageMemoryHandle, * mut * mut libc::c_void) -> OfxStatus;
pub type ImageMemoryUnlockType = extern fn (OfxImageMemoryHandle) -> OfxStatus;

// TODO
extern fn get_property_set(image_effect_ptr: OfxImageEffectHandle, prop_handle: * mut OfxPropertySetHandle) -> OfxStatus {
    if !image_effect_ptr.is_null() {
        //use std::ptr;
        let image_effect : & mut OfxImageEffectStruct = unsafe{mem::transmute(image_effect_ptr)};
        unsafe {*prop_handle = image_effect.props as * mut libc::c_void};
        unsafe {trace!("getPropertySet setting props {:?}", *prop_handle as * const _)};
        kOfxStatOK
    } else {
        kOfxStatErrBadHandle
    }
}
extern fn get_param_set(handle: OfxImageEffectHandle, params: * mut OfxParamSetHandle) -> OfxStatus {0}
extern fn clip_define(handle: OfxImageEffectHandle, name:* const libc::c_char, props: * mut OfxPropertySetHandle ) -> OfxStatus {0}
extern fn clip_get_handle(handle: OfxImageEffectHandle, 
                          name: * const libc::c_char,  
                          clip_handle:* mut OfxImageClipHandle, 
                          props: * mut OfxPropertySetHandle) -> OfxStatus {0}
extern fn clip_get_property_set(handle: OfxImageClipHandle, props: * mut OfxPropertySetHandle) -> OfxStatus {0}
extern fn clip_get_image(handle: OfxImageClipHandle, time: OfxTime, region:* const OfxRectD, props: * mut OfxPropertySetHandle) -> OfxStatus {0}
extern fn clip_release_image(handle: OfxPropertySetHandle) -> OfxStatus {0}
extern fn clip_get_region_of_definition(handle: OfxImageClipHandle, time: OfxTime, rod: * mut OfxRectD) -> OfxStatus {0}
extern fn abort(handle: OfxImageEffectHandle) -> i32 {0}
extern fn image_memory_alloc(handle: OfxImageEffectHandle, size: libc::size_t, mem: * mut OfxImageMemoryHandle) -> OfxStatus {0}
extern fn image_memory_free(handle: OfxImageMemoryHandle) -> OfxStatus {0}
extern fn image_memory_lock(handle: OfxImageMemoryHandle, lock: *mut *mut libc::c_void) -> OfxStatus {0}
extern fn image_memory_unlock(handle: OfxImageMemoryHandle) -> OfxStatus {0}
//pub type ClipGetPropertySetTypeI = typeof(clip_get_property_set); 

#[repr(C)]
#[allow(non_snake_case)]
pub struct OfxImageEffectSuiteV1 {
  // Parameters and properties
  getPropertySet: GetPropertySetType,
  getParamSet: GetParamSetType,
  // Clips
  clipDefine: ClipDefineType,
  clipGetHandle: ClipGetHandleType,
  clipGetPropertySet: ClipGetPropertySetType,
  clipGetImage: ClipGetImageType,
  clipReleaseImage: ClipReleaseImageType, 
  clipGetRegionOfDefinition: ClipGetRegionOfDefinitionType,
  abort: AbortType,
  // Image Memory
  imageMemoryAlloc: ImageMemoryAllocType,
  imageMemoryFree: ImageMemoryFreeType,
  imageMemoryLock: ImageMemoryLockType,
  imageMemoryUnlock: ImageMemoryUnlockType,
}


//impl OfxImageEffectSuiteV1 {
//    
//    pub fn new () -> Self {
//        OfxImageEffectSuiteV1 {
//            getPropertySet: get_property_set,    
//            getParamSet: get_param_set,
//            clipDefine: clip_define,
//            clipGetHandle: clip_get_handle,
//            clipGetPropertySet: clip_get_property_set,
//            clipGetImage: clip_get_image,
//            clipReleaseImage: clip_release_image,
//            clipGetRegionOfDefinition: clip_get_region_of_definition,
//            abort: abort,
//            imageMemoryAlloc: image_memory_alloc,
//            imageMemoryFree: image_memory_free,
//            imageMemoryLock: image_memory_lock,
//            imageMemoryUnlock: image_memory_unlock,
//        }    
//    }    
//}

pub static OFX_IMAGE_EFFECT_SUITE_V1 : OfxImageEffectSuiteV1 = OfxImageEffectSuiteV1 {
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
