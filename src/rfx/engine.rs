
use libc::*;
use suites::core::*;
use suites::plugin::*;
use rfx::propertyset::*;
use rfx::effectnode::*;
use rfx::bundle::Bundle;
use std::collections::HashMap;
use std::path::PathBuf;
use std::mem::transmute;

/// Engine contains everything needed to launch a computation
/// NOTE: this is more a Host than an Engine
pub struct Engine {
    ofx_host: *mut OfxHost,
    bundles: Vec<Bundle>,
    plugins: HashMap<String, (OfxPlugin, EffectNode)>,
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

    pub fn plugins_list(&self) {
        println!("{:?}", self.bundles);
        println!("{:?}", self.plugins);
    }

    // TODO: load_plugin should be update_plugin instead
    #[allow(non_upper_case_globals)]
    pub fn plugins_load(&mut self, paths: Vec<PathBuf>) {
        unsafe {
            trace!("ofx host passed to the plugin {:?}",
                   self.ofx_host as *const _);
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
                trace!("plugin.action_load()");
                match plugin.action_load() {
                    kOfxStatOK | kOfxStatReplyDefault => {
                        trace!("action_load returned kOfxStatOk or kOfxStatReplyDefault");
                        let plugin_desc = EffectNode::new();
                        let plug_desc_ptr: *const c_void = unsafe { transmute(&plugin_desc) };
                        // todo : store information returned by the plugin, like clips and
                        // parameters. Also store image_effect to avoid having it destroyed 
                        // 
                        //
                        match plugin.action_describe(plug_desc_ptr) {
                            kOfxStatOK => {
                                let identifier = plugin.identifier();
                                trace!("inserting new plugin identifier {}", identifier);
                                self.plugins.insert(identifier, (plugin, plugin_desc));
                            }
                            // TODO handle action describe return
                            _ => error!("plugin can't describe itself"),
                        }
                    }
                    kOfxStatFailed => error!("load plugin returned kOfxStatFailed "),
                    kOfxStatErrFatal => panic!("plugins raised a fatal error"),
                    _ => error!("load action returned unhandled error code"),
                }
            }
        }
    }


    #[allow(non_upper_case_globals)]
    pub fn image_effect(&mut self, plugin_name: &str) -> Option<EffectNode> {
        trace!("image_effect creating EffectNode");
        let found = match self.plugins.get_mut(plugin_name) {
            Some( plugin ) => {
                debug!("image_effect found plugin {:?}\n", plugin.0);
                Some(plugin)
            }
            None => {
                debug!("image_effect plugin {:?} not found", plugin_name);
                None
            }
        };
        match found {
            Some(plugin_info) => {
                // Extract plugin struct and parameters/clips from tuple
                let ref mut plugin = plugin_info.0;
                let ref mut description = plugin_info.1;
                // Clone parameters to a new image effect, this is now 
                // an "instance"
                let mut image_effect = description.clone();
                // This plugin will be used in a general context
                image_effect.properties().insert(clone_keyword(kOfxImageEffectPropContext), 0, keyword_ptr(kOfxImageEffectContextGeneral));
                let instance_ptr: *const c_void = unsafe { transmute(&image_effect) };
                trace!("about to call plugin.action_describe_in_context");
                // TODO: does action describe in context need an instance or an effect
                match plugin.action_describe_in_context(instance_ptr) {
                    // TODO : return relevant information for each code path
                    kOfxStatOK => {
                        trace!("describe in context suceeded, able to create a new image effect");
                        // TODO handle status returned from create instance
                        plugin.action_create_instance(instance_ptr);
                        Some(image_effect)
                    }
                    // in which the context will be ignored by the host, the plugin may post a message
                    kOfxStatErrMissingHostFeature => {
                        error!("image effect require a feature not implemented");
                        None
                    }
                    kOfxStatErrMemory => {
                        error!("describe in context returned a memory error"); //in which case the action may be called again after a memory purge
                        None
                    }
                    // something wrong, but no error code appropriate, plugin to post message
                    kOfxStatFailed => {
                        error!("something went wrong in describe in context");
                        None
                    }
                    kOfxStatErrFatal | _ => {
                        error!("describe_in_context returned a fatal error");
                        None
                    }
                }
            }
            _ => None,
        }
    }

    fn describe_capabilities() -> Box<OfxPropertySet> {
        // TODO : add rustfx capabilities 
        // the doc list all the capabilities that must be registered
        // for the moment nothing is implemented,
        let mut properties = OfxPropertySet::new();
        properties.insert("OfxImageEffectPropMultipleClipDepths", 0, 0);
        // TODO: remove overlay and CustomAnimation capability
        // We don't support overlay, it's just for testing the Custom plugin 
        properties.insert("OfxImageEffectPropSupportsOverlays", 0, 1);
        properties.insert("OfxParamHostPropSupportsCustomAnimation", 0, 1);
        properties.insert(clone_keyword(kOfxImageEffectPropContext), 
            0, keyword_ptr(kOfxImageEffectContextGeneral));
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
