use crate::hardware::cpu::{AccessMode, AddressingMode, Bus, CPU, Status};

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

fn compare(cpu: &mut CPU, register: u8, memory: u8) {
    // Calculate the result (Register - Memory)
    let (result, _) = register.overflowing_sub(memory);

    // Update Zero and Negative flags based on the Result
    cpu.update_nz_flags(result);

    // Update Carry Flag: Set if Register >= Memory
    if register >= memory {
        cpu.registers.status.insert(Status::CARRY);
    } else {
        cpu.registers.status.remove(Status::CARRY);
    }
}
