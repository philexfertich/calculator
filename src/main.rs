use std::io;

use calculator::{Error, Tokens, parser::op::Operator};

// use calculator::parser::parse;


fn main() {
    let mut expression = String::new();
    
    let mut tokens = loop {
        io::stdin()
            .read_line(&mut expression)
            .expect("Failed to read line.");
        
        println!("Input an expression: {}", expression.trim());

        match Tokens::from(expression.trim()) {
            Ok(tks) => break tks,
            Err(e) => {
                match e {
                    Error::EmptyExpr => println!("Enter expression."),
                    Error::UnexpChar(msg) => println!("{}", msg),
                    Error::WrongLiteral(msg) => println!("{}", msg),
                }
                continue;
            }
        }
    };
    println!("Tokens: {:?}", tokens.tokens);
}
