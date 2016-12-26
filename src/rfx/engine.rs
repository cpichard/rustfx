
use libc::*;
use bindings::core::*;
use bindings::plugin::*;
use rfx::propertyset::*;
use rfx::bundle::Bundle;
use bindings::imageeffect::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::mem::transmute;

/// Engine contains everything needed to launch a computation
pub struct Engine {
    ofx_host: *mut OfxHost,
    bundles: Vec<Bundle>,
    plugins: HashMap<String, OfxPlugin>,
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
        self.bundles = Bundle::from_paths(paths);
        for bundle in &self.bundles {
            for n in 0..bundle.nb_plugins {
                let mut plugin = bundle.get_plugin(n);
                // TODO: test for plugin version compatibility
                // Register the host in the plugin
                let host_ptr: *mut c_void = unsafe { transmute(self.ofx_host) };
                (plugin.setHost)(host_ptr);
                match plugin.action_load() {
                    kOfxStatOK => {
                        match plugin.action_describe() {
                            kOfxStatOK => {
                                let identifier = plugin.identifier();
                                self.plugins.insert(identifier, plugin);
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
    }


    #[allow(non_upper_case_globals)]
    pub fn node(&mut self, plugin_name: &str) -> Option<OfxImageEffectStruct> {
        let found = match self.plugins.get_mut(plugin_name) {
            Some(plugin) => {
                debug!("found plugin {:?}\n", plugin);
                Some(plugin)
            }
            None => {
                debug!("plugin {:?} not found", plugin_name);
                None
            }
        };
        match found {
            Some(plugin) => {
                match plugin.action_describe_in_context() {
                    // TODO : return relevant information for each code path
                    kOfxStatOK => {
                        trace!("describe in context suceeded, able to create a new image effect");
                        plugin.action_create_instance()
                    }
                    // in which the context will be ignored by the host, the plugin may post a message
                    kOfxStatErrMissingHostFeature => {
                        trace!("image effect require a feature not");
                        None
                    }
                    kOfxStatErrMemory => {
                        trace!("memory error"); //in which case the action may be called again after a memory purge
                        None
                    }
                    // something wrong, but no error code appropriate, plugin to post message
                    kOfxStatFailed => {
                        trace!("something went wrong in describe in context");
                        None
                    }
                    kOfxStatErrFatal | _ => {
                        panic!("describe_in_context returned a fatal error");
                    }
                }
                // Create a OfxImageEffect which can be used after
                // TODO : return a Node ? create a node and store it ?
            }
            _ => None,
        }
    }

    fn describe_capabilities() -> Box<OfxPropertySet> {
        let mut properties = OfxPropertySet::new();
        // TODO : add rustfx capabilities
        properties.insert("OfxImageEffectPropMultipleClipDepths", 0, 0);
        properties
    }
}

#[cfg(test)]
use std::ffi::CString;

#[test]
fn check_properties() {
    let engine = Engine::new();
    let ofx_str = CString::new("OfxImageEffectPropMultipleClipDepths").unwrap();
    let property_set: *mut OfxPropertySet = unsafe { transmute((*engine.ofx_host).host) };
    let result = unsafe { (*property_set).get(&ofx_str, 0) };
    assert_eq!(result, Some(&PropertyValue::from(0)))
}
