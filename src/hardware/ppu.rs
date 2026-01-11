pub struct PPU {
    // Registers accessible by CPU via $2000-$2007
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
            0x2000 => 0, // Write-only
            0x2001 => 0, // Write-only
            0x2002 => self.status,
            0x2004 => self.oam_data, // Read OAM data
            0x2007 => self.data,     // Read VRAM data
            _ => 0,
        }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) {
        match addr {
            0x2000 => self.ctrl = data,
            0x2001 => self.mask = data,
            // 0x2002 is Read-only!
            0x2003 => self.oam_addr = data,
            0x2004 => self.oam_data = data,
            0x2005 => self.scroll = data,
            0x2006 => self.addr = data,
            0x2007 => {
                self.data = data;
                // TODO: When we write to DATA, we actually write to VRAM
                // using the address set in 0x2006. We will implement this next.
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
