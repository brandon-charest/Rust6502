use crate::hardware::cpu::{AddressingMode, Bus, CPU, Status, addressing::AccessMode};

pub fn cmp(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    compare(cpu, cpu.registers.accumulator, value);
}

pub fn cpx(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    compare(cpu, cpu.registers.x_register, value);
}

pub fn cpy(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    compare(cpu, cpu.registers.y_register, value);
}

pub(crate) fn compare(cpu: &mut CPU, register: u8, memory: u8) {
    let (result, _) = register.overflowing_sub(memory);
    cpu.update_nz_flags(result);
    if register >= memory {
        cpu.registers.status.insert(Status::CARRY);
    } else {
        cpu.registers.status.remove(Status::CARRY);
    }
}
