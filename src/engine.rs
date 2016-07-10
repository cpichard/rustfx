
use libc::*;
use libc;
use ofx::core::OfxHost;
use bundle::PluginList;

pub struct Engine {
    ofx_host: OfxHost,
    plugins: PluginList,
}

impl Engine {
    pub fn new(plugins: PluginList) -> Engine {
        Engine { 
            ofx_host: OfxHost::new(),
            plugins: plugins,
        }
    }

    pub fn instanciate(&self, plugin_name: &str) {
        match self.plugins.get(plugin_name) {
            Some(bundle) => {
                //print!("plug {:?}\n", *plugin);

                //// TODO: what is the lifetime of host_ptr ?
                //unsafe {
                //    let host_ptr : * mut libc::c_void = mem::transmute(& mut self.ofx_host);
                //    ((*plugin).setHost)(host_ptr);

                //    // Call init functions
                //    let ofx_action_load = CString::new("OfxActionLoad").unwrap();
                //    ((*plugin).mainEntry)(ofx_action_load.as_ptr(), 
                //                            ptr::null_mut(), 
                //                            ptr::null_mut(), 
                //                            ptr::null_mut());
                //    }
                //// TODO keep plugins alive ?
            },
            None => {println!("not found")},
        }
    }
}

