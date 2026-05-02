use crate::{ Delimiter, Token, Tokens, parser::op::Operator };
use core::{panic, slice::Iter};
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

        pub fn is_left_associative(&self) -> bool {
            match self {
                Operator::Neg => false,
                Operator::Pos => false,
                Operator::Exp => false,
                Operator::Mul => true,
                Operator::Div => true,
                Operator::Sci => false,
                Operator::Sum => true,
                Operator::Sub => true,
            }
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
            Operator::Sci => 1,
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

#[derive(PartialEq, Debug)]
pub enum StackItem {
    Op(Operator),
    LP,
}


impl RPN {
    fn iterate_stack(stack: &mut Vec<StackItem>, output: &mut Vec<Data>,  op: Operator) {
        while let Some(si) = dbg!(stack.pop()) {
            let StackItem::Op(op_2) = si else {
                stack.push(dbg!(si));
                break;
            };
            
            println!("Completing");
            match op_2.have_precedence(&op) {
                Ordering::Equal => {
                    println!("{:?} have equal precedence over {:?}", op_2, op);
                    if op.is_left_associative() {
                        output.push(Data::Op(op_2));
                    } else {
                        stack.push(dbg!(StackItem::Op(op_2)));
                        break;
                    }
                },
                Ordering::Less => {
                    println!("{:?} have less precedence over {:?}", op_2, op);
                    stack.push(dbg!(StackItem::Op(op_2)));
                    break;
                }
                Ordering::Greater => {
                    println!("{:?} have greater precedence over {:?}", op_2, op);
                    output.push(Data::Op(op_2));
                }
            };
        }
        stack.push(dbg!(StackItem::Op(op)));
    }
    
    pub fn from<'a>(tokens: Tokens) -> Result<Self, Error> {
        let iter = tokens.tokens.iter();

        let mut stack: Vec<StackItem> = Vec::new();
        let mut output: Vec<Data> = Vec::new();

        'token_iter: for token in iter {
            
            println!("{stack:?}");
            match token {
                Token::Delim(_, delim) => {
                    match delim {
                        Delimiter::Positive => RPN::iterate_stack(&mut stack, &mut output, Operator::Sum),
                        Delimiter::Negative => RPN::iterate_stack(&mut stack, &mut output, Operator::Sub),
                        Delimiter::Asterisk => RPN::iterate_stack(&mut stack, &mut output, Operator::Mul),
                        Delimiter::Slash => RPN::iterate_stack(&mut stack, &mut output, Operator::Div),
                        Delimiter::Exponent => RPN::iterate_stack(&mut stack, &mut output, Operator::Exp),
                        Delimiter::Scientific => RPN::iterate_stack(&mut stack, &mut output, Operator::Sci),
                        Delimiter::Open => stack.push(StackItem::LP),
                        Delimiter::Close => {
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
            while let Some(si) = stack.pop() {
                if let StackItem::Op(op) = si {
                    output.push(Data::Op(op));
                } else {
                    panic!("Parentheses ar not closed on finish.")
                }
            }
        }

        Ok(RPN{ data: output })
    }
}
