#[derive(Debug, Clone)]
pub enum Instr {
    LoadConst { dst: usize, value: i64 },

    Mov { dst: usize, src: usize },

    Add { dst: usize, a: usize, b: usize },
    Sub { dst: usize, a: usize, b: usize },

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

                Instr::Mov { dst, src } => {
                    self.regs[*dst] = self.regs[*src];
                }

                Instr::Add { dst, a, b } => {
                    self.regs[*dst] = self.regs[*a] + self.regs[*b];
                }

                Instr::Sub { dst, a, b } => {
                    self.regs[*dst] = self.regs[*a] - self.regs[*b];
                }

                Instr::Print { src } => {
                    println!("{}", self.regs[*src]);
                }
            }

            self.pc += 1;
        }
    }
}
