use std::sync::{RwLock, Arc};
use crate::cpu::Register::a;

#[derive(Debug)]
pub struct Memory {
    boot: [u8;256],
    map: [u8;1024*64],
    rom: [u8; 32*1024],
    vram: [u8; 8 * 1024],
    wram: [u8;8*1024],
    oam: [u8; 160],
    banks: Vec<u8>
}

impl Memory {

    fn map_address(&mut self, address: u16) -> (&mut [u8], u16) {

        if address >= 0 && address <= 0x7FFF {
            return (&mut self.rom,address);
        } else if address >= 0x8000 && address <= 0x9FFF {
            return (&mut self.vram,address - 0x8000);
        } else if address >= 0xC000 && address <= 0xDFFF {
            return (&mut self.wram,address - 0xC000);
        } else if address >= 0xFE00 && address <= 0xFE9F {
            return (&mut self.oam,address - 0xFE00);
        } else {
            return (&mut self.map,address);
        }
    }

    pub fn read(&mut self, address: u16) -> u8 {

        if address < 256 && self.map[0xFF50] == 0 {
            return self.boot[address as usize];
        }

        let (mem,add) = self.map_address(address);

        return mem[add as usize];

    }

    pub fn writerom(&mut self,address: u16,val: u8){
        if(val > 255) {
            return;
        }
        self.boot[address as usize] = val;
    }

    pub fn write(&mut self, address: u16, val: u8) {
        //println!("Mem write to {:x} of {:x}",address,val);
        let (mem,add) = self.map_address(address);
        mem[add as usize] = val;
    }

    pub fn read16(&self, address: u16) -> u16 {
        return self.map[address as usize] as u16 | ((self.map[address as usize +1] as u16) << 8);
    }

    pub fn write16(&mut self, address: u16, val: u16) {
        self.map[address as usize] = (val & 0xFF) as u8;
        self.map[address as usize +1] = ((val & 0xFF00) >> 8) as u8;
    }

    pub fn new(sz: u32) -> Memory {
        Memory {
            boot: [0;256],
            map: [0;1024 * 64],
            rom: [0;32*1024],
            vram: [0;8*1024],
            wram: [0;8*1024],
            oam: [0;160],
            banks: Vec::with_capacity(sz as usize)
        }
    }
}