use super::registers::Registers;

#[derive(Debug)]
pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_new() {
        let cpu = CPU::new();
        assert_eq!(cpu.registers.accumulator, 0);
        assert_eq!(cpu.registers.x_register, 0);
        assert_eq!(cpu.registers.y_register, 0);
        assert_eq!(cpu.registers.stack_pointer, 0);
        assert_eq!(cpu.registers.program_counter, 0);
        assert_eq!(cpu.registers.status_register, 0);
    }
}