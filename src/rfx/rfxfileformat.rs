///
/// RFX file format parser
///

use std::fs::File;
use std::io::Read;
use rfx::project::{Project, Node};
use rfx::project::NodeHandle;
use std::collections::HashMap;
use std::ffi::CString;


/// Recognized commands
#[derive(Debug, PartialEq, Clone)]
enum CommandType {
    Name = 0, // set name
    Node, // create node
    Param, // set param
    Path, // plugin path
    Property, // set property
}

lazy_static! {
    // List of recognized commands
    static ref COMMAND_MAP: HashMap<&'static str, CommandType> = {
        let mut commands = HashMap::new();
        commands.insert("name", CommandType::Name);
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
                            ' ' | '\n' | '\t' | '\r' => {
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
                            c @ _ => {
                                panic!("character {} not recognized", c);
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
                    match COMMAND_MAP.get(cmd_str) {
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
                            _ => {}
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
                            _ => {}
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

/// Stores the content to parse
/// For now storing the whole project file in a string is ok,
/// we might have to used buffered read later on if the projects gets really big 
pub struct RfxFileFormat {
    content: String,
}

impl RfxFileFormat {
    /// Returns a new parser data
    pub fn new(file: &mut File) -> RfxFileFormat {
        let mut content: String = String::new();
        file.read_to_string(&mut content)
            .unwrap_or_else(|_| panic!("unable to read content of the file"));
        RfxFileFormat { content: content }
    }

    /// Update a project with the content of the parsed file
    /// Returns the updated project
    pub fn update_project(&mut self, mut project: Project) -> Project {

        let mut lexer = Lexer::new(&self.content);
        loop {
            let token = lexer.next_token();
            match token { // at top level we should only find a command
                Token::Command(c) => {
                    trace!("found command {:?}", c);
                    if c == CommandType::Node {
                        node(&mut lexer, &mut project);
                    }
                } 
                Token::EOF => {
                    break;
                }
                _ => {
                    // TODO : return error
                    panic!("error: expecting a command, got xxxx");
                }
            }
        }
        project
    }
}

///
/// Parse a node in the top level
///
fn node(lexer: &mut Lexer, mut project: &mut Project) {
    // Expect a string literal which is the plugin name
    if let Token::StringLiteral(plugin_name) = lexer.next_token() {
        //
        let mut node = project.node_new(&plugin_name);
        let mut node_name = String::new();

        // Once we have a new node, we can continue parsing in context or just returning
        match lexer.next_token() {
            Token::OpenBrace => {
                // Parse commands in the node context
                node_name = node_commands(lexer, &mut node);
            } 
            Token::SemiColon => {}
            _ => {
                panic!("syntax error after node, expecting ';' or '{'");
            }
        }
        // TODO: get the name/id of the node from the commands ?
        project.node_insert(node_name, node);
    } else {
        panic!("syntax error, expecting quoted string with the name of the plugin");
    }
}

/// WIP
fn node_param(lexer: &mut Lexer, node: &mut Node, param_name: String) {

    // internal parser state ??
    // 0 => start
    // 1 => found 1 int
    // 2 => found 1 float
    let mut ints: [i32; 3] = [0; 3];
    let mut idx: usize = 0;
    loop {
        // read parameter values until a semicolon is found
        let t = lexer.next_token();
        match t {
            Token::FloatNumber(_) => {
                panic!("no float parameter yet");
            } 
            Token::IntNumber(n) => {
                if idx > 2 {
                    panic!("found more than 3 ints in parameter xxx, node xxx");
                }
                ints[idx] = n;
                idx += 1;
            }
            Token::SemiColon => {
                if idx == 1 {
                    let c_param_name = CString::new(param_name.as_str()).unwrap();
                    let mut param = node.parameters().param_get(&c_param_name).unwrap();
                    param.int1_set(ints[0]);
                }
            }
            _ => {
                panic!("expected semicolon or xxxxx");
            }

        }
    }
}

fn node_commands(lexer: &mut Lexer, node: &mut Node) -> NodeHandle {
    let mut node_name: NodeHandle = String::new();
    loop {
        // read commands inside the context of a node
        let t = lexer.next_token();
        match t {
            Token::Command(CommandType::Name) => {
                if let Token::StringLiteral(node_name_) = lexer.next_token() {
                    node_name = node_name_;
                }
                if Token::SemiColon != lexer.next_token() {
                    panic!("expected semicolon after name");
                }
            }
            Token::Command(CommandType::Param) => {
                if let Token::StringLiteral(param_name) = lexer.next_token() {
                    node_param(lexer, node, param_name);
                } else {
                    panic!("expected string literal after param");
                }
            }
            Token::CloseBrace => {
                break;
            }
            _ => {
                println!("unexpected token {:?}", t);
                break;
            }
        }
    }
    node_name
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
        Ok(mut file) => {
            let mut project = Project::new();
            let mut parser = RfxFileFormat::new(&mut file);
            project = parser.update_project(project);
            // read one node ?
            assert!(project.node_qty() == 1);
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
fn parse_two_named_nodes() {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.pop();
    path.pop();
    path.push("tests/projects/2.rfx");
    match File::open(&path) {
        Ok(mut file) => {
            let mut project = Project::new();
            let mut parser = RfxFileFormat::new(&mut file);
            project = parser.update_project(project);
            assert!(project.node_qty() == 2);
            assert!(project.node_get("Gain.1".to_string()).is_some());
            assert!(project.node_get("Gain.2".to_string()).is_some());
            assert!(project.node_get("Gain.3".to_string()).is_none());
        }
        Err(_) => {
            panic!("unable to open {:?}", &path);
            assert!(false);
        }
    }
}
