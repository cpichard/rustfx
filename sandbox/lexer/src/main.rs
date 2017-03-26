use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::str::CharIndices;
use std::str::Chars;
use std::path::PathBuf;

///
/// Test parsing in rust
///
/// TODO: return meaningful parsing errors
/// TODO: test "toto".len() is optimized and compiles directly with the len of the string
#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    PluginCommand,
    NodeCommand,
    ConnectNodeCommand,
    PathCommand,
    ParamCommand,
    PropertyCommand,
    SemiColon,
    StringLiteral(String),
    FloatNumber(f32),
    IntNumber(i32),
    EOF,
}

struct Lexer<'a> {
    input: &'a str,
    begin: usize,
    end: usize,
}

// This macro is here because the borrow checked disallow to use a function
// that does the same. This BC is utterly stupid
//
// slice_unchecked could be replaced by input[self.begin..self.end]
macro_rules! cursor {
    ($var:ident, $ret:expr) => {
        unsafe {
            if $var.begin + 1 > $var.input.len() {
                return $ret;
            }
            $var.input.slice_unchecked($var.begin, $var.input.len())
        }
    }    
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Lexer {
            input: input,
            begin: 0,
            end: 0,
        }
    }

    fn skip_whitespaces(&mut self) {
        let mut chars = cursor!(self, ()).chars();
        loop {
            match chars.next() {
                Some(s) => {
                    if s.is_whitespace() {
                        self.begin += 1;
                        self.end = self.begin;
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespaces(); // could integrate skip_whitespace in the loop
            let cursor = cursor!(self, Token::EOF);
            // match a comment
            if cursor.starts_with("#") {
                // Eat until end of line comment and continue the main loop
                let mut chars = cursor.chars();
                loop {
                    match chars.next() {
                        Some('\n') => {
                            self.end += 1;
                            break;
                        }
                        Some(c) => self.end += 1,
                        None => break,  // Error or eof ?
                    }
                }
                self.begin = self.end;
                continue;
            } else if cursor.starts_with(";") {
                self.begin += ";".len();
                self.end = self.begin;
                return Token::SemiColon;
            } else if cursor.starts_with("{") {
                self.begin += "{".len();
                self.end = self.begin;
                return Token::OpenBrace;
            } else if cursor.starts_with("}") {
                self.begin += "}".len();
                self.end = self.begin;
                return Token::CloseBrace;
            } else if cursor.starts_with("plugin") {
                self.begin += "plugin".len();
                self.end = self.begin;
                return Token::PluginCommand;
            } else if cursor.starts_with("path") {
                self.begin += "path".len();
                self.end = self.begin;
                return Token::PathCommand;
            } else if cursor.starts_with("node") {
                self.begin += "node".len();
                self.end = self.begin;
                return Token::NodeCommand;
            } else if cursor.starts_with("param") {
                self.begin += "param".len();
                self.end = self.begin;
                return Token::ParamCommand;
            } else if cursor.starts_with("\"") {
                self.begin += "\"".len();
                self.end = self.begin;
                let mut chars = cursor!(self, Token::EOF).chars();
                loop {
                    match chars.next() {
                        Some(a) => {
                            if a == '"' {
                                break;
                            } else {
                                self.end += 1;
                            }
                        }
                        None => return Token::EOF,
                    }
                }
                let captured = &self.input[self.begin..self.end];
                self.end += 1;
                self.begin = self.end;
                return Token::StringLiteral(captured.to_string());
            } else {
                // Capture all number characters
                let mut chars = cursor.chars();
                loop {
                    match chars.next() {
                        Some(c) => {
                            match c {
                                '0'...'9' | '.' | 'e' | '+' | '-' => {
                                    self.end += 1;
                                }
                                _ => break,
                            }
                        }
                        None => break,    
                    }
                }
                let captured = &self.input[self.begin..self.end];
                self.begin = self.end;
                // First try integer
                let int_parsed = captured.parse::<i32>();
                if int_parsed.is_ok() {
                    return Token::IntNumber(int_parsed.unwrap());
                }
                // else try float
                let float_parsed = captured.parse::<f32>();
                if float_parsed.is_ok() {
                    return Token::FloatNumber(float_parsed.unwrap());
                }
                // TODO : return and error
                return Token::EOF;
            }
        }
    }
}

fn node_commands(lexer: &mut Lexer) {
    
    let token = lexer.next_token();
    match token {
        Token::ParamCommand => {
            println!("parameter ");
            // Allocate Node data and context
        }
        Token::PropertyCommand => {

        }
        _ => println!("unable to parse node name"),
    }
    
}

/// Parse a Node
fn node(lexer: &mut Lexer) {
    // eat node token
    println!("found node");
    let token = lexer.next_token();
    match token {
        Token::StringLiteral(plugin_name) => {
            println!("node plugin name is {}", plugin_name);
            // Allocate Node data and context
        }
        _ => println!("unable to parse node name"),
    }

    let token = lexer.next_token();
    match token {
        Token::OpenBrace => {
            println!("entering node context commands");
            node_commands(lexer); // TODO pass node
        }
        _ => println!("expected node context"),
    }
}

fn main() {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.push("test1.rfx");
    let mut file = File::open(path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content);
    let mut lexer = Lexer::new(&file_content);
    loop {
        let token = lexer.next_token();
        match token {
            Token::NodeCommand => {
                // eat node
                node(& mut lexer);
                // Change state
                // expect
                break;
            }
            _ => println!("{:?}", token),
        }
    }

}

#[test]
fn test_token_eof() {
    let text1 = "".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn test_token_plugin() {
    let text1 = "plugin".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::PluginCommand);
}

#[test]
fn test_token_plugin_with_spaces() {
    let text1 = "  plugin  ".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::PluginCommand);
}

#[test]
fn test_token_createnode() {
    let text1 = "node".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::NodeCommand);
}

#[test]
fn test_two_tokens() {
    let text1 = "plugin node".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::PluginCommand);
    assert!(lexer.next_token() == Token::NodeCommand);
}

#[test]
fn test_string_literal() {
    let text1 = "\"literal string test\"".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::StringLiteral("literal string test".to_string()));
}

#[test]
fn test_two_string_literal() {
    let text1 = "\"literal string test\"\"new text\"".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::StringLiteral("literal string test".to_string()));
    assert!(lexer.next_token() == Token::StringLiteral("new text".to_string()));
}

#[test]
fn test_unfinished_string_literal() {
    let text1 = "\"literal string test".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn test_float() {
    let text1 = "20.2334".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::FloatNumber(20.2334));
}

#[test]
fn test_int() {
    let text1 = "202334".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::IntNumber(202334));
}

#[test]
fn test_int_and_float() {
    let text1 = "202334 982.345".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::IntNumber(202334));
    assert!(lexer.next_token() == Token::FloatNumber(982.345));
}

#[test]
fn test_int_semicolon_float() {
    let text1 = "202334;982.345".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::IntNumber(202334));
    assert!(lexer.next_token() == Token::SemiColon);
    assert!(lexer.next_token() == Token::FloatNumber(982.345));
}

#[test]
fn test_file() {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.push("test1.rfx");
    let mut file = File::open(path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content);
    let mut lexer = Lexer::new(&file_content);

    assert!(lexer.next_token() == Token::PathCommand);
    assert!(lexer.next_token() == Token::StringLiteral("".to_string()));
    assert!(lexer.next_token() == Token::SemiColon);
    assert!(lexer.next_token() == Token::PluginCommand);
    assert!(lexer.next_token() == Token::StringLiteral("uk.co.thefoundry.BasicGain".to_string()));
    assert!(lexer.next_token() == Token::SemiColon);
    assert!(lexer.next_token() == Token::NodeCommand);
    assert!(lexer.next_token() == Token::StringLiteral("uk.co.thefoundry.BasicGain".to_string()));
    assert!(lexer.next_token() == Token::OpenBrace);
    assert!(lexer.next_token() == Token::ParamCommand);
    assert!(lexer.next_token() == Token::StringLiteral("name".to_string()));
    assert!(lexer.next_token() == Token::StringLiteral("testnode1".to_string()));
    assert!(lexer.next_token() == Token::SemiColon);
    assert!(lexer.next_token() == Token::ParamCommand);
    assert!(lexer.next_token() == Token::StringLiteral("offset".to_string()));
    assert!(lexer.next_token() == Token::FloatNumber(0.89));
    assert!(lexer.next_token() == Token::SemiColon);
    assert!(lexer.next_token() == Token::ParamCommand);
    assert!(lexer.next_token() == Token::StringLiteral("color".to_string()));
    assert!(lexer.next_token() == Token::IntNumber(23));
    assert!(lexer.next_token() == Token::IntNumber(23));
    assert!(lexer.next_token() == Token::IntNumber(10));

}

#[test]
fn test_token_2_lines() {
    let text1 = "node\nnode\n".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::NodeCommand);
    assert!(lexer.next_token() == Token::NodeCommand);
}

#[test]
fn test_token_2_comments() {
    let text1 = "# random comment\nnode# and a new comment\nnode".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::NodeCommand);
    assert!(lexer.next_token() == Token::NodeCommand);
}

#[test]
fn test_token_comment() {
    let text1 = "# random comment\nnode".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::NodeCommand);
}

#[test]
fn test_skip_linebreaks() {
    let text1 = "\n\n\n\n\nnode".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::NodeCommand);
}

#[test]
fn test_token_open_close_brace() {
    let text1 = "node {\nnode}".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::NodeCommand);
    assert!(lexer.next_token() == Token::OpenBrace);
    assert!(lexer.next_token() == Token::NodeCommand);
    assert!(lexer.next_token() == Token::CloseBrace);
}
