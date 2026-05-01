#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug)]
pub enum Token {
    Liter { l: usize, r: usize },
    Delim(usize, Delimiter),
}