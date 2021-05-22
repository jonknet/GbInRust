use std::sync::{RwLock, Arc};

#[derive(Debug)]
pub struct Memory {
    boot: [u8;256],
    map: [u8;1024 * 64]
}

impl Memory {
    pub fn read(&self, address: u16) -> u8 {
        return self.map[address as usize];
    }

    pub fn writerom(&mut self,address: u16,val: u8){
        self.boot[address as usize] = val;
    }

    pub fn write(&mut self, address: u16, val: u8) {
        println!("Mem write to {:x} of {:x}",address,val);
        self.map[address as usize] = val;
    }

    pub fn read16(&self, address: u16) -> u16 {
        return self.map[address as usize] as u16 | ((self.map[address as usize +1] as u16) << 8);
    }

    pub fn write16(&mut self, address: u16, val: u16) {
        self.map[address as usize] = (val & 0xFF) as u8;
        self.map[address as usize +1] = ((val & 0xFF00) >> 8) as u8;
    }

    pub fn new() -> Memory {
        Memory {
            boot: [0;256],
            map: [0;1024 * 64]
        }
    }
}