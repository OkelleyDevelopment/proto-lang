use lexer::lexer::Lexer;

mod token;
mod lexer;

fn main() {
    let input = String::from("1 + 3 = 4;");
    let lex = Lexer::new(input);
    
    println!("{}", lex.input);

}
