use std::io;

use calculator::lexer::{LexerError, tokenize};


fn main() {
    
    let tokens = loop {
        let mut expression = String::new();
        io::stdin()
            .read_line(&mut expression)
            .expect("Failed to read line.");

        println!("Input an expression: {}", expression.trim());

        match tokenize(&expression.trim()) {
            Ok(tks) => break tks,
            Err(e) => {
                match e {
                    LexerError::EmptyExpr => println!("Enter expression."),
                    LexerError::UnexpChar(msg) => println!("{}", msg),
                    LexerError::WrongLiteral(msg) => println!("{}", msg),
                }
                continue;
            }
        }
    };

    println!("Tokens: {tokens:?}");
    
}
