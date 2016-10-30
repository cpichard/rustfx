
use libc::*;
use bindings::core::*;
use bindings::plugin::*;
use rfx::propertyset::*;
use bindings::imageeffect::*;
use rfx::bundle::Bundle;
use std::collections::HashMap;
use std::path::PathBuf;
use std::mem::transmute;
use std::ptr;
use std::ffi::*;

/// Engine contains everything needed to launch a computation
pub struct Engine {
    ofx_host: *mut OfxHost,
    bundles: Vec<Bundle>,
    plugins: HashMap<String, *mut OfxPlugin>,
}

impl Engine {
    pub fn new() -> Engine {
        trace!("creating host property set");
        let host_properties = Engine::describe_capabilities();
        let engine = Engine {
            ofx_host: Box::into_raw(OfxHost::new(host_properties)),
            bundles: Vec::new(),
            plugins: HashMap::new(),
        };
        engine
    }

    #[allow(non_upper_case_globals)]
    pub fn load_plugins(&mut self, paths: Vec<PathBuf>) {
        trace!("ofx host passed to the plugin {:?}",
               self.ofx_host as *const _);
        unsafe {
            trace!("ofx host properties passed to the plugin {:?}",
                   (*self.ofx_host).host as *const _)
        };
        let bundles = Bundle::from_paths(paths);
        for bundle in &bundles {
            for n in 0..bundle.nb_plugins {
                let plugin = bundle.get_plugin(n);
                // TODO: test for plugin version compatibility
                // Register the host in the plugin
                let host_ptr: *mut c_void = unsafe { transmute(self.ofx_host) };
                (plugin.setHost)(host_ptr);
                match self.action_load(plugin) {
                    kOfxStatOK => {
                        match self.action_describe(plugin) {
                            kOfxStatOK => {
                                self.plugins.insert(plugin.identifier(), plugin);
                            }
                            // TODO handle action describe return
                            _ => error!("plugin can't describe itself"),
                        }
                    }
                    kOfxStatReplyDefault => println!("load plugin returned kOfxStatReplyDefault "),
                    kOfxStatFailed => println!("load plugin returned kOfxStatFailed "),
                    kOfxStatErrFatal => panic!("plugins raised a fatal error"),
                    _ => error!("load action returned unhandled error code"),
                }
            }
        }
        self.bundles = bundles;
    }

    fn action_load(&mut self, plugin: &OfxPlugin) -> OfxStatus {
        (plugin.mainEntry)(keyword_ptr(kOfxActionLoad),
                           ptr::null_mut(),
                           ptr::null_mut(),
                           ptr::null_mut())
    }

    ///
    fn action_describe(&mut self, plugin: &OfxPlugin) -> OfxStatus {
        // TODO check the plugin is not keeping this property set
        let image_effect = OfxImageEffectStruct::new();
        let plug_desc_ptr: *const c_void = unsafe { transmute(&image_effect) };
        trace!("plugin description is {:?}", plug_desc_ptr as *const _);
        (plugin.mainEntry)(keyword_ptr(kOfxActionDescribe),
                           plug_desc_ptr,
                           ptr::null_mut(),
                           ptr::null_mut())
    }

    fn action_describe_in_context(&mut self, plugin: &OfxPlugin) -> OfxStatus {
        let image_effect = OfxImageEffectStruct::new(); // This is the context cast as a OfxImageEffectStruct
        let plug_desc_ptr: *const c_void = unsafe { transmute(&image_effect) };
        trace!("describe in context with image effect {:?}", plug_desc_ptr as *const _);

        // TODO check the plugin is not keeping this property set
        let mut prop_set = OfxPropertySet::new();
        prop_set.insert(clone_keyword(kOfxImageEffectPropContext),
                        0,
                        keyword_ptr(kOfxImageEffectContextGeneral));

        (plugin.mainEntry)(keyword_ptr(kOfxImageEffectActionDescribeInContext),
                           plug_desc_ptr,
                           properties_ptr(prop_set),
                           ptr::null_mut())
    }

    #[allow(non_upper_case_globals)]
    fn action_create_instance(&mut self, plugin: &mut OfxPlugin) -> Option<OfxImageEffectStruct> {
        // let create_instance_str = CString::new("OfxActionCreateInstance").unwrap();
        let image_effect = OfxImageEffectStruct::new();
        let plug_desc_ptr: *const c_void = unsafe { transmute(&image_effect) };
        trace!("create instance with image effect {:?}", plug_desc_ptr);

        match (plugin.mainEntry)(keyword_ptr(kOfxActionCreateInstance), // create_instance_str.as_ptr(),
                                 plug_desc_ptr,
                                 ptr::null_mut(),
                                 ptr::null_mut()) {
            kOfxStatOK => Some(image_effect),
            // TODO catch and handle other returned values
            _ => None,
        }
    }
    #[allow(non_upper_case_globals)]
    pub fn node(&mut self, plugin_name: &str) {
        let found = match self.plugins.get(plugin_name) {
            Some(plugin) => {
                debug!("found plugin {:?}\n", plugin);
                Some(*plugin)
            }
            None => {
                debug!("plugin {:?} not found", plugin_name);
                None
            }
        };
        match found {
            Some(k) => {
                match self.action_describe_in_context(unsafe { transmute(k) }) {
                    kOfxStatOK => trace!("ok"),//the action was trapped and all was well
                    kOfxStatErrMissingHostFeature => trace!("context ignored"),// in which the context will be ignored by the host, the plugin may post a message
                    kOfxStatErrMemory => trace!("memory error"), //in which case the action may be called again after a memory purge
                    kOfxStatFailed => trace!("something went wrong"),//something wrong, but no error code appropriate, plugin to post message
                    kOfxStatErrFatal | _ => panic!(""),    
                }
                self.action_create_instance(unsafe { transmute(k) });
            }
            _ => (),
        }
    }

    fn describe_capabilities() -> Box<OfxPropertySet> {
        let mut properties = OfxPropertySet::new();
        properties.insert("OfxImageEffectPropMultipleClipDepths", 0, 0);
        properties
    }
}

// #[cfg(test)]
// use rfx::propertyset::*;

#[test]
fn check_properties() {
    let engine = Engine::new();
    let ofx_str = CString::new("OfxImageEffectPropMultipleClipDepths").unwrap();
    let property_set: *mut OfxPropertySet = unsafe { transmute((*engine.ofx_host).host) };
    let result = unsafe { (*property_set).get(&ofx_str, 0) };
    assert_eq!(result, Some(&PropertyValue::from(0)))
}
