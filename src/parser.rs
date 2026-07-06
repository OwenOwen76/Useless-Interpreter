use crate::lexer::Tokens;

#[derive(Debug, Clone)]
pub enum Op {
    Add,
    Sub,
    Eq,
    Neq,
}

#[derive(Debug, Clone)]
pub enum Instr {
    Declare {
        name: String,
        value: String,
    },

    Drop,
    None,

    BinaryOp {
        op: Op,
        left: String,
        right: String,
        out: String,
    },

    Print {
        target: String,
    },
}

#[derive(Debug, Clone)]
pub enum IdentType {
    VarName,
    VarValue,
}

pub struct Parser {
    tokens: Vec<Tokens>,
    pos: usize,
    pub output: Vec<Instr>,
}

impl Parser {
    pub fn new(tokens: Vec<Tokens>) -> Self {
        Self {
            tokens,
            pos: 0,
            output: Vec::new(),
        }
    }

    fn peek(&self) -> &Tokens {
        &self.tokens[self.pos]
    }

    fn advance(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn is_eof(&self) -> bool {
        matches!(self.tokens.get(self.pos), Some(Tokens::EOF))
    }

    pub fn parse(&mut self) {
        let mut buffer: Vec<Tokens> = Vec::new();

        while self.pos < self.tokens.len() {
            let tok = self.tokens[self.pos].clone();
            self.pos += 1;

            if matches!(tok, Tokens::EOF) {
                break;
            }

            buffer.push(tok);

            if self.is_block_end(&buffer) {
                let instr = self.parse_block(&buffer);
                self.output.push(instr);
                buffer.clear();
            }
        }

        if !buffer.is_empty() {
            let instr = self.parse_block(&buffer);
            self.output.push(instr);
        }

        self.output.reverse();
    }

    fn is_block_end(&self, buffer: &[Tokens]) -> bool {
        let mut depth = 0;

        for t in buffer {
            match t {
                Tokens::LeftBracket => depth += 1,
                Tokens::RightBracket => depth -= 1,
                _ => {}
            }
        }

        depth == 0 && !buffer.is_empty()
    }

    fn parse_block(&self, tokens: &[Tokens]) -> Instr {
        let mut t = tokens.to_vec();
        t.reverse();

        let mut left: Option<String> = None;
        let mut right: Option<String> = None;
        let mut op: Option<Op> = None;
        let mut is_newline: bool = false;

        let mut i = 0;
        while i < t.len() {
            match &t[i] {
                Tokens::Ident(s) => {
                    if left.is_none() {
                        left = Some(s.clone());
                    } else {
                        right = Some(s.clone());
                    }
                }

                Tokens::Number(s) => {
                    if left.is_none() {
                        left = Some(s.clone());
                    } else {
                        right = Some(s.clone());
                    }
                }

                Tokens::Add => op = Some(Op::Add),
                Tokens::Subtract => op = Some(Op::Sub),
                Tokens::Equal => op = Some(Op::Eq),
                Tokens::NotEqual => op = Some(Op::Neq),
                Tokens::Newline => is_newline = true,

                Tokens::Quote => {
                    return Instr::Print {
                        target: left.unwrap_or_else(|| "null".to_string()),
                    };
                }

                _ => {}
            }

            i += 1;
        }

        match (left, op, right, is_newline) {
            (Some(l), Some(o), Some(r), false) => Instr::BinaryOp {
                op: o,
                left: l,
                right: r,
                out: "tmp".to_string(),
            },

            (Some(l), None, Some(r), false) => Instr::Declare { name: l, value: r },

            (Some(l), None, None, false) => Instr::Print { target: l },

            (None, None, None, true) => Instr::Drop,

            _ => Instr::None,
        }
    }
}
