use super::*;
use crate::hardware::cpu::flags::*;
use crate::hardware::{bus::Memory, status::Status};

struct MockBus {
    memory: Memory,
}

impl MockBus {
    fn new() -> Self {
        Self {
            memory: Memory::new(),
        }
    }
}

impl crate::hardware::bus::Bus for MockBus {
    fn read(&mut self, address: u16) -> u8 {
        self.memory.read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.memory.write(address, value);
    }
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
