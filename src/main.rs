use lexer::lexer::Lexer;

use crate::token::{Token, Types};

mod token;
mod lexer;

fn main() {
    //let input = String::from("1 + 3 = 4;").chars().collect();
    println!("---- testing the Lexer ----");
    let input = String::from("\n 4;").chars().collect();
    let mut lex = Lexer::new(input);
    lex.next();
    lex.advance_whitespace();
    lex.advance_whitespace();
    
    println!("{:#?}", lex.input);
    println!("{:?}", lex);
    let _res = lex.next_token();
    println!("\nTesting Generic Struct with custom formatted display");
    println!("-------- {{ Enum , Data }}");
    let token = Token::new(Types::Integer, 1);
    token.display();
    let token2 = Token::new(Types::EOF, '0');
    token2.display();
    println!("Are the tokens equal? ---> {}", token2.is_equal(token.get_token_type()));

}
