#[derive(Clone, Debug)]
pub enum Delimiter {
    Positive,
    Negative,
    Asterisk,
    Slash,
    Exponent,
    Scientific,
    LeftParen,
    RightParen,
}

#[derive(Clone, Debug)]
pub enum Token {
    Liter { l: usize, r: usize },
    Delim(usize, Delimiter),
}