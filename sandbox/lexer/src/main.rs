use std::io;
use std::io::BufReader;
use std::fs::File;
use std::io::Read;
use std::str::CharIndices;
use std::str::Chars;
use std::path::PathBuf;

/// Test parsing in rust
/// TODO : return error
///


#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    LoadPluginCommand,
    CreateNodeCommand,
    ConnectNodeCommand,
    SemiColon,
    StringLiteral(String),
    FloatNumber(f32),
    IntNumber(i32),
    EOF,
}

struct Lexer<'a> {
    input: &'a str,
    last_pos: usize,
}

// This macro is here because the borrow checked disallow to use a function
// that does the same. This BC is utterly stupid
macro_rules! cursor {
    ($var:ident, $ret:expr) => {
        unsafe {
            if $var.last_pos + 1 > $var.input.len() {
                return $ret;
            }
            $var.input.slice_unchecked($var.last_pos, $var.input.len())
        }
    }    
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Self {
        Lexer {
            input: input,
            last_pos: 0,
        }
    }

    fn skip_whitespaces(&mut self) {
        let mut chars = cursor!(self, ()).chars();
        loop {
            let c = chars.nth(0);
            match c {
                Some(s) => {
                    if s.is_whitespace() {
                        self.last_pos += 1;
                        if self.last_pos + 1 > self.input.len() {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        let cursor = cursor!(self, Token::EOF);
        if cursor.starts_with("loadplugin") {
            self.last_pos += "loadplugin".len();
            return Token::LoadPluginCommand;
        } else if cursor.starts_with("node") {
            self.last_pos += "node".len();
            return Token::CreateNodeCommand;
        } else if cursor.starts_with("\"") {
            self.last_pos += "\"".len();
            let mut string_end = self.last_pos;
            let mut chars = cursor!(self, Token::EOF).chars();
            loop {
                match chars.next() {
                    Some(a) => {
                        if a == '"' {
                            break;
                        } else {
                            string_end += 1;
                        }
                    }
                    None => return Token::EOF,
                }
            }
            let captured = unsafe { self.input.slice_unchecked(self.last_pos, string_end) };
            self.last_pos = string_end + 1;
            return Token::StringLiteral(captured.to_string());


            // TODO : float and int

        } else {
            // Capture all number characters 
            let mut chars = cursor.chars();
            let mut string_end = self.last_pos;
            loop {
                match chars.next() {
                    Some(c) => {
                        match c {
                            '0'...'9' | '.' | 'e' | '+' | '-' => {
                                string_end += 1;
                            }
                            _ => break,
                        }
                    }
                    None => break,    
                }
            }
            //let captured = unsafe { self.input.slice_unchecked(self.last_pos, string_end) };
            let captured = &self.input[self.last_pos..string_end];
            self.last_pos = string_end;
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


fn main() {
    let mut path = PathBuf::from(file!());
    path.pop();
    path.push("test1.rfx");
    let mut file = File::open(path).unwrap();

    let mut file_content = String::new();
    file.read_to_string(&mut file_content);
    // loop {
    //    let token = next_token(&mut file_content.char_indices());
    //    match token {
    //        Token::EOF => {
    //            break;
    //        }
    //        _ => println!("{:?}", token),
    //    }
    // }

}

#[test]
fn test_token_eof() {
    let text1 = "".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::EOF);
}

#[test]
fn test_token_loadplugin() {
    let text1 = "loadplugin".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::LoadPluginCommand);
}

#[test]
fn test_token_loadplugin_with_spaces() {
    let text1 = "  loadplugin  ".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::LoadPluginCommand);
}

#[test]
fn test_token_createnode() {
    let text1 = "node".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::CreateNodeCommand);
}

#[test]
fn test_two_tokens() {
    let text1 = "loadplugin node".to_string();
    let mut lexer = Lexer::new(&text1);
    assert!(lexer.next_token() == Token::LoadPluginCommand);
    assert!(lexer.next_token() == Token::CreateNodeCommand);
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

    assert!(lexer.next_token() == Token::LoadPluginCommand);
    assert!(lexer.next_token() == Token::StringLiteral("uk.co.thefoundry.BasicGain".to_string()));
    
}

