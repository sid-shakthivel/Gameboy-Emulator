use crate::cpu::CPU;
use crate::mmu::MMU;
use std::cell::RefCell;
use std::rc::Rc;

// Write interrupt call

pub struct GPU {
    scanline_counter: u32,
    mmu: Rc<RefCell<MMU>>,
    cpu: Rc<RefCell<CPU>>,
    pub screen_data: [u32; 23040],
}

impl GPU {
    pub fn new(mmu: Rc<RefCell<MMU>>, cpu: Rc<RefCell<CPU>>) -> Self {
        Self {
            scanline_counter: 0,
            mmu: mmu,
            cpu: cpu,
            screen_data: [0; 23040],
        }
    }

    pub fn update_graphics(&mut self, cycles: u16) {
        self.set_lcd_status();

        if self.is_lcd_enabled() == 0x80 {
            self.scanline_counter += cycles as u32;
            // self.scanline_counter -= (cycles as i32);
        } else {
            println!("LCD not enabled?");
            return;
        }

        if self.scanline_counter >= 456 {
            let previous_scanline: u8 = self.mmu.borrow().rb(0xFF44);

            if previous_scanline > 153 {
                self.mmu.borrow_mut().io_ram[0xFF44 - 0xFF00] = 0;
                self.draw_scanline();
            }

            let v = self.mmu.borrow_mut().io_ram[0xFF44 - 0xFF00].wrapping_add(1);
            self.mmu.borrow_mut().io_ram[0xFF44 - 0xFF00] = v;
            let current_scanline: u8 = self.mmu.borrow().rb(0xFF44);
            self.scanline_counter = 0;

            if current_scanline == 144 {
                // V-Blank Interrupt
                self.cpu.borrow_mut().request_interrupt(0);
            } else if current_scanline > 153 {
                // Reset Scanline
                self.mmu.borrow_mut().io_ram[0xFF44 - 0xFF00] = 0;
                self.draw_scanline();
            } else if current_scanline < 144 {
                // Draw scanline
                self.draw_scanline();
            }

        }
    }

    fn set_lcd_status(&mut self) {
        if self.is_lcd_enabled() == 0 {
            self.scanline_counter = 0;
            self.mmu.borrow_mut().io_ram[0xFF44 - 0xFF00] = 0;
            let status: u8 = self.mmu.borrow().rb(0xFF41) | 0x1;
            self.mmu.borrow_mut().wb(0xFF41, status);
            return;
        }

        let current_mode = self.mmu.borrow().rb(0xFF41) & 0x3;
        let current_scanline: u8 = self.mmu.borrow().rb(0xFF44);
        let mut new_mode = 0;
        let mut check_bit = 0;
        let status: u8 = self.mmu.borrow().rb(0xFF41);

        if current_scanline > 144 {
            // V-Blank
            new_mode = 1;
            check_bit = status & (1 << 3);
            self.mmu.borrow_mut().wb(0xFF41, status | 0x1);
        } else if self.scanline_counter >= (456 - 80) {
            // Searching Sprites Attributes
            new_mode = 2;
            self.mmu.borrow_mut().wb(0xFF41, status | 0x2);
            check_bit = status & (1 << 4);
        } else if self.scanline_counter >= (456 - 172) {
            // Transferring Data to LCD Driver
            new_mode = 3;
            self.mmu.borrow_mut().wb(0xFF41, status & 0x3);
        } else {
            // H-Blank
            new_mode = 0;
            self.mmu.borrow_mut().wb(0xFF41, status & 252);
            check_bit = status & (1 << 5);
        }

        if new_mode != current_mode && check_bit > 0 {
            self.cpu.borrow_mut().request_interrupt(1);
        }

        // Coincidence Flag
        if current_scanline == self.mmu.borrow().rb(0xFF45) {
            self.mmu.borrow_mut().wb(0xFF41, status | (1 << 2));
        } else {
            self.mmu.borrow_mut().wb(0xFF41, status & !(1 << 2));
        }

        if self.mmu.borrow().rb(0xFF41) & (1 << 2) == 1
            && self.mmu.borrow().rb(0xFF41) & (1 << 6) == 1
        {
            self.cpu.borrow_mut().request_interrupt(1);
        }

    }

    fn is_lcd_enabled(&self) -> u8 {
        self.mmu.borrow().rb(0xFF40) & (1 << 7)
    }

    fn draw_scanline(&mut self) {
        let lcd_control = self.mmu.borrow().rb(0xFF40);
        if lcd_control & (1 << 1) == 1 {
            // self.render_sprites();
        }

        if lcd_control & (1 << 0) == 1 {
            self.render_tiles();
        }
    }

    // Redo with generics
    fn get_bit(&mut self, byte: u8, index: i8) -> u8 {
        if byte & (1 << index) > 0 {
            return 1;
        }
        0
    }

    fn render_tiles(&mut self) {
        // Identify tile in background using coords
        // Lookup tile data in tile data region
        // Get 2 Bytes & Identify Colour

        let scroll_y: u8 = self.mmu.borrow().rb(0xFF42); // Y position of BACKGROUND to draw view
        let scroll_x: u8 = self.mmu.borrow().rb(0xFF43); // X position of BACKGROUND to draw view

        let window_y: u8 = self.mmu.borrow().rb(0xFF4A); // Y position of VIEW to draw window
        let window_x: u8 = self.mmu.borrow().rb(0xFF4B); // X position of VIEW to draw window

        let lcd_control: u8 = self.mmu.borrow().rb(0xFF40);
        let current_scanline: u8 = self.mmu.borrow().rb(0xFF44);
        let colour_palette: u8 = self.mmu.borrow().rb(0xFF47);

        let tile_identity_address: u16; // Location in which tile identification numbers are stored
        let mut tile_data_address: u16; // Location in which tile data (pixel information) is stored

        let mut is_signed: bool = false;
        let mut is_window: bool = false;

        let mut x_pos: u8 = 0;
        let mut y_pos: u8 = 0;

        if lcd_control & (1 << 5) == 1 {
            if window_y <= current_scanline {
                is_window = true;
            }
            panic!("Window!");
        }

        if is_window == false {
            if lcd_control & (1 << 3) > 0 {
                tile_identity_address = 0x9C00;
            } else {
                tile_identity_address = 0x9800;
            }
        } else {
            if lcd_control & (1 << 3) > 0 {
                tile_identity_address = 0x9C00;
            } else {
                tile_identity_address = 0x9800;
            }
        }

        if lcd_control & (1 << 4) > 0 {
            tile_data_address = 0x8000;
        } else {
            tile_data_address = 0x8800;
            is_signed = true;
            panic!("Signed!");
        }

        // Determine vertical tile
        if is_window {
            // Must subtract as window_y just gives coordinates of view
            y_pos = current_scanline - window_y;
        } else {
            // Must add scanline onto scroll_y as it just gives coordinates of background
            y_pos = current_scanline + scroll_y;
        }

        // println!("Y-POS: {:#X} Y-OFFSET: {:#X}", y_pos, scroll_y);
        let tile_row: u16 = (((y_pos / 8) as u16) * 32) as u16;

        // 160 vertical pixels and 20 tiles
        for i in 0..20 {
            // Determine Horizontal Tile
            // Determine Line
            // For Each Horizontal Pixel Loop and Adjust Framebuffer

            let base = i * 8;
            x_pos = base + scroll_x;
            let tile_col: u16 = (x_pos / 8) as u16;
            let mut signed_tile_identifier: i16 = 0;
            let mut unsigned_tile_identifier: u16 = 0;
            let tile_identifier_address = tile_identity_address + tile_col + tile_row;

            if is_signed {
                // signed_tile_identifier = self.mmu.borrow().rb(tile_identifier_address) as i16;
            } else {
                unsigned_tile_identifier = self.mmu.borrow().rb(tile_identifier_address) as u16;
            }

            if is_signed {
                // tile_data_address += (signed_tile_identifier + 128) * 16;
            } else {
                tile_data_address = 0x8000 + (unsigned_tile_identifier * 16);
                if tile_data_address != 0x8200 {
                    // println!("{:#X}", tile_data_address);
                }
            }

            let mut line: u16 = (y_pos % 8) as u16;
            line *= 2;

            let data1 = self.mmu.borrow().rb(tile_data_address + line);
            let data2 = self.mmu.borrow().rb(tile_data_address + line + 1);
            for mut j in (0..8).rev() {
                let data_colour: u8 = self.get_bit(data2, j) << 1 | self.get_bit(data1, j);

                let rgb = match data_colour {
                    0b00 => self.get_colour(colour_palette, 0),
                    0b01 => self.get_colour(colour_palette, 2),
                    0b10 => self.get_colour(colour_palette, 4),
                    0b11 => self.get_colour(colour_palette, 6),
                    _ => {
                        panic!("Wrong Combination");
                    }
                };

                j -= 7;
                j *= -1;

                let mut res: u32 = 0;
                res = res << 8 | (rgb.0 as u32);
                res = res << 8 | (rgb.1 as u32);
                res = res << 8 | (rgb.2 as u32);

                let test: usize = j as usize + base as usize;
                if current_scanline <= 143 && test <= 159 {
                    let index: usize =
                        ((current_scanline) as usize * 160) + (j as usize + base as usize);
                    self.screen_data[index] = res;
                }
            }
        }
    }

    // Refactor
    fn get_colour(&mut self, palette: u8, index: i8) -> (u8, u8, u8) {
        let bit: u8 = self.get_bit(palette, index) << 1 | self.get_bit(palette, index + 1);

        match bit {
            0b00 => (0xFF, 0xFF, 0xFF),
            0b01 => (0xCC, 0xCC, 0xCC),
            0b10 => (0x77, 0x77, 0x77),
            0b11 => (0, 0, 0),
            _ => {
                panic!("Wrong Combination");
            }
        }
    }

    pub fn output_thing(&mut self, tile_data_address: u16) {
        let colour_palette: u8 = self.mmu.borrow().rb(0xFF47);

        // 8 Words
        let mut i: u16 = 0;
        let y_offset = 20;

        while i < 16 {
            let data1 = self.mmu.borrow().rb(tile_data_address + i);
            let data2 = self.mmu.borrow().rb(tile_data_address + i + 1);

            // 8 Bits in Byte
            for mut j in (0..8).rev() {
                let data_colour: u8 = self.get_bit(data2, j) << 1 | self.get_bit(data1, j);

                let rgb = match data_colour {
                    0b00 => self.get_colour(colour_palette, 0),
                    0b01 => self.get_colour(colour_palette, 2),
                    0b10 => self.get_colour(colour_palette, 4),
                    0b11 => self.get_colour(colour_palette, 6),
                    _ => {
                        panic!("Wrong Combination");
                    }
                };

                j -= 7;
                j *= -1;

                let mut res: u32 = 0;
                res = res << 8 | (rgb.0 as u32);
                res = res << 8 | (rgb.1 as u32);
                res = res << 8 | (rgb.2 as u32);
                let index: usize = ((i / 2 + y_offset) as usize * 160) + (j as usize);
                println!("{}", index);
                self.screen_data[index] = res;
            }
            i += 2;
        }
    }
}
