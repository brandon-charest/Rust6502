use crate::hardware::bus::Bus;
use crate::hardware::cpu::{AddressingMode, CPU};

#[derive(PartialEq, Eq)]
pub(crate) enum AccessMode {
    Read,  // LDA, LDX, LDY, EOR, AND, ORA, ADC, SBC, CMP, BIT
    Write, // STA, STX, STY, INC, DEC, ASL, LSR, ROL, ROR
}

impl CPU {
    pub(crate) fn get_operand_address(
        &mut self,
        mode: &AddressingMode,
        bus: &mut dyn Bus,
        access_mode: AccessMode,
    ) -> u16 {
        match mode {
            AddressingMode::Absolute => {
                let addr = self.fetch_word(bus);
                addr
            }
            AddressingMode::AbsoluteX => {
                let base = self.fetch_word(bus);
                let addr = base.wrapping_add(self.registers.x_register as u16);
                let page_crossed = (base & 0xFF00) != (addr & 0xFF00);
                if access_mode == AccessMode::Write || page_crossed {
                    // Crossing Page, Burn a cycle
                    let _ = self.read(bus, addr.wrapping_sub(0x0100));
                }

                addr
            }
            AddressingMode::AbsoluteY => {
                let base = self.fetch_word(bus);
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
                let ptr_addr = self.fetch_word(bus);

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
            _ => panic!("Addressing mode {:?} not yet implemented", mode),
        }
    }
}
