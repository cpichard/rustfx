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
    project.load_plugins(bundle_paths); // project.load_plugins() ??

    // Load project, graph of effects
    let test_node = project.new_node("Test"); // not found
    let checkerboard = project.new_node("tuttle.checkerboard"); // not found
    let gain_plugin = project.new_node("uk.co.thefoundry.BasicGainPlugin");

    project.set_value(&gain_plugin, "gain".to_string(), 10);
    
    let gain_input = project.get_input(&gain_plugin, "Source");
    let checkerboard_output = project.get_output(&checkerboard);
    project.connect(&checkerboard, &gain_plugin, &gain_input);

}
