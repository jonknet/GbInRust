use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::mem::Memory;
use std::{borrow::BorrowMut, fs};
use std::path::Path;
use std::fs::File;
use std::sync::{Arc,RwLock,Mutex};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time;

pub mod cpu;
pub mod mem;
mod ppu;

extern crate sdl2;

fn main() {




    let mut mem = Memory::new();
    
    
    mem.write(0xFF50,0);
    
    let pal : [u8;4] = [0xFF,0xAC,0x63,0x00];
    
        let bootrom = fs::read("DMG_BOOT.bin").expect("Unable to read boot rom");

        let mut index = 0;
        for val in bootrom.iter() {
            mem.writerom(index,*val);
            index += 1;
        }

        let rom = fs::read("tetris.gb").expect("Unable to read rom");
        index = 0;
        for val in rom.iter() {
            mem.write(index,*val);
            index += 1;
        }
    
        let mut cpu = Cpu::new(mem);
            
            let cpu_thread = thread::spawn(move||{
                
                
                cpu.R.pc = 0;
                loop {
                    let memmtx = Arc::new(Mutex::new(mem));
                
                    cpu.executeop(memmtx);
                }
                
            });
            
            
        
            let sdl_context = sdl2::init().unwrap();
            
            let video_sub = sdl_context.video().unwrap();
        
            let window = video_sub.window("GbInRust",256,256)
                .opengl()
                .resizable()
                .build()
                .unwrap();

            let mut ppu = Ppu::new(Arc::new(Mutex::new(mem)));
            let mut ppumutex = Arc::new(Mutex::new(ppu));

            let mut canvas = window.into_canvas().build().unwrap();
            let texture_creator = canvas.texture_creator();
        
            let mut texture = texture_creator
                .create_texture_streaming(PixelFormatEnum::RGB24,256,256).unwrap();

            let ppu2 = ppumutex.clone();
            let ppu_thread = thread::spawn(move||loop{
                let ppulock = ppu2.lock().unwrap();    
                //ppu2.lock().unwrap().render_screen_to_fb(); 
                println!("PPU");
                drop(ppulock);   
            });

            let mut eventpump = sdl_context.event_pump().unwrap();
            
            let mut ppumutex2 = ppumutex.clone();
            
            'mainloop : loop {
                for event in eventpump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => break 'mainloop,
                        _ => {}
                    }
                } 
                let mut ppulock = ppumutex2.lock().unwrap();
                //ppulock.copy_fb_to_texture(&mut texture);
                canvas.clear();
                //canvas.copy(&texture,None,None);
                canvas.present();
                drop(ppulock);
            }
            


    }
/*
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            let mut offs = 0;
            for tile in 0..384 {
                for y in 0..8 {
                    for x in 0..8 {
                        fb[(tile / 32) * 2048 + (y*256) + (tile % 32) * 8 + x] = pal[(((cpu.M.read((0x8000 + (tile * 16) + (y * 2)) as u16) >> (7 - x)) & 1) |
                            (((cpu.M.read((0x8000 + (tile * 16) + (y * 2) + 1) as u16) >> (7 - x)) & 1) << 1)) as usize];
                    }
                }
            }
            for y in 0..256 {
                for x in 0..256 {
                    let offset = ((y*256)+x)*3;
                    buffer[offset] = fb[x+y*256];
                    buffer[offset+1] = fb[x+y*256];
                    buffer[offset+2] = fb[x+y*256];
                }
            }
        });
        canvas.clear();
        canvas.copy(&texture,None,None);
        canvas.present();

        cpu.executeop();
    }
*/


