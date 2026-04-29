use std::io;

use calculator::{Tokens, Error};


fn main() {
    let mut expression = String::new();
    
    let tokens = loop {
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

    println!("Tokens: {:?}", tokens.get_tokens());
}
