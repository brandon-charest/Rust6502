#[derive(Debug, PartialEq, Eq)]
pub enum OpcodeSyntax {
    BRK,
    LDA,
    LDX,
    LDY,
    NOP,
    STA,
    STX,
    STY,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    IndirectX,
    IndirectY,
    Implied,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

pub struct Opcode {
    pub code: u8,
    pub syntax: OpcodeSyntax,
    pub mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

impl Opcode {
    pub fn from_u8(code: u8) -> Option<Self> {
        match code {
            // 0x00 => Some(Opcode {
            //     code: 0x00,
            //     syntax: OpcodeSyntax::BRK,
            //     mode: AddressingMode::Implied,
            //     bytes: 1,
            //     cycles: 7,
            // }),
            0x8D => Some(Opcode {
                code: 0x8D,
                syntax: OpcodeSyntax::STA,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0x9D => Some(Opcode {
                code: 0x9D,
                syntax: OpcodeSyntax::STA,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 5,
            }),
            0x99 => Some(Opcode {
                code: 0x99,
                syntax: OpcodeSyntax::STA,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 5,
            }),
            0x85 => Some(Opcode {
                code: 0x85,
                syntax: OpcodeSyntax::STA,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x95 => Some(Opcode {
                code: 0x95,
                syntax: OpcodeSyntax::STA,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0x81 => Some(Opcode {
                code: 0x81,
                syntax: OpcodeSyntax::STA,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6,
            }),
            0x91 => Some(Opcode {
                code: 0x91,
                syntax: OpcodeSyntax::STA,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 6,
            }),
            0xA9 => Some(Opcode {
                code: 0xA9,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0xA5 => Some(Opcode {
                code: 0xA5,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0xB5 => Some(Opcode {
                code: 0xB5,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0xAD => Some(Opcode {
                code: 0xAD,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0xBD => Some(Opcode {
                code: 0xBD,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),
            0xB9 => Some(Opcode {
                code: 0xB9,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),
            0xA1 => Some(Opcode {
                code: 0xA1,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::IndirectX,
                bytes: 3,
                cycles: 6,
            }),
            0xB1 => Some(Opcode {
                code: 0xB1,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::IndirectY,
                bytes: 3,
                cycles: 5,
            }),
            0xEA => Some(Opcode {
                code: 0xEA,
                syntax: OpcodeSyntax::NOP,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            _ => todo!("Opcode {:#X} not implemented!", code),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_opcode() {
        if let Some(opcode) = Opcode::from_u8(0xEA) {
            assert_eq!(opcode.code, 0xEA);
            assert_eq!(opcode.syntax, OpcodeSyntax::NOP);
            assert_eq!(opcode.mode, AddressingMode::Implied);
            assert_eq!(opcode.bytes, 1);
            assert_eq!(opcode.cycles, 2);
        } else {
            panic!("Opcode 0xEA should have been found!");
        }
    }
}
