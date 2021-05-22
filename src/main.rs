use crate::cpu::Cpu;
use std::fs;
use std::path::Path;
use std::fs::File;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod cpu;
pub mod mem;
mod ppu;

extern crate sdl2;

fn main() {
    let mut cpu = Cpu::new();

    let bootrom = fs::read("DMG_BOOT.bin").expect("Unable to read boot rom");

    let mut index = 0;
    for val in bootrom.iter() {
        cpu.M.writerom(index,*val);
        index += 1;
    }

    let rom = fs::read("tetris.gb").expect("Unable to read rom");
    index = 0;
    for val in rom.iter() {
        cpu.M.write(index,*val);
        index += 1;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_sub = sdl_context.video().unwrap();

    let window = video_sub.window("GbInRust",256,256)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24,256,256).unwrap();

    let mut fb : [u8;256 * 256] = [0;256 * 256];
    cpu.M.write(0xFF50,0);
    cpu.R.pc = 0;
    let pal : [u8;4] = [0xFF,0xAC,0x63,0x00];

    let mut event_pump = sdl_context.event_pump().unwrap();

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


}
