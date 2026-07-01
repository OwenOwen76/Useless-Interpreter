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
}

impl VM {
    pub fn new() -> Self {
        Self { regs: vec![0; 256] }
    }
}
