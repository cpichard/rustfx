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
    bundle_paths.push(PathBuf::from("/Users/cyril/develop/rustfx/tests/plugins/OFX"));
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

// #[test]
// fn list_plugins() {
//
// }
