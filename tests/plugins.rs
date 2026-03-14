#[macro_use(warn, trace, debug, error, log)]
extern crate log;
extern crate env_logger;
extern crate rustfx;
use rustfx::rfx::engine::*;
use rustfx::rfx::project::*;
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
    engine.plugins_load(find_ofx_test_plugins());

    assert!(engine.image_effect("plugins").is_none()); // not found
}

#[test]
fn load_basic_plugin() {
    // trace!("Initializing rustfx");
    // env_logger::init().unwrap();
    let mut engine = Engine::new();
    engine.plugins_load(find_ofx_test_plugins());

    // TODO : load image_effect and double check information stored in the node
    let image_effect = engine.image_effect("uk.co.thefoundry.BasicGainPlugin");
    assert!(image_effect.is_some());
}

#[test]
fn load_invert_plugin() {
    let mut engine = Engine::new();
    engine.plugins_load(find_ofx_test_plugins());

    assert!(engine.image_effect("uk.co.thefoundry.OfxInvertExample").is_some());
}

#[test]
fn load_custom_plugin() {
    let mut engine = Engine::new();
    engine.plugins_load(find_ofx_test_plugins());

    assert!(engine.image_effect("uk.co.thefoundry.CustomParamPlugin").is_some());
}

#[test]
fn load_rectangle_plugin() {
    let mut engine = Engine::new();
    engine.plugins_load(find_ofx_test_plugins());

    assert!(engine.image_effect("uk.co.thefoundry.GeneratorExample").is_some());
}

use std::ffi::{CString, CStr};
#[test]
fn connect_plugins() {
    let mut project = Project::new();
    project.plugins_load(find_ofx_test_plugins());
    let generator_effect = project.node_new("uk.co.thefoundry.GeneratorExample");
    let generator_id = project.node_insert("Generator".to_string(), generator_effect);
    let gain_effect = project.node_new("uk.co.thefoundry.BasicGainPlugin");
    let gain_id = project.node_insert("Gain".to_string(), gain_effect);

    let gain_clip = CString::new("Source").unwrap();

    if project.node_input(&gain_id, &gain_clip).is_some() {
        project.node_connect(&generator_id, &gain_id, &gain_clip);
        // TODO: check connections
    } else {
        assert!(false); // TODO: comment
    }
}
