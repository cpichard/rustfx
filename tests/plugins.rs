#[macro_use(warn, trace, debug, error, log)]
extern crate log;
extern crate env_logger;
extern crate rustfx;
use rustfx::rfx::engine::*;
use rustfx::rfx::bundle::*;
use std::path::PathBuf;

fn find_ofx_test_plugins() -> Vec<PathBuf> {
    let mut bundle_paths: Vec<PathBuf> = Vec::new();
    // FIXME: replace OFX path with relative
    bundle_paths.push(PathBuf::from("/Users/cyril/develop/rustfx/OFX"));
    bundle_paths
}

#[test]
fn load_plugin_not_found() {
    //
    env_logger::init().unwrap();
    trace!("Initializing rustfx");

    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    assert!(engine.image_effect("Test").is_none()); // not found
}

#[test]
fn load_basic_plugin() {
    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    // TODO : load image_effect and double check information stored in the node
    assert!(engine.image_effect("uk.co.thefoundry.BasicGainPlugin").is_some());
}

#[test]
fn load_custom_plugin() {
    // Get env OFX and list all the plugins specified in the path
    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    assert!(engine.image_effect("uk.co.thefoundry.CustomParamPlugin").is_some());
}

#[test]
fn load_invert_plugin() {
    // Get env OFX and list all the plugins specified in the path
    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    engine.image_effect("uk.co.thefoundry.OfxInvertExample");
}

#[test]
fn list_plugins() {
    
}

