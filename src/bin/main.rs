#[macro_use(warn, trace, debug, error, log)]
extern crate log;
extern crate env_logger;
extern crate rustfx;
use rustfx::rfx::engine::*;
use rustfx::rfx::bundle::*;

fn main() {
    //
    env_logger::init().unwrap();
    trace!("Initializing rustfx");

    // Get env OFX and list all the plugins specified in the path
    let bundle_paths = default_bundle_paths();
    // Start an engine with those plugins
    //println!("Starting engine");
    let mut engine = Engine::new();
    engine.load_plugins(bundle_paths);
    //engine.load_script("test.rfx");

    // Load project, graph of effects
    engine.node("Test"); // not found
    engine.node("tuttle.checkerboard"); // not found
    engine.node("uk.co.thefoundry.BasicGainPlugin");
}
