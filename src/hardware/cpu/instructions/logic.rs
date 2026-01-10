use crate::hardware::{
    cpu::{AddressingMode, Bus, CPU, addressing::AccessMode},
    status::Status,
};

pub fn and(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);

    cpu.registers.accumulator &= value;
    cpu.update_nz_flags(cpu.registers.accumulator);
}

pub fn ora(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);

    cpu.registers.accumulator |= value;
    cpu.update_nz_flags(cpu.registers.accumulator);
}

pub fn eor(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);

    cpu.registers.accumulator ^= value;
    cpu.update_nz_flags(cpu.registers.accumulator);
}

pub fn bit(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    let memory_flags = Status::from_bits_truncate(value);
    let result = cpu.registers.accumulator & value;

    cpu.registers.status.set(Status::ZERO, result == 0);
    cpu.registers
        .status
        .set(Status::NEGATIVE, memory_flags.contains(Status::NEGATIVE));
    cpu.registers
        .status
        .set(Status::OVERFLOW, memory_flags.contains(Status::OVERFLOW));
}
