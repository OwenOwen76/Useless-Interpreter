#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Ident(String),
    NumberDigits(String),
    Commands,
    Error,
    None,
}

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

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn read_ident(&mut self) -> String {
        let mut s = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() {
                s.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        s
    }

    fn read_number(&mut self) -> String {
        let mut s = String::new();

        while let Some(&c) = self.chars.peek() {
            if !c.is_ascii() && !c.is_whitespace() {
                s.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        s
    }

    pub fn next_token(&mut self) -> Tokens {
        self.skip_whitespace();

        let c = match self.chars.peek() {
            Some(c) => *c,
            None => return Tokens::None,
        };

        if c.is_alphabetic() {
            let ident = self.read_ident();
            return Tokens::Commands;
        }

        if c.is_ascii_digit() {
            return Tokens::Error;
        }

        if !c.is_ascii() {
            let num = self.read_number();
            return Tokens::NumberDigits(num);
        } else {
            return Tokens::Error;
        }
    }
}
