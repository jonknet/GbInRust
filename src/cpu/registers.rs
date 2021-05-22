#[derive(Debug)]
pub struct CpuRegisters {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl CpuRegisters {
    pub fn get_a(&self) -> u8 {
        self.a
    }

    pub fn get_f(&self) -> u8 {
        self.f
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn get_c(&self) -> u8 {
        self.c
    }

    pub fn get_d(&self) -> u8 {
        self.d
    }

    pub fn get_e(&self) -> u8 {
        self.e
    }

    pub fn get_h(&self) -> u8 {
        self.h
    }

    pub fn get_l(&self) -> u8 {
        self.l
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_a(&mut self, v: u8) {
        self.a = v;
    }

    pub fn set_f(&mut self, v: u8) {
        self.f = v;
    }

    pub fn set_b(&mut self, v: u8) {
        self.b = v;
    }

    pub fn set_c(&mut self, v: u8) {
        self.c = v;
    }

    pub fn set_d(&mut self, v: u8) {
        self.d = v;
    }

    pub fn set_e(&mut self, v: u8) {
        self.e = v;
    }

    pub fn set_h(&mut self, v: u8) {
        self.h = v;
    }

    pub fn set_l(&mut self, v: u8) {
        self.l = v;
    }

    pub fn set_sp(&mut self, v: u16) {
        self.sp = v;
    }

    pub fn set_pc(&mut self, v: u16) {
        self.pc = v;
    }

    pub fn set_bc(&mut self, v: u16) {
        self.b = ((v & 0xFF00) >> 8) as u8;
        self.c = (v & 0xFF) as u8;
    }

    pub fn set_de(&mut self, v: u16) {
        self.d = ((v & 0xFF00) >> 8) as u8;
        self.e = (v & 0xFF) as u8;
    }

    pub fn set_hl(&mut self, v: u16) {
        self.h = ((v & 0xFF00) >> 8) as u8;
        self.l = (v & 0xFF) as u8;
    }

    pub fn set_af(&mut self, v: u16) {
        self.a = ((v & 0xFF00) >> 8) as u8;
        self.f = (v & 0xFF) as u8;
    }

    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    }

    pub fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f as u16
    }

    pub fn new() -> CpuRegisters {
        CpuRegisters {
            a:0,f:0,b:0,c:0,d:0,e:0,h:0,l:0,pc:0,sp:0
        }
    }
}