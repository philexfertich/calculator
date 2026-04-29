use crate::Token;

enum Operation {
    Sum,
    Sub,
    Mul,
    Div,
    Exp,
    Sci,
}

enum Node {
    Expr(Operation, Box<Node>, Box<Node>),
    Value(String),
}

fn expect(iter: &mut core::slice::Iter<'_, Token>) {
    iter.next();
}

fn definition(iter: &mut core::slice::Iter<'_, Token>) -> Node {
    expect(iter);
    
    Node::Value(String::new())
    // for t in iter {
    //     match t {
    //         Token::Literal(l, r) => {
    //             break Node::Value(tokens.get_expr()[])
    //         },
    //         Token::Operator(pos, op) => break Node::Value(String::new()),
    //         Token::Paren(pos, paren) => break Node::Value(String::new()),
    //     }
    // }
}

// pub fn parse(tokens: Tokens) -> Node {
    
//     let mut iter = tokens.get_tokens().iter();

//     definition(&mut iter)
// }