# Rust6502

[![CI](https://github.com/brandon-charest/Rust6502/workflows/Rust%20CI/badge.svg)](https://github.com/brandon-charest/Rust6502/actions)
[![codecov](https://codecov.io/gh/brandon-charest/Rust6502/branch/main/graph/badge.svg)](https://codecov.io/gh/brandon-charest/Rust6502)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-blue.svg)
[![License](https://img.shields.io/github/license/brandon-charest/Rust6502)](LICENSE)
[![Last Commit](https://img.shields.io/github/last-commit/brandon-charest/Rust6502)](https://github.com/brandon-charest/Rust6502/commits/main)

A cycle-accurate 6502 CPU emulator written in Rust, implementing the full instruction set of the MOS Technology 6502 processor.

## Overview

This purpose of this project is to explore and implement an emulator for the 6502 microprocessor, the CPU that powered iconic systems like the Apple II, Commodore 64, and Nintendo Entertainment System (NES). The emulator focuses on accuracy and includes support for all official and unofficial opcodes.

## Features

- **Complete 6502 Instruction Set**: All official and unofficial instructions fully implemented
- **Cycle-Accurate Execution**: Proper timing for each instruction and addressing mode
- **Flexible Memory Bus**: Abstracted bus interface for easy integration with different systems
- **Comprehensive Testing**: Extensive unit tests organized by instruction category
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
make build

# Run tests to verify the build
make test
```

## Usage

### Running the Emulator

The project includes several executable targets:

```bash
# Run the basic emulator example
make run-emulator

# Run the nestest ROM with trace logging
make run-nestest

# Run the disassembler
make run-disasm

# Or simply use (defaults to run-emulator)
make run
```

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
make test
```

### Code Coverage

Generate code coverage reports:

```bash
make coverage
```

## Project Structure

```
rust-6502-emulator/
├── src/
│   ├── lib.rs                    # Library root
│   ├── main.rs                   # Example program runner
│   ├── bin/                      # Binary/test runners
│   │   ├── disasm.rs             # Disassembler tool
│   │   └── nestest.rs            # nestest.rom runner with trace logging
│   └── hardware/
│       ├── mod.rs                # Hardware module exports
│       ├── bus.rs                # Memory bus abstraction
│       ├── registers.rs          # CPU register definitions
│       ├── status.rs             # Status flag implementation
│       ├── opcodes.rs            # Opcode definitions (all 256 opcodes)
│       └── cpu/
│           ├── mod.rs            # CPU core implementation
│           ├── addressing.rs     # Addressing mode logic
│           ├── memory_access.rs  # Memory read/write helpers
│           ├── disassembler.rs   # Instruction disassembly
│           ├── disassembler_tests.rs  # Disassembler tests
│           ├── instructions/     # Instruction implementations
│           │   ├── mod.rs        # Instruction module exports
│           │   ├── arithmetic.rs # ADC, SBC
│           │   ├── logic.rs      # AND, ORA, EOR, BIT
│           │   ├── shift.rs      # ASL, LSR, ROL, ROR
│           │   ├── load.rs       # LDA, LDX, LDY, STA, STX, STY
│           │   ├── compare.rs    # CMP, CPX, CPY
│           │   ├── branch.rs     # BCC, BCS, BEQ, etc.
│           │   ├── transfer.rs   # TAX, TAY, TXA, etc.
│           │   ├── stack.rs      # PHA, PLA, PHP, PLP, JSR, RTS, etc.
│           │   ├── increment.rs  # INC, DEC, INX, etc.
│           │   ├── control.rs    # JMP, JSR, RTS, RTI, BRK
│           │   ├── flags.rs      # CLC, SEC, CLI, etc.
│           │   ├── noop.rs       # NOP
│           │   └── unofficial.rs # Unofficial/undocumented opcodes
│           └── tests/            # Modular unit tests
│               ├── mod.rs        # Test module exports
│               ├── core.rs       # Core CPU tests
│               ├── arithmetic.rs # Arithmetic instruction tests
│               ├── branch.rs     # Branch instruction tests
│               ├── compare.rs    # Compare instruction tests
│               ├── control.rs    # Control flow tests
│               ├── flags.rs      # Flag manipulation tests
│               ├── increment.rs  # Increment/decrement tests
│               ├── load_store.rs # Load/store tests
│               ├── shift.rs      # Shift/rotate tests
│               ├── stack.rs      # Stack operation tests
│               ├── transfer.rs   # Transfer instruction tests
│               └── unofficial.rs # Unofficial opcode tests
├── Cargo.toml
└── README.md
```

## Development

The emulator is organized into clean, modular components:

- **Hardware Layer**: CPU, memory bus, and supporting hardware abstractions
- **Instruction Categories**: Instructions grouped by function for maintainability
- **Type Safety**: Leverages Rust's type system for correctness
- **Testing**: Comprehensive test coverage including real-world test suites

## Roadmap

### CPU Implementation

- [x] All official and unofficial NES 6502 instructions
- [x] Cycle-accurate execution
- [x] All addressing modes
- [x] Comprehensive unit test suite
- [x] Modular instruction organization
- [x] Disassembler with trace logging
- [ ] BCD (Binary-Coded Decimal) mode for ADC/SBC
- [ ] Pass nestest.rom validation suite
- [ ] Pass Klaus Dormann's functional test suite

### NES Emulator Components

- [ ] **PPU (Picture Processing Unit)**
  - [ ] Background rendering
  - [ ] Sprite rendering with sprite 0 hit
  - [ ] Scrolling and nametables
  - [ ] Pattern tables and palettes
  - [ ] Frame timing and NMI generation

- [ ] **APU (Audio Processing Unit)**
  - [ ] Pulse wave channels (2x)
  - [ ] Triangle wave channel
  - [ ] Noise channel
  - [ ] DMC (Delta Modulation Channel)
  - [ ] Audio mixing and output

- [ ] **Memory Mappers**
  - [ ] NROM (Mapper 0)
  - [ ] MMC1 (Mapper 1)
  - [ ] UxROM (Mapper 2)
  - [ ] CNROM (Mapper 3)
  - [ ] Additional mappers as needed

- [ ] **Input & Integration**
  - [ ] Controller interface (standard NES controller)
  - [ ] Cartridge loading (.nes ROM files)
  - [ ] Frontend with rendering and audio
  - [ ] Game loop and proper timing
  - [ ] Save state support

## Resources

- [NES Dev Wiki](https://wiki.nesdev.com/w/index.php/CPU)
- [6502 Instruction Reference](http://www.6502.org/tutorials/6502opcodes.html)
- [6502 Instruction Set](https://www.masswerk.at/6502/6502_instruction_set.html)
- [NES Emulator](https://www.youtube.com/watch?v=F8kx56OZQhg&list=PLrOv9FMX8xJHqMvSGB_9G9nZZ_4IgteYf&index=2)
- [Klaus Dormann's Test Suite](https://github.com/Klaus2m5/6502_65C02_functional_tests)
