use crate::hardware::cpu::CPU;

pub fn tax(cpu: &mut CPU) {
    cpu.registers.x_register = cpu.registers.accumulator;
    cpu.update_nz_flags(cpu.registers.x_register);
}

pub fn tay(cpu: &mut CPU) {
    cpu.registers.y_register = cpu.registers.accumulator;
    cpu.update_nz_flags(cpu.registers.y_register);
}

pub fn tsx(cpu: &mut CPU) {
    cpu.registers.x_register = cpu.registers.stack_pointer;
    cpu.update_nz_flags(cpu.registers.x_register);
}

pub fn txa(cpu: &mut CPU) {
    cpu.registers.accumulator = cpu.registers.x_register;
    cpu.update_nz_flags(cpu.registers.accumulator);
}

pub fn txs(cpu: &mut CPU) {
    cpu.registers.stack_pointer = cpu.registers.x_register;
    // does not affect any of the flags.
}

pub fn tya(cpu: &mut CPU) {
    cpu.registers.accumulator = cpu.registers.y_register;
    cpu.update_nz_flags(cpu.registers.accumulator);
}
