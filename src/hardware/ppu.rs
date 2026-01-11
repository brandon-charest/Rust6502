// Register Addresses (CPU Memory Map)
pub const PPU_CTRL: u16 = 0x2000;
pub const PPU_MASK: u16 = 0x2001;
pub const PPU_STATUS: u16 = 0x2002;
pub const OAM_ADDR: u16 = 0x2003;
pub const OAM_DATA: u16 = 0x2004;
pub const PPU_SCROLL: u16 = 0x2005;
pub const PPU_ADDR: u16 = 0x2006;
pub const PPU_DATA: u16 = 0x2007;

// Status Register Flags (Bit Masks)
pub const STATUS_OVERFLOW: u8 = 0b00100000; // Bit 5 (32)
pub const STATUS_SPRITE_ZERO: u8 = 0b01000000; // Bit 6 (64)
pub const STATUS_VBLANK: u8 = 0b10000000; // Bit 7 (128)

#[derive(PartialEq, Debug)]
pub struct PPU {
    pub ctrl: u8,     // $2000
    pub mask: u8,     // $2001
    pub status: u8,   // $2002
    pub oam_addr: u8, // $2003
    pub oam_data: u8, // $2004
    pub scroll: u8,   // $2005
    pub addr: u8,     // $2006
    pub data: u8,     // $2007

    pub vram: [u8; 2048],
    pub palette_table: [u8; 32],
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            ctrl: 0,
            mask: 0,
            status: 0,
            oam_addr: 0,
            oam_data: 0,
            scroll: 0,
            addr: 0,
            data: 0,
            vram: [0; 2048],
            palette_table: [0; 32],
        }
    }

    pub fn cpu_read(&mut self, addr: u16) -> u8 {
        match addr {
            PPU_CTRL => 0, // Write-only
            PPU_MASK => 0, // Write-only
            PPU_STATUS => {
                let data = self.status;
                self.status &= 0x7F;
                data
            }
            OAM_DATA => self.oam_data, // Read OAM data
            PPU_DATA => self.data,     // Read VRAM data
            _ => 0,
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            PPU_CTRL => self.ctrl = data,
            PPU_MASK => self.mask = data,
            // 0x2002 is Read-only!
            OAM_ADDR => self.oam_addr = data,
            OAM_DATA => self.oam_data = data,
            PPU_SCROLL => self.scroll = data,
            PPU_ADDR => self.addr = data,
            PPU_DATA => {
                self.data = data;
                // TODO:
            }
            _ => {}
        }
    }
}

impl Default for PPU {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_ppu() -> PPU {
        PPU::new()
    }

    #[test]
    fn test_ppu_default() {
        let ppu = PPU::default();
        assert_eq!(ppu.ctrl, 0);
        assert_eq!(ppu.mask, 0);
        assert_eq!(ppu.status, 0);
        assert_eq!(ppu.oam_addr, 0);
        assert_eq!(ppu.oam_data, 0);
        assert_eq!(ppu.scroll, 0);
        assert_eq!(ppu.addr, 0);
        assert_eq!(ppu.data, 0);
        assert_eq!(ppu.vram, [0; 2048]);
        assert_eq!(ppu.palette_table, [0; 32]);
    }

    #[test]
    fn test_ppu_ctrl_write() {
        let mut ppu = new_ppu();
        ppu.cpu_write(PPU_CTRL, 0xAB);
        assert_eq!(ppu.ctrl, 0xAB);
    }

    #[test]
    fn test_ppu_mask_write() {
        let mut ppu = new_ppu();
        // Write to PPUMASK ($2001)
        ppu.cpu_write(PPU_MASK, 0x55);
        assert_eq!(ppu.mask, 0x55);
    }

    #[test]
    fn test_ppu_status_read_clears_vblank() {
        let mut ppu = new_ppu();
        ppu.status = 0b10000000;

        let status = ppu.cpu_read(PPU_STATUS);

        assert_eq!(status & 0b10000000, 0b10000000, "Should read VBlank flag");

        // Verify VBlank flag should now be CLEARED
        assert_eq!(
            ppu.status & 0b10000000,
            0,
            "VBlank flag should be cleared after read"
        );
    }

    #[test]
    fn test_ppu_oam_addr_write() {
        let mut ppu = new_ppu();
        ppu.cpu_write(OAM_ADDR, 0x10);
        assert_eq!(ppu.oam_addr, 0x10);
    }

    #[test]
    fn test_ppu_oam_data_write() {
        let mut ppu = new_ppu();
        ppu.cpu_write(OAM_DATA, 0x99);
        assert_eq!(ppu.oam_data, 0x99);
    }

    #[test]
    fn test_write_only_registers_return_zero_on_read() {
        let mut ppu = new_ppu();

        ppu.cpu_write(PPU_CTRL, 0xFF);

        // Should be Write Only
        let value = ppu.cpu_read(PPU_CTRL);

        assert_eq!(
            value, 0,
            "Reading a write-only register should return 0 (or open bus)"
        );
    }
}
