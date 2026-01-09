pub mod addressing;
pub mod disassembler;
pub mod instructions;
pub mod memory_access;
use crate::hardware::{
    bus::Bus,
    cpu::instructions::{
        arithmetic, branch, compare, control, flags, increment, load, logic, noop, shift, stack,
        transfer, unofficial,
    },
    opcodes::{AddressingMode, Opcode, OpcodeSyntax},
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

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            cycles: 0,
            halted: false,
        }
    }

    pub fn step(&mut self, bus: &mut dyn Bus) {
        if self.halted {
            // simulate time passing even though the CPU is dead.
            self.cycles += 1;
            return;
        }

        let pc_before = self.registers.program_counter;
        let raw_data = self.fetch_byte(bus);
        let opcode = Opcode::from_u8(raw_data)
            .unwrap_or_else(|| panic!("Unknown opcode ${:02X} at PC=${:04X}", raw_data, pc_before));

        match opcode.syntax {
            // ==== ARITHMETIC ====
            OpcodeSyntax::ADC => arithmetic::adc(self, bus, &opcode.mode),
            OpcodeSyntax::SBC => arithmetic::sbc(self, bus, &opcode.mode),
            // ==== INCREMENT/DECREMENT ====
            OpcodeSyntax::DEX => increment::dex(self),
            OpcodeSyntax::DEY => increment::dey(self),
            OpcodeSyntax::INX => increment::inx(self),
            OpcodeSyntax::INY => increment::iny(self),
            OpcodeSyntax::INC => increment::inc(self, bus, &opcode.mode),
            OpcodeSyntax::DEC => increment::dec(self, bus, &opcode.mode),
            OpcodeSyntax::DCP => unofficial::dcp(self, bus, &opcode.mode),
            // ==== LOGIC ====
            OpcodeSyntax::AND => logic::and(self, bus, &opcode.mode),
            OpcodeSyntax::ORA => logic::ora(self, bus, &opcode.mode),
            OpcodeSyntax::EOR => logic::eor(self, bus, &opcode.mode),
            OpcodeSyntax::BIT => logic::bit(self, bus, &opcode.mode),
            // ==== FLAGS ====
            OpcodeSyntax::CLC => flags::clc(self),
            OpcodeSyntax::CLD => flags::cld(self),
            OpcodeSyntax::CLI => flags::cli(self),
            OpcodeSyntax::CLV => flags::clv(self),
            OpcodeSyntax::SEC => flags::sec(self),
            OpcodeSyntax::SED => flags::sed(self),
            OpcodeSyntax::SEI => flags::sei(self),
            // ==== BRANCH ====
            OpcodeSyntax::BCC => branch::bcc(self, bus),
            OpcodeSyntax::BCS => branch::bcs(self, bus),
            OpcodeSyntax::BEQ => branch::beq(self, bus),
            OpcodeSyntax::BMI => branch::bmi(self, bus),
            OpcodeSyntax::BNE => branch::bne(self, bus),
            OpcodeSyntax::BPL => branch::bpl(self, bus),
            OpcodeSyntax::BVS => branch::bvs(self, bus),
            OpcodeSyntax::BVC => branch::bvc(self, bus),
            // ==== COMPARE ====
            OpcodeSyntax::CMP => compare::cmp(self, bus, &opcode.mode),
            OpcodeSyntax::CPX => compare::cpx(self, bus, &opcode.mode),
            OpcodeSyntax::CPY => compare::cpy(self, bus, &opcode.mode),
            // ===== LOAD/STORE =====
            OpcodeSyntax::LDA => load::lda(self, bus, &opcode.mode),
            OpcodeSyntax::LDX => load::ldx(self, bus, &opcode.mode),
            OpcodeSyntax::LDY => load::ldy(self, bus, &opcode.mode),
            OpcodeSyntax::LAX => unofficial::lax(self, bus, &opcode.mode),
            OpcodeSyntax::STA => load::sta(self, bus, &opcode.mode),
            OpcodeSyntax::STX => load::stx(self, bus, &opcode.mode),
            OpcodeSyntax::STY => load::sty(self, bus, &opcode.mode),
            OpcodeSyntax::SAX => unofficial::sax(self, bus, &opcode.mode),
            OpcodeSyntax::TAX => transfer::tax(self),
            OpcodeSyntax::TAY => transfer::tay(self),
            OpcodeSyntax::TSX => transfer::tsx(self),
            OpcodeSyntax::TXA => transfer::txa(self),
            OpcodeSyntax::TXS => transfer::txs(self),
            OpcodeSyntax::TYA => transfer::tya(self),
            // ==== STACK ====
            OpcodeSyntax::PHA => stack::pha(self, bus),
            OpcodeSyntax::PHP => stack::php(self, bus),
            OpcodeSyntax::PLA => stack::pla(self, bus),
            OpcodeSyntax::PLP => stack::plp(self, bus),
            // ==== CONTROL ====
            OpcodeSyntax::BRK => control::brk(self, bus),
            OpcodeSyntax::JMP => control::jmp(self, bus, &opcode.mode),
            OpcodeSyntax::JSR => control::jsr(self, bus, &opcode.mode),
            OpcodeSyntax::RTS => control::rts(self, bus),
            OpcodeSyntax::RTI => control::rti(self, bus),
            // ==== SHIFT ====
            OpcodeSyntax::ASL => shift::asl(self, bus, &opcode.mode),
            OpcodeSyntax::LSR => shift::lsr(self, bus, &opcode.mode),
            OpcodeSyntax::ROL => shift::rol(self, bus, &opcode.mode),
            OpcodeSyntax::ROR => shift::ror(self, bus, &opcode.mode),
            OpcodeSyntax::RLA => unofficial::rla(self, bus, &opcode.mode),
            // ==== NOOP ====
            OpcodeSyntax::NOP => noop::noop(self, bus),
            OpcodeSyntax::KIL => unofficial::kil(self, bus, &opcode.mode),

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
        self.registers.status = Status::default();
        self.halted = false;
        self.registers.program_counter = self.read_word(bus, RESET_VECTOR);
    }

    pub(crate) fn update_nz_flags(&mut self, value: u8) {
        self.registers.status.set(Status::ZERO, value == 0);
        self.registers.status.set(Status::NEGATIVE, (value & 0x80) != 0);
    }

    pub fn trace(&mut self, bus: &mut dyn Bus) -> String {
        let pc = self.registers.program_counter;
        let opcode_byte = bus.read(pc); // Don't use self.read to avoid cycle count
        let opcode = match Opcode::from_u8(opcode_byte) {
            Some(op) => op,
            None => return format!("{:04X}  {:02X}        UNKNOWN", pc, opcode_byte),
        };

        let mut raw_bytes = vec![opcode_byte];
        for i in 1..opcode.bytes {
            raw_bytes.push(bus.read(pc + i as u16));
        }
        let hex_dump =
            raw_bytes.iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join(" ");

        let asm = disassembler::disassemble_instruction(&opcode, &raw_bytes, pc);
        if asm.contains("BNE") || asm.contains("BEQ") {
            println!("{:04X}: {} <--- BRANCH TARGET", pc, asm);
        } else {
            println!("{:04X}: {}", pc, asm);
        }
        format!(
            "{:04X}  {: <9} {: <15} A:{:02X} X:{:02X} Y:{:02X} P:{:02X} SP:{:02X} CYC:{}",
            pc,
            hex_dump,
            asm,
            self.registers.accumulator,
            self.registers.x_register,
            self.registers.y_register,
            self.registers.status.bits(),
            self.registers.stack_pointer,
            self.cycles
        )
    }

    pub fn debug_info(&self, bus: &mut dyn Bus) {
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
