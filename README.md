# Rust6502

[![CI](https://github.com/brandon-charest/Rust6502/workflows/Rust%20CI/badge.svg)](https://github.com/brandon-charest/Rust6502/actions)
[![codecov](https://codecov.io/gh/brandon-charest/Rust6502/branch/main/graph/badge.svg)](https://codecov.io/gh/brandon-charest/Rust6502)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)
[![License](https://img.shields.io/github/license/brandon-charest/Rust6502)](LICENSE)
[![Last Commit](https://img.shields.io/github/last-commit/brandon-charest/Rust6502)](https://github.com/brandon-charest/Rust6502/commits/main)

A cycle-accurate 6502 CPU emulator written in Rust, implementing the full instruction set of the MOS Technology 6502 processor.

## Overview

This project is a comprehensive emulator for the 6502 microprocessor, the CPU that powered iconic systems like the Apple II, Commodore 64, and Nintendo Entertainment System (NES). The emulator focuses on accuracy and includes support for all official opcodes, including Binary-Coded Decimal (BCD) arithmetic mode.

## Features

- **Complete 6502 Instruction Set**: All 56 official instructions fully implemented
- **Cycle-Accurate Execution**: Proper timing for each instruction and addressing mode
- **BCD Arithmetic Support**: Accurate decimal mode operations for ADC and SBC
- **Flexible Memory Bus**: Abstracted bus interface for easy integration with different systems
- **Comprehensive Testing**: Includes Klaus Dormann's functional test suite for validation
- **Clean Architecture**: Modular organization with separate modules for different instruction categories

### Implemented Components

- **CPU Core**: Full register set (A, X, Y, PC, SP) and status flags
- **Status Register**: All processor flags (N, V, B, D, I, Z, C) using Rust's `bitflags` crate
- **Memory Bus**: Abstract bus trait with simple memory implementation
- **Addressing Modes**: All 13 addressing modes (Immediate, Zero Page, Absolute, Indexed, Indirect, etc.)
- **Instructions**:
  - Load/Store operations (LDA, LDX, LDY, STA, STX, STY)
  - Arithmetic (ADC, SBC)
  - Logic (AND, ORA, EOR, BIT)
  - Shifts and Rotates (ASL, LSR, ROL, ROR)
  - Increments/Decrements (INC, DEC, INX, INY, DEX, DEY)
  - Transfers (TAX, TAY, TXA, TYA, TSX, TXS)
  - Stack operations (PHA, PLA, PHP, PLP)
  - Comparisons (CMP, CPX, CPY)
  - Branches (BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS)
  - Jumps and Calls (JMP, JSR, RTS, RTI)
  - Flag manipulation (CLC, CLD, CLI, CLV, SEC, SED, SEI)
  - System (BRK, NOP)

## Installation

### Prerequisites

- Rust toolchain (1.70.0 or later recommended)
- Cargo package manager

### Building from Source

```bash
# Clone the repository
git clone https://github.com/brandon-charest/rust-6502-emulator.git
cd rust-6502-emulator

# Build the project
cargo build --release

# Run tests to verify the build
cargo test
```

## Usage

### Running the Basic Example

The project includes a simple demonstration program:

```bash
cargo run
```

This runs a small program that demonstrates:
- Loading immediate values into the accumulator
- Executing a JMP instruction
- Display of CPU state during execution

### Library Usage

You can use this emulator as a library in your own projects:

```rust
use rust_6502_emulator::hardware::{
    cpu::CPU,
    bus::{Bus, Memory},
};

fn main() {
    let mut cpu = CPU::new();
    let mut bus = Memory::new();
    
    // Load a program at 0x8000
    bus.write(0x8000, 0xA9); // LDA #$42
    bus.write(0x8001, 0x42);
    
    // Set reset vector to point to our program
    bus.write(0xFFFC, 0x00);
    bus.write(0xFFFD, 0x80);
    
    // Reset and run
    cpu.reset(&mut bus);
    cpu.step(&mut bus);
}
```

## Testing

### Running Unit Tests

The project includes extensive unit tests for all instructions:

```bash
cargo test
```

## Project Structure

```
rust-6502-emulator/
├── src/
│   ├── lib.rs              # Library root
│   ├── main.rs             # Example program runner
│   ├── bin/                # Binary/test runners
│   └── hardware/
│       ├── mod.rs          # Hardware module exports
│       ├── bus.rs          # Memory bus abstraction
│       ├── registers.rs    # CPU register definitions
│       ├── status.rs       # Status flag implementation
│       ├── opcodes.rs      # Opcode definitions (all 256 opcodes)
│       └── cpu/
│           ├── mod.rs      # CPU core implementation
│           ├── tests.rs    # CPU unit tests
│           └── instructions/
│               ├── arithmetic.rs   # ADC, SBC
│               ├── logic.rs        # AND, ORA, EOR, BIT
│               ├── shift.rs        # ASL, LSR, ROL, ROR
│               ├── load.rs         # LDA, LDX, LDY
│               ├── compare.rs      # CMP, CPX, CPY
│               ├── branch.rs       # BCC, BCS, BEQ, etc.
│               ├── transfer.rs     # TAX, TAY, TXA, etc.
│               ├── stack.rs        # PHA, PLA, PHP, PLP
│               ├── increment.rs    # INC, DEC, INX, etc.
│               ├── control.rs      # JMP, JSR, RTS, RTI, BRK
│               ├── flags.rs        # CLC, SEC, CLI, etc.
│               └── noop.rs         # NOP
├── Cargo.toml
└── README.md
```

## Development

The emulator is organized into clean, modular components:

- **Hardware Layer**: CPU, memory bus, and supporting hardware abstractions
- **Instruction Categories**: Instructions grouped by function for maintainability
- **Type Safety**: Leverages Rust's type system for correctness
- **Testing**: Comprehensive test coverage including real-world test suites

## Next Steps

The roadmap for this project includes the following milestones:

### Short Term: CPU Validation

- **nestest.rom Validation**: Run and pass the comprehensive nestest.rom test suite to ensure full CPU accuracy
- **Trace Logging**: Implement detailed execution trace matching nestest.log format for debugging
- **Cycle Accuracy**: Fine-tune instruction timing to match hardware behavior exactly

### Long Term: Full NES Emulation

Once the CPU is fully validated, expand into a complete NES emulator by implementing:

- **PPU (Picture Processing Unit)**
  - Background rendering
  - Sprite rendering
  - Scrolling and nametables
  - Pattern tables and palettes
  
- **APU (Audio Processing Unit)**
  - Pulse wave channels (2x)
  - Triangle wave channel
  - Noise channel
  - DMC (Delta Modulation Channel)
  
- **Memory Mapper Support**
  - NROM (Mapper 0)
  - MMC1 (Mapper 1)
  - Additional mappers as needed
  
- **Input Handling**
  - Controller interface
  - Input polling mechanism
  
- **Integration**
  - Cartridge loading (.nes ROM files)
  - Rendering pipeline
  - Audio output
  - Game loop and timing

## Resources

- [NES Dev Wiki](https://wiki.nesdev.com/w/index.php/CPU)
- [6502 Instruction Reference](http://www.6502.org/tutorials/6502opcodes.html)
- [6502 Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html)
- [NES Emulator](https://www.youtube.com/watch?v=F8kx56OZQhg&list=PLrOv9FMX8xJHqMvSGB_9G9nZZ_4IgteYf&index=2)
- [Klaus Dormann's Test Suite](https://github.com/Klaus2m5/6502_65C02_functional_tests)

