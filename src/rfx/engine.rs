
use libc;
use ofx::core::*;
use ofx::plugin::*;
//use ofx::property::*;
use rfx::propertyset::*;
use ofx::imageeffect::*;
use rfx::bundle::Bundle;
use std::collections::HashMap;
use std::mem::transmute;
use std::ffi::*;
use std::ptr;


pub struct Engine {
    ofx_host: * mut OfxHost,
    bundles: Vec<Bundle>,
    plugins: HashMap<String, * const OfxPlugin>,
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
    pub fn load_plugins(& mut self, bundles : Vec<Bundle>) {
        trace!("ofx host passed to the plugin {:?}", self.ofx_host as * const _);
        unsafe {trace!("ofx host properties passed to the plugin {:?}", (*self.ofx_host).host as * const _)};
        for bundle in &bundles {
            for n in 0..bundle.nb_plugins {
                let plugin = bundle.get_plugin(n);
                // TODO: test for plugin version compatibility 
                // Register the host in the plugin
                let host_ptr : * mut libc::c_void = unsafe {transmute(self.ofx_host)};
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

    fn action_load(& mut self, plugin: &OfxPlugin) -> OfxStatus {
        let ofx_action_load = CString::new("OfxActionLoad").unwrap();
        (plugin.mainEntry)(ofx_action_load.as_ptr(), 
                                ptr::null_mut(), 
                                ptr::null_mut(), 
                                ptr::null_mut())
    }

    fn action_describe(& mut self, plugin: &OfxPlugin) -> OfxStatus {
        let ofx_action_describe = CString::new("OfxActionDescribe").unwrap();
        // TODO check the plugin is not keeping this property set
        let image_effect = OfxImageEffectStruct::new();
        let plug_desc_ptr : * const libc::c_void = unsafe{transmute(& image_effect)};
        trace!("plugin description is {:?}", plug_desc_ptr as * const _);
        (plugin.mainEntry)(ofx_action_describe.as_ptr(), 
                                plug_desc_ptr,
                                ptr::null_mut(), 
                                ptr::null_mut())
    }

    pub fn node(& mut self, plugin_name: &str) {
        match self.plugins.get(plugin_name) {
            Some(plugin) => {
                debug!("found plugin {:?}\n", plugin);
            },
            None => {debug!("plugin {:?} not found", plugin_name)},
        }
    }

    fn describe_capabilities() -> Box<OfxPropertySet> {
        let mut properties = OfxPropertySet::new();
        properties.insert("OfxImageEffectPropMultipleClipDepths", 0);
        properties
    }
}

//#[cfg(test)]
//use rfx::propertyset::*;

#[test]
fn check_properties() {
    let engine = Engine::new();    
    let ofx_str = CString::new("OfxImageEffectPropMultipleClipDepths").unwrap();
    let property_set : * mut OfxPropertySet = unsafe{transmute((*engine.ofx_host).host)};
    let result = unsafe {(*property_set).get(&ofx_str)};
    assert_eq!(result, Some(&PropertyValue::from(0)))
}
