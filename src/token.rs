use std::fmt::Debug;
use std::fmt;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token<T> {
    token_type: Types,
    value: T
}

impl<T: Debug> Token<T> {
    fn new(token_type: Types, value: T) -> Self {
        Self {
            token_type,
            value
        }
    }
    fn println_t(self) {
        println!("{:?}", self.value);
    }

    fn display(self) {
        println!("Token --> {}", self.token_type);
        self.println_t()
    }

    fn is_equal(&self, target: Types) -> bool {
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




