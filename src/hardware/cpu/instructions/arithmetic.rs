use crate::hardware::bus::Bus;
use crate::hardware::cpu::{AccessMode, CPU, Status};
use crate::hardware::opcodes::AddressingMode;

pub fn adc(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);
    add_with_carry(cpu, value);
}

pub fn sbc(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    let addr = cpu.get_operand_address(mode, bus, AccessMode::Read);
    let value = cpu.read(bus, addr);

    add_with_carry(cpu, value ^ 0xFF);
}

fn add_with_carry(cpu: &mut CPU, memory_val: u8) {
    let a = cpu.registers.accumulator as u16;
    let m = memory_val as u16;
    let c = if cpu.registers.status.contains(Status::CARRY) {
        1
    } else {
        0
    };

    let sum = a + m + c;

    if sum > 0xFF {
        cpu.registers.status.insert(Status::CARRY);
    } else {
        cpu.registers.status.remove(Status::CARRY);
    }

    let result = sum as u8;
    let overflow = (a ^ sum) & (m ^ sum) & 0x0080;

    if overflow != 0 {
        cpu.registers.status.insert(Status::OVERFLOW);
    } else {
        cpu.registers.status.remove(Status::OVERFLOW);
    }

    cpu.registers.accumulator = result;
    cpu.update_nz_flags(result);
}
