const ZERO: usize = 0;
const NULL_CHAR: char = '0';

#[derive(Debug)]
pub struct Lexer {
    pub input: String,
    pos: usize,
    curr_pos: usize,
    curr_char: char,
}


impl Lexer {
    pub fn new(input: String) -> Self {
       Self {
           input,
           pos: ZERO,
           curr_pos: ZERO,
           curr_char: NULL_CHAR
       } 
    }
}

#[test]
fn can_instantiate_lexer() {
    let input = String::from("1 + 3 = 4;");
    let lex = Lexer::new(input.clone());
    assert_eq!(lex.input, input);
    assert_eq!(lex.curr_pos, ZERO);
    assert_eq!(lex.curr_char, NULL_CHAR);
}

