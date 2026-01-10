use crate::hardware::{
    bus::Bus,
    cpu::{AddressingMode, CPU, addressing::AccessMode},
};

pub fn dex(cpu: &mut CPU, bus: &mut dyn Bus) {
    cpu.read(bus, cpu.registers.program_counter); // Dummy read for cycle accuracy
    cpu.registers.x_register = cpu.registers.x_register.wrapping_sub(1);
    cpu.update_nz_flags(cpu.registers.x_register);
}

pub fn dey(cpu: &mut CPU, bus: &mut dyn Bus) {
    cpu.read(bus, cpu.registers.program_counter); // Dummy read for cycle accuracy
    cpu.registers.y_register = cpu.registers.y_register.wrapping_sub(1);
    cpu.update_nz_flags(cpu.registers.y_register);
}

pub fn inx(cpu: &mut CPU, bus: &mut dyn Bus) {
    cpu.read(bus, cpu.registers.program_counter); // Dummy read for cycle accuracy
    cpu.registers.x_register = cpu.registers.x_register.wrapping_add(1);
    cpu.update_nz_flags(cpu.registers.x_register);
}

pub fn iny(cpu: &mut CPU, bus: &mut dyn Bus) {
    cpu.read(bus, cpu.registers.program_counter); // Dummy read for cycle accuracy
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

    cpu.write(bus, addr, value);
    value = value.wrapping_sub(1);
    cpu.write(bus, addr, value);

    cpu.update_nz_flags(value);
}
