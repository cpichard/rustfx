use std::fs::File;
use rfx::project::Project;
use rfx::project::NodeHandle;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

// Stores temporary parsing data
pub struct RfxFileFormat<'a> {
    reader: BufReader<&'a File>,
    current_line: String,
    line_bytes: usize,
}

impl<'a> RfxFileFormat<'a> {
    /// returns a new parser
    pub fn new(file: &File) -> RfxFileFormat {
        RfxFileFormat {
            reader: BufReader::new(file),
            current_line: String::with_capacity(1024),
            line_bytes: 0,
        }
    }

    /// Update a project with the content of the file the parser was initialized with
    /// Returns the updated project
    pub fn update(&mut self, mut project: Project) -> Project {
        while let Ok(bytes) = self.next() {
            if bytes == 0 {
                return project;
            }
            self.line_bytes = bytes;
            self.parse_top_level(&mut project);
        }
        project
    }

    /// Next line
    fn next(&mut self) -> Result<usize, Error> {
        self.reader.read_line(&mut self.current_line)
    }

    /// Parse top level command and setup a context
    fn parse_top_level(&mut self, project: &mut Project) {
        if self.current_line.starts_with("node") {
            // debug println!("node {}", self.current_line);
            self.parse_add_node_cmd(project);
        } else if self.current_line.starts_with("bundle") {
            println!("bundle {}", self.current_line);
            // simple test to alter the project p
            let plugins = Vec::new();
            project.load_plugins(plugins);
        } else if self.current_line.starts_with("#") {
            // Comment next line
        } else {
            panic!("Unrecognized token {}", self.current_line);
        }
    }

    fn parse_add_node_cmd(&mut self, project: &mut Project) {
        // Read node type
        let mut node_created: Option<NodeHandle> = {
            let plugin_name = unsafe { self.current_line.slice_unchecked("node".len()+1, self.current_line.len()-1)};
            project.new_node(plugin_name)
        };

        match node_created {
            Some(node) => {
                // Read node parameters
                self.parse_node_parameters(project, node)
            }
            None => {
        //        let (_, node_type) = self.current_line.split_at(5); // replace by sizeof("node") + 1 ?
        //        panic!("unable to create node {}", node_type);
            }
        }
    }

    fn parse_node_parameters(&mut self, project: &mut Project, node: NodeHandle) {
        // should start with a space
        // count number of spaces ?
        if self.current_line.starts_with(" ") {
            // TODO check next character is not a space
            // grab the parameter name
            // and its value
            let mut words = self.current_line.trim().split_whitespace();
            let key = words.nth(0);
            let value = words.nth(1);
            match (key, value) {
                (Some(k), Some(v)) => {
                    let some_node = Some(node);
                    project.set_value(&some_node, k.to_string(), v.to_string())
                }
                (_, _) => panic!("unrecognized line"),
            }

        }
    }
}

#[cfg(test)]
use std::path::PathBuf;

#[test]
fn parse_one_node() {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.pop();
    path.pop();
    path.push("tests/projects/1.rfx");
    match File::open(&path) {
        Ok(file) => {
            let mut project = Project::new();
            let mut parser = RfxFileFormat::new(&file);
            project = parser.update(project);
            // read one node ?
            println!("NB NODES: {}", project.nb_nodes());
            assert!(project.nb_nodes() == 1);
            // get node and get its value
        } 
        Err(_) => {
            panic!("unable to open {:?}", &path);
            assert!(false);
        }
    }
}
