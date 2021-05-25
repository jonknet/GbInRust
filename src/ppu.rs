use crate::sdl2::render::Texture;
use std::{ptr::null, sync::{Arc, Mutex, MutexGuard}, thread};
use crate::mem::*;
use std::ops::Deref;
use std::cell::RefCell;
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};

enum LCDC {
    BGENABLE = 1,
    OBJSIZE = 2,
    BGMAP = 4,
    BGDATA = 8,
    WINENABLE = 16,
    WINMAP = 32,
    LCDENABLE = 64
}

enum STAT {
    HBLANK = 0,
    VBLANK = 1,
    OAM = 2,
    XFER = 3,
    LYCEQ = 4,
    M0INT = 8,
    M1INT = 16,
    M2INT = 32,
    LYCINT = 64
}

enum SpriteAttributes {
    XFLIP = 0x20,
    YFLIP = 0x40,
    BGOVER = 0x80
}

#[derive(Debug)]
pub struct Sprite {
    y: u8,
    x: u8,
    index: i16,
    attrib: u8
}

pub struct Ppu {
    sprites: Vec<Sprite>,
    framebuffer: [u8; 256*256],
    cycles: Arc<AtomicU64>
}

const pal : [u8;4] = [0xFF,0xAC,0x63,0x00];

impl Ppu {

    pub fn getCycles(&self) -> Arc<AtomicU64> {
        return self.cycles.clone();
    }

    pub fn render_line(&mut self, y: u16, mlock: &mut MutexGuard<Memory>){
        let mut buffer: [u8;256] = [0;256];
        // Draw bg first
        let bgmap_addr: u16;
        let bgdata: u16;
        if mlock.read(0xFF40) & (LCDC::BGMAP) as u8 > 0 {
            bgmap_addr = 0x9800;
        } else {
            bgmap_addr = 0x9C00;
        }
        if mlock.read(0xFF40) & (LCDC::BGDATA) as u8 > 0 {
            bgdata = 0x8000;
        } else {
            bgdata = 0x8800;
        }
        let scy = mlock.read(0xFF42);
        let scx = mlock.read(0xFF43);
        let bgidx = (y / 8) * 32;
        let yadd = y % 8;
        Ppu::find_intersect_sprites(y,mlock,&mut self.sprites);
        for x in 0..32 {
            let i = bgmap_addr + bgidx as u16 + x;
            let bgtileindex = mlock.read(i);
            for p in 0..8 {
                self.framebuffer[((y * 256) + (x * 8) + p) as usize] = pal[(((mlock.read(bgdata + (bgtileindex * 16) as u16 + p as u16 + (yadd*2) as u16) >> (7-p)) & 0x1) |
                                                ((mlock.read(bgdata + (bgtileindex * 16) as u16 + p as u16 + 1 + (yadd*2) as u16) >> (7-p) & 0x1) << 1)) as usize];
            }
        }
        for s in self.sprites.iter() {
            let tiledata = 0x8000;
            let sy = (s.y as u16 - y) * 2;
            let sx: i16 = (s.x as i16 - 8);
            let mut i = 0;
            if sx < 0 {
                i = sx.abs();
            }
            for mut x in i..8 {
                self.framebuffer[((y*256)+sx as u16 +x as u16) as usize] = pal[((((mlock.read(tiledata + (s.index * 16) as u16) as u16 + sy) >> (7-x)) & 0x1) |
                    (((mlock.read(tiledata + (s.index * 16) as u16) as u16 + sy + 1) >> (7-x)) & 0x1) << 1) as usize];
                println!("{} ",self.framebuffer[((y*256)+sx as u16 +x as u16) as usize]);
            }

        }
        println!("draw {}",mlock.read(0xFF44));
        let ly = mlock.read(0xFF44);
        if ly >= 153 {
            mlock.write(0xFF44,0);
        } else {
            mlock.write(0xFF44, ly.wrapping_add(1));
        }

    }

    fn find_intersect_sprites(y: u16, lock: &mut MutexGuard<Memory>, sprtbl: &mut Vec<Sprite>){
        sprtbl.clear();
        let baseaddr = 0xFE00;
        for sprindex in 0..40 {

            let ypos = lock.read(baseaddr + (sprindex * 4));
            let xpos = lock.read(baseaddr + (sprindex * 4) + 1);
            let idx = lock.read(baseaddr + (sprindex * 4) + 2);
            let attr = lock.read(baseaddr + (sprindex * 4) + 3);
            //println!("{} {} {} {}",ypos,xpos,idx,attr);
            if y >= (ypos.wrapping_sub(8)) as u16 && y <= (ypos as u16) { // not sure about this
                sprtbl.push(Sprite { y: ypos, x: xpos, index: idx as i16, attrib: attr });
                println!("Found sprite! {:?}",idx);
            }
            if sprtbl.len() >= 10 {
                break;
            }
        }
        for x in sprtbl.iter(){
            println!("{:?} ",x.index);
        }
        
    }

    pub fn copy_fb_to_texture(&mut self, texture: &mut Texture){

        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| { 
            for y in 0..=65535 {
            buffer[(y) * 3] = self.framebuffer[y];
            buffer[(y) * 3 + 1] = self.framebuffer[y];
            buffer[y * 3 + 2] = self.framebuffer[y];
            }
        });

    }

    pub fn render_screen_to_fb(&mut self, mut lock: MutexGuard<Memory>){ ;
        for y in 0..=255 {
            self.render_line(y,&mut lock);
        }
    }

    fn read_tile_line(&self, line_address: u16,mut lock: MutexGuard<Memory>) -> [u8;8] {
        let mut line : [u8; 8] = [0;8];
        for x in 0..8 {
            line[x] = ((lock.read(line_address + (x * 2) as u16) >> (7-x)) & 0x1) |
            ((lock.read(line_address + (x * 2) as u16) >> (7-x) & 0x1) << 1);
        }
        return line;
    }

    pub fn new() -> Ppu {
        Ppu {
            sprites: Vec::new(),
            framebuffer: [0;256*256],
            cycles: Arc::new(AtomicU64::new(0))
        }
    }

}