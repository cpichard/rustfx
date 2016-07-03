extern crate libc;
mod ofx;
use ofx::plugin::*;
use ofx::core::*;

mod engine;
use engine::*;

mod bundle;
use bundle::*;

fn main() {
    // Get env OFX and list all the plugins specified in the path
    let bundle_paths = get_bundle_paths();

    // This finds all the bundles 
    let bundles = init_bundles(bundle_paths);

    // List the available plugins from the bundles
    let plugins = PluginList::from_bundles(bundles);

    // Test
    //for name in plugins.plugin_names.keys() {
    //    println!("Plugin: {}", name);
    //}

    // Start an engine with those plugins
    let engine = Engine::new(plugins);

    // What would be a simple api for the interaction of engine/host/plugins ?
    // host.instanciate("fr.inria.openfx.ReadPNG")
    //plugins.instanciate("fr.inria.openfx.ReadPNG");
    //let read_plugin = plugins.create_instance(ofxhost, "readexr");
    //let write_plugin = plugins.create_instance(ofxhost, "writeexr");

}
