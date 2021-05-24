use super::mem::*;
use crate::cpu::registers::CpuRegisters;
use crate::cpu::stats::CpuStats;
use std::{borrow::{Borrow, BorrowMut}, sync::{Arc, Mutex, MutexGuard, RwLock}};
use std::ops::BitAnd;
use std::sync::atomic::AtomicU64;

pub mod ops;
pub mod registers;
pub mod stats;

#[derive(Clone, Copy, PartialEq)]
pub enum Register {
    b,c,d,e,h,l,a,f,
    af,bc,de,hl,
    pc,sp
}

pub enum Interrupts {
    NONE = 0,
    VBLANK = 1,
    STAT = 2,
    TIMER = 4,
    SERIAL = 8,
    JOYPAD = 16
}

pub const Z:u8 = 0x80;
pub const N:u8 = 0x40;
pub const H:u8 = 0x20;
pub const C:u8 = 0x10;

pub struct Cpu {
    pub R: CpuRegisters,
    pub S: CpuStats,
    pub C: Arc<AtomicU64>
}

impl Cpu {

    pub fn new()-> Cpu {
        Cpu {
            R: CpuRegisters::new(),
            S: CpuStats::new(),
            C: Arc::new(AtomicU64::new(0))
        }
    }

    pub fn getCycles(&self) -> Arc<AtomicU64> {
        return self.C.clone();
    }

    pub fn process_interrupts(&mut self,mtx:&mut MutexGuard<Memory>){
        let mut lock = mtx;
        let irq = lock.read(0xFF0F);
        let ie = lock.read(0xFFFF);
        let mut int: Interrupts = Interrupts::NONE;
        let mut intaddr = 0;
        if irq > 0 && self.S.ime && irq & ie > 0 {
            if irq & Interrupts::VBLANK as u8 > 0 && ie & Interrupts::VBLANK as u8 > 0 {
                int = Interrupts::VBLANK;
                intaddr = 0x40;
            } else if irq & Interrupts::STAT as u8 > 0 && ie & Interrupts::STAT as u8 > 0 {
                int = Interrupts::STAT;
                intaddr = 0x48;
            } else if irq & Interrupts::TIMER as u8 > 0 && ie & Interrupts::TIMER as u8 > 0 {
                int = Interrupts::TIMER;
                intaddr = 0x50;
            } else if irq & Interrupts::SERIAL as u8 > 0 && ie & Interrupts::SERIAL as u8 > 0 {
                int = Interrupts::SERIAL;
                intaddr = 0x58;
            } else if irq & Interrupts::JOYPAD as u8 > 0 && ie & Interrupts::JOYPAD as u8 > 0 {
                int = Interrupts::JOYPAD;
                intaddr = 0x60;
            }
            lock.write(0xFFFF,ie & !(int as u8));
            self.push(self.R.pc, lock);
            self.R.pc = intaddr;
            self.S.ime = false;
            
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

    pub fn pop(&mut self,m: &mut MutexGuard<Memory>) -> u16 {
        let mut val: u16;
        let mut lock = m;
        val = lock.read(self.R.sp) as u16;
        self.R.sp = self.R.sp.wrapping_add(1);
        val |= ((lock.read(self.R.sp) as u16) << 8) as u16;
        self.R.sp = self.R.sp.wrapping_add(1);
        return val;
    }

    pub fn push(&mut self, val: u16, mut m: &mut MutexGuard<Memory>){
        self.R.sp = self.R.sp.wrapping_sub(1);
        m.write(self.R.sp,((val & 0xFF00) >> 8) as u8);
        self.R.sp = self.R.sp.wrapping_sub(1);
        m.write(self.R.sp,(val & 0xFF) as u8);
    }

}






