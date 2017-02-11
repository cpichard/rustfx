use libc;
use rfx::propertyset::*;
use rfx::imageclip::*;
use rfx::paramset::*;
use std::collections::HashMap;
use std::mem;
use std::ffi::{CString, CStr};
use std::ops::DerefMut;

/// EffectNode contains all the data needed by an image effect.
/// it is directly bound to the OFX api
#[derive(Clone)]
pub struct EffectNode {
    props: *mut OfxPropertySet,
    params: *mut OfxParameterSet,
    pub clips: HashMap<CString, Box<OfxImageClip>>,
}

impl EffectNode {
    pub fn new() -> Self {
        EffectNode {
            props: Box::into_raw(OfxPropertySet::new()),
            params: Box::into_raw(OfxParameterSet::new()),
            clips: HashMap::new(),
        }
    }
    pub unsafe fn properties_handle(&self) -> *mut libc::c_void {
        self.props as *mut libc::c_void
    }

    pub unsafe fn parameter_handle(&self) -> *mut libc::c_void {
        self.params as *mut libc::c_void
    }

    // This returns the pointer on the clip props
    // TODO: this should go in another function
    pub unsafe fn new_clip(&mut self, key: CString) -> *mut libc::c_void {
        let mut clip = OfxImageClip::new();
        self.clips.insert(key.clone(), Box::new(clip));
        // TODO: it doesn't look very efficient to query the map here
        match self.clips.get_mut(&key) {
            Some(clip) => mem::transmute(clip.props.deref_mut()),
            None => panic!("unable to create clip"),
        }
    }

    pub fn set_value(&mut self, key: & String, value: String) {
        // depending on the key, convert the string representation to a value     
        // TODO self.params.set_value_literal(key, value)
    }
}
