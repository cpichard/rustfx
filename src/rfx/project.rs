///
/// a Project contains the graph of processing nodes
///

use rfx::engine::*;
use rfx::effectnode::*;
use rfx::rfxfileformat::RfxFileFormat;
use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::File;
// use std::ffi::{CString, CStr};

pub type Node = EffectNode;
pub type NodeHandle = String;
pub type ClipHandle = String;
pub type NodeInput = (NodeHandle, ClipHandle);
pub type NodeOutput = NodeHandle;


/// Returns a new id for the node, it basically prefix the name
/// With a number or increase the number if there is already one
/// ex: "Gain.2" == next_id("Gain.1");
///     "Blur.1" == next_id("Blur");
fn next_id(id: &NodeHandle) -> NodeHandle {
    let mut prefix = id.clone();
    let mut number_str = 1.to_string(); 

    let split_string: Vec<&str> = id.rsplitn(2, '.').collect();
    
    if split_string.len() == 2 { // Found a '.' 
        // try to convert
        if let Ok(mut number) = split_string[0].parse::<i32>() {
            number += 1;
            number_str = number.to_string();
            prefix = split_string[1].to_string();
        }
    }

    prefix + "." + &number_str
}

/// An rfx project contains the graph of image effect nodes
/// used to process the images
pub struct Project {
    // TODO: Nodehandle should be unsigned int
    // and we store the nodes in a vector
    nodes: HashMap<NodeHandle, Node>,

    // Connections
    connections: HashMap<NodeOutput, Vec<NodeInput>>,

    // The host related code.
    engine: Engine,
}

impl Project {
    pub fn new() -> Project {
        Project {
            nodes: HashMap::new(),
            connections: HashMap::new(),
            engine: Engine::new(),
        }
    }

    pub fn plugins_load(&mut self, bundle_paths: Vec<PathBuf>) {
        self.engine.plugins_load(bundle_paths)
    }

    pub fn plugins_list(&self) {
        self.engine.plugins_list();
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
            node_id = next_id(&mut node_id)
        }
        self.nodes.insert(node_id.clone(), node);
        node_id
    }

    /// Returns a node by its unique name
    pub fn node_get(&self, node_id: NodeHandle) -> Option<&Node> {
        self.nodes.get(&node_id)
    }

    /// Return the number of nodes in the graph
    pub fn node_qty(&self) -> usize {
        self.nodes.len()
    }

    // TODO
    // pub fn get_input(&self,
    //                 node_handle: &Option<NodeHandle>,
    //                 clip_name: &String)
    //                 -> Option<ClipHandle> {

    //    // TODO return correct input
    //    None
    // }

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


    // TODO: graph and node connection
    pub fn node_connect(&mut self,
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


    // TODO:
    // pub fn save_project(project: Project) {
    // }
}


#[test]
fn test_next_id() {
    let mut id1 = "Gain.1".to_string();
    id1 = next_id(&mut id1);
    assert!(id1 == "Gain.2".to_string());

    let mut id2 = "Gain".to_string();
    id2 = next_id(&mut id2);
    assert!(id2 == "Gain.1".to_string());

    let mut id3 = "".to_string();
    id3 = next_id(&mut id3);
    assert!(id3 == ".1".to_string());

    let mut id4 = "Gain.toto".to_string();
    id4 = next_id(&mut id4);
    assert!(id4 == "Gain.toto.1".to_string());
}
