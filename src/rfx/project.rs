///
/// a Project contains the graph of processing nodes
///

use rfx::engine::*;
use rfx::effectnode::*;
use rfx::rfxfileformat::RfxFileFormat;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
use std::ffi::{CString, CStr};

pub type Node = EffectNode;
pub type NodeHandle = String;
pub type ClipHandle = String;
pub type NodeInput = (NodeHandle, ClipHandle);
pub type NodeOutput = NodeHandle;

/// An rfx project contains the graph of image effect nodes
/// used to process the images
pub struct Project {
    // NOTE: if performance is an issue, we can replace HashMap with Vec and
    // have NodeHandle as an int. Using string is handy for debugging
    // The same goes for connections
    nodes: HashMap<NodeHandle, Node>,

    // Connections
    connections: HashMap<NodeOutput, Vec<NodeInput>>,

    // The host related code.
    engine: Engine,
}

/// This would the principal "API" object exposed
/// with simple function like load_project
impl Project {
    pub fn new() -> Project {
        Project {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            engine: Engine::new(),
        }
    }

    pub fn load_plugins(&mut self, bundle_paths: Vec<PathBuf>) {
        self.engine.load_plugins(bundle_paths)
    }

    // NodeHandle : change Option<String> to String
    // if the node can't be created by the plugin,
    // create a dummy placeholder node
    pub fn new_node(&mut self, plugin_name: &str) -> Option<NodeHandle> {

        // Try to create a node using the plugins
        let engine_returned = self.engine.image_effect(plugin_name);
        match engine_returned {
            Some(ofx_image_effect) => {
                // TODO: Make sure the key is not taken yet
                // otherwise raise an error
                // TODO: This should return a unique name
                println!("found image effect");
                self.nodes.insert(plugin_name.to_string(), ofx_image_effect);
                Some(plugin_name.to_string())
            }
            None => {
                // Creates an empty node // dummy node
                // TODO: flag the node as being "pluginless"
                let node = Node::new();
                println!("not found image effect {}", plugin_name);
                warn!("no plugin found for node xxx");
                self.nodes.insert(plugin_name.to_string(), node);
                Some(plugin_name.to_string())
            }
        }
    }

    pub fn set_value(&mut self,
                     node_handle: &NodeHandle,
                     param_name: String,
                     param_value: String) {
        // DEBUGGING
        println!("Set values {:} {:}", param_name, param_value);
        let maybe_node = self.nodes.get_mut(node_handle);
        match maybe_node {
            Some(node) => {
                node.set_value(&param_name, param_value);
            }
            None => {}    
        }
    }

    pub fn get_input(&self,
                     node_handle: &Option<NodeHandle>,
                     clip_name: &String)
                     -> Option<ClipHandle> {

        None
    }

    pub fn load_project(file_name: PathBuf) -> Project {
        // Read a file and re-construct a Project
        // Open file
        match File::open(file_name) {
            Ok(file) => {
                let mut project = Project::new();
                let mut parser = RfxFileFormat::new(&file);
                parser.update(project)
            }
            Err(e) => {
                panic!("unable to load file {}", e);
            }
        }
    }
    pub fn connect(&mut self,
                   in_node: &Option<NodeHandle>,
                   out_node: &Option<NodeHandle>,
                   out_clip: &Option<ClipHandle>) {
        // TODO check connection validity
        match (in_node, out_node, out_clip) {
            (&Some(ref node_in), &Some(ref node_out), &Some(ref clip_out)) => {
                println!("test");
                self.connections
                    .entry(node_in.clone())
                    .or_insert(Vec::new())
                    .push((node_out.clone(), clip_out.clone()));
            }
            _ => println!("bb"), // TODO panic or warn ????
        }
    }
    pub fn nb_nodes(&self) -> usize {
        self.nodes.len()
    }
    // pub fn save_project(project: Project) {
    // }
}
