use std::fs::File;
use rfx::project::Project;
use rfx::project::NodeHandle;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

///
/// RFX file format parser
///
///


enum RfxParsingContext {
    TopLevel,
    NodeParsed(NodeHandle),
}

// Stores temporary parsing data
pub struct RfxFileFormat<'a> {
    reader: BufReader<&'a File>,
    current_line: String,
    line_bytes: usize,
}

impl<'a> RfxFileFormat<'a> {

    /// Returns a new parser data
    pub fn new(file: &File) -> RfxFileFormat {
        RfxFileFormat {
            reader: BufReader::new(file),
            current_line: String::with_capacity(1024),
            line_bytes: 0,
        }
    }

    /// Update a project with the content of the parsed file 
    /// Returns the updated project
    pub fn update(&mut self, mut project: Project) -> Project {
        let mut context = RfxParsingContext::TopLevel;
        // NOTE should we parse tokens instead of lines ?
        while let Ok(bytes) = self.next() {
            if bytes == 0 {
                return project;
            }
            self.line_bytes = bytes;
            // let mut lexer = Lexer::new(self.current_line)
            // for ()
            //
            // Parse depending on the current parser context
            let newcontext = match context {
                // Expect top level command
                RfxParsingContext::TopLevel => self.parse_top_level(&mut project),
                // a Node was parsed
                RfxParsingContext::NodeParsed(ref node) => {
                    // Expect parsing parameters
                    let param_parsed = self.parse_node_parameters(&mut project, node);
                    if param_parsed {
                        RfxParsingContext::NodeParsed(node.clone()) // TODO avoid cloning here, it shouldn't be necessary 
                    } else {
                        self.parse_top_level(&mut project)
                    }
                }
            };
            context = newcontext;
        } // TODO : handle when there was an error while reading the line
        project
    }

    /// Next line
    /// iterator trait ?
    fn next(&mut self) -> Result<usize, Error> {
        self.current_line.clear();
        self.reader.read_line(&mut self.current_line)
    }

    /// Parse top level command and setup a context
    /// Returns true if the line has been consumed
    fn parse_top_level(&self, project: &mut Project) -> RfxParsingContext {
        if self.current_line.starts_with("node") {
            match self.parse_command_node(project) {
                Some(node) => RfxParsingContext::NodeParsed(node),
                None => RfxParsingContext::TopLevel, 
            }
        } else if self.current_line.starts_with("bundle") {
            println!("bundle {}", self.current_line);
            // simple test to alter the project p
            let plugins = Vec::new();
            project.load_plugins(plugins);
            RfxParsingContext::TopLevel
        } else if self.current_line.starts_with("#") {
            // Comment next line
            RfxParsingContext::TopLevel
        } else if self.current_line.trim().is_empty() {
            RfxParsingContext::TopLevel
        } else {
            panic!("unable to parse top level command {}", self.current_line);
        }
    }

    fn parse_command_node(&self, project: &mut Project) -> Option<NodeHandle> {
        // Read node type
        let plugin_name = unsafe {
            self.current_line.slice_unchecked("node".len() + 1, self.current_line.len() - 1)
        };
        project.new_node(plugin_name)
    }

    fn parse_node_parameters(&self, project: &mut Project, node: &NodeHandle) -> bool {
        // should start with a space
        // count number of spaces ?
        if self.current_line.starts_with(" ") {
            // TODO check next character is not a space
            // grab the parameter name
            // and its value
            let mut words = self.current_line.trim().split_whitespace();
            let key = words.nth(0);
            let value = words.nth(0);
            match (key, value) {
                (Some(k), Some(v)) => project.set_value(node, k.to_string(), v.to_string()),
                (_, _) => {
                    panic!("unable to parse key value {:?} {:?}", key, value);
                }
            }

            return true;
        } else {
            return false;
        };
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
            assert!(project.nb_nodes() == 1);
            // get node and get its value
            // TODO test parameters
        } 
        Err(_) => {
            panic!("unable to open {:?}", &path);
            assert!(false);
        }
    }
}

#[test]
fn parse_two_nodes() {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.pop();
    path.pop();
    path.push("tests/projects/2.rfx");
    match File::open(&path) {
        Ok(file) => {
            let mut project = Project::new();
            let mut parser = RfxFileFormat::new(&file);
            project = parser.update(project);
            // read one node ?
            println!("NB NODES: {}", project.nb_nodes());
            assert!(project.nb_nodes() == 2);
            // get node and get its value
        } 
        Err(_) => {
            panic!("unable to open {:?}", &path);
            assert!(false);
        }
    }
}
