mod cpu;
mod gpu;
mod mmu;
mod registers;

use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::{thread, time};

use cpu::CPU;
use gpu::GPU;
use mmu::MMU;

extern crate minifb;
use minifb::{Key, Window, WindowOptions, KeyRepeat};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let mut file_content: Vec<u8> = Vec::new();
    let mut file: File = File::open("ROMS/tetris.gb").unwrap();
    file.read_to_end(&mut file_content).unwrap();
    let mmu: Rc<RefCell<MMU>> = Rc::new(RefCell::new(MMU::new(file_content)));

    let cpu = Rc::new(RefCell::new(CPU::new(Rc::clone(&mmu))));
    let gpu = GPU::new(Rc::clone(&mmu), Rc::clone(&cpu));

    let mut window = Window::new(
        "Gameboy Emulator - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    cycle(cpu, RefCell::new(gpu), window);
}

// pc is incremented in fetch_byte() so to get actual value, -1
fn cycle(cpu: Rc<RefCell<CPU>>, gpu: RefCell<GPU>, mut window: Window) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    const MAXCYCLES: u32 = 70221 * 1;
    let mut cycles_elapsed: u32 = 0;
    let mut total_cycles = 0;
    let mut cycles: u16 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {

        // let buttons: [Key; 8] = [
        //     Key::Left,
        //     Key::Right,
        //     Key::Up,
        //     Key::Down,
        //     Key::A,
        //     Key::S,
        //     Key::Space,
        //     Key::Enter
        // ];
        //
        // for i in 0..buttons.len() {
        //     cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(i as u8, window.is_key_pressed(buttons[i], KeyRepeat::No));
        //     cpu.borrow_mut().mmu.borrow_mut().poll_key_released(i as u8, window.is_key_released(buttons[i]));
        // }

        while cycles_elapsed < MAXCYCLES {
            if cpu.borrow().is_stopped == false {
                let opcode = cpu.borrow_mut().fetch_byte();

                cycles = (cpu.borrow_mut().execute(opcode) as u16) * 4;
                cycles_elapsed += cycles as u32;
                total_cycles += cycles as u32;
                cpu.borrow_mut().mmu.borrow_mut().update_timers(cycles);
                gpu.borrow_mut().update_graphics(cycles);
                cpu.borrow_mut().do_interrupts();
            }
        }

        for (i, pixel) in gpu.borrow().screen_data.iter().enumerate() {
            buffer[i] = *pixel;
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        cycles_elapsed = 0;

        if window.is_key_pressed(Key::A, KeyRepeat::No) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(4);
        }
        if window.is_key_pressed(Key::S, KeyRepeat::No) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(5);
        }
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(6);
        }
        if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(7);
        }
        if window.is_key_pressed(Key::Right, KeyRepeat::No) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(0);
        }
        if window.is_key_pressed(Key::Left, KeyRepeat::No) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(1);
        }
        if window.is_key_pressed(Key::Up, KeyRepeat::No) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(2);
        }
        if window.is_key_pressed(Key::Down, KeyRepeat::No) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(3);
        }

        if window.is_key_released(Key::A) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_released(4);
        }
        if window.is_key_released(Key::S) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_released(5);
        }
        if window.is_key_released(Key::Space) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_released(6);
        }
        if window.is_key_released(Key::Enter) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_released(7);
        }
        if window.is_key_released(Key::Right) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_released(0);
        }
        if window.is_key_released(Key::Left) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_released(1);
        }
        if window.is_key_released(Key::Up) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_released(2);
        }
        if window.is_key_released(Key::Down) {
            cpu.borrow_mut().mmu.borrow_mut().poll_key_released(3);
        }
    }
}

