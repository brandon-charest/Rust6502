use crate::hardware::{
    bus::Bus,
    cpu::{AddressingMode, CPU, addressing::AccessMode},
    status::Status,
};

pub fn asl(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    if *mode == AddressingMode::Accumulator {
        cpu.read(bus, cpu.registers.program_counter);
        let value = cpu.registers.accumulator;
        cpu.registers.accumulator = shift_left(cpu, value);
    } else {
        let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
        let value = cpu.read(bus, addr);
        cpu.write(bus, addr, value);
        let new_val = shift_left(cpu, value);
        cpu.write(bus, addr, new_val);
    }
}
pub fn lsr(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    if *mode == AddressingMode::Accumulator {
        cpu.read(bus, cpu.registers.program_counter);
        let value = cpu.registers.accumulator;
        cpu.registers.accumulator = shift_right(cpu, value);
    } else {
        let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
        let value = cpu.read(bus, addr);
        cpu.write(bus, addr, value);
        let new_val = shift_right(cpu, value);
        cpu.write(bus, addr, new_val);
    }
}
pub fn rol(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    if *mode == AddressingMode::Accumulator {
        cpu.read(bus, cpu.registers.program_counter);
        let value = cpu.registers.accumulator;
        cpu.registers.accumulator = rotate_left(cpu, value);
    } else {
        let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
        let value = cpu.read(bus, addr);
        cpu.write(bus, addr, value);
        let new_val = rotate_left(cpu, value);
        cpu.write(bus, addr, new_val);
    }
}
pub fn ror(cpu: &mut CPU, bus: &mut dyn Bus, mode: &AddressingMode) {
    if *mode == AddressingMode::Accumulator {
        let value = cpu.registers.accumulator;
        cpu.registers.accumulator = rotate_right(cpu, value);
    } else {
        let addr = cpu.get_operand_address(mode, bus, AccessMode::Write);
        let value = cpu.read(bus, addr);
        cpu.write(bus, addr, value);
        let new_val = rotate_right(cpu, value);
        cpu.write(bus, addr, new_val);
    }
}

fn shift_left(cpu: &mut CPU, data: u8) -> u8 {
    if data & (1 << 7) != 0 {
        cpu.registers.status.insert(Status::CARRY);
    } else {
        cpu.registers.status.remove(Status::CARRY);
    }
    let new_value = data << 1;
    cpu.update_nz_flags(new_value);
    new_value
}

fn shift_right(cpu: &mut CPU, data: u8) -> u8 {
    if data & 1 != 0 {
        cpu.registers.status.insert(Status::CARRY);
    } else {
        cpu.registers.status.remove(Status::CARRY);
    }
    let new_value = data >> 1;
    cpu.update_nz_flags(new_value);
    new_value
}

fn rotate_left(cpu: &mut CPU, data: u8) -> u8 {
    let old_carry = if cpu.registers.status.contains(Status::CARRY) {
        1
    } else {
        0
    };

    if (data & 0x80) != 0 {
        cpu.registers.status.insert(Status::CARRY);
    } else {
        cpu.registers.status.remove(Status::CARRY);
    }

    let result = (data << 1) | old_carry;

    cpu.update_nz_flags(result);
    result
}

fn rotate_right(cpu: &mut CPU, data: u8) -> u8 {
    let old_carry = if cpu.registers.status.contains(Status::CARRY) {
        0x80
    } else {
        0
    };

    if (data & 1) != 0 {
        cpu.registers.status.insert(Status::CARRY);
    } else {
        cpu.registers.status.remove(Status::CARRY);
    }

    let result = (data >> 1) | old_carry;

    cpu.update_nz_flags(result);
    result
}
