#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Registers {
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub stack_pointer: u8,
    pub program_counter: u16,
    pub status_register: u8
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    #[must_use]
    pub fn new() -> Self {
        Self {
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            stack_pointer: 0,
            program_counter: 0,
            status_register: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registers_new() {
        let registers = Registers::default();
        assert_eq!(registers.accumulator, 0);
        assert_eq!(registers.x_register, 0);
        assert_eq!(registers.y_register, 0);
        assert_eq!(registers.stack_pointer, 0);
        assert_eq!(registers.program_counter, 0);
        assert_eq!(registers.status_register, 0);
    }

    #[test]
    fn test_registers_default() {
        let registers = Registers::new();
        assert_eq!(registers.accumulator, 0);
        assert_eq!(registers.x_register, 0);
        assert_eq!(registers.y_register, 0);
        assert_eq!(registers.stack_pointer, 0);
        assert_eq!(registers.program_counter, 0);
        assert_eq!(registers.status_register, 0);
    }
}