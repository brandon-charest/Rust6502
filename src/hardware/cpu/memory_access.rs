use crate::hardware::{bus::Bus, cpu::CPU};

impl CPU {
    #[inline(always)]
    pub(crate) fn read(&mut self, bus: &mut dyn Bus, addr: u16) -> u8 {
        self.cycles += 1;
        bus.read(addr)
    }

    #[inline(always)]
    pub(crate) fn write(&mut self, bus: &mut dyn Bus, addr: u16, data: u8) {
        self.cycles += 1;
        bus.write(addr, data);
    }

    #[inline(always)]
    pub(crate) fn fetch_byte(&mut self, bus: &mut dyn Bus) -> u8 {
        let addr = self.registers.program_counter;
        let data = self.read(bus, addr);
        self.registers.program_counter = self.registers.program_counter.wrapping_add(1);
        data
    }

    #[inline(always)]
    pub(crate) fn fetch_word(&mut self, bus: &mut dyn Bus) -> u16 {
        let lo = self.fetch_byte(bus) as u16;
        let hi = self.fetch_byte(bus) as u16;
        (hi << 8) | lo
    }

    pub(crate) fn read_word(&mut self, bus: &mut dyn Bus, addr: u16) -> u16 {
        let lo = self.read(bus, addr) as u16;
        let hi = self.read(bus, addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }

    pub(crate) fn push(&mut self, bus: &mut dyn Bus, data: u8) {
        self.write(bus, 0x0100 + self.registers.stack_pointer as u16, data);
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_sub(1);
    }

    pub(crate) fn pop(&mut self, bus: &mut dyn Bus) -> u8 {
        self.registers.stack_pointer = self.registers.stack_pointer.wrapping_add(1);
        self.read(bus, 0x0100 + self.registers.stack_pointer as u16)
    }
}
