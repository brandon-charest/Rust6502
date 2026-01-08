use crate::hardware::cpu::CPU;
use crate::hardware::status::Status;

pub fn clc(cpu: &mut CPU) {
    cpu.registers.status.remove(Status::CARRY);
}
pub fn sec(cpu: &mut CPU) {
    cpu.registers.status.insert(Status::CARRY);
}
pub fn cli(cpu: &mut CPU) {
    cpu.registers.status.remove(Status::DISABLE_INTERRUPTS);
}
pub fn sei(cpu: &mut CPU) {
    cpu.registers.status.insert(Status::DISABLE_INTERRUPTS);
}
pub fn cld(cpu: &mut CPU) {
    cpu.registers.status.remove(Status::DECIMAL_MODE);
}
pub fn sed(cpu: &mut CPU) {
    cpu.registers.status.insert(Status::DECIMAL_MODE);
}
pub fn clv(cpu: &mut CPU) {
    cpu.registers.status.remove(Status::OVERFLOW);
}
