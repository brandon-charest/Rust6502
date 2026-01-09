use crate::hardware::cpu::{AddressingMode, Opcode};

pub fn disassemble_instruction(opcode: &Opcode, raw_bytes: &[u8], pc: u16) -> String {
    let mnemonic = format!("{:?}", opcode.syntax);

    match opcode.mode {
        AddressingMode::Implied | AddressingMode::Accumulator => mnemonic,

        AddressingMode::Immediate => format!("{} #${:02X}", mnemonic, raw_bytes[1]),

        AddressingMode::ZeroPage => format!("{} ${:02X}", mnemonic, raw_bytes[1]),
        AddressingMode::ZeroPageX => format!("{} ${:02X},X", mnemonic, raw_bytes[1]),
        AddressingMode::ZeroPageY => format!("{} ${:02X},Y", mnemonic, raw_bytes[1]),

        AddressingMode::Relative => {
            let offset = raw_bytes[1] as i8;
            let addr = pc.wrapping_add(2).wrapping_add(offset as u16);
            format!("{} {}", mnemonic, format_address(addr))
        }

        AddressingMode::Absolute => {
            let addr = u16::from_le_bytes([raw_bytes[1], raw_bytes[2]]);
            format!("{} {}", mnemonic, format_address(addr))
        }
        AddressingMode::AbsoluteX => {
            let addr = u16::from_le_bytes([raw_bytes[1], raw_bytes[2]]);
            format!("{} {}", mnemonic, format_address(addr))
        }
        AddressingMode::AbsoluteY => {
            let addr = u16::from_le_bytes([raw_bytes[1], raw_bytes[2]]);
            format!("{} {}", mnemonic, format_address(addr))
        }

        AddressingMode::Indirect => {
            let addr = u16::from_le_bytes([raw_bytes[1], raw_bytes[2]]);
            format!("{} {}", mnemonic, format_address(addr))
        }
        AddressingMode::IndirectX => format!("{} (${:02X},X)", mnemonic, raw_bytes[1]),
        AddressingMode::IndirectY => format!("{} (${:02X}),Y", mnemonic, raw_bytes[1]),
    }
}

fn format_address(addr: u16) -> String {
    match addr {
        0x2000 => "PPU_CTRL".to_string(),
        0x2001 => "PPU_MASK".to_string(),
        0x2002 => "PPU_STATUS".to_string(),
        0x2003 => "OAM_ADDR".to_string(),
        0x2004 => "OAM_DATA".to_string(),
        0x2005 => "PPU_SCROLL".to_string(),
        0x2006 => "PPU_ADDR".to_string(),
        0x2007 => "PPU_DATA".to_string(),
        0x4014 => "OAM_DMA".to_string(),
        0x4016 => "JOYPAD1".to_string(),
        0x4017 => "JOYPAD2".to_string(),
        _ => format!("${:04X}", addr),
    }
}
