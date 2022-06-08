use std::fmt::Debug;
use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Types {
    Integer, 
    Plus,
    EOF
}

impl fmt::Display for Types  {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let string = match self {
          Types::Integer => "Integer",
          Types::Plus => "PlusOp",
          Types::EOF => "EOF"
      };
      write!(f, "{}", string)
   }
}

#[derive(Debug, Copy, Clone)]
pub struct Token<T> {
    token_type: Types,
    value: T
}

impl<T: Clone + Debug> Token<T> {
    pub fn new(token_type: Types, value: T) -> Self {
        Self {
            token_type,
            value
        }
    }

    pub fn get_token_type(&self) -> Types {
       self.token_type 
    }

    pub fn _get_value(&self) -> T {
       let v: T = self.value.clone();
       return v;
    }

    pub fn display(&self) {
        println!("Token --> {}, {:?}", self.token_type, self.value);
    }

    pub fn is_equal(&self, target: Types) -> bool {
        match (&self.token_type, target) {
            (Types::Plus, Types::Plus) => true,
            (Types::EOF, Types::EOF) => true,
            (Types::Integer, Types::Integer) => true,
            _ => false,
        }
    }
}

#[test]
fn test_equal() {
    let token = Token::new(Types::Plus,"+");
    let want = true;
    assert_eq!(token.is_equal(Types::Plus), want);
}

#[test]
fn test_value() {
    let token = Token::new(Types::Plus,"+");
    let plusop = "+";
    assert_eq!(token.value, plusop);
}

#[test]
fn test_value_not_equal() {
    let eof = Types::EOF;
    let token = Token::new(Types::Plus,"+");
    let want = false;
    assert_eq!(token.is_equal(eof), want);
}




