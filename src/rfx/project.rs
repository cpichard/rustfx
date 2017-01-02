///
/// a Project contains the graph of processing nodes
///
use bindings::imageeffect::*;
use rfx::engine::*;
use std::path::PathBuf;
use std::collections::HashMap;

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

/// This is the base API object
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

        let new_node = self.engine.node(plugin_name);
        match new_node {
            Some(node) => {
                // TODO: Make sure the key is not taken yet
                // this should raise an error
                self.nodes.insert("test_node".to_string(), node);
                Some("test_node".to_string())
            }
            None => None, // Might give some logs ?
        }
    }

    pub fn set_value(&mut self, node_handle: &Option<NodeHandle>, param_name: String, value: i32) {}

    pub fn get_input(& self, node_handle: &Option<NodeHandle>, clip_name: &String) -> Option<ClipHandle> {
        None 
    }

    pub fn load_project(file_name: PathBuf) -> Project {
        // Read a file and re-construct a Project
        Project {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            engine: Engine::new(),
        }
    }

    pub fn connect(&mut self,
                   in_node: &NodeHandle,
                   out_node: &NodeHandle,
                   out_clip: &ClipHandle) {
        // TODO check connection validity
        self.connections.entry( in_node.clone() ).or_insert(Vec::new()).push( (out_node.clone(), out_clip.clone()) );
    }
    // pub fn save_project(project: Project) {
    // }
}
