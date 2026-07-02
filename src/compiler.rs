use crate::lexer::Tokens;
use crate::vm::Instr;
use std::collections::HashMap;

pub struct Compiler {
    pub var_map: HashMap<String, usize>,
    pub next_reg: usize,
    pub output: Vec<Instr>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            next_reg: 0,
            output: Vec::new(),
        }
    }

    fn get_reg(&mut self, name: String) -> usize {
        if let Some(&r) = self.var_map.get(&name) {
            return r;
        }

        let r = self.next_reg;
        self.var_map.insert(name, r);
        self.next_reg += 1;
        r
    }

    fn binary_to_i64(bits: &str) -> i64 {
        let mut value = 0;

        for (i, c) in bits.chars().enumerate() {
            if c == ',' {
                value |= 1 << i;
            }
        }

        value
    }

    fn split_blocks(tokens: &[Tokens]) -> Vec<Vec<Tokens>> {
        let mut blocks = Vec::new();
        let mut current = Vec::new();
        let mut inside = false;

        for token in tokens {
            match token {
                Tokens::LeftBracket => {
                    inside = true;
                    current.clear();
                }

                Tokens::RightBracket => {
                    inside = false;
                    blocks.push(current.clone());
                }

                Tokens::StartFile | Tokens::EndFile(_) | Tokens::EOF => {}

                other => {
                    if inside {
                        current.push(other.clone());
                    }
                }
            }
        }

        blocks
    }

    fn compile_block(&mut self, block: &[Tokens]) {
        match block {
            [Tokens::Number(bits), Tokens::Ident(name)] => {
                let dst = self.get_reg(name.clone());
                let value = Self::binary_to_i64(bits);

                self.output.push(Instr::LoadConst { dst, value });
            }

            [
                Tokens::Ident(lhs),
                Tokens::Ident(rhs),
                Tokens::Add,
                Tokens::Ident(dst_name),
            ] => {
                let dst = self.get_reg(dst_name.clone());
                let a = self.get_reg(lhs.clone());
                let b = self.get_reg(rhs.clone());

                self.output.push(Instr::Add { dst, a, b });
            }

            [
                Tokens::Ident(lhs),
                Tokens::Ident(rhs),
                Tokens::Subtract,
                Tokens::Ident(dst_name),
            ] => {
                let dst = self.get_reg(dst_name.clone());
                let a = self.get_reg(lhs.clone());
                let b = self.get_reg(rhs.clone());

                self.output.push(Instr::Sub { dst, a, b });
            }

            _ => {
                panic!("Invalid declaration block: {:?}", block);
            }
        }
    }

    pub fn compile(&mut self, tokens: &[Tokens]) {
        let mut blocks = Self::split_blocks(tokens);

        blocks.reverse();

        for block in blocks {
            self.compile_block(&block);
        }

        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                Tokens::Quote => {
                    if let Some(Tokens::Ident(name)) = tokens.get(i + 1) {
                        let reg = self.get_reg(name.clone());

                        self.output.push(Instr::Print { src: reg });

                        i += 2;
                        continue;
                    }
                }

                _ => {}
            }

            i += 1;
        }
    }
}
