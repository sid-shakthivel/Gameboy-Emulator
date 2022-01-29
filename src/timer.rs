use crate::cpu::CPU;
use crate::mmu::MMU;
use std::cell::RefCell;
use std::rc::Rc;

// TODO: Implement way to change frequency && write interrupt call

pub struct Timer {
    timer_counter: u16,
    divider_counter: u16,
    mmu: Rc<RefCell<MMU>>,
    cpu: Rc<RefCell<CPU>>,
}

impl Timer {
    pub fn new(mmu: Rc<RefCell<MMU>>, cpu: Rc<RefCell<CPU>>) -> Self {
        Self {
            timer_counter: 1024,
            divider_counter: 0,
            mmu: mmu,
            cpu: cpu,
        }
    }

    fn is_clock_enabled(&self) -> u8 {
        self.mmu.borrow_mut().rb(0xFF07) & (1 << 3)
    }

    fn get_frequency(&self) -> u32 {
        let speed: u16 = (self.mmu.borrow_mut().rb(0xFF07)
            & (1 << 1)
            & self.mmu.borrow_mut().rb(0xFF07)
            & (1 << 2)) as u16;
        match speed {
            0 => 4096,
            1 => 262144,
            10 => 65536,
            11 => 16384,
            _ => 0x01,
        }
    }

    fn reset_timer_counter(&self) -> u32 {
        match self.get_frequency() {
            4096 => 1024,
            16384 => 256,
            65536 => 64,
            262144 => 16,
            _ => 0x01,
        }
    }

    pub fn update_timers(&mut self, cycles: u16) {
        self.update_timer(cycles);
        self.update_divisor_register(cycles);
    }

    fn update_timer(&mut self, cycles: u16) {
        if self.is_clock_enabled() == 1 {
            self.timer_counter -= cycles;

            if self.timer_counter <= 0 {
                self.reset_timer_counter();
                let timer_value: u8 = self.mmu.borrow_mut().rb(0xFF06);
                if timer_value == 0xFF {
                    self.mmu
                        .borrow_mut()
                        .wb(0xFF05, self.mmu.borrow_mut().rb(0xFF06));
                    self.mmu.borrow_mut().request_interrupt(2);
                } else {
                    self.mmu.borrow_mut().wb(0xFF05, timer_value + 1);
                }
            }
        }
    }

    fn update_divisor_register(&mut self, cycles: u16) {
        self.divider_counter += cycles;
        if self.divider_counter >= 255 {
            self.divider_counter = 0;
            let divider_value: u8 = self.mmu.borrow_mut().rb(0xFF04);
            self.mmu
                .borrow_mut()
                .wb(0xFF04, divider_value.wrapping_add(1));
        }
    }
}
