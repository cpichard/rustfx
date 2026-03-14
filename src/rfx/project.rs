///
/// a Project contains the graph of processing nodes
///

use rfx::engine::*;
use rfx::effectnode::*;
use rfx::imageclip::*;
use rfx::rfxfileformat::RfxFileFormat;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
use std::ffi::{CString, CStr};

// TODO: describe a node container impl

/// An rfx project contains the graph of image effect nodes
/// used to process the images
pub struct Project {
    // TODO: NodeHandle should be unsigned int
    // and we store the nodes in a vector
    nodes: HashMap<NodeHandle, Node>,

    // TODO: should handles die with the node ?
    wires: HashMap<NodeHandle, (NodeHandle, ClipHandle)>,

    // The host related code.
    engine: Engine,
}

impl Project {
    pub fn new() -> Project {
        Project {
            nodes: HashMap::new(),
            wires: HashMap::new(),
            engine: Engine::new(),
        }
    }

    /// Find and load the OFX plugins in bundle_path
    pub fn plugins_load(&mut self, bundle_paths: Vec<PathBuf>) {
        self.engine.plugins_load(bundle_paths)
    }

    pub fn plugins_list(&self) {
        self.engine.plugins_list();
    }

    /// Create a new node and return it to the caller.
    /// If the plugin which should create the node is not found,
    /// return a dummy node
    pub fn node_new(&mut self, from_plugin: &str) -> Node {
        let plugin_node = self.engine.image_effect(from_plugin);
        match plugin_node {
            Some(node) => node,
            None => Node::new(),
        }
    }

    /// Takes ownership of the node and insert it in the project
    /// Returns the choosen unique NodeHandle
    /// This should be part of the "NodeContainer" impl
    pub fn node_insert(&mut self, mut node_id: NodeHandle, node: Node) -> NodeHandle {
        while self.nodes.contains_key(&node_id) {
            node_id = next_id(&mut node_id)
        }
        self.nodes.insert(node_id.clone(), node);
        node_id
    }

    /// Returns a node by its unique id
    pub fn node_get(&self, node_id: &NodeHandle) -> Option<&Node> {
        self.nodes.get(node_id)
    }

    pub fn node_get_mut(&mut self, node_id: &NodeHandle) -> Option<&mut Node> {
        self.nodes.get_mut(node_id)
    }

    /// Return the number of nodes in the graph
    pub fn node_len(&self) -> usize {
        self.nodes.len()
    }

    /// Read a project file and returns a Project object
    pub fn project_load(file_name: PathBuf) -> Project {
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

    /// Returns the node input, it might not be connected
    /// node_id must belong to the project
    pub fn node_input(&self,
                      node_id: &NodeHandle,
                      clip_name: &ClipHandle)
                      -> Option<&OfxImageClip> {
        if let Some(ref node) = self.node_get(node_id) {
            node.clip_get(clip_name)
        } else {
            None
        }
    }

    pub fn node_output(&self, node: &NodeHandle) -> Option<ClipHandle> {
        Some(ClipHandle::new("Output").unwrap()) // FIXME
    }

    // TODO: graph and node connection
    // Connect should return an error if the nodes are 
    // incompatible or can't be connected because of cycles
    pub fn node_connect(&mut self,
                        in_handle: &NodeHandle,
                        out_handle: &NodeHandle,
                        clip_in: &ClipHandle) {
        // TODO test for non-existant nodes and clip
        // TODO check already made connection
        // Get nodes
        // TODO : node_exists() instead of node_get
        if let Some(_node_in) = self.node_get(in_handle) {
        } else {
            error!("unable to find node {}", in_handle);
            return;
        }
        if let Some(_node_out) = self.node_get(out_handle) {
        } else {
            error!("unable to find node {}", out_handle);
            return;
        }
        self.wires.insert(out_handle.clone(), (in_handle.clone(), clip_in.clone()));
        // TODO check for cycle
    }

    // TODO: render a clip
    pub fn render(&mut self, node_handle: &NodeHandle) {
        // Render node if node is an output node
        let node = self.node_get_mut(node_handle).unwrap();

        // Go down to sources and get source image size
        //let ref incoming = node.incoming;
        //for n in incoming {
        //    println!("incoming {:?}", n);
        //}
    }
}

#[test]
fn test_new_project() {
    let mut project = Project::new();
    assert_eq!(project.node_len(), 0);

    let node1 = project.node_new("no_plugin");
    project.node_insert("Test".to_string(), node1);
    let node2 = project.node_new("no_plugin");
    project.node_insert("Test".to_string(), node2);

    assert_eq!(project.node_len(), 2);
}

// TODO: test connections failures
#[test]
fn test_connection() {
    let mut project = Project::new();
    assert_eq!(project.node_len(), 0);

    let node1 = project.node_new("dummy_plugin");
    let node1_handle = project.node_insert("Test".to_string(), node1);
    let node2 = project.node_new("dummy_plugin");
    let node2_handle = project.node_insert("Test".to_string(), node2);
    assert_eq!(project.node_len(), 2);

    let clip_handle = ClipHandle::new("Source").unwrap();
    project.node_connect(&node1_handle, &node2_handle, &clip_handle);
    
    // TODO: test connection
}
