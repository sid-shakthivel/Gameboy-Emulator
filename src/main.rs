mod cpu;
mod gpu;
mod mmu;
mod registers;

use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::env;

use cpu::CPU;
use gpu::GPU;
use mmu::MMU;

extern crate minifb;
use minifb::{Key, Window, WindowOptions, KeyRepeat};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("No ROM chosen");
    }
    let mut file_content: Vec<u8> = Vec::new();
    let mut file: File = File::open(&args[1]).unwrap();
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
        while cycles_elapsed < MAXCYCLES {
            if cpu.borrow().is_halted == false {
                let opcode = cpu.borrow_mut().fetch_byte();

                cycles = (cpu.borrow_mut().execute(opcode) as u16) * 4;
                cycles_elapsed += cycles as u32;
                total_cycles += cycles as u32;
            }

            cpu.borrow_mut().mmu.borrow_mut().update_timers(cycles);
            gpu.borrow_mut().update_graphics(cycles);
            cpu.borrow_mut().do_interrupts();
        }

        let keys = vec![Key::Right, Key::Left, Key::Up, Key::Down, Key::A, Key::S, Key::Space, Key::Enter];
        for (i, key) in keys.iter().enumerate() {
            if window.is_key_pressed(*key, KeyRepeat::No) {
                cpu.borrow_mut().mmu.borrow_mut().poll_key_pressed(i as u8);
            } else {
                cpu.borrow_mut().mmu.borrow_mut().poll_key_released(i as u8);
            }
        }

        for (i, pixel) in gpu.borrow().screen_data.iter().enumerate() {
            buffer[i] = *pixel;
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        cycles_elapsed = 0;
    }
}

