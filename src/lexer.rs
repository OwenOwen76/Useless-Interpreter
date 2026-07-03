#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    Ident(String),
    Number(String),

    LeftBracket,
    RightBracket,
    IfStart,
    IfEnd,
    Quote,

    Add,
    Subtract,
    Equal,
    NotEqual,

    StartFile,
    EndFile(String),

    EOF,
    Error,
    WhiteSpaceError,
    Newline,
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

    fn read_ident(&mut self) -> String {
        let mut s = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_alphabetic() {
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
        let valid_digits = ".,";

        while let Some(&c) = self.chars.peek() {
            if valid_digits.contains(c) {
                s.push(c);
                self.chars.next();
            } else {
                break;
            }
        }
        s
    }

    fn read_trashcan(&mut self) -> Option<Tokens> {
        let mut s = String::new();

        while let Some(c) = self.chars.next() {
            s.push(c);

            if c == '>' {
                break;
            }
        }

        if s == "<?EMPTYTRASHCAN>" {
            return Some(Tokens::StartFile);
        }

        if s.starts_with("<!") && s.ends_with("TRASHCAN>") {
            let num = s
                .trim_start_matches("<!")
                .trim_end_matches("TRASHCAN>")
                .trim_end_matches(|c: char| c.is_alphabetic());

            return Some(Tokens::EndFile(num.to_string()));
        }
        Some(Tokens::Error)
    }

    fn read_newline(&mut self) -> Option<Tokens> {
        let c = self.chars.peek();
        if c == Some(&'n') {
            return Some(Tokens::Newline);
        }
        Some(Tokens::Error)
    }

    pub fn next_token(&mut self) -> Option<Tokens> {
        let c = self.chars.peek();
        if c == Some(&' ') {
            return Some(Tokens::WhiteSpaceError);
        }

        let c = match self.chars.peek() {
            Some(c) => *c,
            None => return None,
        };

        if let Some('<') = self.chars.peek().copied() {
            return self.read_trashcan();
        }

        if c.is_alphabetic() {
            let ident = self.read_ident();
            return Some(Tokens::Ident(ident));
        }

        if c == '.' || c == ',' {
            let num = self.read_number();
            return Some(Tokens::Number(num));
        }

        self.chars.next();

        match c {
            '[' => Some(Tokens::LeftBracket),
            ']' => Some(Tokens::RightBracket),
            ':' => Some(Tokens::IfStart),
            ';' => Some(Tokens::IfEnd),
            '"' => Some(Tokens::Quote),
            '|' => Some(Tokens::Add),
            '/' => Some(Tokens::Subtract),
            '~' => Some(Tokens::Equal),
            '-' => Some(Tokens::NotEqual),
            '\\' => self.read_newline(),
            _ => Some(Tokens::Error),
        }
    }

    pub fn tokenize(mut self) -> Vec<Tokens> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens.push(Tokens::EOF);
        tokens
    }
}
