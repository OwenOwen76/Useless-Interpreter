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
        *self.var_map.entry(name).or_insert_with(|| {
            let r = self.next_reg;
            self.next_reg += 1;
            r
        })
    }

    fn emit(&mut self, instr: Instr) -> usize {
        self.output.push(instr);
        self.output.len() - 1
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

    fn patch_jump(&mut self, idx: usize, target: usize) {
        match &mut self.output[idx] {
            Instr::JumpIfTrue { target: t, .. } => *t = target,
            _ => panic!("Invalid JumpIfTrue patch"),
        }
    }

    fn patch_jump_unconditional(&mut self, idx: usize, target: usize) {
        match &mut self.output[idx] {
            Instr::Jump { target: t } => *t = target,
            _ => panic!("Invalid Jump patch"),
        }
    }

    pub fn compile(&mut self, tokens: &[Tokens]) {
        let mut i = 0;

        while i < tokens.len() {
            match &tokens[i] {
                Tokens::Quote => {
                    if let Some(Tokens::Ident(name)) = tokens.get(i + 1) {
                        let reg = self.get_reg(name.clone());
                        self.emit(Instr::Print { src: reg });
                        i += 2;
                        continue;
                    }
                }

                Tokens::LeftBracket => {
                    if let Some(Tokens::Number(bits)) = tokens.get(i + 1) {
                        if let Some(Tokens::Ident(name)) = tokens.get(i + 2) {
                            let dst = self.get_reg(name.clone());
                            let value = Self::binary_to_i64(bits);

                            self.emit(Instr::LoadConst { dst, value });
                            i += 3;
                            continue;
                        }
                    }
                }

                Tokens::IfStart => {
                    if let Some(Tokens::Ident(cond_name)) = tokens.get(i + 1) {
                        let cond = self.get_reg(cond_name.clone());

                        let jmp_if_false = self.emit(Instr::JumpIfTrue { cond, target: 0 });

                        i += 2;

                        let mut j = i;
                        let mut found_else = false;

                        while j < tokens.len() {
                            if let Tokens::IfEnd = tokens[j] {
                                found_else = true;
                                break;
                            }
                            j += 1;
                        }

                        let else_start = self.output.len();

                        while i < j {
                            self.compile_token(&tokens[i]);
                            i += 1;
                        }

                        let jmp_end = self.emit(Instr::Jump { target: 0 });

                        let if_end = self.output.len();

                        self.patch_jump(jmp_if_false, else_start);

                        if found_else {
                            i += 1;
                            while i < tokens.len() {
                                if let Tokens::EOF = tokens[i] {
                                    break;
                                }
                                self.compile_token(&tokens[i]);
                                i += 1;
                            }
                        }

                        let end = self.output.len();

                        self.patch_jump_unconditional(jmp_end, end);

                        continue;
                    }
                }

                _ => {}
            }

            i += 1;
        }
    }

    fn compile_token(&mut self, token: &Tokens) {
        match token {
            Tokens::Quote => {
                // reorganizing
            }

            _ => {}
        }
    }
}
