use crate::hardware::opcodes::{AddressingMode, Opcode, OpcodeSyntax};

// === LOAD/STORE TESTS ===
#[test]
fn test_lda_addressing_modes() {
    // LDA Immediate
    let op = Opcode::from_u8(0xA9).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::LDA);
    assert_eq!(op.mode, AddressingMode::Immediate);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // LDA Absolute
    let op = Opcode::from_u8(0xAD).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::LDA);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 4);

    // LDA IndirectY
    let op = Opcode::from_u8(0xB1).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::LDA);
    assert_eq!(op.mode, AddressingMode::IndirectY);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 5);
}

#[test]
fn test_sta_opcodes() {
    // STA ZeroPage
    let op = Opcode::from_u8(0x85).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::STA);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 3);

    // STA AbsoluteX
    let op = Opcode::from_u8(0x9D).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::STA);
    assert_eq!(op.mode, AddressingMode::AbsoluteX);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 5);
}

#[test]
fn test_ldx_ldy_opcodes() {
    // LDX Immediate
    let op = Opcode::from_u8(0xA2).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::LDX);
    assert_eq!(op.mode, AddressingMode::Immediate);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // LDY ZeroPageX
    let op = Opcode::from_u8(0xB4).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::LDY);
    assert_eq!(op.mode, AddressingMode::ZeroPageX);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 4);
}

// === ARITHMETIC TESTS ===
#[test]
fn test_adc_opcodes() {
    // ADC Immediate
    let op = Opcode::from_u8(0x69).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::ADC);
    assert_eq!(op.mode, AddressingMode::Immediate);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // ADC AbsoluteY
    let op = Opcode::from_u8(0x79).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::ADC);
    assert_eq!(op.mode, AddressingMode::AbsoluteY);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 4);
}

#[test]
fn test_sbc_opcodes() {
    // SBC Immediate (official)
    let op = Opcode::from_u8(0xE9).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::SBC);
    assert_eq!(op.mode, AddressingMode::Immediate);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // SBC ZeroPageX
    let op = Opcode::from_u8(0xF5).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::SBC);
    assert_eq!(op.mode, AddressingMode::ZeroPageX);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 4);

    // SBC Immediate (unofficial 0xEB)
    let op = Opcode::from_u8(0xEB).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::SBC);
    assert_eq!(op.mode, AddressingMode::Immediate);
}

// === LOGIC TESTS ===
#[test]
fn test_and_ora_eor_opcodes() {
    // AND Immediate
    let op = Opcode::from_u8(0x29).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::AND);
    assert_eq!(op.mode, AddressingMode::Immediate);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // ORA ZeroPage
    let op = Opcode::from_u8(0x05).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::ORA);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 3);

    // EOR Absolute
    let op = Opcode::from_u8(0x4D).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::EOR);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 4);
}

#[test]
fn test_bit_opcode() {
    // BIT ZeroPage
    let op = Opcode::from_u8(0x24).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::BIT);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 3);

    // BIT Absolute
    let op = Opcode::from_u8(0x2C).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::BIT);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 4);
}

// === BRANCH TESTS ===
#[test]
fn test_branch_opcodes() {
    // BCC
    let op = Opcode::from_u8(0x90).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::BCC);
    assert_eq!(op.mode, AddressingMode::Relative);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // BEQ
    let op = Opcode::from_u8(0xF0).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::BEQ);
    assert_eq!(op.mode, AddressingMode::Relative);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // BMI
    let op = Opcode::from_u8(0x30).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::BMI);
    assert_eq!(op.mode, AddressingMode::Relative);

    // BVS
    let op = Opcode::from_u8(0x70).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::BVS);
    assert_eq!(op.mode, AddressingMode::Relative);
}

// === TRANSFER TESTS ===
#[test]
fn test_transfer_opcodes() {
    // TAX
    let op = Opcode::from_u8(0xAA).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::TAX);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 2);

    // TYA
    let op = Opcode::from_u8(0x98).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::TYA);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 2);

    // TSX
    let op = Opcode::from_u8(0xBA).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::TSX);
    assert_eq!(op.mode, AddressingMode::Implied);
}

// === COMPARE TESTS ===
#[test]
fn test_compare_opcodes() {
    // CMP Immediate
    let op = Opcode::from_u8(0xC9).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::CMP);
    assert_eq!(op.mode, AddressingMode::Immediate);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // CPX ZeroPage
    let op = Opcode::from_u8(0xE4).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::CPX);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 3);

    // CPY Absolute
    let op = Opcode::from_u8(0xCC).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::CPY);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 4);
}

// === INCREMENT/DECREMENT TESTS ===
#[test]
fn test_increment_decrement_opcodes() {
    // INC ZeroPage
    let op = Opcode::from_u8(0xE6).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::INC);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 5);

    // DEC Absolute
    let op = Opcode::from_u8(0xCE).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::DEC);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 6);

    // INX
    let op = Opcode::from_u8(0xE8).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::INX);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 2);

    // DEY
    let op = Opcode::from_u8(0x88).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::DEY);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 2);
}

// === SHIFT/ROTATE TESTS ===
#[test]
fn test_shift_rotate_opcodes() {
    // ASL Accumulator
    let op = Opcode::from_u8(0x0A).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::ASL);
    assert_eq!(op.mode, AddressingMode::Accumulator);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 2);

    // LSR ZeroPage
    let op = Opcode::from_u8(0x46).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::LSR);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 5);

    // ROL Absolute
    let op = Opcode::from_u8(0x2E).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::ROL);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 6);

    // ROR AbsoluteX
    let op = Opcode::from_u8(0x7E).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::ROR);
    assert_eq!(op.mode, AddressingMode::AbsoluteX);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 7);
}

// === STACK TESTS ===
#[test]
fn test_stack_opcodes() {
    // PHA
    let op = Opcode::from_u8(0x48).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::PHA);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 3);

    // PLA
    let op = Opcode::from_u8(0x68).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::PLA);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 4);

    // PHP
    let op = Opcode::from_u8(0x08).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::PHP);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 3);

    // PLP
    let op = Opcode::from_u8(0x28).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::PLP);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 4);
}

// === JUMP/SUBROUTINE TESTS ===
#[test]
fn test_jump_subroutine_opcodes() {
    // JMP Absolute
    let op = Opcode::from_u8(0x4C).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::JMP);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 3);

    // JMP Indirect
    let op = Opcode::from_u8(0x6C).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::JMP);
    assert_eq!(op.mode, AddressingMode::Indirect);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 5);

    // JSR
    let op = Opcode::from_u8(0x20).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::JSR);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 6);

    // RTS
    let op = Opcode::from_u8(0x60).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::RTS);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 6);

    // RTI
    let op = Opcode::from_u8(0x40).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::RTI);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 6);
}

// === FLAG TESTS ===
#[test]
fn test_flag_opcodes() {
    // CLC
    let op = Opcode::from_u8(0x18).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::CLC);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 2);

    // SEC
    let op = Opcode::from_u8(0x38).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::SEC);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 2);

    // SEI
    let op = Opcode::from_u8(0x78).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::SEI);

    // CLD
    let op = Opcode::from_u8(0xD8).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::CLD);
}

// === CONTROL TESTS ===
#[test]
fn test_control_opcodes() {
    // NOP
    let op = Opcode::from_u8(0xEA).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::NOP);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 2);

    // BRK
    let op = Opcode::from_u8(0x00).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::BRK);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 7);
}

// === UNOFFICIAL OPCODE TESTS ===
#[test]
fn test_unofficial_nop_variants() {
    // Implied NOPs
    let op = Opcode::from_u8(0x1A).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::NOP);
    assert_eq!(op.mode, AddressingMode::Implied);

    // Immediate NOPs
    let op = Opcode::from_u8(0x80).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::NOP);
    assert_eq!(op.mode, AddressingMode::Immediate);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 2);

    // ZeroPage NOPs
    let op = Opcode::from_u8(0x04).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::NOP);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 3);

    // AbsoluteX NOPs
    let op = Opcode::from_u8(0x1C).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::NOP);
    assert_eq!(op.mode, AddressingMode::AbsoluteX);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 4);
}

#[test]
fn test_unofficial_lax_sax() {
    // LAX ZeroPage
    let op = Opcode::from_u8(0xA7).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::LAX);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 3);

    // LAX IndirectX
    let op = Opcode::from_u8(0xA3).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::LAX);
    assert_eq!(op.mode, AddressingMode::IndirectX);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 6);

    // SAX ZeroPage
    let op = Opcode::from_u8(0x87).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::SAX);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 3);
}

#[test]
fn test_unofficial_dcp_rla() {
    // DCP ZeroPage
    let op = Opcode::from_u8(0xC7).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::DCP);
    assert_eq!(op.mode, AddressingMode::ZeroPage);
    assert_eq!(op.bytes, 2);
    assert_eq!(op.cycles, 5);

    // RLA Absolute
    let op = Opcode::from_u8(0x2F).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::RLA);
    assert_eq!(op.mode, AddressingMode::Absolute);
    assert_eq!(op.bytes, 3);
    assert_eq!(op.cycles, 6);
}

#[test]
fn test_kil_opcodes() {
    // KIL opcodes (halt CPU)
    let op = Opcode::from_u8(0x02).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::KIL);
    assert_eq!(op.mode, AddressingMode::Implied);
    assert_eq!(op.bytes, 1);
    assert_eq!(op.cycles, 0);

    let op = Opcode::from_u8(0xF2).unwrap();
    assert_eq!(op.syntax, OpcodeSyntax::KIL);
}

// === EDGE CASE TESTS ===
#[test]
fn test_invalid_opcodes() {
    // There are some undefined opcodes that should return None
    // But in this implementation, most are defined as unofficial opcodes
    // We test that from_u8 doesn't panic for any input
    for code in 0u8..=255u8 {
        let _ = Opcode::from_u8(code);
    }
}

#[test]
fn test_opcode_bytes_consistency() {
    // Immediate mode should always be 2 bytes
    let op = Opcode::from_u8(0xA9).unwrap(); // LDA Immediate
    assert_eq!(op.bytes, 2);

    // Absolute mode should always be 3 bytes
    let op = Opcode::from_u8(0xAD).unwrap(); // LDA Absolute
    assert_eq!(op.bytes, 3);

    // Implied mode should always be 1 byte
    let op = Opcode::from_u8(0xAA).unwrap(); // TAX
    assert_eq!(op.bytes, 1);
}
