use crate::hardware::{bus::Bus, cpu::CPU, status::Status};

pub fn pha(cpu: &mut CPU, bus: &mut dyn Bus) {
    cpu.push(bus, cpu.registers.accumulator);
}

pub fn php(cpu: &mut CPU, bus: &mut dyn Bus) {
    let mut flags = cpu.registers.status;
    flags.set(Status::BRK, true);
    flags.set(Status::UNUSED, true);
    cpu.push(bus, flags.bits());
}

pub fn pla(cpu: &mut CPU, bus: &mut dyn Bus) {
    let popped_byte = cpu.pop(bus);
    cpu.registers.accumulator = popped_byte;
    cpu.update_nz_flags(popped_byte);
}

pub fn plp(cpu: &mut CPU, bus: &mut dyn Bus) {
    let popped_byte = cpu.pop(bus);
    let mut new_status = Status::from_bits_truncate(popped_byte);
    new_status.remove(Status::BRK);
    new_status.insert(Status::UNUSED);
    cpu.registers.status = new_status;
}
