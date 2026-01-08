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
        let pc_before = self.registers.program_counter;
        let raw_data = self.fetch_byte(bus);
        let opcode = Opcode::from_u8(raw_data)
            .unwrap_or_else(|| panic!("Unknown opcode ${:02X} at PC=${:04X}", raw_data, pc_before));

        match opcode.syntax {
            OpcodeSyntax::ADC => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);
                self.add_with_carry(value);
            }
            OpcodeSyntax::SBC => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);

                // SBC is just ADC with the value inverted (One's Complement)
                // The math A + !M + C handles this automatically.
                self.add_with_carry(value ^ 0xFF);
            }
            OpcodeSyntax::EOR => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);

                // Perform Bitwise XOR
                self.registers.accumulator ^= value;

                // Update Flags
                self.update_nz_flags(self.registers.accumulator);
            }
            OpcodeSyntax::AND => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);

                // Perform Bitwise AND
                self.registers.accumulator &= value;

                // Update Flags
                self.update_nz_flags(self.registers.accumulator);
            }
            OpcodeSyntax::ORA => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);

                // Perform Bitwise OR
                self.registers.accumulator |= value;

                // Update Flags
                self.update_nz_flags(self.registers.accumulator);
            }
            // Branch Not Equal
            OpcodeSyntax::BNE => {
                let condition = !self.registers.status.contains(Status::ZERO);
                self.branch(bus, condition);
            }
            // Branch Equal
            OpcodeSyntax::BEQ => {
                let condition = self.registers.status.contains(Status::ZERO);
                self.branch(bus, condition);
            }
            OpcodeSyntax::BCC => {
                let condition = !self.registers.status.contains(Status::CARRY);
                self.branch(bus, condition);
            }
            OpcodeSyntax::BCS => {
                let condition = self.registers.status.contains(Status::CARRY);
                self.branch(bus, condition);
            }
            OpcodeSyntax::BVC => {
                let condition = !self.registers.status.contains(Status::OVERFLOW);
                self.branch(bus, condition);
            }
            OpcodeSyntax::BVS => {
                let condition = self.registers.status.contains(Status::OVERFLOW);
                self.branch(bus, condition);
            }
            OpcodeSyntax::BPL => {
                let condition = !self.registers.status.contains(Status::NEGATIVE);
                self.branch(bus, condition);
            }
            OpcodeSyntax::BMI => {
                let condition = self.registers.status.contains(Status::NEGATIVE);
                self.branch(bus, condition);
            }
            OpcodeSyntax::DEX => {
                self.registers.x_register = self.registers.x_register.wrapping_sub(1);
                self.update_nz_flags(self.registers.x_register);
            }
            OpcodeSyntax::DEY => {
                self.registers.y_register = self.registers.y_register.wrapping_sub(1);
                self.update_nz_flags(self.registers.y_register);
            }
            // Clear Carry
            OpcodeSyntax::CLC => {
                self.registers.status.remove(Status::CARRY);
            }
            // Clear Decimal Mode
            OpcodeSyntax::CLD => {
                self.registers.status.remove(Status::DECIMAL_MODE);
            }
            // Clear Interrupt Disable
            OpcodeSyntax::CLI => {
                self.registers.status.remove(Status::DISABLE_INTERRUPTS);
            }
            // Clear Overflow
            OpcodeSyntax::CLV => {
                self.registers.status.remove(Status::OVERFLOW);
            }
            // Set Carry
            OpcodeSyntax::SEC => {
                self.registers.status.insert(Status::CARRY);
            }
            // Set Decimal Mode
            OpcodeSyntax::SED => {
                self.registers.status.insert(Status::DECIMAL_MODE);
            }
            // Set Interrupt Disable
            OpcodeSyntax::SEI => {
                self.registers.status.insert(Status::DISABLE_INTERRUPTS);
            }
            OpcodeSyntax::CMP => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);
                self.compare(self.registers.accumulator, value);
            }
            OpcodeSyntax::BIT => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);
                let memory_flags = Status::from_bits_truncate(value);

                // Zero Flag: (A & M) == 0
                let result = self.registers.accumulator & value;
                self.registers.status.set(Status::ZERO, result == 0);

                // Negative Flag: Copy bit 7 of memory value
                self.registers
                    .status
                    .set(Status::NEGATIVE, memory_flags.contains(Status::NEGATIVE));

                // Overflow Flag: Copy bit 6 of memory value
                self.registers
                    .status
                    .set(Status::OVERFLOW, memory_flags.contains(Status::OVERFLOW));
            }
            OpcodeSyntax::CPX => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);
                self.compare(self.registers.x_register, value);
            }
            OpcodeSyntax::CPY => {
                let addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let value = self.read(bus, addr);
                self.compare(self.registers.y_register, value);
            }
            OpcodeSyntax::INX => {
                self.registers.x_register = self.registers.x_register.wrapping_add(1);
                self.update_nz_flags(self.registers.x_register);
            }
            OpcodeSyntax::INY => {
                self.registers.y_register = self.registers.y_register.wrapping_add(1);
                self.update_nz_flags(self.registers.y_register);
            }
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
            // (PusH Accumulator)
            OpcodeSyntax::PHA => {
                self.push(bus, self.registers.accumulator);
            }
            // (PusH Processor status)
            OpcodeSyntax::PHP => {
                let mut flags = self.registers.status;
                flags.set(Status::BRK, true);
                flags.set(Status::UNUSED, true);
                self.push(bus, flags.bits());
            }
            OpcodeSyntax::PLA => {
                let popped_byte = self.pop(bus);
                self.registers.accumulator = popped_byte;
                self.update_nz_flags(popped_byte);
            }
            OpcodeSyntax::PLP => {
                let popped_byte = self.pop(bus);
                let mut new_status = Status::from_bits_truncate(popped_byte);
                new_status.remove(Status::BRK);
                new_status.insert(Status::UNUSED);
                self.registers.status = new_status;
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
            OpcodeSyntax::JSR => {
                let target_addr = self.get_operand_address(&opcode.mode, bus, AccessMode::Read);
                let return_addr = self.registers.program_counter.wrapping_sub(1);
                self.push_u16(bus, return_addr);
                self.registers.program_counter = target_addr;
            }
            OpcodeSyntax::RTS => {
                let return_addr = self.pop_u16(bus);
                self.registers.program_counter = return_addr.wrapping_add(1);
            }
            OpcodeSyntax::NOP => {
                let _ = self.read(bus, self.registers.program_counter);
            }
            OpcodeSyntax::BRK => {
                let _ = self.fetch_byte(bus);
                self.push_u16(bus, self.registers.program_counter);
                let mut flags = self.registers.status.bits();
                flags |= 0x30;
                self.push(bus, flags);
                self.registers.status.insert(Status::DISABLE_INTERRUPTS);

                self.registers.program_counter = self.read_u16(bus, 0xFFFE);
            }
            OpcodeSyntax::RTI => {
                // Even though RTI implies no operand, the CPU reads the PC anyway.
                let _ = self.read(bus, self.registers.program_counter);
                // The CPU reads the current stack address while the SP increments.
                let _ = self.read(bus, 0x0100 + self.registers.stack_pointer as u16);
                let popped_flags = self.pop(bus);
                let mut new_status = Status::from_bits_truncate(popped_flags);
                new_status.remove(Status::BRK);
                new_status.insert(Status::UNUSED);
                self.registers.status = new_status;

                self.registers.program_counter = self.pop_u16(bus);
            }
            _ => panic!(
                "Unimplemented opcode: ${:02X} ({:?} {:?}) at PC=${:04X}",
                raw_data, opcode.syntax, opcode.mode, pc_before
            ),
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
            _ => panic!("Addressing mode {:?} not yet implemented", mode),
        }
    }

    fn push(&mut self, bus: &mut dyn Bus, data: u8) {
        self.write(bus, 0x0100 + self.registers.stack_pointer as u16, data);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    fn push_u16(&mut self, bus: &mut dyn Bus, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.push(bus, hi);
        self.push(bus, lo);
    }

    fn pop_u16(&mut self, bus: &mut dyn Bus) -> u16 {
        let lo = self.pop(bus) as u16;
        let hi = self.pop(bus) as u16;
        (hi << 8) | lo
    }

    fn pop(&mut self, bus: &mut dyn Bus) -> u8 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        self.read(bus, 0x0100 + self.registers.stack_pointer as u16)
    }

    fn update_nz_flags(&mut self, value: u8) {
        self.registers.status.set(Status::ZERO, value == 0);
        self.registers.status.set(Status::NEGATIVE, (value & 0x80) != 0);
    }

    fn branch(&mut self, bus: &mut dyn Bus, condition: bool) {
        // Read the signed offset byte
        let offset = self.fetch_byte(bus) as i8;

        if condition {
            // Calculate target address using signed offset
            // PC is already at the next instruction after the offset byte
            let jump_addr = self.registers.program_counter.wrapping_add_signed(offset as i16);

            // Cycles: Branch Taken (+1)
            let _ = self.read(bus, self.registers.program_counter);

            // Cycles: Page Crossing (+1)
            if (self.registers.program_counter & 0xFF00) != (jump_addr & 0xFF00) {
                let _ = self.read(bus, jump_addr.wrapping_sub(0x0100)); // Burn cycle
            }

            // Update PC
            self.registers.program_counter = jump_addr;
        }
        // If false: We already incremented PC via fetch_byte
    }

    fn compare(&mut self, register: u8, memory: u8) {
        // Calculate the result (Register - Memory)
        let (result, _) = register.overflowing_sub(memory);

        // Update Zero and Negative flags based on the Result
        self.update_nz_flags(result);

        // Update Carry Flag: Set if Register >= Memory
        if register >= memory {
            self.registers.status.insert(Status::CARRY);
        } else {
            self.registers.status.remove(Status::CARRY);
        }
    }
    // Sum = A + M + C
    fn add_with_carry(&mut self, memory_val: u8) {
        let a = self.registers.accumulator as u16;
        let m = memory_val as u16;
        let c = if self.registers.status.contains(Status::CARRY) {
            1
        } else {
            0
        };

        let sum = a + m + c;

        if sum > 0xFF {
            self.registers.status.insert(Status::CARRY);
        } else {
            self.registers.status.remove(Status::CARRY);
        }

        let result = sum as u8;
        let overflow = (a ^ sum) & (m ^ sum) & 0x0080;

        if overflow != 0 {
            self.registers.status.insert(Status::OVERFLOW);
        } else {
            self.registers.status.remove(Status::OVERFLOW);
        }

        // 4. Save Result and Update N/Z
        self.registers.accumulator = result;
        self.update_nz_flags(result);
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
