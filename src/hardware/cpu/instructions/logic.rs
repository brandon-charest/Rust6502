use crate::hardware::{
    cpu::{AccessMode, AddressingMode, Bus, CPU},
    status::Status,
};

pub fn and(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);

    // Perform Bitwise AND
    cpu.registers.accumulator &= value;

    // Update Flags
    cpu.update_nz_flags(cpu.registers.accumulator);
}

pub fn ora(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);

    // Perform Bitwise OR
    cpu.registers.accumulator |= value;

    // Update Flags
    cpu.update_nz_flags(cpu.registers.accumulator);
}

pub fn eor(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);

    // Perform Bitwise XOR
    cpu.registers.accumulator ^= value;

    // Update Flags
    cpu.update_nz_flags(cpu.registers.accumulator);
}

pub fn bit(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    let memory_flags = Status::from_bits_truncate(value);

    // Zero Flag: (A & M) == 0
    let result = cpu.registers.accumulator & value;
    cpu.registers.status.set(Status::ZERO, result == 0);

    // Negative Flag: Copy bit 7 of memory value
    cpu.registers.status.set(Status::NEGATIVE, memory_flags.contains(Status::NEGATIVE));

    // Overflow Flag: Copy bit 6 of memory value
    cpu.registers.status.set(Status::OVERFLOW, memory_flags.contains(Status::OVERFLOW));
}
