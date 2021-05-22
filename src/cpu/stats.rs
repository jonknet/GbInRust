#[derive(Debug)]
pub struct CpuStats {
    pub ime: bool,
    pub cycles: u32,
    pub halted: bool,
    pub opcode: u8,
    pub last_jump: bool
}

impl CpuStats {
    pub fn new() -> CpuStats {
        CpuStats {
            ime: true,
            cycles: 0,
            halted: true,
            opcode: 0,
            last_jump: false
        }
    }
}