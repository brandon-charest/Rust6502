pub use super::ppu::PPU;

// Memory Map Ranges
const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x3FFF;
const ROM_START: u16 = 0x8000;
const ROM_END: u16 = 0xFFFF;

// Component Sizes & Masks
const RAM_SIZE: usize = 2048;
const RAM_MIRROR_MASK: u16 = 0x07FF; // 0x0000 - 0x07FF
const PPU_REGISTER_MASK: u16 = 0x0007; // 0x2000 - 0x2007

pub trait Bus {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}
pub struct NesBus {
    ram: [u8; RAM_SIZE],
    ppu: PPU,
    prg_rom: Vec<u8>,
}

impl NesBus {
    pub fn new() -> NesBus {
        NesBus {
            ram: [0; RAM_SIZE],
            ppu: PPU::new(),
            prg_rom: Vec::new(),
        }
    }

    fn read_prg_rom(&self, addr: u16) -> u8 {
        let mut addr = addr - 0x8000; // Map $8000 -> 0

        // MIRRORING LOGIC (NROM-128):
        // If the game is small (16KB) but the CPU tries to read from the
        // upper 16KB range ($C000+), we must mirror it back to the start.
        if self.prg_rom.len() == 0x4000 && addr >= 0x4000 {
            addr %= 0x4000;
        }

        // Safety check to prevent crashing
        if (addr as usize) < self.prg_rom.len() {
            self.prg_rom[addr as usize]
        } else {
            0 // "Open Bus" behavior (or just return 0 for now)
        }
    }
}

impl Bus for NesBus {
    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            RAM_START..=RAM_END => {
                let mirror_addr = addr & RAM_MIRROR_MASK;
                self.ram[mirror_addr as usize]
            }
            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                let mirror_addr = addr & PPU_REGISTER_MASK;
                self.ppu.cpu_read(PPU_REGISTERS_START + mirror_addr)
            }
            ROM_START..=ROM_END => self.read_prg_rom(addr),
            _ => 0,
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match addr {
            RAM_START..=RAM_END => {
                let mirror_addr = addr & RAM_MIRROR_MASK;
                self.ram[mirror_addr as usize] = value;
            }

            PPU_REGISTERS_START..=PPU_REGISTERS_END => {
                let mirror_addr = addr & PPU_REGISTER_MASK;
                self.ppu.cpu_write(PPU_REGISTERS_START + mirror_addr, value);
            }

            ROM_START..=ROM_END => {}

            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_new() {
        let mem = NesBus::new();
        assert_eq!(mem.ram.len(), RAM_SIZE);
    }

    #[test]
    fn test_memory_initialization() {
        let mem = NesBus::new();
        // Ensure all memory is initialized to zero
        for i in 0..RAM_SIZE {
            assert_eq!(mem.ram[i], 0, "Memory at index {} was not zero", i);
        }
    }

    #[test]
    fn test_read_write_low_boundary() {
        let mut mem = NesBus::new();
        let addr = RAM_START;
        let val = 0x42;

        mem.write(addr, val);
        assert_eq!(mem.read(addr), val, "Failed to read/write at 0x0000");
    }

    #[test]
    fn test_read_write_high_boundary() {
        let mut mem = NesBus::new();
        let addr = RAM_END;
        let val = 0xFE;

        mem.write(addr, val);
        assert_eq!(mem.read(addr), val, "Failed to read/write at 0x1FFF");
    }

    #[test]
    fn test_persistence() {
        let mut mem = NesBus::new();
        mem.write(0x1234, 0xAA);
        mem.write(0x1235, 0xBB);

        assert_eq!(mem.read(0x1234), 0xAA);
        assert_eq!(mem.read(0x1235), 0xBB);
    }
}
