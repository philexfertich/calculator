#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Paren {
    Left,
    Right,
}

#[derive(Debug)]
pub enum Token {
    Literal(usize, usize),
    Operator(usize, Operation),
    Paren(usize, Paren)
}

#[derive(PartialEq, Debug)]
enum State {
    Ini,
    Lit,
}

fn is_num(c: char) -> bool {
    "0123456789".contains(c)
}

fn is_dot(c: char) -> bool {
    c == '.'
}


fn check_punct(c: char) -> Option<Paren> {
    match c {
        '(' => Some(Paren::Left),
        ')' => Some(Paren::Right),
        _ => None,
    }
}

fn is_sp(c: char) -> bool {
    c == ' '
}

fn check_op(c: char) -> Option<Operation> {
    match c {
        '+' => Some(Operation::Add),
        '-' => Some(Operation::Sub),
        '*' => Some(Operation::Mul),
        '/' => Some(Operation::Div),
        _ => None,
    }
}

use State::{ Ini, Lit };


pub fn tokenize(expr: &str) -> Option<Vec<Token>> {
    if expr.len() == 0 {
        return None;
    }

    let mut iter = expr.chars().enumerate();

    let mut tokens = Vec::<Token>::new();
    
    let mut state = Ini;

    let mut buf: usize = 0;

    let mut dot: bool = false;

    loop {
        match state {
            Ini => {
                let Some((i, c)) = iter.next() else {
                    break Some(tokens);
                };

                if let Some(op) = check_op(c) {
                    tokens.push(Token::Operator(i, op));
                } else if let Some(punct) = check_punct(c) {
                    tokens.push(Token::Paren(i, punct));
                } else if is_num(c) {
                    buf = i;
                    state = Lit;
                } else if !is_sp(c) {
                    println!("Wrong character {c} at {i} position.");
                    break None;
                }
            },
            Lit => {
                let Some((i, c)) = iter.next() else {
                    tokens.push(Token::Literal(buf, expr.len()));            
                    break Some(tokens);
                };

                if !is_num(c) && !is_dot(c) {
                    dot = false;
                    tokens.push(Token::Literal(buf, i));
                    state = Ini;

                    if let Some(op) = check_op(c) {
                        tokens.push(Token::Operator(i, op));
                    } else if let Some(punct) = check_punct(c) {
                        tokens.push(Token::Paren(i, punct));
                    } else if !is_sp(c) {
                        let mut indices = expr.char_indices().map(|(i, _)| i);
                        let l = indices.nth(buf).unwrap();
                        let r = indices.nth(i).unwrap_or(expr.len());
                        println!("Wrong literal {} at {i} position", &expr[l..r]);
                        break None; 
                    }
                } else if is_dot(c) && dot {
                    println!("Additional dot added.");
                    break None;
                } else if is_dot(c) {
                    dot = true;
                }
            }
        }
    }
}