use crate::{ Delimiter, Token, Tokens, parser::op::Operator };
use core::slice::Iter;
use std::{cmp::Ordering, collections::HashMap, f32};

#[derive(Debug)]
pub enum Error {
    NoTokens,
    WrongToken,
    NumberExpected,
    NullToken,
    Stack,
}

pub mod op {
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub enum Operator {
        Sum,
        Sub,
        Mul,
        Div,
        Exp,
        Sci,
        Neg,
        Pos,
    }

    impl Operator {
        pub fn have_precedence(&self, op: &Operator) -> std::cmp::Ordering {
            precedence(self).cmp(&precedence(op))
        }
    }

    fn precedence(op: &Operator) -> u8{
        // The precedence of operators
        //
        // 1) NEG POS   <- highest
        // 2) ^
        // 3) * / E
        // 4) - +       <- lowest
        match op {
            Operator::Neg => 3,
            Operator::Pos => 3,
            Operator::Exp => 2,
            Operator::Mul => 1,
            Operator::Div => 1,
            Operator::Sci => 0,
            Operator::Sum => 0,
            Operator::Sub => 0,
        }
    }
}

#[derive(Debug)]
pub enum Data {
    Op(op::Operator),
    Val(f64),
}

pub struct RPN {
    pub data: Vec<Data>
}

// fn define_delim(delim: &Delimiter) -> Operator {
//     match delim {
//         Delimiter::Positive => ,
//         Delimiter::Negative => ,
//         Delimiter::Asterisk => ,
//         Delimiter::Slash => ,
//         Delimiter::Exponent => ,
//         Delimiter::Scientific => ,
//         _ => ,
//     }
// }

#[derive(PartialEq)]
pub enum StackItem {
    Op(Operator),
    LP,
}


impl RPN {
    pub fn from<'a>(tokens: Tokens) -> Result<Vec<Data>, Error> {
        let iter = tokens.tokens.iter();

        let mut stack: Vec<StackItem> = Vec::new();
        let mut output: Vec<Data> = Vec::new();

        'token_iter: for token in iter {
            match token {
                Token::Delim(_, delim) => {
                    match delim {
                        Delimiter::Positive => {
                        },
                        Delimiter::Negative => {},
                        Delimiter::Asterisk => {},
                        Delimiter::Slash => {},
                        Delimiter::Exponent => {},
                        Delimiter::Scientific => {},
                        Delimiter::Open => {
                            stack.push(StackItem::LP)
                        },
                        Delimiter::Close => {
                            // let Some(mut op) = stack.pop() else {
                            //     panic!("Stack is empty and parens not closed.")
                            // };

                            while let Some(op) = stack.pop() {
                                match op {
                                    StackItem::LP => continue 'token_iter,
                                    StackItem::Op(o) => {
                                        output.push(Data::Op(o));
                                    }
                                };
                            }
                            panic!("Parentheses are not closed.");
                        },
                    }
                },
                Token::Liter { l, r } => {
                    let mut indices = tokens.get_expr().char_indices().map(|(i, _)| i);
                    let start = dbg!(indices.nth(*l).unwrap_or_default());
                    let end = dbg!(indices.nth(dbg!(r - l - 1)).unwrap_or(tokens.get_expr().len()));
                    
                    let Ok(val) = tokens.get_expr()[start..end].parse() else {
                        panic!("Could not parse a literal {} at {l} {r}", &tokens.get_expr()[start..end]);
                    };
                    
                    output.push(Data::Val(val));
                },
            }
        }

        if !stack.is_empty() {
            for si in stack {
                if let StackItem::Op(op) = si {
                    output.push(Data::Op(op));
                } else {
                    panic!("Parentheses ar not closed on finish.")
                }
            }
        }

        Ok(output)
    }
}
