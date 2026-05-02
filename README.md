# About

A parser for arithmetic expressions.

## Realization

### Tokenizer

The **tokenizer** was made using state machine computation model. With posible `Liter` (literal) and `Delim` (delimiter) tokens where delimiter has possible variations and all implements `enum` types:

```rust
pub enum Delimiter {
    Positive,
    Negative,
    Asterisk,
    Slash,
    Exponent,
    Scientific,
    Open,
    Close,
}
```

### Parser

To achieve arithmetics expression parsing, it implements shunting-yard algorithm, that is an operator-precedence parser, and as a result it returns data in postfix notation, or reverse polish notation (**RPN**).


# Example

Input: `((2^3) + 5.5) / 1.2E(0-2)`

Tokenizing result: `[Delim(0, Open), Delim(1, Open), Liter { l: 2, r: 3 }, ..., Liter { l: 23, r: 24 }, Delim(24, Close)]`

Parsing result: `[Val(2.0), Val(3.0), Op(Exp), Val(5.5), Op(Sum), Val(1.2), Val(0.0), Val(2.0), Op(Sub), Op(Sci), Op(Div)]`

Test expression evaluation result: `1125`

Code sample:

```rust
let tokens = Tokens::from(("((2^3) + 5.5) / 1.2E(0-2)".trim()).expect("Could not tokeinze the expression.");
let rpn = RPN::from(tokens).expect("Some error occured.");

let mut stack: Vec<f64> = Vec::new();
for data in rpn.data {
    if let Data::Op(op) = data {
        let v1 = stack.pop().unwrap();
        let v2 = stack.pop().unwrap();
        let result = match op {
            Operator::Sum => v1 + v2,
            Operator::Sub => v2 - v1,
            Operator::Mul => v1 * v2,
            Operator::Div => v2 / v1,
            Operator::Sci => v2 * 10_f64.powf(v1),
            Operator::Exp => v2.powf(v1),
            Operator::Neg => -v1,
            Operator::Pos => v1,
        };
        stack.push(result);
    } else if let Data::Val(v) = data {
        stack.push(v);
    }
}

println!("\nExpression result: {}", stack.pop().unwrap());
```
