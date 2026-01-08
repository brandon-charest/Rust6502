# Klaus Dormann's 6502 Functional Test

This directory contains test ROMs for validating the 6502 emulator implementation.

## 6502 Functional Test

**Source**: [Klaus2m5/6502_65C02_functional_tests](https://github.com/Klaus2m5/6502_65C02_functional_tests)

The functional test binary (`6502_functional_test.bin`) is a comprehensive test suite that validates:
- All 6502 opcodes
- All addressing modes
- Flag behavior
- Edge cases and undocumented behavior

### How It Works

1. The test ROM is loaded into memory at address `$0000`
2. Execution starts at `$0400`
3. Tests run sequentially through all opcodes
4. **Success**: PC reaches `$3469` and loops indefinitely
5. **Failure**: PC gets trapped at a different location

### Running the Test

```bash
# Run the full test (may take a while)
cargo test --test klaus_functional -- --ignored

# Run with trace output (first 1000 instructions)
cargo test --test klaus_functional -- --ignored trace_klaus
```

### Test ROM Download

The test ROM is automatically downloaded when you build. If you need to re-download:

```bash
curl -L -o tests/roms/6502_functional_test.bin \
  https://github.com/Klaus2m5/6502_65C02_functional_tests/raw/master/bin_files/6502_functional_test.bin
```

### Expected Results

- **All opcodes implemented correctly**: Test passes, PC loops at `$3469`
- **Missing opcodes**: Test panics with "unknown opcode" error
- **Incorrect implementation**: Test traps at specific address (see source for trap meanings)

### Debugging Failed Tests

1. Run the trace version to see where it fails:
   ```bash
   cargo test --test klaus_functional -- --ignored trace_klaus 2>&1 | tee trace.log
   ```

2. Check the last PC value before trap
3. Consult the [source assembly](https://github.com/Klaus2m5/6502_65C02_functional_tests/blob/master/6502_functional_test.a65) to identify which test failed

## Other Test ROMs (Future)

- **nestest.nes**: NES-specific test with golden log
- **Tom Harte's ProcessorTests**: Cycle-accurate JSON test suite
