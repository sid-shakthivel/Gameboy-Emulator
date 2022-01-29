use std::cell::RefCell;
use std::rc::Rc;

pub struct MMU {
    pub rom: [u8; 32769],
    pub graphics_ram: [u8; 8192],
    pub external_ram: [u8; 8192],
    pub working_ram: [u8; 8192],
    pub io_ram: [u8; 128],
    pub high_ram: [u8; 127], // Stack
    pub interrupt_enabled_register: u8,
    pub joypad_state: u8,
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut mmu = Self {
            rom: [0; 32769],
            graphics_ram: [0; 8192],
            working_ram: [0; 8192],
            external_ram: [0; 8192],
            io_ram: [0; 128],
            high_ram: [0; 127],
            interrupt_enabled_register: 0,
            joypad_state: 0,
        };

        mmu.wb(0xFF05, 0x00);
        mmu.wb(0xFF06, 0x00);
        mmu.wb(0xFF07, 0x00);
        mmu.wb(0xFF10, 0x80);
        mmu.wb(0xFF11, 0xBF);
        mmu.wb(0xFF12, 0xF3);
        mmu.wb(0xFF14, 0xBF);
        mmu.wb(0xFF16, 0x3F);
        mmu.wb(0xFF17, 0x00);
        mmu.wb(0xFF19, 0xBF);
        mmu.wb(0xFF1A, 0x7F);
        mmu.wb(0xFF1B, 0xFF);
        mmu.wb(0xFF1C, 0x9F);
        mmu.wb(0xFF1E, 0xBF);
        mmu.wb(0xFF20, 0xFF);
        mmu.wb(0xFF21, 0x00);
        mmu.wb(0xFF22, 0x00);
        mmu.wb(0xFF23, 0xBF);
        mmu.wb(0xFF24, 0x77);
        mmu.wb(0xFF25, 0xF3);
        mmu.wb(0xFF26, 0xF1);
        mmu.wb(0xFF40, 0x91);
        mmu.wb(0xFF42, 0x00);
        mmu.wb(0xFF43, 0x00);
        mmu.wb(0xFF45, 0x00);
        mmu.wb(0xFF47, 0xFC);
        mmu.wb(0xFF48, 0xFF);
        mmu.wb(0xFF49, 0xFF);
        mmu.wb(0xFF4A, 0x00);
        mmu.wb(0xFF4B, 0x00);
        mmu.wb(0xFFFF, 0x00);

        mmu.io_ram[0] = 0xF;

        let mut i: usize = 0;
        for byte in rom {
            mmu.rom[i] = byte;
            i += 1;
        }

        mmu
    }

    pub fn rb(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize],
            0x8000..=0x9FFF => self.graphics_ram[(address - 0x8000) as usize],
            0xA000..=0xBFFF => self.external_ram[(address - 0xA000) as usize],
            0xC000..=0xDFFF => self.working_ram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.working_ram[(address - 0xE000) as usize],
            0xFE00..=0xFE9F => 0, // Graphics - Sprite
            0xFF00 => self.get_joypad_state(),
            0xFF00..=0xFF7F => self.io_ram[(address - 0xFF00) as usize],
            0xFF80..=0xFFFE => self.high_ram[(address - 0xFF80) as usize],
            0xFFFF => self.interrupt_enabled_register,
            _ => 0,
        }
    }

    pub fn rw(&self, address: u16) -> u16 {
        return self.rb(address) as u16 + ((self.rb(address + 1) as u16) << 8);
    }

    pub fn wb(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.rom[address as usize] = value,
            0x8000..=0x9FFF => self.graphics_ram[(address - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.external_ram[(address - 0xA000) as usize] = value,
            0xC000..=0xDFFF => self.working_ram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.working_ram[(address - 0xE000) as usize] = value,
            0xFE00..=0xFE9F => (), // Graphics - Sprite
            0xFF04 => self.io_ram[0xFF04 - 0xFF00] = 0,
            0xFF44 => self.io_ram[0xFF44 - 0xFF00] = 0,
            0xFF46 => self.dma_transfer(address),
            0xFF00..=0xFF7F => self.io_ram[(address - 0xFF00) as usize] = value,
            0xFF80..=0xFFFE => self.high_ram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enabled_register = value,
            _ => (),
        };
    }

    pub fn ww(&mut self, address: u16, value: u16) {
        self.wb(address, value as u8 & 255);
        self.wb(address + 1, (value >> 8) as u8);
    }

    fn dma_transfer(&mut self, data: u16) {
        let address: u16 = data << 8;
        for i in 0x00..0xA0 {
            self.wb(0xFE00 + i, self.rb(address + i));
        }
    }

    pub fn request_interrupt(&mut self, index: u8) {
        let v: u8 = self.rb(0xFF0F) | (1 << index);
        self.wb(0xFF0F, v);
    }

    pub fn key_pressed(&mut self, key: u8) {
        let mut previously_unset: bool = false;

        if (self.joypad_state & (1 << key)) == 0 {
            previously_unset = true;
        }

        self.joypad_state &= 1 << key;

        let mut is_standard_button: bool = true;

        if key < 3 {
            is_standard_button = false;
        }

        let mut request_interrupt: bool = false;
        let key_req: u8 = self.rb(0xFF00);

        if is_standard_button && ((key_req & (1 << 5)) == 0) {
            request_interrupt = true;
        } else if !is_standard_button && (key_req & (1 << 5) == 1) {
            request_interrupt = true;
        }

        if request_interrupt && !previously_unset {
            self.request_interrupt(4);
        }
    }

    fn get_joypad_state(&self) -> u8 {
        let mut res: u8 = self.io_ram[0];
        res ^= 0xFF;
        if res & (1 << 4) == 0 {
            let mut top_joypad: u8 = self.joypad_state >> 4;
            top_joypad |= 0xF0;
            res &= top_joypad;
        } else if res & (1 << 5) == 0 {
            let mut bottom_joypad: u8 = self.joypad_state & 0xF;
            bottom_joypad |= 0xF0;
            res &= bottom_joypad;
        }
        res
    }

    pub fn key_released(&mut self, key: u8) {
        self.joypad_state |= 1 << key;
    }
}
