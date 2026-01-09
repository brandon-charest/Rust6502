#[derive(Debug, PartialEq, Eq)]
pub enum OpcodeSyntax {
    // Load/Store
    LDA,
    LDX,
    LDY,
    LAX,
    STA,
    STX,
    STY,
    SAX,
    // Transfer
    TAX,
    TXA,
    TAY,
    TYA,
    TSX,
    TXS,
    // Arithmetic
    ADC,
    SBC,
    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,
    DCP,
    // Logical
    AND,
    ORA,
    EOR,
    // Shift/Rotate
    ASL,
    LSR,
    ROL,
    ROR,
    RLA,
    // Compare
    CMP,
    CPX,
    CPY,
    BIT,
    // Branch
    BCC,
    BCS,
    BEQ,
    BNE,
    BMI,
    BPL,
    BVC,
    BVS,
    // Jump/Subroutine
    JMP,
    JSR,
    RTS,
    RTI,
    // Stack
    PHA,
    PHP,
    PLA,
    PLP,
    // Flag
    CLC,
    SEC,
    CLI,
    SEI,
    CLV,
    CLD,
    SED,
    // Control
    NOP,
    BRK,
    KIL,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AddressingMode {
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
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
            // === LOAD/STORE ===
            // LDA - Load Accumulator
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
                bytes: 2,
                cycles: 6,
            }),
            0xB1 => Some(Opcode {
                code: 0xB1,
                syntax: OpcodeSyntax::LDA,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 5,
            }),

            // LDX - Load X Register
            0xA2 => Some(Opcode {
                code: 0xA2,
                syntax: OpcodeSyntax::LDX,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0xA6 => Some(Opcode {
                code: 0xA6,
                syntax: OpcodeSyntax::LDX,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0xB6 => Some(Opcode {
                code: 0xB6,
                syntax: OpcodeSyntax::LDX,
                mode: AddressingMode::ZeroPageY,
                bytes: 2,
                cycles: 4,
            }),
            0xAE => Some(Opcode {
                code: 0xAE,
                syntax: OpcodeSyntax::LDX,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0xBE => Some(Opcode {
                code: 0xBE,
                syntax: OpcodeSyntax::LDX,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),

            // LDY - Load Y Register
            0xA0 => Some(Opcode {
                code: 0xA0,
                syntax: OpcodeSyntax::LDY,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0xA4 => Some(Opcode {
                code: 0xA4,
                syntax: OpcodeSyntax::LDY,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0xB4 => Some(Opcode {
                code: 0xB4,
                syntax: OpcodeSyntax::LDY,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0xAC => Some(Opcode {
                code: 0xAC,
                syntax: OpcodeSyntax::LDY,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0xBC => Some(Opcode {
                code: 0xBC,
                syntax: OpcodeSyntax::LDY,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),

            // STA - Store Accumulator
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

            // STX - Store X Register
            0x86 => Some(Opcode {
                code: 0x86,
                syntax: OpcodeSyntax::STX,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x96 => Some(Opcode {
                code: 0x96,
                syntax: OpcodeSyntax::STX,
                mode: AddressingMode::ZeroPageY,
                bytes: 2,
                cycles: 4,
            }),
            0x8E => Some(Opcode {
                code: 0x8E,
                syntax: OpcodeSyntax::STX,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),

            // STY - Store Y Register
            0x84 => Some(Opcode {
                code: 0x84,
                syntax: OpcodeSyntax::STY,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x94 => Some(Opcode {
                code: 0x94,
                syntax: OpcodeSyntax::STY,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0x8C => Some(Opcode {
                code: 0x8C,
                syntax: OpcodeSyntax::STY,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),

            // === TRANSFER ===
            0xAA => Some(Opcode {
                code: 0xAA,
                syntax: OpcodeSyntax::TAX,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0x8A => Some(Opcode {
                code: 0x8A,
                syntax: OpcodeSyntax::TXA,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0xA8 => Some(Opcode {
                code: 0xA8,
                syntax: OpcodeSyntax::TAY,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0x98 => Some(Opcode {
                code: 0x98,
                syntax: OpcodeSyntax::TYA,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0xBA => Some(Opcode {
                code: 0xBA,
                syntax: OpcodeSyntax::TSX,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0x9A => Some(Opcode {
                code: 0x9A,
                syntax: OpcodeSyntax::TXS,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),

            // === ARITHMETIC ===
            // ADC - Add with Carry
            0x69 => Some(Opcode {
                code: 0x69,
                syntax: OpcodeSyntax::ADC,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0x65 => Some(Opcode {
                code: 0x65,
                syntax: OpcodeSyntax::ADC,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x75 => Some(Opcode {
                code: 0x75,
                syntax: OpcodeSyntax::ADC,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0x6D => Some(Opcode {
                code: 0x6D,
                syntax: OpcodeSyntax::ADC,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0x7D => Some(Opcode {
                code: 0x7D,
                syntax: OpcodeSyntax::ADC,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),
            0x79 => Some(Opcode {
                code: 0x79,
                syntax: OpcodeSyntax::ADC,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),
            0x61 => Some(Opcode {
                code: 0x61,
                syntax: OpcodeSyntax::ADC,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6,
            }),
            0x71 => Some(Opcode {
                code: 0x71,
                syntax: OpcodeSyntax::ADC,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 5,
            }),

            // SBC - Subtract with Carry
            0xE9 => Some(Opcode {
                code: 0xE9,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0xE5 => Some(Opcode {
                code: 0xE5,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0xF5 => Some(Opcode {
                code: 0xF5,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0xED => Some(Opcode {
                code: 0xED,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0xFD => Some(Opcode {
                code: 0xFD,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),
            0xF9 => Some(Opcode {
                code: 0xF9,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),
            0xE1 => Some(Opcode {
                code: 0xE1,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6,
            }),
            0xF1 => Some(Opcode {
                code: 0xF1,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 5,
            }),

            // INC/DEC Memory
            0xE6 => Some(Opcode {
                code: 0xE6,
                syntax: OpcodeSyntax::INC,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            }),
            0xF6 => Some(Opcode {
                code: 0xF6,
                syntax: OpcodeSyntax::INC,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            }),
            0xEE => Some(Opcode {
                code: 0xEE,
                syntax: OpcodeSyntax::INC,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0xFE => Some(Opcode {
                code: 0xFE,
                syntax: OpcodeSyntax::INC,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            }),

            0xC6 => Some(Opcode {
                code: 0xC6,
                syntax: OpcodeSyntax::DEC,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            }),
            0xD6 => Some(Opcode {
                code: 0xD6,
                syntax: OpcodeSyntax::DEC,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            }),
            0xCE => Some(Opcode {
                code: 0xCE,
                syntax: OpcodeSyntax::DEC,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0xDE => Some(Opcode {
                code: 0xDE,
                syntax: OpcodeSyntax::DEC,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            }),

            // INC/DEC Registers
            0xE8 => Some(Opcode {
                code: 0xE8,
                syntax: OpcodeSyntax::INX,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0xC8 => Some(Opcode {
                code: 0xC8,
                syntax: OpcodeSyntax::INY,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0xCA => Some(Opcode {
                code: 0xCA,
                syntax: OpcodeSyntax::DEX,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0x88 => Some(Opcode {
                code: 0x88,
                syntax: OpcodeSyntax::DEY,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),

            // === LOGICAL ===
            // AND - Bitwise AND
            0x29 => Some(Opcode {
                code: 0x29,
                syntax: OpcodeSyntax::AND,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0x25 => Some(Opcode {
                code: 0x25,
                syntax: OpcodeSyntax::AND,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x35 => Some(Opcode {
                code: 0x35,
                syntax: OpcodeSyntax::AND,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0x2D => Some(Opcode {
                code: 0x2D,
                syntax: OpcodeSyntax::AND,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0x3D => Some(Opcode {
                code: 0x3D,
                syntax: OpcodeSyntax::AND,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),
            0x39 => Some(Opcode {
                code: 0x39,
                syntax: OpcodeSyntax::AND,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),
            0x21 => Some(Opcode {
                code: 0x21,
                syntax: OpcodeSyntax::AND,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6,
            }),
            0x31 => Some(Opcode {
                code: 0x31,
                syntax: OpcodeSyntax::AND,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 5,
            }),

            // ORA - Bitwise OR
            0x09 => Some(Opcode {
                code: 0x09,
                syntax: OpcodeSyntax::ORA,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0x05 => Some(Opcode {
                code: 0x05,
                syntax: OpcodeSyntax::ORA,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x15 => Some(Opcode {
                code: 0x15,
                syntax: OpcodeSyntax::ORA,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0x0D => Some(Opcode {
                code: 0x0D,
                syntax: OpcodeSyntax::ORA,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0x1D => Some(Opcode {
                code: 0x1D,
                syntax: OpcodeSyntax::ORA,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),
            0x19 => Some(Opcode {
                code: 0x19,
                syntax: OpcodeSyntax::ORA,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),
            0x01 => Some(Opcode {
                code: 0x01,
                syntax: OpcodeSyntax::ORA,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6,
            }),
            0x11 => Some(Opcode {
                code: 0x11,
                syntax: OpcodeSyntax::ORA,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 5,
            }),

            // EOR - Bitwise Exclusive OR
            0x49 => Some(Opcode {
                code: 0x49,
                syntax: OpcodeSyntax::EOR,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0x45 => Some(Opcode {
                code: 0x45,
                syntax: OpcodeSyntax::EOR,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x55 => Some(Opcode {
                code: 0x55,
                syntax: OpcodeSyntax::EOR,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0x4D => Some(Opcode {
                code: 0x4D,
                syntax: OpcodeSyntax::EOR,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0x5D => Some(Opcode {
                code: 0x5D,
                syntax: OpcodeSyntax::EOR,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),
            0x59 => Some(Opcode {
                code: 0x59,
                syntax: OpcodeSyntax::EOR,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),
            0x41 => Some(Opcode {
                code: 0x41,
                syntax: OpcodeSyntax::EOR,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6,
            }),
            0x51 => Some(Opcode {
                code: 0x51,
                syntax: OpcodeSyntax::EOR,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 5,
            }),

            // === SHIFT/ROTATE ===
            // ASL - Arithmetic Shift Left
            0x0A => Some(Opcode {
                code: 0x0A,
                syntax: OpcodeSyntax::ASL,
                mode: AddressingMode::Accumulator,
                bytes: 1,
                cycles: 2,
            }),
            0x06 => Some(Opcode {
                code: 0x06,
                syntax: OpcodeSyntax::ASL,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            }),
            0x16 => Some(Opcode {
                code: 0x16,
                syntax: OpcodeSyntax::ASL,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            }),
            0x0E => Some(Opcode {
                code: 0x0E,
                syntax: OpcodeSyntax::ASL,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0x1E => Some(Opcode {
                code: 0x1E,
                syntax: OpcodeSyntax::ASL,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            }),

            // LSR - Logical Shift Right
            0x4A => Some(Opcode {
                code: 0x4A,
                syntax: OpcodeSyntax::LSR,
                mode: AddressingMode::Accumulator,
                bytes: 1,
                cycles: 2,
            }),
            0x46 => Some(Opcode {
                code: 0x46,
                syntax: OpcodeSyntax::LSR,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            }),
            0x56 => Some(Opcode {
                code: 0x56,
                syntax: OpcodeSyntax::LSR,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            }),
            0x4E => Some(Opcode {
                code: 0x4E,
                syntax: OpcodeSyntax::LSR,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0x5E => Some(Opcode {
                code: 0x5E,
                syntax: OpcodeSyntax::LSR,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            }),

            // ROL - Rotate Left
            0x2A => Some(Opcode {
                code: 0x2A,
                syntax: OpcodeSyntax::ROL,
                mode: AddressingMode::Accumulator,
                bytes: 1,
                cycles: 2,
            }),
            0x26 => Some(Opcode {
                code: 0x26,
                syntax: OpcodeSyntax::ROL,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            }),
            0x36 => Some(Opcode {
                code: 0x36,
                syntax: OpcodeSyntax::ROL,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            }),
            0x2E => Some(Opcode {
                code: 0x2E,
                syntax: OpcodeSyntax::ROL,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0x3E => Some(Opcode {
                code: 0x3E,
                syntax: OpcodeSyntax::ROL,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            }),

            // ROR - Rotate Right
            0x6A => Some(Opcode {
                code: 0x6A,
                syntax: OpcodeSyntax::ROR,
                mode: AddressingMode::Accumulator,
                bytes: 1,
                cycles: 2,
            }),
            0x66 => Some(Opcode {
                code: 0x66,
                syntax: OpcodeSyntax::ROR,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            }),
            0x76 => Some(Opcode {
                code: 0x76,
                syntax: OpcodeSyntax::ROR,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            }),
            0x6E => Some(Opcode {
                code: 0x6E,
                syntax: OpcodeSyntax::ROR,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0x7E => Some(Opcode {
                code: 0x7E,
                syntax: OpcodeSyntax::ROR,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            }),

            // === COMPARE ===
            // CMP - Compare Accumulator
            0xC9 => Some(Opcode {
                code: 0xC9,
                syntax: OpcodeSyntax::CMP,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0xC5 => Some(Opcode {
                code: 0xC5,
                syntax: OpcodeSyntax::CMP,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0xD5 => Some(Opcode {
                code: 0xD5,
                syntax: OpcodeSyntax::CMP,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0xCD => Some(Opcode {
                code: 0xCD,
                syntax: OpcodeSyntax::CMP,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0xDD => Some(Opcode {
                code: 0xDD,
                syntax: OpcodeSyntax::CMP,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),
            0xD9 => Some(Opcode {
                code: 0xD9,
                syntax: OpcodeSyntax::CMP,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),
            0xC1 => Some(Opcode {
                code: 0xC1,
                syntax: OpcodeSyntax::CMP,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6,
            }),
            0xD1 => Some(Opcode {
                code: 0xD1,
                syntax: OpcodeSyntax::CMP,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 5,
            }),

            // CPX - Compare X Register
            0xE0 => Some(Opcode {
                code: 0xE0,
                syntax: OpcodeSyntax::CPX,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0xE4 => Some(Opcode {
                code: 0xE4,
                syntax: OpcodeSyntax::CPX,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0xEC => Some(Opcode {
                code: 0xEC,
                syntax: OpcodeSyntax::CPX,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),

            // CPY - Compare Y Register
            0xC0 => Some(Opcode {
                code: 0xC0,
                syntax: OpcodeSyntax::CPY,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),
            0xC4 => Some(Opcode {
                code: 0xC4,
                syntax: OpcodeSyntax::CPY,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0xCC => Some(Opcode {
                code: 0xCC,
                syntax: OpcodeSyntax::CPY,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),

            // BIT - Test Bits
            0x24 => Some(Opcode {
                code: 0x24,
                syntax: OpcodeSyntax::BIT,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x2C => Some(Opcode {
                code: 0x2C,
                syntax: OpcodeSyntax::BIT,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),

            // === BRANCH ===
            0x10 => Some(Opcode {
                code: 0x10,
                syntax: OpcodeSyntax::BPL,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            }),
            0x30 => Some(Opcode {
                code: 0x30,
                syntax: OpcodeSyntax::BMI,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            }),
            0x50 => Some(Opcode {
                code: 0x50,
                syntax: OpcodeSyntax::BVC,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            }),
            0x70 => Some(Opcode {
                code: 0x70,
                syntax: OpcodeSyntax::BVS,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            }),
            0x90 => Some(Opcode {
                code: 0x90,
                syntax: OpcodeSyntax::BCC,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            }),
            0xB0 => Some(Opcode {
                code: 0xB0,
                syntax: OpcodeSyntax::BCS,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            }),
            0xD0 => Some(Opcode {
                code: 0xD0,
                syntax: OpcodeSyntax::BNE,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            }),
            0xF0 => Some(Opcode {
                code: 0xF0,
                syntax: OpcodeSyntax::BEQ,
                mode: AddressingMode::Relative,
                bytes: 2,
                cycles: 2,
            }),

            // === JUMP/SUBROUTINE ===
            0x4C => Some(Opcode {
                code: 0x4C,
                syntax: OpcodeSyntax::JMP,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 3,
            }),
            0x6C => Some(Opcode {
                code: 0x6C,
                syntax: OpcodeSyntax::JMP,
                mode: AddressingMode::Indirect,
                bytes: 3,
                cycles: 5,
            }),
            0x20 => Some(Opcode {
                code: 0x20,
                syntax: OpcodeSyntax::JSR,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0x60 => Some(Opcode {
                code: 0x60,
                syntax: OpcodeSyntax::RTS,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 6,
            }),
            0x40 => Some(Opcode {
                code: 0x40,
                syntax: OpcodeSyntax::RTI,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 6,
            }),

            // === STACK ===
            0x48 => Some(Opcode {
                code: 0x48,
                syntax: OpcodeSyntax::PHA,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 3,
            }),
            0x08 => Some(Opcode {
                code: 0x08,
                syntax: OpcodeSyntax::PHP,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 3,
            }),
            0x68 => Some(Opcode {
                code: 0x68,
                syntax: OpcodeSyntax::PLA,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 4,
            }),
            0x28 => Some(Opcode {
                code: 0x28,
                syntax: OpcodeSyntax::PLP,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 4,
            }),

            // === FLAGS ===
            0x18 => Some(Opcode {
                code: 0x18,
                syntax: OpcodeSyntax::CLC,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0x38 => Some(Opcode {
                code: 0x38,
                syntax: OpcodeSyntax::SEC,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0x58 => Some(Opcode {
                code: 0x58,
                syntax: OpcodeSyntax::CLI,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0x78 => Some(Opcode {
                code: 0x78,
                syntax: OpcodeSyntax::SEI,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0xB8 => Some(Opcode {
                code: 0xB8,
                syntax: OpcodeSyntax::CLV,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0xD8 => Some(Opcode {
                code: 0xD8,
                syntax: OpcodeSyntax::CLD,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),
            0xF8 => Some(Opcode {
                code: 0xF8,
                syntax: OpcodeSyntax::SED,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),

            // === CONTROL ===
            0xEA => Some(Opcode {
                code: 0xEA,
                syntax: OpcodeSyntax::NOP,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),

            0x00 => Some(Opcode {
                code: 0x00,
                syntax: OpcodeSyntax::BRK,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 7,
            }),

            // ==== UNOFFICIAL OPCODES ====

            // ==== NO OP ====
            0x1A | 0x3A | 0x5A | 0x7A | 0xDA | 0xFA => Some(Opcode {
                code: code,
                syntax: OpcodeSyntax::NOP,
                mode: AddressingMode::Implied,
                bytes: 1,
                cycles: 2,
            }),

            0x80 | 0x82 | 0x89 | 0xC2 | 0xE2 => Some(Opcode {
                code: code,
                syntax: OpcodeSyntax::NOP,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),

            0x04 | 0x44 | 0x64 => Some(Opcode {
                code: code,
                syntax: OpcodeSyntax::NOP,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x14 | 0x34 | 0x54 | 0x74 | 0xD4 | 0xF4 => Some(Opcode {
                code: code,
                syntax: OpcodeSyntax::NOP,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 4,
            }),
            0x0C => Some(Opcode {
                code: 0x0C,
                syntax: OpcodeSyntax::NOP,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),

            0x1C | 0x3C | 0x5C | 0x7C | 0xDC | 0xFC => Some(Opcode {
                code: code,
                syntax: OpcodeSyntax::NOP,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 4,
            }),

            // ==== LOAD ====
            0xA3 => Some(Opcode {
                code: 0xA3,
                syntax: OpcodeSyntax::LAX,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 6,
            }),
            0xB3 => Some(Opcode {
                code: 0xB3,
                syntax: OpcodeSyntax::LAX,
                mode: AddressingMode::ZeroPageY,
                bytes: 2,
                cycles: 5,
            }),
            0xA7 => Some(Opcode {
                code: 0xA7,
                syntax: OpcodeSyntax::LAX,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0xB7 => Some(Opcode {
                code: 0xB7,
                syntax: OpcodeSyntax::LAX,
                mode: AddressingMode::ZeroPageY,
                bytes: 2,
                cycles: 4,
            }),
            0xAF => Some(Opcode {
                code: 0xAF,
                syntax: OpcodeSyntax::LAX,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0xBF => Some(Opcode {
                code: 0xBF,
                syntax: OpcodeSyntax::LAX,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 4,
            }),
            0x87 => Some(Opcode {
                code: 0x87,
                syntax: OpcodeSyntax::SAX,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 3,
            }),
            0x97 => Some(Opcode {
                code: 0x97,
                syntax: OpcodeSyntax::SAX,
                mode: AddressingMode::ZeroPageY,
                bytes: 2,
                cycles: 4,
            }),
            0x8F => Some(Opcode {
                code: 0x8F,
                syntax: OpcodeSyntax::SAX,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 4,
            }),
            0x83 => Some(Opcode {
                code: 0x83,
                syntax: OpcodeSyntax::SAX,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 6,
            }),

            0xEB => Some(Opcode {
                code: 0xEB,
                syntax: OpcodeSyntax::SBC,
                mode: AddressingMode::Immediate,
                bytes: 2,
                cycles: 2,
            }),

            0xC7 => Some(Opcode {
                code: 0xC7,
                syntax: OpcodeSyntax::DCP,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            }),
            0xD7 => Some(Opcode {
                code: 0xD7,
                syntax: OpcodeSyntax::DCP,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            }),
            0xCF => Some(Opcode {
                code: 0xCF,
                syntax: OpcodeSyntax::DCP,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0xDF => Some(Opcode {
                code: 0xDF,
                syntax: OpcodeSyntax::DCP,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            }),
            0xDB => Some(Opcode {
                code: 0xDB,
                syntax: OpcodeSyntax::DCP,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 7,
            }),
            0xC3 => Some(Opcode {
                code: 0xC3,
                syntax: OpcodeSyntax::DCP,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 8,
            }),
            0xD3 => Some(Opcode {
                code: 0xD3,
                syntax: OpcodeSyntax::DCP,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 8,
            }),

            0x27 => Some(Opcode {
                code: 0x27,
                syntax: OpcodeSyntax::RLA,
                mode: AddressingMode::ZeroPage,
                bytes: 2,
                cycles: 5,
            }),
            0x37 => Some(Opcode {
                code: 0x37,
                syntax: OpcodeSyntax::RLA,
                mode: AddressingMode::ZeroPageX,
                bytes: 2,
                cycles: 6,
            }),

            0x2F => Some(Opcode {
                code: 0x2F,
                syntax: OpcodeSyntax::RLA,
                mode: AddressingMode::Absolute,
                bytes: 3,
                cycles: 6,
            }),
            0x3F => Some(Opcode {
                code: 0x3F,
                syntax: OpcodeSyntax::RLA,
                mode: AddressingMode::AbsoluteX,
                bytes: 3,
                cycles: 7,
            }),
            0x3B => Some(Opcode {
                code: 0x3B,
                syntax: OpcodeSyntax::RLA,
                mode: AddressingMode::AbsoluteY,
                bytes: 3,
                cycles: 7,
            }),

            0x23 => Some(Opcode {
                code: 0x23,
                syntax: OpcodeSyntax::RLA,
                mode: AddressingMode::IndirectX,
                bytes: 2,
                cycles: 8,
            }),

            0x33 => Some(Opcode {
                code: 0x33,
                syntax: OpcodeSyntax::RLA,
                mode: AddressingMode::IndirectY,
                bytes: 2,
                cycles: 8,
            }),

            // KIL / JAM (Halt CPU)
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xB2 | 0xD2 | 0xF2 => {
                Some(Opcode {
                    code: code,
                    syntax: OpcodeSyntax::KIL,
                    mode: AddressingMode::Implied, // It doesn't really matter, it just dies
                    bytes: 1,
                    cycles: 0,
                })
            }
            _ => None,
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
