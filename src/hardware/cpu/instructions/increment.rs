use crate::hardware::{
    bus::Bus,
    cpu::{AccessMode, AddressingMode, CPU},
};
pub fn dex(cpu: &mut CPU) {
    cpu.registers.x_register = cpu.registers.x_register.wrapping_sub(1);
    cpu.update_nz_flags(cpu.registers.x_register);
}

pub fn dey(cpu: &mut CPU) {
    cpu.registers.y_register = cpu.registers.y_register.wrapping_sub(1);
    cpu.update_nz_flags(cpu.registers.y_register);
}

pub fn inx(cpu: &mut CPU) {
    cpu.registers.x_register = cpu.registers.x_register.wrapping_add(1);
    cpu.update_nz_flags(cpu.registers.x_register);
}

pub fn iny(cpu: &mut CPU) {
    cpu.registers.y_register = cpu.registers.y_register.wrapping_add(1);
    cpu.update_nz_flags(cpu.registers.y_register);
}

pub fn inc(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
    let mut value = cpu.read(bus, addr);

    cpu.write(bus, addr, value);
    value = value.wrapping_add(1);
    cpu.write(bus, addr, value);

    cpu.update_nz_flags(value);
}

pub fn dec(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
    let mut value = cpu.read(bus, addr);

    value = value.wrapping_sub(1);
    cpu.write(bus, addr, value);

    cpu.update_nz_flags(value);
}
