#[derive(Debug, Clone)]
pub enum Instr {
    LoadConst { dst: usize, value: i64 },

    Add { dst: usize, a: usize, b: usize },
    Sub { dst: usize, a: usize, b: usize },
    Eq { dst: usize, a: usize, b: usize },
    Ne { dst: usize, a: usize, b: usize },

    JumpIfTrue { cond: usize, target: usize },
    Jump { target: usize },

    Print { src: usize },
}

pub struct VM {
    pub regs: Vec<i64>,
    pub pc: usize,
}

impl VM {
    pub fn new() -> Self {
        Self {
            regs: vec![0; 256],
            pc: 0,
        }
    }

    pub fn run(&mut self, program: &[Instr]) {
        self.pc = 0;

        while self.pc < program.len() {
            match &program[self.pc] {
                Instr::LoadConst { dst, value } => {
                    self.regs[*dst] = *value;
                }

                Instr::Add { dst, a, b } => {
                    self.regs[*dst] = self.regs[*a] + self.regs[*b];
                }

                Instr::Sub { dst, a, b } => {
                    self.regs[*dst] = self.regs[*a] - self.regs[*b];
                }

                Instr::Eq { dst, a, b } => {
                    self.regs[*dst] = (self.regs[*a] == self.regs[*b]) as i64;
                }

                Instr::Ne { dst, a, b } => {
                    self.regs[*dst] = (self.regs[*a] != self.regs[*b]) as i64;
                }

                Instr::JumpIfTrue { cond, target } => {
                    if *target >= program.len() {
                        panic!("Invalid jump target: {target}");
                    }

                    if self.regs[*cond] != 0 {
                        self.pc = *target;
                        continue;
                    }
                }

                Instr::Jump { target } => {
                    if *target >= program.len() {
                        panic!("Invalid jump target: {target}");
                    }

                    self.pc = *target;
                    continue;
                }

                Instr::Print { src } => {
                    println!("{}", self.regs[*src]);
                }
            }

            self.pc += 1;
        }
    }
}
