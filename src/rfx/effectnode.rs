use libc;
use rfx::propertyset::*;
use rfx::imageclip::*;
use rfx::paramset::*;
use std::collections::HashMap;
use std::mem;
use std::ffi::CString;
use std::ops::DerefMut;
use suites::core::*;

pub type ClipHandle = CString;
pub type EndPoint = (NodeHandle, ClipHandle);
pub type Node = EffectNode;
pub type NodeHandle = String;
pub type NodeId = NodeHandle;
///
/// Returns a new id for the node, it basically postfix the name
/// With a number or increase the number if there is already one
/// ex: "Gain.2" == next_id("Gain.1");
///     "Blur.1" == next_id("Blur");
pub fn next_id(id: &NodeHandle) -> NodeHandle {
    let mut prefix = id.clone();
    let mut number_str = 1.to_string();

    let split_string: Vec<&str> = id.rsplitn(2, '.').collect();

    if split_string.len() == 2 {
        // Found a '.'
        // try to convert
        if let Ok(mut number) = split_string[0].parse::<u32>() {
            number += 1;
            number_str = number.to_string();
            prefix = split_string[1].to_string();
        }
    }

    prefix + "." + &number_str
}

/// EffectNode contains all the data needed by an image effect.
/// it is directly bound to the OFX api
#[derive(Clone, Debug)]
pub struct EffectNode {
    // OFX
    props: Box<OfxPropertySet>,
    params: Box<OfxParameterSet>,

    // This contains the list of inputs and outputs
    pub clips: HashMap<ClipHandle, Box<OfxImageClip>>,
    
    // Connections
    // TODO: store the list of input and output available
    //       They might be connected or not
    //pub incoming: Vec<EndPoint>, // [(node src, in clip)]
    //pub outgoing: Vec<EndPoint>, // [(node dst, node dst clip)]
}

impl EffectNode {
    /// Creates a new effect node
    pub fn new() -> Self {
        EffectNode {
            props: OfxPropertySet::new(),
            params: OfxParameterSet::new(),
            clips: HashMap::new(),
            //incoming: Vec::new(),
            //outgoing: Vec::new(),
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

    // This returns the pointer on the clip props,
    // intented for the suite
    // TODO: this should go in another function
    pub unsafe fn clip_new(&mut self, key: CString) -> *mut libc::c_void {
        let clip = OfxImageClip::new();
        self.clips.insert(key.clone(), Box::new(clip));
        match self.clips.get_mut(&key) {
            Some(clip) => mem::transmute(clip.props.deref_mut()),
            None => panic!("unable to create clip"),
        }
    }

    pub fn clip_get(&self, clip_id: &ClipHandle) -> Option<&OfxImageClip> {
        if let Some(clip) = self.clips.get(clip_id) {
            Some(clip.as_ref())
        } else {
            None
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
        self.props.insert(kOfxImageEffectPropSupportsMultipleClipDepths, 0, 0);
        self.props.insert(kOfxImageEffectPropSupportsMultipleClipPARs, 0, 0);
        self.props.insert(kOfxImageEffectPluginPropOverlayInteractV1,
                          0,
                          0 as *const libc::c_void);
        self.props.insert(kOfxImageEffectPropSupportedContexts, 0, "");
        self.props.insert(kOfxImageEffectPropSupportedPixelDepths, 0, "");
        self.props.insert(kOfxImageEffectPropClipPreferencesSlaveParam, 0, "");
    }
}


#[test]
fn test_next_id() {
    let mut id1 = "Gain.1".to_string();
    id1 = next_id(&mut id1);
    assert!(id1 == "Gain.2".to_string());

    let mut id2 = "Gain".to_string();
    id2 = next_id(&mut id2);
    assert!(id2 == "Gain.1".to_string());

    let mut id3 = "".to_string();
    id3 = next_id(&mut id3);
    assert!(id3 == ".1".to_string());

    let mut id4 = "Gain.toto".to_string();
    id4 = next_id(&mut id4);
    assert!(id4 == "Gain.toto.1".to_string());
}

#[test]
fn create_node() {

}
