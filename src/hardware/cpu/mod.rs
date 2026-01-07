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
            // ===== LOAD/STORE =====
            OpcodeSyntax::LDA => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);
                self.registers.accumulator = value;
                self.update_nz_flags(value);
            }
            OpcodeSyntax::LDX => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);
                self.registers.x_register = value;
                self.update_nz_flags(value);
            }
            OpcodeSyntax::LDY => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);
                self.registers.y_register = value;
                self.update_nz_flags(value);
            }
            OpcodeSyntax::STA => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Write);
                self.write(bus, addr, self.registers.accumulator);
            }
            OpcodeSyntax::STX => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Write);
                self.write(bus, addr, self.registers.x_register);
            }
            OpcodeSyntax::STY => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Write);
                self.write(bus, addr, self.registers.y_register);
            }
            // TAX (Transfer Accumulator -> X)
            OpcodeSyntax::TAX => {
                self.registers.x_register = self.registers.accumulator;
                self.update_nz_flags(self.registers.x_register);
            }
            // TAY (Transfer Accumulator -> Y)
            OpcodeSyntax::TAY => {
                self.registers.y_register = self.registers.accumulator;
                self.update_nz_flags(self.registers.y_register);
            }
            // TXA (Transfer X -> Accumulator)
            OpcodeSyntax::TXA => {
                self.registers.accumulator = self.registers.x_register;
                self.update_nz_flags(self.registers.accumulator);
            }
            // TYA (Transfer Y -> Accumulator)
            OpcodeSyntax::TYA => {
                self.registers.accumulator = self.registers.y_register;
                self.update_nz_flags(self.registers.accumulator);
            }
            // TXS (Transfer X -> Stack Pointer)
            OpcodeSyntax::TXS => {
                self.registers.stack_pointer = self.registers.x_register;
                // DO NOT UPDATE FLAGS
            }
            // TSX (Transfer Stack Pointer -> X)
            OpcodeSyntax::TSX => {
                self.registers.x_register = self.registers.stack_pointer;
                self.update_nz_flags(self.registers.x_register);
            }
            OpcodeSyntax::JMP => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                self.registers.program_counter = addr;
            }
            OpcodeSyntax::NOP => {
                let _ = self.read(bus, self.registers.program_counter);
            }
            OpcodeSyntax::BRK => {
                self.registers.status.insert(Status::BRK);
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
    pub fn trace(&self) -> String {
        format!(
            "PC:{:04X} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X}",
            self.registers.program_counter,
            self.registers.accumulator,
            self.registers.x_register,
            self.registers.y_register,
            self.registers.status.bits(), // Assuming 'bits()' returns the u8 value of flags
            self.registers.stack_pointer
        )
    }
    fn get_operand_address(
        &mut self,
        mode: &AddressingMode,
        bus: &mut dyn Bus,
        access_mode: AccessMode,
    ) -> u16 {
        match mode {
            AddressingMode::Absolute => {
                let addr = self.fetch_u16(bus);
                addr
            }
            AddressingMode::AbsoluteX => {
                let base = self.fetch_u16(bus);
                let addr = base.wrapping_add(self.registers.x_register as u16);
                let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
                if access_mode == AccessMode::Write || page_crossed {
                    // Crossing Page, Burn a cycle
                    let _ = self.read(bus, addr.wrapping_sub(0x0100));
                }

                addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.fetch_u16(bus);
                let addr = base.wrapping_add(self.registers.y_register as u16);
                let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
                if access_mode == AccessMode::Write || page_crossed {
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
            AddressingMode::Indirect => {
                let ptr_addr = self.fetch_u16(bus);

                // Handle 6502 page boundary bug
                let lo = self.read(bus, ptr_addr) as u16;
                let hi_addr = if (ptr_addr & 0x00FF) == 0x00FF {
                    ptr_addr & 0xFF00 // Wrap back to e.g. $3000
                } else {
                    ptr_addr.wrapping_add(1)
                };

                let hi = self.read(bus, hi_addr) as u16;

                (hi << 8) | lo
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
                let page_crossed = (addr & 0xFF00) != (base_addr & 0xFF00);
                if access_mode == AccessMode::Write || page_crossed {
                    // Burn cycle
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
        self.registers.status.set(Status::NEGATIVE, (value & 0x80) != 0);
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

        #[rustfmt::skip]
        let flags = [
            if status.contains(Status::NEGATIVE) { 'N' } else { 'n' },
            if status.contains(Status::OVERFLOW) { 'V' } else { 'v' },
            '-',
            if status.contains(Status::BRK) { 'B' } else { 'b' },
            if status.contains(Status::DECIMAL_MODE) { 'D' } else { 'd' },
            if status.contains(Status::DISABLE_INTERRUPTS) { 'I' } else { 'i' },
            if status.contains(Status::ZERO) { 'Z' } else { 'z' },
            if status.contains(Status::CARRY) { 'C' } else { 'c' },
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
mod tests;
