use crate::hardware::{
    cpu::{AccessMode, Bus, CPU, Status},
    opcodes::AddressingMode,
};

pub fn brk(cpu: &mut CPU, bus: &mut dyn Bus) {
    let _ = cpu.fetch_byte(bus);
    push_u16(cpu, bus, cpu.registers.program_counter);
    let mut flags = cpu.registers.status.bits();
    flags |= 0x30;
    cpu.push(bus, flags);
    cpu.registers.status.insert(Status::DISABLE_INTERRUPTS);

    cpu.registers.program_counter = cpu.read_u16(bus, 0xFFFE);
}

pub fn jmp(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    cpu.registers.program_counter = addr;
}

pub fn jsr(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let target_addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let return_addr = cpu.registers.program_counter.wrapping_sub(1);
    push_u16(cpu, bus, return_addr);
    cpu.registers.program_counter = target_addr;
}

pub fn rti(cpu: &mut CPU, bus: &mut dyn Bus) {
    // Even though RTI implies no operand, the CPU reads the PC anyway.
    let _ = cpu.read(bus, cpu.registers.program_counter);
    // The CPU reads the current stack address while the SP increments.
    let _ = cpu.read(bus, 0x0100 + cpu.registers.stack_pointer as u16);
    let popped_flags = cpu.pop(bus);
    let mut new_status = Status::from_bits_truncate(popped_flags);
    new_status.remove(Status::BRK);
    new_status.insert(Status::UNUSED);
    cpu.registers.status = new_status;

    cpu.registers.program_counter = pop_u16(cpu, bus);
}

pub fn rts(cpu: &mut CPU, bus: &mut dyn Bus) {
    let return_addr = pop_u16(cpu, bus);
    cpu.registers.program_counter = return_addr.wrapping_add(1);
}

fn push_u16(cpu: &mut CPU, bus: &mut dyn Bus, data: u16) {
    let hi = (data >> 8) as u8;
    let lo = (data & 0xff) as u8;
    cpu.push(bus, hi);
    cpu.push(bus, lo);
}

fn pop_u16(cpu: &mut CPU, bus: &mut dyn Bus) -> u16 {
    let lo = cpu.pop(bus) as u16;
    let hi = cpu.pop(bus) as u16;
    (hi << 8) | lo
}
