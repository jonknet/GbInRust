use crate::cpu::Cpu;
use crate::ppu::Ppu;
use crate::mem::Memory;
use std::{borrow::BorrowMut, fs};
use std::path::Path;
use std::fs::File;
use std::sync::{Arc, RwLock, Mutex};
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time;
use std::cell::RefCell;
use lazy_static::lazy_static;
use std::ops::{Deref, AddAssign};
use std::sync::atomic::{AtomicU64,Ordering};
use std::time::Duration;
use std::thread::{JoinHandle, Thread};
use std::error::Error;
use std::convert::{TryInto, Infallible};
use crossbeam::atomic::AtomicCell;

pub mod cpu;
pub mod mem;
mod ppu;

extern crate sdl2;

lazy_static!(
    static ref cpu_resume: AtomicCell<bool> = AtomicCell::new(false);
);

const pal: [u8; 4] = [0xFF, 0xAC, 0x63, 0x00];

fn main() {

    let mut globalCycles = Arc::new(Mutex::new(0));

    let mut mem = Arc::new(Mutex::new(Memory::new(0)));
    let mut cpu = Arc::new(Mutex::new(Cpu::new()));
    let mut ppu = Arc::new(Mutex::new(Ppu::new()));

    let mut memmtx = Arc::clone(&mem);
    let mut memlock = memmtx.lock().unwrap();
    memlock.write(0xFF50, 0);

    let bootrom = fs::read("DMG_BOOT.bin").expect("Unable to read boot rom");
    let mut index = 0;
    for val in bootrom.iter() {
        memlock.writerom(index, *val);
        index += 1;
    }

    let rom = fs::read("tetris.gb").expect("Unable to read rom");
    index = 0;
    for val in rom.iter() {
        memlock.write(index, *val);
        index += 1;
    }
    drop(memlock);


    let sdl_context = sdl2::init().unwrap();

    let video_sub = sdl_context.video().unwrap();

    let window = video_sub.window("GbInRust", 256, 256)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256).unwrap();





    let mut ppumtx = Arc::clone(&ppu);
    let mut mtx2 = Arc::clone(&mem);
    let mut gc1 = Arc::clone(&globalCycles);
    let mut cpu_thread: Box<Option<JoinHandle<_>>> = Box::new(None);
    let ppu_thread = thread::spawn( move || {
        loop {
            static ppumtx2: Arc<Mutex<Ppu>> = ppumtx2.into();
            let mut gc2 = gc1.try_lock().unwrap();
            let mut mtx3 = mtx2.try_lock();
            for i in 0..255 {
                {


                        ppumtx2.lock().unwrap().render_line(i, &mut mtx3.as_ref().unwrap());

                    gc2.add_assign(70);

                }

            }
            drop(gc2);
            drop(mtx3);


                cpu_resume.store(true);
                thread::park();


        }});

    let cpumtx = Arc::clone(&cpu);
    let mut mtx1 = Arc::clone(&mem);
    cpu_thread = Box::from(Some((thread::spawn(move || {
        let mut cpumtx1 = cpumtx.lock().unwrap();
        let mut start = time::Instant::now();

        cpumtx1.R.pc = 0;
        loop {
            {
                let mut mtx4 = mtx1.try_lock();
                if mtx4.is_ok() {
                    cpumtx1.executeop(&mut mtx4.unwrap());
                }
            }

            if time::Instant::now().duration_since(start) >= Duration::from_micros(150) {
                start = time::Instant::now();
                ppu_thread.thread().unpark();
                while(!cpu_resume.load()){
                    // Do nothing
                }
            }
        }
    }))));

    cpu_thread.unwrap().thread().unpark();


    let mut eventpump = sdl_context.event_pump().unwrap();
    let mut ppumtx2 = Arc::clone(&ppu);

    'mainloop: loop {
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
        {
            let mut mtx = ppumtx2.try_lock();
            if mtx.is_ok() {
                mtx.unwrap().copy_fb_to_texture(&mut texture);
            }
        }

        canvas.clear();
        canvas.copy(&texture,None,None);
        canvas.present();



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


