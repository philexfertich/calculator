use crate::{ Delimiter, Token, Tokens };
use core::slice::Iter;
use std::{collections::HashMap, f32};

#[derive(Debug)]
pub enum Error {
    NoTokens,
    WrongToken,
    NumberExpected,
    NullToken,
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
enum Data<T> {
    Op(op::Operator),
    Val(T),
}

pub fn parse_rpn<'a>(tokens: &'a mut Tokens) -> Result<Vec<Data<f64>>, Error> {
    let mut iter = tokens.tokens.iter();

    let Some(t) = iter.next() else {
        return Err(Error::NoTokens);
    };
    
    Ok(Vec::new())
}
