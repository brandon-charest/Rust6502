use crate::hardware::cpu::{Bus, CPU};

pub fn noop(cpu: &mut CPU, bus: &mut dyn Bus) {
    let _ = cpu.read(bus, cpu.registers.program_counter);
}
