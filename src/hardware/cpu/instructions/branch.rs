use crate::hardware::cpu::{Bus, CPU, Status};

pub fn bcc(cpu: &mut CPU, bus: &mut dyn Bus) {
    let condition = !cpu.registers.status.contains(Status::CARRY);
    branch(cpu, bus, condition)
}

pub fn bcs(cpu: &mut CPU, bus: &mut dyn Bus) {
    let condition = cpu.registers.status.contains(Status::CARRY);
    branch(cpu, bus, condition)
}

pub fn beq(cpu: &mut CPU, bus: &mut dyn Bus) {
    let condition = cpu.registers.status.contains(Status::ZERO);
    branch(cpu, bus, condition)
}

pub fn bmi(cpu: &mut CPU, bus: &mut dyn Bus) {
    let condition = cpu.registers.status.contains(Status::NEGATIVE);
    branch(cpu, bus, condition)
}

pub fn bne(cpu: &mut CPU, bus: &mut dyn Bus) {
    let condition = !cpu.registers.status.contains(Status::ZERO);
    branch(cpu, bus, condition)
}
pub fn bpl(cpu: &mut CPU, bus: &mut dyn Bus) {
    let condition = !cpu.registers.status.contains(Status::NEGATIVE);
    branch(cpu, bus, condition)
}
pub fn bvc(cpu: &mut CPU, bus: &mut dyn Bus) {
    let condition = !cpu.registers.status.contains(Status::OVERFLOW);
    branch(cpu, bus, condition)
}
pub fn bvs(cpu: &mut CPU, bus: &mut dyn Bus) {
    let condition = cpu.registers.status.contains(Status::OVERFLOW);
    branch(cpu, bus, condition)
}

fn branch(cpu: &mut CPU, bus: &mut dyn Bus, condition: bool) {
    // Read the signed offset byte
    let offset = cpu.fetch_byte(bus) as i8;

    if condition {
        // Calculate target address using signed offset
        // PC is already at the next instruction after the offset byte
        let jump_addr = cpu
            .registers
            .program_counter
            .wrapping_add_signed(offset as i16);

        // Cycles: Branch Taken (+1)
        let _ = cpu.read(bus, cpu.registers.program_counter);

        // Cycles: Page Crossing (+1)
        if (cpu.registers.program_counter & 0xFF00) != (jump_addr & 0xFF00) {
            let _ = cpu.read(bus, jump_addr.wrapping_sub(0x0100)); // Burn cycle
        }

        // Update PC
        cpu.registers.program_counter = jump_addr;
    }
    // If false: We already incremented PC via fetch_byte
}
