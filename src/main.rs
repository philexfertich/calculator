use std::io;

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Paren {
    Left,
    Right,
}

#[derive(Debug)]
enum Token {
    Literal(String),
    Operator(Operation),
    Paren(Paren)
}

enum State {
    Init,
    End,
    Num,
}

fn main() {
    let mut expression = String::new();
    
    io::stdin()
        .read_line(&mut expression)
        .expect("Failed to read line.");

    println!("Input an expression: {expression}");    
}
