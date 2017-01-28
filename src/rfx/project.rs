///
/// a Project contains the graph of processing nodes

use bindings::imageeffect::*;
use rfx::engine::*;
use rfx::rfxfileformat::RfxFileFormat;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;

pub type Node = OfxImageEffectStruct;
pub type NodeHandle = String;
pub type ClipHandle = String;
// Output Node, Input node, Input clip
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

/// This is the principal "API" object exposed to the rest of the world
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
        let node_created = self.engine.node(plugin_name);
        match node_created {
            Some(node) => {
                // TODO: Make sure the key is not taken yet
                // this should raise an error
                // This should return a unique name
                self.nodes.insert(plugin_name.to_string(), node);
                Some(plugin_name.to_string())
            }
            None => {
                // Creates an empty node // dummy node
                // TODO: flag the node as being "pluginless"
                let node = Node::new();
                warn!("no plugin found for node xxx");
                self.nodes.insert(plugin_name.to_string(), node);
                Some(plugin_name.to_string())
            }
        }
    }

    pub fn set_value(&mut self,
                     node_handle: &Option<NodeHandle>,
                     param_name: String,
                     param_value: String) {
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
