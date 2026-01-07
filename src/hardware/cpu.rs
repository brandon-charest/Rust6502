use crate::hardware::{
    bus::Bus,
    instructions::{AddressingMode, Opcode, OpcodeSyntax},
    status::Status,
};

use super::registers::Registers;

const RESET_VECTOR: u16 = 0xFFFC;

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
    pub cycles: u64,
    pub halted: bool,
}

#[derive(PartialEq, Eq)]
enum AccessMode {
    Read,  // LDA, LDX, LDY, EOR, AND, ORA, ADC, SBC, CMP, BIT
    Write, // STA, STX, STY, INC, DEC, ASL, LSR, ROL, ROR
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            cycles: 0,
            halted: false,
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
        let raw_data = self.fetch_byte(bus);
        let opcode = Opcode::from_u8(raw_data).expect("Unknown Opcode");

        match opcode.syntax {
            OpcodeSyntax::LDA => {
                let addr = self.get_operand_address(&opcode.mode, bus);
                let value = self.read(bus, addr);
                self.registers.accumulator = value;
                self.update_nz_flags(value);
            }
            OpcodeSyntax::NOP => {
                let _ = self.read(bus, self.registers.program_counter);
            }
            OpcodeSyntax::STA => {
                let addr = self.get_operand_address(&opcode.mode, bus);
                self.write(bus, addr, self.registers.accumulator);
            }
            _ => todo!(),
        }
    }

    pub fn reset(&mut self, bus: &mut dyn Bus) {
        self.registers.accumulator = 0;
        self.registers.x_register = 0;
        self.registers.y_register = 0;
        self.registers.stack_pointer = 0xFD;
        self.registers.program_counter = self.read_u16(bus, RESET_VECTOR);
        self.registers.status = Status::default();
    }

    pub fn fetch_byte(&mut self, bus: &mut dyn Bus) -> u8 {
        let addr = self.registers.program_counter;
        let data = self.read(bus, addr);
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
        data
    }

    pub fn read_u16(&mut self, bus: &mut dyn Bus, addr: u16) -> u16 {
        let lo = self.read(bus, addr) as u16;
        let hi = self.read(bus, addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub fn fetch_u16(&mut self, bus: &mut dyn Bus) -> u16 {
        let lo = self.fetch_byte(bus) as u16;
        let hi = self.fetch_byte(bus) as u16;
        (hi << 8) | lo
    }

    fn get_operand_address(&mut self, mode: &AddressingMode, bus: &mut dyn Bus) -> u16 {
        match mode {
            AddressingMode::Absolute => {
                let addr = self.fetch_u16(bus);
                addr
            }
            AddressingMode::AbsoluteX => {
                let base = self.fetch_u16(bus);
                let addr = base.wrapping_add(self.registers.x_register as u16);

                if (base & 0xFF00) != (addr & 0xFF00) {
                    // Crossing Page, Burn a cycle
                    let _ = self.read(bus, addr.wrapping_sub(0x0100));
                }

                addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.fetch_u16(bus);
                let addr = base.wrapping_add(self.registers.y_register as u16);

                if (base & 0xFF00) != (addr & 0xFF00) {
                    // Crossing Page, Burn a cycle
                    let _ = self.read(bus, addr.wrapping_sub(0x0100));
                }

                addr
            }
            AddressingMode::Immediate => {
                let addr = self.registers.program_counter;
                self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
                addr
            }
            AddressingMode::IndirectX => {
                let base = self.fetch_byte(bus);
                let ptr = base.wrapping_add(self.registers.x_register);

                // Burn cycle for addition
                let _ = self.read(bus, base as u16);

                let lo = self.read(bus, ptr as u16) as u16;
                let hi = self.read(bus, ptr.wrapping_add(1) as u16) as u16;

                (hi << 8) | lo
            }
            AddressingMode::IndirectY => {
                let ptr = self.fetch_byte(bus);

                let lo = self.read(bus, ptr as u16) as u16;
                let hi = self.read(bus, ptr.wrapping_add(1) as u16) as u16;
                let base_addr = (hi << 8) | lo;
                let addr = base_addr.wrapping_add(self.registers.y_register as u16);

                if (addr & 0xFF00) != (base_addr & 0xFF00) {
                    let _ = self.read(bus, addr.wrapping_sub(0x0100));
                }

                addr
            }
            AddressingMode::ZeroPage => {
                let zp_addr = self.fetch_byte(bus);
                zp_addr as u16
            }
            AddressingMode::ZeroPageX => {
                let pos = self.fetch_byte(bus);
                let addr = pos.wrapping_add(self.registers.x_register) as u16;

                // Burn a cycle for the addition!
                // The real 6502 spends 1 cycle doing the math (pos + X) inside the ALU
                let _ = self.read(bus, pos as u16);
                addr as u16
            }
            AddressingMode::ZeroPageY => {
                let pos = self.fetch_byte(bus);
                let addr = pos.wrapping_add(self.registers.y_register) as u16;

                // Burn a cycle for the addition!
                // The real 6502 spends 1 cycle doing the math (pos + Y) inside the ALU
                let _ = self.read(bus, pos as u16);
                addr as u16
            }
            _ => todo!("Addressing mode {:?} not yet implemented", mode),
        }
    }

    fn update_nz_flags(&mut self, value: u8) {
        // .set(FLAG, bool)
        self.registers.status.set(Status::ZERO, value == 0);
        self.registers
            .status
            .set(Status::NEGATIVE, (value & 0x80) != 0);
    }

    pub fn debug_info(&self, bus: &dyn Bus) {
        let registers = &self.registers;
        let status = registers.status;
        print!(
            "PC: {:04X} | A: {:02X} X: {:02X} Y: {:02X} | SP: {:02X} | Flags: ",
            registers.program_counter,
            registers.accumulator,
            registers.x_register,
            registers.y_register,
            registers.stack_pointer
        );

        let flags = [
            if status.contains(Status::NEGATIVE) {
                'N'
            } else {
                'n'
            },
            if status.contains(Status::OVERFLOW) {
                'V'
            } else {
                'v'
            },
            '-',
            if status.contains(Status::BRK) {
                'B'
            } else {
                'b'
            },
            if status.contains(Status::DECIMAL_MODE) {
                'D'
            } else {
                'd'
            },
            if status.contains(Status::DISABLE_INTERRUPTS) {
                'I'
            } else {
                'i'
            },
            if status.contains(Status::ZERO) {
                'Z'
            } else {
                'z'
            },
            if status.contains(Status::CARRY) {
                'C'
            } else {
                'c'
            },
        ];

        for f in flags {
            print!("{}", f);
        }

        print!(" | MEM: ");
        for i in 0..8 {
            let addr = registers.program_counter.wrapping_add(i);
            print!("{:02X} ", bus.read(addr));
        }

        println!("| Cycles: {}", self.cycles);
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
        assert_eq!(cpu.registers.status, Status::default());

        assert_eq!(cpu.cycles, 0);
        assert_eq!(cpu.halted, false);
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
        assert!(cpu.registers.status.contains(Status::DISABLE_INTERRUPTS));
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
    fn test_fetch_u16() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        bus.write(0x2000, 0x34);
        bus.write(0x2001, 0x12);

        let value = cpu.read_u16(&mut bus, 0x2000);

        assert_eq!(value, 0x1234);
    }

    #[test]
    fn test_fetch_u16_wrapping() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        bus.write(0xFFFF, 0xAA);
        bus.write(0x0000, 0xBB);

        let value = cpu.read_u16(&mut bus, 0xFFFF);

        assert_eq!(value, 0xBBAA);
    }

    #[test]
    fn test_lda_immediate_timing() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // Program: LDA $10 (Immediate)
        bus.write(0x8000, 0xA9); // Opcode (Cycle 1)
        bus.write(0x8001, 0x10); // Operand: $10 (The value 16)

        cpu.registers.program_counter = 0x8000;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.cycles, 2, "Immediate should take exactly 2 cycles");
        assert_eq!(cpu.registers.program_counter, 0x8002);
        assert_eq!(cpu.registers.accumulator, 0x10);
    }

    #[test]
    fn test_lda_zeropage_timing() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // Program: LDA $44 (Zero Page)
        bus.write(0x8000, 0xA5); // Opcode (Cycle 1)
        bus.write(0x8001, 0x44); // Address $44 (Cycle 2)
        bus.write(0x0044, 0x55); // Data at target (Cycle 3)

        cpu.registers.program_counter = 0x8000;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.cycles, 3, "Zero Page should take exactly 3 cycles");
        assert_eq!(cpu.registers.program_counter, 0x8002);
        assert_eq!(cpu.registers.accumulator, 0x55);
    }

    #[test]
    fn test_lda_zeropage_x_timing() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // Program: LDA $44 (Zero Page X)
        bus.write(0x8000, 0xB5);
        bus.write(0x8001, 0x44);
        bus.write(0x0049, 0xAF);

        cpu.registers.program_counter = 0x8000;
        cpu.registers.x_register = 0x05;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.cycles, 4, "Zero Page should take exactly 4 cycles");
        assert_eq!(cpu.registers.program_counter, 0x8002);
        assert_eq!(cpu.registers.accumulator, 0xAF);
    }

    #[test]
    fn test_lda_absolute() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // LDA $1234
        bus.write(0x8000, 0xAD); // Opcode
        bus.write(0x8001, 0x34); // Low Byte
        bus.write(0x8002, 0x12); // High Byte
        bus.write(0x1234, 0x55);

        cpu.registers.program_counter = 0x8000;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.cycles, 4, "Absolute should take exactly 4 cycles");
        assert_eq!(cpu.registers.program_counter, 0x8003);
        assert_eq!(cpu.registers.accumulator, 0x55);
    }

    #[test]
    fn test_lda_absolute_x_no_crossing() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // LDA $1000, X
        bus.write(0x8000, 0xBD);
        bus.write(0x8001, 0x00); // Low
        bus.write(0x8002, 0x10); // High
        bus.write(0x1005, 0xCD);

        cpu.registers.program_counter = 0x8000;
        cpu.registers.x_register = 0x05;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.registers.accumulator, 0xCD);
        assert_eq!(cpu.cycles, 4, "Absolute,X (No Cross) should take 4 cycles");
    }

    #[test]
    fn test_lda_absolute_x_page_crossing() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // LDA $10FF, X
        bus.write(0x8000, 0xBD);
        bus.write(0x8001, 0xFF);
        bus.write(0x8002, 0x10);
        bus.write(0x1100, 0x77);

        cpu.registers.program_counter = 0x8000;
        cpu.registers.x_register = 0x01;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.registers.accumulator, 0x77);
        assert_eq!(cpu.cycles, 5, "Absolute,X (Cross) should take 5 cycles");
    }

    // Base: $2080, Y: $80 -> Target: $2100 ($2080 + $80 = $2100)
    #[test]
    fn test_lda_absolute_y_page_crossing() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        bus.write(0x8000, 0xB9); // LDA Absolute,Y
        bus.write(0x8001, 0x80);
        bus.write(0x8002, 0x20);

        bus.write(0x2100, 0x88);

        cpu.registers.program_counter = 0x8000;
        cpu.registers.y_register = 0x80;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.registers.accumulator, 0x88);
        assert_eq!(cpu.cycles, 5, "Absolute,Y (Cross) should take 5 cycles");
    }

    #[test]
    fn test_lda_indirect_x() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // LDA ($20, X)
        bus.write(0x8000, 0xA1);
        bus.write(0x8001, 0x20);
        bus.write(0x0024, 0x34);
        bus.write(0x0025, 0x12);

        // Put the value 0x99 at $1234
        bus.write(0x1234, 0x99);

        cpu.registers.program_counter = 0x8000;
        cpu.registers.x_register = 0x04;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.registers.accumulator, 0x99);
        assert_eq!(cpu.cycles, 6, "Indirect X should take 6 cycles");
    }

    #[test]
    fn test_lda_indirect_y_no_crossing() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // DA ($20), Y
        bus.write(0x8000, 0xB1);
        bus.write(0x8001, 0x20); // Base ZP address

        // At $0020, store the address $1000
        bus.write(0x0020, 0x00); // Low Byte
        bus.write(0x0021, 0x10); // High Byte

        // Put value 0x88 at $1005 ($1000 + $05)
        bus.write(0x1005, 0x88);

        cpu.registers.program_counter = 0x8000;
        cpu.registers.y_register = 0x05;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.registers.accumulator, 0x88);
        assert_eq!(cpu.cycles, 5, "Indirect Y (No Cross) should take 5 cycles");
    }

    #[test]
    fn test_lda_indirect_y_page_crossing() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // LDA ($20), Y
        bus.write(0x8000, 0xB1);
        bus.write(0x8001, 0x20);
        bus.write(0x0020, 0xFF); // Low Byte
        bus.write(0x0021, 0x10); // High Byte

        // Put value 0x77 at $1100 ($10FF + $01)
        bus.write(0x1100, 0x77);

        cpu.registers.program_counter = 0x8000;
        cpu.registers.y_register = 0x01;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(cpu.registers.accumulator, 0x77);
        assert_eq!(cpu.cycles, 6, "Indirect Y (Cross) should take 6 cycles");
    }

    #[test]
    fn test_lda_indirect_x_zp_wrap_torture() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        // 1. Set X to a value that forces the pointer to $FF
        // Base ($F0) + X ($0F) = $FF
        cpu.registers.x_register = 0x0F;

        // PROGRAM: LDA ($F0, X)
        // Code lives at $8000
        bus.write(0x8000, 0xA1);
        bus.write(0x8001, 0xF0);

        // 2. POINTER SETUP (The Wrap)
        // We want to point to address $4000 (Safe location)
        // Low Byte ($00) goes to $00FF
        bus.write(0x00FF, 0x00);
        // High Byte ($40) goes to $0000 (Wrap!)
        bus.write(0x0000, 0x40);

        // If CPU does not wrap, it reads High Byte from $0100.
        bus.write(0x0100, 0x99);

        // 4. DATA
        // Put the value at $4000
        bus.write(0x4000, 0x42);

        cpu.registers.program_counter = 0x8000;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(
            cpu.registers.accumulator, 0x42,
            "Failed to wrap Zero Page pointer!"
        );
    }

    #[test]
    fn test_sta_absolute() {
        let mut bus = Memory::new();
        let mut cpu = CPU::new();

        bus.write(0x8000, 0x8D);
        bus.write(0x8001, 0x00);
        bus.write(0x8002, 0x10);

        cpu.registers.accumulator = 0x55;
        cpu.registers.program_counter = 0x8000;
        cpu.cycles = 0;

        cpu.step(&mut bus);

        assert_eq!(bus.read(0x1000), 0x55);
        assert_eq!(cpu.cycles, 4, "STA should take 4 cycles");
    }
}
