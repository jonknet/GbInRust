use super::mem::*;
use crate::cpu::registers::CpuRegisters;
use crate::cpu::stats::CpuStats;


pub mod ops;
pub mod registers;
pub mod stats;

#[derive(Clone, Copy, PartialEq)]
pub enum Register {
    b,c,d,e,h,l,a,f,
    af,bc,de,hl,
    pc,sp
}

pub const Z:u8 = 0x80;
pub const N:u8 = 0x40;
pub const H:u8 = 0x20;
pub const C:u8 = 0x10;

pub struct Cpu {
    pub R: CpuRegisters,
    pub S: CpuStats,
    pub M: Memory,
}

impl Cpu {

    pub fn new()-> Cpu {
        Cpu {
            R: CpuRegisters::new(),
            S: CpuStats::new(),
            M: Memory::new()
        }
    }

    pub fn get(&self,flag: u8)-> u8{
        return self.R.f & flag;
    }

    pub fn set(&mut self,flag: u8) {
        self.R.f |= flag;
    }

    pub fn clr(&mut self,flag: u8) {
        self.R.f &= !flag;
    }

    pub fn pop(&mut self) -> u16 {
        let mut val: u16;
        val = self.M.read(self.R.sp) as u16;
        self.R.sp = self.R.sp.wrapping_add(1);
        val |= ((self.M.read(self.R.sp) as u16) << 8) as u16;
        self.R.sp = self.R.sp.wrapping_add(1);
        return val;
    }

    pub fn push(&mut self,val: u16){
        self.R.sp = self.R.sp.wrapping_sub(1);
        self.M.write(self.R.sp,((val & 0xFF00) >> 8) as u8);
        self.R.sp = self.R.sp.wrapping_sub(1);
        self.M.write(self.R.sp,(val & 0xFF) as u8);
    }

}






