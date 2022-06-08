use core::fmt;

use crate::token::{Token, Types};

const ZERO: usize = 0;
const NULL_CHAR: char = '0';

// TODO: Move this to an error util crate ? 
pub enum SErr {
    Reason(String)
}

impl fmt::Display for SErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            SErr::Reason(r) => r.clone()
        };

        write!(f, "{}", string)
    }
}

#[derive(Debug)]
pub struct Lexer {
    pub input: Vec<char>,
    pos: usize,
    curr_pos: usize,
    curr_char: char,
}


impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
       Self {
           input,
           pos: ZERO,
           curr_pos: ZERO,
           curr_char: NULL_CHAR
       } 
    }

    pub fn advance_whitespace(&mut self) {
        let c: char = self.curr_char;
        if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            self.next();
        }
    }

    pub fn next(&mut self) {

        if self.curr_pos >= self.input.len() {
            self.curr_char = '0'
        } else {
            self.curr_char = self.input[self.curr_pos];
        }

        self.pos = self.curr_pos;
        self.curr_pos = self.pos + 1;
    }

    fn _handle_numerics(&mut self) {
        unimplemented!()
        
    }

    fn _handle_indentifiers(&mut self) {
        unimplemented!()
        
    }

    pub fn next_token(&mut self) -> Result<Token<String>, SErr> {

        match self.curr_char {
            '+' => Ok(Token::new(Types::Plus, String::from('+'))),
            _ => {
               Err(SErr::Reason("Token not supported".to_string()))
            }
        }

    }

    pub fn _tokenize(&mut self) -> Result<Vec<Token<Types>>, SErr> {
        unimplemented!()
    }
}


#[test]
fn can_instantiate_lexer() {
    let input: Vec<char> = "1 + 3 = 4;".chars().collect();
    let input_copy = input.clone();
    let lex = Lexer::new(input);
    assert_eq!(lex.input, input_copy);
    assert_eq!(lex.curr_pos, ZERO);
    assert_eq!(lex.curr_char, NULL_CHAR);
}

#[test]
fn can_skip_whitespace() {
    let input = String::from("\n  4");
    let mut lex = Lexer::new(input.chars().collect());
    // start the tokenizing
    lex.next();
    // skip the 2 white space tokens
    lex.advance_whitespace();
    lex.advance_whitespace();
    // "resume" tokenizing
    lex.next();
 
    assert_eq!(lex.pos, 3);
    assert_eq!(lex.curr_pos, 4);
    assert_eq!(lex.curr_char.clone(), '4');
}


#[test]
fn checks_bounds_correctly() {
    let input = String::from("\n  4");
    let mut lex = Lexer::new(input.chars().collect());
    lex.curr_pos = 4;
    lex.next();
    assert_eq!(lex.curr_char.clone(), '0');
}

