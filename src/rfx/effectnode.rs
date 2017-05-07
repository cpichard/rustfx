use libc;
use rfx::propertyset::*;
use rfx::imageclip::*;
use rfx::paramset::*;
use std::collections::HashMap;
use std::mem;
use std::ffi::CString;
use std::ops::DerefMut;
use suites::core::*;

/// EffectNode contains all the data needed by an image effect.
/// it is directly bound to the OFX api
#[derive(Clone, Debug)]
pub struct EffectNode {
    props: Box<OfxPropertySet>,
    params: Box<OfxParameterSet>,
    pub clips: HashMap<CString, Box<OfxImageClip>>,
}

impl EffectNode {
    /// Creates a new effect node
    pub fn new() -> Self {
        EffectNode {
            props: OfxPropertySet::new(),
            params: OfxParameterSet::new(),
            clips: HashMap::new(),
        }
    }

    /// Returns a pointer to the property container
    /// It's used in the C plugin code
    pub unsafe fn properties_handle(&self) -> *mut libc::c_void {
        mem::transmute(self.props.as_ref())
    }

    /// Returns a pointer to the parameter container
    /// It's used in the C plugin code
    pub unsafe fn parameter_handle(&self) -> *mut libc::c_void {
        mem::transmute(self.params.as_ref())
    }

    // This returns the pointer on the clip props
    // TODO: this should go in another function
    pub unsafe fn clip_new(&mut self, key: CString) -> *mut libc::c_void {
        let clip = OfxImageClip::new();
        self.clips.insert(key.clone(), Box::new(clip));
        // TODO: it doesn't look very efficient to query the map here
        match self.clips.get_mut(&key) {
            Some(clip) => mem::transmute(clip.props.deref_mut()),
            None => panic!("unable to create clip"),
        }
    }

    pub fn parameters(&mut self) -> &mut OfxParameterSet {
        self.params.deref_mut()
    }

    pub fn properties(&mut self) -> &mut OfxPropertySet {
        self.props.deref_mut()
    }

    /// Default properties of an effect node
    pub fn set_default_properties(&mut self) {
        self.props.insert(kOfxPropLabel, 0, "");
        self.props.insert(kOfxPropShortLabel, 0, "");
        self.props.insert(kOfxPropLongLabel, 0, "");
        self.props.insert(kOfxImageEffectPluginPropGrouping, 0, "");
        self.props.insert(kOfxPluginPropFilePath, 0, "");
        self.props.insert(kOfxPropType, 0, kOfxTypeImageEffect);
        self.props.insert(kOfxImageEffectPluginRenderThreadSafety,
                          0,
                          kOfxImageEffectRenderFullySafe);
        self.props.insert(kOfxImageEffectPluginPropSingleInstance, 0, 0);
        self.props.insert(kOfxImageEffectPluginPropHostFrameThreading, 0, 0);
        self.props.insert(kOfxImageEffectPropSupportsMultiResolution, 0, 1);
        self.props.insert(kOfxImageEffectInstancePropSequentialRender, 0, 0);
        self.props.insert(kOfxImageEffectPropSupportsTiles, 0, 1);
        self.props.insert(kOfxImageEffectPropTemporalClipAccess, 0, 0);
        self.props.insert(kOfxImageEffectPluginPropFieldRenderTwiceAlways, 0, 1);
        self.props.insert(kOfxImageEffectPropMultipleClipDepths, 0, 0);
        self.props.insert(kOfxImageEffectPropSupportsMultipleClipPARs, 0, 0);
        self.props.insert(kOfxImageEffectPluginPropOverlayInteractV1,
                          0,
                          0 as *const libc::c_void);
        self.props.insert(kOfxImageEffectPropSupportedContexts, 0, "");
        self.props.insert(kOfxImageEffectPropSupportedPixelDepths, 0, "");
        self.props.insert(kOfxImageEffectPropClipPreferencesSlaveParam, 0, "");
    }
}
