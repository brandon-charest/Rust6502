use crate::hardware::{bus::Bus, status::Status};

use super::registers::Registers;

const RESET_VECTOR: u16 = 0xFFFC;

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub cycles: u64,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            cycles: 0,
        }
    }
    #[inline]
    pub fn read(&mut self, bus: &dyn Bus, addr: u16) -> u8 {
        self.cycles += 1;
        bus.read(addr)
    }

    #[inline]
    pub fn write(&mut self, bus: &mut dyn Bus, addr: u16, data: u8) {
        self.cycles += 1;
        bus.write(addr, data);
    }

    pub fn step(&mut self, bus: &mut dyn Bus) {
        let opcode = self.fetch_byte(bus);

        match opcode {
            0xEA => {
                self.cycles += 1;
            }
            _ => todo!("Opcode {:#X} not implemented", opcode),
        }
    }

    pub fn reset(&mut self, bus: &mut dyn Bus) {
        self.registers.accumulator = 0;
        self.registers.x_register = 0;
        self.registers.y_register = 0;
        self.registers.stack_pointer = 0xFD;
        self.registers.program_counter = self.read_u16(bus, RESET_VECTOR);
        self.registers.status_register = Status::default();
    }

    pub fn fetch_byte(&mut self, bus: &mut dyn Bus) -> u8 {
        let addr = self.registers.program_counter;
        let data = self.read(bus, addr);
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
        data
    }

    pub fn read_u16(&self, bus: &dyn Bus, addr: u16) -> u16 {
        let lo = self.read(bus, addr) as u16;
        let hi = self.read(bus, addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
}

#[cfg(test)]
mod tests {
    use crate::hardware::{bus::Memory, status::Status};

    use super::*;

    #[test]
    fn test_cpu_new() {
        let cpu = CPU::new();
        assert_eq!(cpu.registers.accumulator, 0);
        assert_eq!(cpu.registers.x_register, 0);
        assert_eq!(cpu.registers.y_register, 0);
        assert_eq!(cpu.registers.stack_pointer, 0xFD);
        assert_eq!(cpu.registers.program_counter, 0);
        assert_eq!(cpu.registers.status_register, Status::default());
    }

    #[test]
    fn test_cpu_reset() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        bus.write(0xFFFC, 0x00);
        bus.write(0xFFFD, 0x80);

        cpu.reset(&mut bus);

        assert_eq!(cpu.registers.program_counter, 0x8000);
        assert_eq!(cpu.registers.stack_pointer, 0xFD);
        assert!(
            cpu.registers
                .status_register
                .contains(Status::DISABLE_INTERRUPTS)
        );
    }

    #[test]
    fn test_fetch_byte() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // Put an opcode at 0x8000
        let pc_start = 0x8000;
        cpu.registers.program_counter = pc_start;
        bus.write(pc_start, 0xEA); // NOP instruction

        let opcode = cpu.fetch_byte(&mut bus);

        assert_eq!(opcode, 0xEA);
        assert_eq!(cpu.registers.program_counter, pc_start + 1);
    }

    #[test]
    fn test_read_u16() {
        let mut bus = Memory::new();
        let cpu = CPU::new();

        bus.write(0x2000, 0x34);
        bus.write(0x2001, 0x12);

        let value = cpu.read_u16(&bus, 0x2000);

        assert_eq!(value, 0x1234);
    }

    #[test]
    fn test_read_u16_wrapping() {
        let mut bus = Memory::new();
        let cpu = CPU::new();

        bus.write(0xFFFF, 0xAA);
        bus.write(0x0000, 0xBB);

        let value = cpu.read_u16(&bus, 0xFFFF);

        assert_eq!(value, 0xBBAA);
    }
}
