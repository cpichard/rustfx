
use ofx::core::OfxHost;
use bundle::PluginList;

pub struct Engine {
    ofx_host: OfxHost,
    plugins: PluginList,
    // renders ?
    // projects ?
}

impl Engine {
    pub fn new(plugins: PluginList) -> Engine {
        Engine { 
            ofx_host: OfxHost::new(),
            plugins: plugins,
        }
    }
}

struct EngineBuilder {

}


impl EngineBuilder {
    
    pub fn new(){}
    pub fn set_plugins(){}
    pub fn nb_thread(){}
    pub fn finish(){}

}
