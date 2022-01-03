mod cpu;
mod gpu;
mod mmu;
mod registers;
mod timer;

use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::{thread, time};

use cpu::CPU;
use gpu::GPU;
use mmu::MMU;
use timer::Timer;

extern crate minifb;
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let mut file_content: Vec<u8> = Vec::new();
    //Passed
    // let mut file: File = File::open("ROMS/cpu_instrs/individual/05-op rp.gb").unwrap();
    // let mut file: File = File::open("ROMS/cpu_instrs/individual/06-ld r,r.gb").unwrap();

    // Failed
    // let mut file: File = File::open("ROMS/cpu_instrs/individual/04-op r,imm.gb").unwrap();
    // let mut file: File = File::open("ROMS/cpu_instrs/individual/01-special.gb").unwrap();
    // let mut file: File = File::open("ROMS/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb").unwrap();
    // let mut file: File = File::open("ROMS/cpu_instrs/individual/08-misc instrs.gb").unwrap();
    // let mut file: File = File::open("ROMS/cpu_instrs/individual/09-op r,r.gb").unwrap();
    let mut file: File = File::open("ROMS/cpu_instrs/individual/10-bit ops.gb").unwrap();
    // let mut file: File = File::open("ROMS/cpu_instrs/individual/11-op a,(hl).gb").unwrap();
    file.read_to_end(&mut file_content).unwrap();
    let mmu: Rc<RefCell<MMU>> = Rc::new(RefCell::new(MMU::new(file_content)));

    let cpu = Rc::new(RefCell::new(CPU::new(Rc::clone(&mmu))));
    let gpu = GPU::new(Rc::clone(&mmu), Rc::clone(&cpu));
    let timer = Timer::new(Rc::clone(&mmu), Rc::clone(&cpu));

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

    cycle(cpu, RefCell::new(gpu), RefCell::new(timer), window);
}

// pc is incremented in fetch_byte() so to get actual value, -1

// fn cycle(cpu: Rc<RefCell<CPU>>, gpu: RefCell<GPU>, timer: RefCell<Timer>, mut window: Window) {
// let mut cycles_elapsed: u32 = 0;
// while cycles_elapsed <= 559240 {
// let opcode = cpu.borrow_mut().fetch_special_opcode();
// println!("A: {:#X} F: {:#X} B: {:#X} C: {:#X} D: {:#X} E: {:#X} H: {:#X} L: {:#X} SP: {:#X} PC: {:#X} {:#X}", cpu.borrow().registers.a, cpu.borrow().registers.f, cpu.borrow().registers.b, cpu.borrow().registers.c, cpu.borrow().registers.d, cpu.borrow().registers.e, cpu.borrow().registers.h, cpu.borrow().registers.l, cpu.borrow().registers.sp, cpu.borrow().registers.pc - 1, opcode);
// let cycles: u16 = cpu.borrow_mut().execute(opcode) as u16;
// cycles_elapsed += cycles as u32;
// }
// }

fn cycle(cpu: Rc<RefCell<CPU>>, gpu: RefCell<GPU>, timer: RefCell<Timer>, mut window: Window) {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    const MAXCYCLES: u32 = 70221;
    let mut cycles_elapsed: u32 = 0;
    let mut total_cycles = 0;
    let mut cycles: u16 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        while cycles_elapsed < MAXCYCLES {
            if cpu.borrow().is_stopped == false {
                let opcode = cpu.borrow_mut().fetch_special_opcode();
                let flags = cpu.borrow_mut().registers.compose_flags();
                println!("A: {:#X} F: {} BC: {:#X} DE: {:#X} HL: {:#X} SP: {:#X} PC: {:#X} Opcode: {:#X} 0xFF44: {:#X} CY: {}", cpu.borrow().registers.a, flags, cpu.borrow().registers.bc(), cpu.borrow().registers.de(), cpu.borrow().registers.hl(), cpu.borrow().registers.sp, cpu.borrow().registers.pc - 1, opcode, cpu.borrow().mmu.borrow().rb(0xFF44), total_cycles);
                cycles = (cpu.borrow_mut().execute(opcode) as u16) * 4;
                cycles_elapsed += cycles as u32;
                total_cycles += cycles as u32;
                timer.borrow_mut().update_timers(cycles);
                gpu.borrow_mut().update_graphics(cycles);
                cpu.borrow_mut().do_interrupts();
                if opcode == 0xF0 {
                    let v = cpu.borrow_mut().mmu.borrow_mut().rb(0xFF44);
                    cpu.borrow_mut().registers.a = v;
                }
            }
        }

        for (i, pixel) in gpu.borrow().screen_data.iter().enumerate() {
            buffer[i] = *pixel;
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        cycles_elapsed = 0;
    }
}
