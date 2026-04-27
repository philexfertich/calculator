#[derive(Debug)]
pub enum Error {
    EmptyExpr,
    UnexpChar(String),
    WrongLiteral(String),
}

#[derive(Clone, Debug)]
pub enum Op {
    Positive,
    Negative,
    Asterisk,
    Slash,
    Exponent,
    Scientific,
}

#[derive(Clone, Debug)]
pub enum Paren {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub enum Token {
    Literal(usize, usize),
    Operator(usize, Op),
    Paren(usize, Paren),
}

enum State {
    Ini,
    Lit,
}

use State::{ Ini, Lit };

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

fn check_op(c: char) -> Option<Op> {
    match c {
        '+' => Some(Op::Positive),
        '-' => Some(Op::Negative), // ASCII hyphen
        '−' => Some(Op::Negative), // Unicode minus (U+2212)
        '*' => Some(Op::Asterisk),
        '/' => Some(Op::Slash),
        '^' => Some(Op::Exponent),
        'E' => Some(Op::Scientific),
        _ => None,
    }
}

pub struct Tokens {
    expr: String,
    tokens: Vec<Token>,
}

impl Tokens {
    
    pub fn from(expr: &str) -> Result<Self, Error> {
        if expr.len() == 0 {
            return Err(Error::EmptyExpr);
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
                        break Ok(Self{ expr: expr.to_string(), tokens});
                    };
    
                    if let Some(op) = check_op(c) {
                        tokens.push(Token::Operator(i, op));
                    } else if let Some(punct) = check_punct(c) {
                        tokens.push(Token::Paren(i, punct));
                    } else if is_num(c) {
                        buf = i;
                        state = Lit;
                    } else if !is_sp(c) {
                        break Err(Error::UnexpChar(format!("Wrong character {c} at {i} position.")));
                    }
                },
                Lit => {
                    let Some((i, c)) = iter.next() else {
                        tokens.push(Token::Literal(buf, expr.len()));            
                        break Ok(Self{ expr: expr.to_string(), tokens});
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
                            let l = indices.nth(buf).unwrap_or(0);
                            let r = indices.nth(i).unwrap_or(expr.len());
                            break Err(Error::WrongLiteral(format!("Wrong literal {} at {}", &expr[l..r], i))); 
                        }
                    } else if is_dot(c) && dot {
                        break Err(Error::WrongLiteral(String::from("Additional dot added.")));
                    } else if is_dot(c) {
                        dot = true;
                    }
                }
            }
        }
    }

    pub fn get_expr(&self) -> &str {
        self.expr.as_str()
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }
}
