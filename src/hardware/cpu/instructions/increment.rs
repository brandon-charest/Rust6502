use crate::hardware::cpu::CPU;
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
