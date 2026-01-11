use super::*;

#[test]
fn test_bne_logic() {
    let (mut cpu, mut bus) = setup();

    // $8000: BNE +5 ($05) -> Jumps to $8007
    // $8007: LDA #$01 (Success)
    bus.write(0x8000, 0xD0);
    bus.write(0x8001, 0x05);
    bus.write(0x8007, 0xA9);
    bus.write(0x8008, 0x01);

    cpu.registers.program_counter = 0x8000;

    // Case 1: Zero Flag IS set (Should NOT branch)
    cpu.registers.status.insert(Status::ZERO);
    cpu.step(&mut bus);

    // PC should have advanced 2 bytes (8000 -> 8002)
    assert_eq!(
        cpu.registers.program_counter, 0x8002,
        "Should not branch if Z is set"
    );

    // Zero Flag IS CLEAR (Should branch)
    // Reset PC to test again
    cpu.registers.program_counter = 0x8000;
    cpu.registers.status.remove(Status::ZERO);

    cpu.step(&mut bus); // Execute BNE

    // PC should be $8007
    // Math: $8000 (Opcode) + 1 (Fetch) = $8001. Fetch Offset ($8002).
    // Base PC for math is $8002.
    // $8002 + 5 = $8007.
    assert_eq!(
        cpu.registers.program_counter, 0x8007,
        "Should branch if Z is clear"
    );
}

#[test]
fn test_beq_backward_jump() {
    let (mut cpu, mut bus) = setup();

    // Jump BACKWARDS
    // $8005: BEQ -3 ($FD) -> Target $8004
    bus.write(0x8005, 0xF0);
    bus.write(0x8006, 0xFD); // -3 in signed 8-bit

    cpu.registers.program_counter = 0x8005;
    cpu.registers.status.insert(Status::ZERO); // Force branch

    cpu.step(&mut bus);

    // Opcode read at $8005. PC -> $8006.
    // Offset read at $8006. PC -> $8007.
    // Base PC: $8007.
    // Offset: -3.
    // Target: $8007 - 3 = $8004.
    assert_eq!(
        cpu.registers.program_counter, 0x8004,
        "Backward jump failed"
    );
}

#[test]
fn test_bcc_bcs_logic() {
    let (mut cpu, mut bus) = setup();

    // 1. BCC +5 (Branch if Carry Clear)
    bus.write(0x8000, 0x90);
    bus.write(0x8001, 0x05);

    // 2. BCS +10 (Branch if Carry Set)
    bus.write(0x8002, 0xB0);
    bus.write(0x8003, 0x0A);

    cpu.registers.program_counter = 0x8000;

    // Case 1: Carry is Set (Should NOT branch on BCC)
    cpu.registers.status.insert(Status::CARRY);
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.program_counter, 0x8002); // Fall through

    // Case 2: Carry is Set (Should branch on BCS)
    // PC is now at 8002. BCS + 0x0A. Target: 8004 + A = 800E.
    cpu.step(&mut bus);
    assert_eq!(cpu.registers.program_counter, 0x800E);
}
