extern crate libc;
use std::ffi::*;
use libc::*;
use suites::core::*;
use rfx::propertyset::*;
use std::ptr;

/// Rust <-> C OfxPlugin suites
///
#[repr(C)]
#[derive(Clone, Debug)]
#[allow(non_snake_case)]
pub struct OfxPlugin {
    /// Defines the type of the plug-in, this will tell the host what the plug-in
    /// does. e.g.: an image effects plug-in would be a "OfxImageEffectPlugin"
    pub pluginApi: *const c_char,

    /// Defines the version of the pluginApi that this plug-in implements
    pub apiVersion: c_int,

    /// String that uniquely labels the plug-in among all plug-ins that implement an API.
    ///    It need not necessarily be human sensible, however the preference is to use reverse
    ///    internet domain name of the developer, followed by a '.' then by a name that represents
    ///    the plug-in.. It must be a legal ASCII string and have no whitespace in the
    ///    name and no non printing chars.
    ///    For example "uk.co.somesoftwarehouse.myPlugin"
    pub pluginIdentifier: *const c_char,

    /// Major version of this plug-in, this gets incremented when
    /// backwards compatibility is broken.
    pub pluginVersionMajor: c_uint,

    /// Minor version of this plug-in, this gets incremented when software
    /// is changed, but does not break backwards compatibility.
    pub pluginVersionMinor: c_uint,

    /// Function the host uses to connect the plug-in to the host's api fetcher
    pub setHost: extern "C" fn(*mut c_void) -> c_void,

    /// Main entry point for plug-ins
    pub mainEntry: extern "C" fn(*const c_char, *const c_void, *mut c_void, *mut c_void)
                                     -> OfxStatus,
}

///
/// OfxPlugin related functions
///
impl OfxPlugin {
    pub fn identifier(&self) -> String {
        let ptr_wrap = unsafe { CStr::from_ptr(self.pluginIdentifier) };
        ptr_wrap.to_str().unwrap().to_string()
    }

    // kOfxActionLoad is the first action passed to a plug-in after the binary containing the
    // plug-in has been loaded. It is there to allow a plug-in to create any global data structures
    // it may need and is also when the plug-in should fetch suites from the host.
    // This action will not be called again while the binary containing the plug-in remains loaded.
    pub fn action_load(&self) -> OfxStatus {
        trace!("action load");
        (self.mainEntry)(keyword_ptr(kOfxActionLoad),
                         ptr::null_mut(),
                         ptr::null_mut(),
                         ptr::null_mut())
    }

    // kOfxActionUnload is the last action passed to the plug-in before the binary containing the
    // plug-in is unloaded. It is there to allow a plug-in to destroy any global data structures it
    // may have created.
    pub fn action_unload(&self) -> OfxStatus {
        trace!("action unload");
        (self.mainEntry)(keyword_ptr(kOfxActionUnload),
                         ptr::null_mut(),
                         ptr::null_mut(),
                         ptr::null_mut())
    }
    // The kOfxActionDescribe is the second action passed to a plug-in. It is where a plugin defines
    // how it behaves and the resources it needs to function.
    // Note that the handle passed in acts as a descriptor for, rather than an instance of the
    // plugin. The handle is global and unique. The plug-in is at liberty to cache the handle away
    // for future reference until the plug-in is unloaded.
    pub fn action_describe(&mut self, plug_desc_ptr : *const c_void) -> OfxStatus {
        trace!("plugin descriptor is {:?}", plug_desc_ptr as *const _);
        (self.mainEntry)(keyword_ptr(kOfxActionDescribe),
                         plug_desc_ptr, // check plug_desc_ptr is not needed after this call
                         ptr::null_mut(),
                         ptr::null_mut())
    }

    // This action is unique to OFX Image Effect plug-ins. Because a plugin is able to exhibit
    // different behaviour depending on the context of use, each separate context will need to be
    // described individually. It is within this action that image effects describe which
    // parameters and input clips it requires.
    //
    // This action will be called multiple times, one for each of the contexts the plugin says it
    // is capable of implementing. If a host does not support a certain context, then it need not
    // call kOfxImageEffectActionDescribeInContext for that context.
    pub fn action_describe_in_context(&mut self, plug_desc_ptr : *const c_void) -> OfxStatus {
        trace!("describe in context with image effect {:?}",
               plug_desc_ptr as *const _);

        // Set the context for the plugin
        let mut prop_set = OfxPropertySet::new();
        prop_set.insert(clone_keyword(kOfxImageEffectPropContext),
                        0,
                        keyword_ptr(kOfxImageEffectContextGeneral));
        // TODO check the plugin is not keeping this property set as it will
        // surely be detroyed after this function returns

        (self.mainEntry)(keyword_ptr(kOfxImageEffectActionDescribeInContext),
                         plug_desc_ptr,
                         properties_ptr(prop_set),
                         ptr::null_mut())
    }

    // The kOfxActionCreateInstance is the first action passed to a plug-in's instance after its
    // creation. It is there to allow a plugin to create any per-instance data structures it may
    // need.
    // kOfxActionDescribe has been called
    // The instance is fully constructed, with all objects requested in the describe actions (eg,
    // parameters and clips) have been constructed and have had their initial values set. This
    // means that if the values are being loaded from an old setup, that load should have taken
    // place before the create instance action is called.
    pub fn action_create_instance(&mut self, plug_desc_ptr : *const c_void ) -> OfxStatus {
        trace!("action_create_instance called {:?}", plug_desc_ptr);
        (self.mainEntry)(keyword_ptr(kOfxActionCreateInstance), // create_instance_str.as_ptr(),
                               plug_desc_ptr,
                               ptr::null_mut(),
                               ptr::null_mut()) 
    }
}

// #[test]
// fn test_clone() {
//
//    OfxPlugin {
//        pluginApi: "test_api",
//        apiVersion: 1,
//        pluginIdentifier: "test_clone_plugin",
//    }
// }
