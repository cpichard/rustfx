#[macro_use(warn, trace, debug, error, log)]
extern crate log;
extern crate env_logger;
extern crate rustfx;
use rustfx::rfx::project::*;
use rustfx::rfx::bundle::*;

fn main() {
    //
    env_logger::init().unwrap();
    trace!("Initializing rustfx");

    // Get env OFX and list all the plugins specified in the path
    let mut project = Project::new();
    let bundle_paths = default_bundle_paths();
    project.plugins_load(bundle_paths); // project.plugins_load() ??

    // TODO: 
    // Load the command line script and execute the command passed in the command line

    project.plugins_list();
    // Load project, graph of effects
    //let test_node = project.node_new("Test"); // not found
    //let checkerboard = project.node_new("tuttle.checkerboard"); // not found
    //let gain_plugin = project.node_new("uk.co.thefoundry.BasicGainPlugin");

    //project.value_set(&gain_plugin, "gain".to_string(), "10".to_string());
    
    //let gain_input = project.get_input(&gain_plugin, &"Source".to_string());
    //let checkerboard_output = project.get_output(&checkerboard);
   // project.connect(&checkerboard, &gain_plugin, &gain_input);

}


