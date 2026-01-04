// A 6502 has a 16-bit address space: 2^16 = 65,536 bytes (0x10000).
pub const MEMORY_ADDRESS_LO: u16 = 0x0000;
pub const MEMORY_ADDRESS_HI: u16 = 0xFFFF;
const MEMORY_SIZE: usize = (MEMORY_ADDRESS_HI - MEMORY_ADDRESS_LO) as usize + 1usize;

pub struct Memory {
    bytes: [u8; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    #[must_use]
    pub const fn new() -> Memory {
        Memory {
            bytes: [0; MEMORY_SIZE],
        }
    }
}

pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

impl Bus for Memory {
    fn read(&self, address: u16) -> u8 {
        self.bytes[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.bytes[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_new() {
        let mem = Memory::new();
        assert_eq!(mem.bytes.len(), MEMORY_SIZE);
    }

    #[test]
    fn test_memory_default() {
        let mem = Memory::default();
        assert_eq!(mem.bytes.len(), MEMORY_SIZE);
    }

    #[test]
    fn test_memory_initialization() {
        let mem = Memory::new();
        // Ensure all memory is initialized to zero
        for i in 0..MEMORY_SIZE {
            assert_eq!(mem.bytes[i], 0, "Memory at index {} was not zero", i);
        }
    }

    #[test]
    fn test_read_write_low_boundary() {
        let mut mem = Memory::new();
        let addr = 0x0000;
        let val = 0x42;

        mem.write(addr, val);
        assert_eq!(mem.read(addr), val, "Failed to read/write at 0x0000");
    }

    #[test]
    fn test_read_write_high_boundary() {
        let mut mem = Memory::new();
        let addr = 0xFFFF;
        let val = 0xFE;

        mem.write(addr, val);
        assert_eq!(mem.read(addr), val, "Failed to read/write at 0xFFFF");
    }

    #[test]
    fn test_persistence() {
        let mut mem = Memory::new();
        mem.write(0x1234, 0xAA);
        mem.write(0x1235, 0xBB);

        assert_eq!(mem.read(0x1234), 0xAA);
        assert_eq!(mem.read(0x1235), 0xBB);
    }

    #[test]
    fn test_default_trait() {
        let mem = Memory::default();
        assert_eq!(mem.read(0x0000), 0);
    }
}
