use std::fs::File;
use rfx::project::Project;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;


// Stores temporary parsing data
pub struct RfxFileFormat<'a> {
    reader: BufReader<&'a File>,
    current_line: String,
}

impl<'a> RfxFileFormat<'a> {
    /// returns a new parser
    pub fn new(file: &File) -> RfxFileFormat {
        RfxFileFormat {
            reader: BufReader::new(file),
            current_line: String::with_capacity(1024),
        }
    }

    /// Update a project with the content of the file the parser was initialized with
    /// Returns the updated project
    pub fn update(&mut self, mut project: Project) -> Project {
        while let Ok(bytes) = self.next() {
            if bytes == 0 {
                return project;
            }
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
            // Comment
        } else {
            panic!("Unrecognized token {}", self.current_line);
        }
    }

    fn parse_add_node_cmd(&mut self, project: &mut Project) {
        // Read node type
        let (_, node_type) = self.current_line.split_at(5); // replace by sizeof("node") + 1 ?
        let node_created = project.new_node(node_type);
        match node_created {
            Some(node) => {
                // Read node parameters
                // self.parse_node_parameters()
            }
            None => {
                panic!("unable to create node {}", node_type);
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
            parser.update(project);
            assert!(true); // Test something meaningfull
        } 
        Err(_) => {
            panic!("unable to open {:?}", &path);
            assert!(false);
        }
    }
}
