#[macro_use(warn, trace, debug, error, log)]
extern crate log;
extern crate env_logger;
extern crate rustfx;
use rustfx::rfx::engine::*;
use rustfx::rfx::bundle::*;
use std::path::PathBuf;
use std::fs;

fn find_ofx_test_plugins() -> Vec<PathBuf> {
    let mut bundle_paths: Vec<PathBuf> = Vec::new();
    let mut path_to_plugins = fs::canonicalize(file!()).unwrap();
    path_to_plugins.pop();
    path_to_plugins.push("plugins");
    path_to_plugins.push("OFX");
    bundle_paths.push(path_to_plugins);
    bundle_paths
}

#[test]
fn load_plugin_not_found() {
    //
    trace!("Initializing rustfx");
    env_logger::init().unwrap();

    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    assert!(engine.image_effect("plugins").is_none()); // not found
}

#[test]
fn load_basic_plugin() {
    // trace!("Initializing rustfx");
    // env_logger::init().unwrap();
    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    // TODO : load image_effect and double check information stored in the node
    let image_effect = engine.image_effect("uk.co.thefoundry.BasicGainPlugin");
    assert!(image_effect.is_some());
}

#[test]
fn load_invert_plugin() {
    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    assert!(engine.image_effect("uk.co.thefoundry.OfxInvertExample").is_some());
}

#[test]
fn load_custom_plugin() {
    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    assert!(engine.image_effect("uk.co.thefoundry.CustomParamPlugin").is_some());
}


#[test]
fn load_rectangle_plugin() {
    let mut engine = Engine::new();
    engine.load_plugins(find_ofx_test_plugins());

    assert!(engine.image_effect("uk.co.thefoundry.GeneratorExample").is_some());
}

// #[test]
// fn list_plugins() {
//
// }
