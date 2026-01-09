use crate::hardware::{
    bus::Bus,
    cpu::{AddressingMode, CPU, addressing::AccessMode, instructions::compare},
    status::Status,
};

pub fn lax(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);

    cpu.registers.accumulator = value;
    cpu.registers.x_register = value;
    cpu.update_nz_flags(value);
}

pub fn sax(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
    let a = cpu.registers.accumulator;
    let x = cpu.registers.x_register;

    let value = a & x;
    cpu.write(bus, addr, value);
}

pub fn dcp(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
    let mut value = cpu.read(bus, addr);

    cpu.write(bus, addr, value);

    value = value.wrapping_sub(1);
    cpu.write(bus, addr, value);
    cpu.update_nz_flags(value);

    compare::compare(cpu, cpu.registers.accumulator, value);
}

pub fn rla(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
    let mut value = cpu.read(bus, addr);

    // dummy write
    cpu.write(bus, addr, value);

    let old_carry = if cpu.registers.status.contains(Status::CARRY) {
        1
    } else {
        0
    };

    if (value & 0x80) != 0 {
        cpu.registers.status.insert(crate::hardware::cpu::Status::CARRY);
    } else {
        cpu.registers.status.remove(crate::hardware::cpu::Status::CARRY);
    }

    value = (value << 1) | old_carry;
    cpu.write(bus, addr, value);

    cpu.registers.accumulator &= value;
    cpu.update_nz_flags(cpu.registers.accumulator);
}

pub fn kil(cpu: &mut CPU, _bus: &mut dyn Bus, _mode: &AddressingMode) {
    cpu.halted = true;
}
