use crate::lexer::Tokens;
use crate::vm::Instr;
use std::collections::HashMap;

pub struct Compiler {
    pub var_map: HashMap<String, usize>,
    pub next_reg: usize,
    pub next_temp: usize,
    pub output: Vec<Instr>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            var_map: HashMap::new(),
            next_reg: 0,
            next_temp: 0,
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

    fn split_blocks(tokens: &[Tokens]) -> Vec<Vec<Tokens>> {
        let mut blocks = Vec::new();
        let mut current = Vec::new();
        let mut inside = false;

        for t in tokens {
            match t {
                Tokens::LeftBracket => {
                    inside = true;
                    current = Vec::new();
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

    fn compile_block(&mut self, tokens: &[Tokens]) -> Option<usize> {
        let mut stack: Vec<usize> = Vec::new();
        let mut last_result: Option<usize> = None;

        for token in tokens {
            match token {
                Tokens::Ident(name) => {
                    let reg = self.get_reg(name.clone());
                    stack.push(reg);
                }

                Tokens::Add => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();

                    let dst = self.next_temp;
                    self.next_temp += 1;

                    self.output.push(Instr::Add { dst, a, b });

                    stack.push(dst);
                    last_result = Some(dst);
                }

                Tokens::Subtract => {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();

                    let dst = self.next_temp;
                    self.next_temp += 1;

                    self.output.push(Instr::Sub { dst, a, b });

                    stack.push(dst);
                    last_result = Some(dst);
                }

                _ => {}
            }
        }

        last_result
    }

    pub fn compile(&mut self, tokens: &[Tokens]) {
        let mut blocks = Self::split_blocks(tokens);

        blocks.reverse();

        let mut block_results: Vec<Option<usize>> = Vec::new();

        for block in blocks {
            let result = self.compile_block(&block);
            block_results.push(result);
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
