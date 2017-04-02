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

pub fn next_id(id: &mut NodeHandle) {
    // if the node ends with .x and x is a number, raise the number 
}

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


    /// Create a new node and give it to the caller.
    /// If the plugin is not able to provide a node, this function
    /// returns a dummy node
    pub fn node_new(&mut self, from_plugin: &str) -> Node {
        let plugin_node = self.engine.image_effect(from_plugin);
        match plugin_node {
            Some(node) => node,
            None => Node::new(),
        }
    }

    /// Takes ownership of the node and insert it in the project
    /// Returns the choosen unique NodeHandle
    pub fn node_insert(&mut self, mut node_id: NodeHandle, node: Node) -> NodeHandle {
        while self.nodes.contains_key(&node_id) {
            next_id(&mut node_id)
        }
        self.nodes.insert(node_id.clone(), node);
        node_id
    }

    pub fn get_input(&self,
                     node_handle: &Option<NodeHandle>,
                     clip_name: &String)
                     -> Option<ClipHandle> {

        // TODO return correct input
        None
    }

    pub fn load_project(file_name: PathBuf) -> Project {
        // Read a file and re-construct a Project
        // Open file
        match File::open(file_name) {
            Ok(mut file) => {
                let project = Project::new();
                let mut parser = RfxFileFormat::new(&mut file);
                parser.update_project(project)
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
