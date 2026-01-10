# Makefile for rust-6502-emulator

.PHONY: help build test coverage clean run run-emulator run-nestest run-disasm

# Default target - show help
help:
	@echo "Available targets:"
	@echo "  make build        - Build the project"
	@echo "  make test         - Run all tests"
	@echo "  make coverage     - Run tests with coverage (excludes bin/main)"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make run-emulator - Run the main emulator"
	@echo "  make run-nestest  - Run the nestest binary"
	@echo "  make run-disasm   - Run the disassembler"

# Build the project
build:
	cargo build --release

# Run all tests
test:
	cargo test --all-features

# Run tests with code coverage, excluding application entry points
coverage:
	cargo llvm-cov --all-features --ignore-filename-regex '(bin/|main\.rs)'

# Clean build artifacts
clean:
	cargo clean

# Run the main emulator (default)
run: run-emulator

# Run specific binaries
run-emulator:
	cargo run --release --bin rust-6502-emulator

run-nestest:
	cargo run --release --bin nestest

run-disasm:
	cargo run --release --bin disasm
