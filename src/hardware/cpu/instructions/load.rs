use crate::hardware::cpu::{AccessMode, Bus, CPU};
use crate::hardware::opcodes::AddressingMode;

pub fn lda(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    cpu.registers.accumulator = value;
    cpu.update_nz_flags(value);
}
pub fn ldx(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    cpu.registers.x_register = value;
    cpu.update_nz_flags(value);
}
pub fn ldy(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    cpu.registers.y_register = value;
    cpu.update_nz_flags(value);
}
pub fn sta(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
    cpu.write(bus, addr, cpu.registers.accumulator);
}
pub fn stx(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
    cpu.write(bus, addr, cpu.registers.x_register);
}
pub fn sty(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
    cpu.write(bus, addr, cpu.registers.y_register);
}
