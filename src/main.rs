use std::io;

use calculator::token::tokenize;


fn main() {
    let mut expression = String::new();
    
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    println!("Input an expression: {}", expression.trim());


    let tokens = tokenize(&expression.trim());

    println!("Tokens: {tokens:?}");

    
}
