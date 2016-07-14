
use libc;
use ofx::core::*;
use ofx::plugin::*;
use ofx::property::*;
use bundle::Bundle;
use std::collections::HashMap;
use std::mem::transmute;
use std::ffi::*;
use std::ptr;

pub struct Engine {
    ofx_host: OfxHost,
    bundles: Vec<Bundle>,
    plugins: HashMap<String, * const OfxPlugin>,
    
}

impl Engine {
    pub fn new() -> Engine {
        Engine { 
            ofx_host: OfxHost::new(),
            bundles: Vec::new(),
            plugins: HashMap::new(),
        }
    }
    
    pub fn load_plugins(& mut self, bundles : Vec<Bundle>) {
        self.bundles = bundles;
        for bundle in &self.bundles {
            for n in 0..bundle.nb_plugins {
                let plugin = (bundle.get_plugin)(n);    
                unsafe {
                    // Provide host to the plugin
                    let host_ptr : * mut libc::c_void = transmute(& mut self.ofx_host);
                    ((*plugin).setHost)(host_ptr);

                    // Tell plugin we need to load it
                    let ofx_action_load = CString::new("OfxActionLoad").unwrap();
                    let ret = ((*plugin).mainEntry)(ofx_action_load.as_ptr(), 
                                            ptr::null_mut(), 
                                            ptr::null_mut(), 
                                            ptr::null_mut());
                    
                    debug!("OfxActionLoad returned {}", ret);
                    // TODO: Action describe
                    // TODO test for plugin compatibility 
                    let ofx_action_describe = CString::new("OfxActionDescribe").unwrap();
                    let mut plugin_description : OfxPropertySet = OfxPropertySet::new();
                    let ret = ((*plugin).mainEntry)(ofx_action_describe.as_ptr(), 
                                            transmute(& mut plugin_description), 
                                            ptr::null_mut(), 
                                            ptr::null_mut());

                }
                //// TODO keep plugins pointers somewher in this structure ?
                // Describe and set host
                // and store or remove
                let ptr_wrap = unsafe {CStr::from_ptr((*plugin).pluginIdentifier)};
                let plugin_name = ptr_wrap.to_str().unwrap().to_string();
                
                // FIXME: check plugin name uniqueness
                self.plugins.insert(plugin_name, plugin);
            }    
        }
    }

    pub fn instanciate(& mut self, plugin_name: &str) {
        match self.plugins.get(plugin_name) {
            Some(plugin) => {
                debug!("found plugin {:?}\n", plugin);
            },
            None => {debug!("plugin {:?} not found", plugin_name)},
        }
    }
}

