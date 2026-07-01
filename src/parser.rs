#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    Plus,
    Minus,
    Equal,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Commands {
    Opr(Operators),
    Print,
    Error,
}
