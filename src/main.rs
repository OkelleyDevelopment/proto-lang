use lexer::lexer::Lexer;

mod token;
mod lexer;

fn main() {
    //let input = String::from("1 + 3 = 4;").chars().collect();
    let input = String::from("\n 4;").chars().collect();
    let mut lex = Lexer::new(input);
    lex.next();
    lex.advance_whitespace();
    lex.advance_whitespace();
    
    println!("{:#?}", lex.input);
    println!("{:?}", lex);

}
