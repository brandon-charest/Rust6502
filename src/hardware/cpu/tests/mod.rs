use super::*;
use crate::hardware::bus::Bus;
use crate::hardware::cpu::flags::*;

struct MockBus {
    pub memory: [u8; 65536], // Raw array instead of the Memory struct
}

impl MockBus {
    fn new() -> Self {
        Self { memory: [0; 65536] }
    }
}

impl Bus for MockBus {
    fn read(&mut self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }
}

fn setup() -> (CPU, MockBus) {
    let cpu = CPU::new();
    let bus = MockBus::new();
    (cpu, bus)
}

mod arithmetic;
mod branch;
mod compare;
mod control;
mod core;
mod flags;
mod increment;
mod load_store;
mod shift;
mod stack;
mod transfer;
mod unofficial;
