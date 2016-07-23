extern crate libc;
#[macro_use(warn, trace, debug, error, log)]
extern crate log;
extern crate env_logger;

mod ofx;

mod engine;
use engine::*;

mod bundle;
use bundle::*;

fn main() {
    //
    env_logger::init().unwrap();
    trace!("Initializing rustfx");

    // Get env OFX and list all the plugins specified in the path
    let bundle_paths = get_bundle_paths();

    // This finds all the bundles 
    let bundles = find_bundles(bundle_paths);

    // Start an engine with those plugins
    //println!("Starting engine");
    let mut engine = Engine::new();
    engine.load_plugins(bundles);

    // Load project, graph of effects
    engine.node("Test"); // not found
    engine.node("tuttle.checkerboard"); // found

}
