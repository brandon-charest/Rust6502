use crate::hardware::cpu::disassembler::*;
use crate::hardware::opcodes::{AddressingMode, Opcode, OpcodeSyntax};

// ============================================================================
// Implied/Accumulator Addressing Mode Tests
// ============================================================================

#[test]
fn test_disassemble_implied() {
    let opcode = Opcode {
        code: 0xAA,
        syntax: OpcodeSyntax::TAX,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    };
    let raw_bytes = [0xAA];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "TAX");
}

#[test]
fn test_disassemble_accumulator() {
    let opcode = Opcode {
        code: 0x0A,
        syntax: OpcodeSyntax::ASL,
        mode: AddressingMode::Accumulator,
        bytes: 1,
        cycles: 2,
    };
    let raw_bytes = [0x0A];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "ASL");
}

// ============================================================================
// Immediate Addressing Mode Tests
// ============================================================================

#[test]
fn test_disassemble_immediate() {
    let opcode = Opcode {
        code: 0xA9,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    };
    let raw_bytes = [0xA9, 0x42];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA #$42");
}

#[test]
fn test_disassemble_immediate_hex() {
    let opcode = Opcode {
        code: 0x29,
        syntax: OpcodeSyntax::AND,
        mode: AddressingMode::Immediate,
        bytes: 2,
        cycles: 2,
    };
    let raw_bytes = [0x29, 0xFF];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "AND #$FF");
}

// ============================================================================
// Zero Page Addressing Mode Tests
// ============================================================================

#[test]
fn test_disassemble_zero_page() {
    let opcode = Opcode {
        code: 0xA5,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::ZeroPage,
        bytes: 2,
        cycles: 3,
    };
    let raw_bytes = [0xA5, 0x42];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA $42");
}

#[test]
fn test_disassemble_zero_page_x() {
    let opcode = Opcode {
        code: 0xB5,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::ZeroPageX,
        bytes: 2,
        cycles: 4,
    };
    let raw_bytes = [0xB5, 0x10];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA $10,X");
}

#[test]
fn test_disassemble_zero_page_y() {
    let opcode = Opcode {
        code: 0xB6,
        syntax: OpcodeSyntax::LDX,
        mode: AddressingMode::ZeroPageY,
        bytes: 2,
        cycles: 4,
    };
    let raw_bytes = [0xB6, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDX $20,Y");
}

// ============================================================================
// Absolute Addressing Mode Tests
// ============================================================================

#[test]
fn test_disassemble_absolute() {
    let opcode = Opcode {
        code: 0xAD,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0xAD, 0x34, 0x12];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA $1234");
}

#[test]
fn test_disassemble_absolute_x() {
    let opcode = Opcode {
        code: 0xBD,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::AbsoluteX,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0xBD, 0x00, 0x80];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA $8000");
}

#[test]
fn test_disassemble_absolute_y() {
    let opcode = Opcode {
        code: 0xB9,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::AbsoluteY,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0xB9, 0xFF, 0x7F];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA $7FFF");
}

// ============================================================================
// Relative Addressing Mode Tests (Branches)
// ============================================================================

#[test]
fn test_disassemble_relative_forward() {
    let opcode = Opcode {
        code: 0x10,
        syntax: OpcodeSyntax::BPL,
        mode: AddressingMode::Relative,
        bytes: 2,
        cycles: 2,
    };
    let raw_bytes = [0x10, 0x05];
    // PC at 0x8000, offset +5, so target is 0x8000 + 2 + 5 = 0x8007
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "BPL $8007");
}

#[test]
fn test_disassemble_relative_backward() {
    let opcode = Opcode {
        code: 0xD0,
        syntax: OpcodeSyntax::BNE,
        mode: AddressingMode::Relative,
        bytes: 2,
        cycles: 2,
    };
    let raw_bytes = [0xD0, 0xFE]; // -2 in two's complement
    // PC at 0x8000, offset -2, so target is 0x8000 + 2 - 2 = 0x8000
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "BNE $8000");
}

#[test]
fn test_disassemble_relative_zero() {
    let opcode = Opcode {
        code: 0xF0,
        syntax: OpcodeSyntax::BEQ,
        mode: AddressingMode::Relative,
        bytes: 2,
        cycles: 2,
    };
    let raw_bytes = [0xF0, 0x00];
    // PC at 0x1234, offset 0, so target is 0x1234 + 2 = 0x1236
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x1234);
    assert_eq!(result, "BEQ $1236");
}

// ============================================================================
// Indirect Addressing Mode Tests
// ============================================================================

#[test]
fn test_disassemble_indirect() {
    let opcode = Opcode {
        code: 0x6C,
        syntax: OpcodeSyntax::JMP,
        mode: AddressingMode::Indirect,
        bytes: 3,
        cycles: 5,
    };
    let raw_bytes = [0x6C, 0x00, 0x80];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "JMP $8000");
}

#[test]
fn test_disassemble_indirect_x() {
    let opcode = Opcode {
        code: 0xA1,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::IndirectX,
        bytes: 2,
        cycles: 6,
    };
    let raw_bytes = [0xA1, 0x40];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA ($40,X)");
}

#[test]
fn test_disassemble_indirect_y() {
    let opcode = Opcode {
        code: 0xB1,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::IndirectY,
        bytes: 2,
        cycles: 5,
    };
    let raw_bytes = [0xB1, 0x50];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA ($50),Y");
}

// ============================================================================
// NES Hardware Register Name Tests
// ============================================================================

#[test]
fn test_disassemble_ppu_ctrl() {
    let opcode = Opcode {
        code: 0x8D,
        syntax: OpcodeSyntax::STA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0x8D, 0x00, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "STA PPU_CTRL");
}

#[test]
fn test_disassemble_ppu_mask() {
    let opcode = Opcode {
        code: 0x8D,
        syntax: OpcodeSyntax::STA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0x8D, 0x01, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "STA PPU_MASK");
}

#[test]
fn test_disassemble_ppu_status() {
    let opcode = Opcode {
        code: 0xAD,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0xAD, 0x02, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA PPU_STATUS");
}

#[test]
fn test_disassemble_oam_addr() {
    let opcode = Opcode {
        code: 0x8D,
        syntax: OpcodeSyntax::STA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0x8D, 0x03, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "STA OAM_ADDR");
}

#[test]
fn test_disassemble_oam_data() {
    let opcode = Opcode {
        code: 0x8D,
        syntax: OpcodeSyntax::STA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0x8D, 0x04, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "STA OAM_DATA");
}

#[test]
fn test_disassemble_ppu_scroll() {
    let opcode = Opcode {
        code: 0x8D,
        syntax: OpcodeSyntax::STA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0x8D, 0x05, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "STA PPU_SCROLL");
}

#[test]
fn test_disassemble_ppu_addr() {
    let opcode = Opcode {
        code: 0x8D,
        syntax: OpcodeSyntax::STA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0x8D, 0x06, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "STA PPU_ADDR");
}

#[test]
fn test_disassemble_ppu_data() {
    let opcode = Opcode {
        code: 0x8D,
        syntax: OpcodeSyntax::STA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0x8D, 0x07, 0x20];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "STA PPU_DATA");
}

#[test]
fn test_disassemble_oam_dma() {
    let opcode = Opcode {
        code: 0x8D,
        syntax: OpcodeSyntax::STA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0x8D, 0x14, 0x40];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "STA OAM_DMA");
}

#[test]
fn test_disassemble_joypad1() {
    let opcode = Opcode {
        code: 0xAD,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0xAD, 0x16, 0x40];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA JOYPAD1");
}

#[test]
fn test_disassemble_joypad2() {
    let opcode = Opcode {
        code: 0xAD,
        syntax: OpcodeSyntax::LDA,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 4,
    };
    let raw_bytes = [0xAD, 0x17, 0x40];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "LDA JOYPAD2");
}

// ============================================================================
// Various Instruction Types Tests
// ============================================================================

#[test]
fn test_disassemble_jmp_absolute() {
    let opcode = Opcode {
        code: 0x4C,
        syntax: OpcodeSyntax::JMP,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 3,
    };
    let raw_bytes = [0x4C, 0x00, 0xC0];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "JMP $C000");
}

#[test]
fn test_disassemble_jsr() {
    let opcode = Opcode {
        code: 0x20,
        syntax: OpcodeSyntax::JSR,
        mode: AddressingMode::Absolute,
        bytes: 3,
        cycles: 6,
    };
    let raw_bytes = [0x20, 0x00, 0x90];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "JSR $9000");
}

#[test]
fn test_disassemble_rts() {
    let opcode = Opcode {
        code: 0x60,
        syntax: OpcodeSyntax::RTS,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 6,
    };
    let raw_bytes = [0x60];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "RTS");
}

#[test]
fn test_disassemble_brk() {
    let opcode = Opcode {
        code: 0x00,
        syntax: OpcodeSyntax::BRK,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 7,
    };
    let raw_bytes = [0x00];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "BRK");
}

#[test]
fn test_disassemble_nop() {
    let opcode = Opcode {
        code: 0xEA,
        syntax: OpcodeSyntax::NOP,
        mode: AddressingMode::Implied,
        bytes: 1,
        cycles: 2,
    };
    let raw_bytes = [0xEA];
    let result = disassemble_instruction(&opcode, &raw_bytes, 0x8000);
    assert_eq!(result, "NOP");
}
