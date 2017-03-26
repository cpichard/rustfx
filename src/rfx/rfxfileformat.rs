use std::fs::File;
use rfx::project::Project;
use rfx::project::NodeHandle;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::collections::HashMap;

///
/// RFX file format parser
///
// Commands recognized,
#[derive(Debug, PartialEq, Clone)]
enum CommandType {
    Node = 0, // create node
    Param, // set param
    Path, // plugin path
    Property, // set property
}

lazy_static! {
static ref CommandMap: HashMap<&'static str, CommandType> = {
    let mut commands = HashMap::new();
        commands.insert("node", CommandType::Node);
        commands.insert("param", CommandType::Param);
        commands.insert("path", CommandType::Path); // plugin path, likely to change
        commands.insert("property", CommandType::Property);
        commands
    };
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenBrace,
    CloseBrace,
    Command(CommandType),
    SemiColon,
    StringLiteral(String),
    FloatNumber(f32),
    IntNumber(i32),
    EOF,
}

#[derive(Debug, PartialEq)]
enum LexerState {
    Start,
    Comment,
    StringLiteral,
    Integer,
    Float,
    Command,
}

struct Lexer<'a> {
    input: &'a str,
    begin: usize,
    end: usize,
    state: LexerState,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Lexer {
            input: input,
            begin: 0,
            end: 0,
            state: LexerState::Start,
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            match self.state {
                LexerState::Start => {
                    let cursor = &self.input[self.begin..self.input.len()];
                    if let Some(c) = cursor.chars().next() {
                        match c {
                            ' ' | '\n' | '\t' => {
                                self.begin += 1;
                                self.end = self.begin;
                                continue;
                            }
                            ';' => {
                                self.begin += 1;
                                self.end = self.begin;
                                return Token::SemiColon;
                            }
                            '{' => {
                                self.begin += 1;
                                self.end = self.begin;
                                return Token::OpenBrace;
                            }
                            '}' => {
                                self.begin += 1;
                                self.end = self.begin;
                                return Token::CloseBrace;
                            }
                            'a'...'z' => {
                                // Match command
                                self.state = LexerState::Command;
                                self.end += 1;
                                continue;
                            }
                            '-' | '0'...'9' => {
                                self.state = LexerState::Integer;
                                self.end += 1;
                                continue;
                            }
                            '#' => {
                                self.state = LexerState::Comment;
                                self.end += 1;
                                continue;
                            }
                            '"' => {
                                self.state = LexerState::StringLiteral;
                                self.begin += 1; // skip quote
                                self.end += 1;
                                continue;
                            }
                            _ => {
                                panic!("character not recognized");
                            }
                        }
                    } else {
                        return Token::EOF;
                    }
                }
                LexerState::Command => {
                    let cursor = &self.input[self.end..self.input.len()];
                    if let Some(c) = cursor.chars().next() {
                        if 'a' <= c && c <= 'z' {
                            self.end += 1;
                            continue; // escape to main loop
                        }
                    }
                    // Extract command string
                    let cmd_str = &self.input[self.begin..self.end];
                    // Find command
                    match CommandMap.get(cmd_str) {
                        Some(cmd_enum) => {
                            // prepare exit
                            self.begin = self.end;
                            self.state = LexerState::Start;
                            return Token::Command(cmd_enum.clone());
                        }

                        None => panic!("unable to find command {}", cmd_str),                 
                    }
                }
                LexerState::Comment => {
                    let cursor = &self.input[self.end..self.input.len()];
                    if cursor.chars().next() == Some('\n') {
                        self.begin = self.end;
                        self.state = LexerState::Start;
                        continue;
                    } else {
                        self.end += 1;
                        if self.end >= self.input.len() {
                            return Token::EOF;
                        }
                    }
                }
                LexerState::Integer => {
                    let cursor = &self.input[self.end..self.input.len()];
                    if let Some(c) = cursor.chars().next() {
                        match c {
                            '0'...'9' => {
                                self.end += 1;
                                continue;
                            }
                            '.' => {
                                self.state = LexerState::Float;
                                self.end += 1;
                                continue;
                            }
                            _ =>{}
                        }
                    }
                    let number = &self.input[self.begin..self.end];
                    // NOTE: we could directly compute the i32 while parsing
                    // instead of reparsing it with number.parser,
                    // it would probably be faster
                    self.state = LexerState::Start;
                    self.begin = self.end;
                    return Token::IntNumber(number.parse::<i32>().unwrap());
                }
                LexerState::Float => {
                    let cursor = &self.input[self.end..self.input.len()];
                    if let Some(c) = cursor.chars().next() {
                        match c {
                            '0'...'9' | 'e' | '-' => {
                                self.end += 1;
                                continue;
                            }
                            _ =>{}
                        }
                    }
                    let number = &self.input[self.begin..self.end];
                    self.state = LexerState::Start;
                    self.begin = self.end;
                    // TODO : the lexer captures string like 900.-40 which are not floats
                    //        => return an appropriate error
                    return Token::FloatNumber(number.parse::<f32>().unwrap());
                }
                LexerState::StringLiteral => {
                    let cursor = &self.input[self.end..self.input.len()];
                    let next = cursor.chars().next();
                    if next == Some('"') {
                        let ltr_str = &self.input[self.begin..self.end];
                        self.end += 1;
                        self.begin = self.end;
                        self.state = LexerState::Start;
                        return Token::StringLiteral(ltr_str.to_string());

                    } else if next == None {
                        return Token::EOF;
                    }
                    self.end += 1;
                }
            }
        }
    }
}

// Stores temporary parsing data
pub struct RfxFileFormat<'a> {
    reader: BufReader<&'a File>,
    current_line: String, // line_bytes: usize,
}

impl<'a> RfxFileFormat<'a> {
    /// Returns a new parser data
    pub fn new(file: &File) -> RfxFileFormat {
        RfxFileFormat {
            reader: BufReader::new(file),
            current_line: String::with_capacity(1024), // line_bytes: 0,
        }
    }

    /// Update a project with the content of the parsed file
    /// Returns the updated project
    pub fn update_project(&mut self, mut project: Project) -> Project {

        // For each line, get the tokens
        while let Ok(bytes_read) = self.next_line() {
            if bytes_read == 0 {
                return project;
            }
            // NOTE: new lexer for each line, so we are yet unable to
            //       parser multiline strings
            //       IT DOES NOT WORK !!!!!
            let mut lexer = Lexer::new(&self.current_line);
            let token = lexer.next_token();
            match token { // top level should be a command
                Token::Command(c) => {
                    println!("found command {:?}", c);
                    // find command function (lexer, project)
                    // but testing with node atm
                    node(& mut lexer, & mut project);
                } 
                _ => {
                    // TODO : return error
                    panic!("error: expecting a command, got xxxx");
                }
            }
        }
        project
    }

    /// Next line
    fn next_line(&mut self) -> Result<usize, Error> {
        self.current_line.clear();
        self.reader.read_line(&mut self.current_line)
    }


///
/// Parse a node in the top level
///
fn node(lexer: &mut Lexer, project: &mut Project) {
    println!("node");    
    // Expect a string literal which is the plugin name
    if let Token::StringLiteral(plugin_name) = lexer.next_token() {
        let node_handle = project.new_node(&plugin_name);
        println!("{:?}", node_handle);
        //
        // Once we have a new node, we can continue parsing in context or just returning
        //
        match lexer.next_token() {
            Token::OpenBrace => {
                // Parse commands in the node context
                node_commands(lexer, project, & mut node_handle.unwrap())
            } 
            Token::SemiColon => {
                return;
            }
            _ => {
                panic!("syntax error, expecting ';' or '{'");
            }
        }
    } else {
        panic!("syntax error, expecting quoted string with the name of the plugin");
    }
}

fn node_commands(lexer: &mut Lexer, project: &mut Project, node: &mut NodeHandle) {
    loop {
        let t = lexer.next_token();
        match t {
            Token::CloseBrace => return,
            _ => {
                println!("unexpected token {:?}", t);
                return;
            }
        }
    } 
}

#[cfg(test)]
use std::path::PathBuf;

#[test]
fn lexer_token_openbrace() {
    let obrace = "{".to_string();
    let mut lexer = Lexer::new(&obrace);

    assert!(lexer.next_token() == Token::OpenBrace);
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_closebrace() {
    let cbrace = "}".to_string();
    let mut lexer = Lexer::new(&cbrace);
    assert!(lexer.next_token() == Token::CloseBrace);
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_space() {
    let spaces = "   {   }   ".to_string();
    let mut lexer = Lexer::new(&spaces);
    assert!(lexer.next_token() == Token::OpenBrace);
    assert!(lexer.next_token() == Token::CloseBrace);
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_command_node() {
    let node = "node".to_string();
    let mut lexer = Lexer::new(&node);
    assert!(lexer.next_token() == Token::Command(CommandType::Node));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_command_param() {
    let param = "param".to_string();
    let mut lexer = Lexer::new(&param);
    assert!(lexer.next_token() == Token::Command(CommandType::Param));
}

#[test]
fn lexer_token_command_path() {
    let path = "path".to_string();
    let mut lexer = Lexer::new(&path);
    assert!(lexer.next_token() == Token::Command(CommandType::Path));
}

#[test]
fn lexer_token_command_property() {
    let property = "property".to_string();
    let mut lexer = Lexer::new(&property);
    assert!(lexer.next_token() == Token::Command(CommandType::Property));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_command_followed_by_brace() {
    let path = "path{".to_string();
    let mut lexer = Lexer::new(&path);
    assert!(lexer.next_token() == Token::Command(CommandType::Path));
    assert!(lexer.next_token() == Token::OpenBrace);
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_comment() {
    let comment = "# comment ".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_comment_followed_by_command() {
    let comment = "# comment \nnode".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::Command(CommandType::Node));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_comment_surrounded_by_command() {
    let comment = "path # comment \nnode  ".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::Command(CommandType::Path));
    assert!(lexer.next_token() == Token::Command(CommandType::Node));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_string_literal() {
    let comment = "\"a string\"".to_string();
    let mut lexer = Lexer::new(&comment);
    // println!("next token = {:?}", lexer.next_token());
    assert!(lexer.next_token() == Token::StringLiteral("a string".to_string()));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_integer() {
    let comment = "3102954".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::IntNumber(3102954));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_negative_integer() {
    let comment = "-3102954".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::IntNumber(-3102954));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_multiple_integer() {
    let comment = "3102954 56 89233".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::IntNumber(3102954));
    assert!(lexer.next_token() == Token::IntNumber(56));
    assert!(lexer.next_token() == Token::IntNumber(89233));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_float() {
    let comment = "3102954.67".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::FloatNumber(3102954.67));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_float_starting_with_zero() {
    let comment = "0.3102954 0.67".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::FloatNumber(0.3102954));
    assert!(lexer.next_token() == Token::FloatNumber(0.67));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_negative_float() {
    let comment = "-3102954.67".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::FloatNumber(-3102954.67));
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn lexer_token_scientific_float() {
    let comment = "-314.67e10".to_string();
    let mut lexer = Lexer::new(&comment);
    assert!(lexer.next_token() == Token::FloatNumber(-314.67e10));
    assert!(lexer.next_token() == Token::EOF);
    // TODO: quickcheck with float numbers
}

#[test]
fn parse_unique_node() {
   let mut path = PathBuf::from(file!());
   path.pop();
   path.pop();
   path.pop();
   path.push("tests/projects/1.rfx");
   match File::open(&path) {
       Ok(file) => {
           let mut project = Project::new();
           let mut parser = RfxFileFormat::new(&file);
           project = parser.update_project(project);
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
