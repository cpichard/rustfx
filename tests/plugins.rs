#[macro_use(warn, trace, debug, error, log)]
extern crate log;
extern crate env_logger;
extern crate rustfx;
use rustfx::rfx::engine::*;
use rustfx::rfx::bundle::*;
use std::path::PathBuf;


#[test]
fn load_plugin_not_found() {
    //
    env_logger::init().unwrap();
    trace!("Initializing rustfx");

    // Get env OFX and list all the plugins specified in the path
    let mut bundle_paths: Vec<PathBuf> = Vec::new();
    // FIXME: replace OFX path
    bundle_paths.push(PathBuf::from("/Users/cyril/develop/rustfx/OFX"));
    let mut engine = Engine::new();
    engine.load_plugins(bundle_paths);

    engine.node("Test"); // not found
}

#[test]
fn load_basic_plugin() {
    // Get env OFX and list all the plugins specified in the path
    let mut bundle_paths: Vec<PathBuf> = Vec::new();
    bundle_paths.push(PathBuf::from("/Users/cyril/develop/rustfx/OFX"));
    let mut engine = Engine::new();
    engine.load_plugins(bundle_paths);

    // TODO : load node and double check information stored in the node
    engine.node("uk.co.thefoundry.BasicGainPlugin");
}

#[test]
fn load_custom_plugin() {
    // Get env OFX and list all the plugins specified in the path
    let mut bundle_paths: Vec<PathBuf> = Vec::new();
    bundle_paths.push(PathBuf::from("/Users/cyril/develop/rustfx/OFX"));
    let mut engine = Engine::new();
    engine.load_plugins(bundle_paths);

    engine.node("uk.co.thefoundry.CustomParamPlugin");
}

#[test]
fn load_invert_plugin() {
    // Get env OFX and list all the plugins specified in the path
    let mut bundle_paths: Vec<PathBuf> = Vec::new();
    bundle_paths.push(PathBuf::from("/Users/cyril/develop/rustfx/OFX"));
    let mut engine = Engine::new();
    engine.load_plugins(bundle_paths);

    engine.node("uk.co.thefoundry.OfxInvertExample");
}

#[test]
fn list_plugins() {
    
}

