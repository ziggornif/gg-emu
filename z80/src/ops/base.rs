//! Unprefixed opcodes, decoded by fields: x (bits 7-6), y (5-3), z (2-0).
//!
//! See "Decoding Z80 Opcodes" (z80.info/decoding.htm) for the full table.

use crate::{Z80, Z80Bus};

impl Z80 {
    /// Executes an unprefixed opcode, already fetched by `step()`.
    pub(crate) fn exec_base<B: Z80Bus>(&mut self, bus: &mut B, opcode: u8) {
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x07;
        let z = opcode & 0x07;
        let p = y >> 1;
        let q = y & 1;

        match x {
            0 => match (y, z) {
                (0, 0) => { /* NOP */ }
                (3, 0) => {
                    // JR d
                    let offset = i16::from(self.fetch_byte(bus).cast_signed());
                    self.registers.pc = self.registers.pc.wrapping_add_signed(offset);

                    // internal: PC + d computation
                    self.cycles += 5;
                }
                (_, 1) if q == 0 => {
                    // LD rp[p], nn
                    let val = self.fetch_word(bus);
                    self.write_rp(p, val);
                }
                (_, 6) => {
                    // LD r[y], n
                    let val = self.fetch_byte(bus);
                    self.write_r(bus, y, val);
                }
                _ => self.unimplemented_opcode(opcode),
            },
            1 => match (y, z) {
                (6, 6) => {
                    /* HALT */
                    panic!("HALT not implemented (needs interrupts)");
                }
                _ => {
                    /* LD r[y], r[z] */
                    let val = self.read_r(bus, z);
                    self.write_r(bus, y, val);
                }
            },
            3 => match (y, z) {
                (0, 3) => {
                    // JP nn
                    let dest = self.fetch_word(bus);
                    self.registers.pc = dest;
                }
                _ => self.unimplemented_opcode(opcode),
            },
            _ => self.unimplemented_opcode(opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Z80;
    use crate::Z80Bus;
    use crate::test_bus::TestBus;

    #[test]
    fn ld_a_b() {
        let mut bus = TestBus::new();
        let mut cpu = Z80::new();
        bus.write(cpu.registers.pc, 0x78);
        cpu.registers.b = 0x42;

        assert_eq!(cpu.step(&mut bus), 4);
        assert_eq!(cpu.registers.a, 0x42);
    }

    #[test]
    fn ld_a_hl() {
        let mut bus = TestBus::new();
        let mut cpu = Z80::new();
        bus.write(cpu.registers.pc, 0x7E);
        cpu.registers.set_hl(0x8000);
        bus.write(cpu.registers.hl(), 0x5A);

        assert_eq!(cpu.step(&mut bus), 7);
        assert_eq!(cpu.registers.a, 0x5A);
    }

    #[test]
    fn ld_hl_b() {
        let mut bus = TestBus::new();
        let mut cpu = Z80::new();
        bus.write(cpu.registers.pc, 0x70);
        cpu.registers.set_hl(0x8000);
        cpu.registers.b = 0x42;

        assert_eq!(cpu.step(&mut bus), 7);
        assert_eq!(bus.read(cpu.registers.hl()), 0x42);
    }

    #[test]
    fn ld_b_n() {
        let mut bus = TestBus::with_program(&[0x06, 0x42]);
        let mut cpu = Z80::new();

        assert_eq!(cpu.step(&mut bus), 7);
        assert_eq!(cpu.registers.b, 0x42);
        assert_eq!(cpu.registers.pc, 2);
    }

    #[test]
    fn ld_a_n() {
        let mut bus = TestBus::with_program(&[0x3E, 0x5A]);
        let mut cpu = Z80::new();

        assert_eq!(cpu.step(&mut bus), 7);
        assert_eq!(cpu.registers.a, 0x5A);
        assert_eq!(cpu.registers.pc, 2);
    }

    #[test]
    fn ld_hl_n() {
        let mut bus = TestBus::with_program(&[0x36, 0x77]);
        let mut cpu = Z80::new();
        cpu.registers.set_hl(0x8000);

        assert_eq!(cpu.step(&mut bus), 10);
        assert_eq!(bus.read(cpu.registers.hl()), 0x77);
        assert_eq!(cpu.registers.pc, 2);
    }

    #[test]
    fn ld_hl_nn() {
        let mut bus = TestBus::with_program(&[0x21, 0x00, 0x80]);
        let mut cpu = Z80::new();

        assert_eq!(cpu.step(&mut bus), 10);
        assert_eq!(cpu.registers.hl(), 0x8000);
        assert_eq!(cpu.registers.pc, 3);
    }

    #[test]
    fn ld_sp_nn() {
        let mut bus = TestBus::with_program(&[0x31, 0x34, 0x12]);
        let mut cpu = Z80::new();

        assert_eq!(cpu.step(&mut bus), 10);
        assert_eq!(cpu.registers.sp, 0x1234);
        assert_eq!(cpu.registers.pc, 3);
    }

    #[test]
    fn jp_nn() {
        let mut bus = TestBus::with_program(&[0xC3, 0x34, 0x12]);
        let mut cpu = Z80::new();

        assert_eq!(cpu.step(&mut bus), 10);
        assert_eq!(cpu.registers.pc, 0x1234);
    }

    #[test]
    fn jr_forward() {
        let mut bus = TestBus::with_program(&[0x18, 0x05]);
        let mut cpu = Z80::new();

        assert_eq!(cpu.step(&mut bus), 12);
        assert_eq!(cpu.registers.pc, 0x0007);
    }

    #[test]
    fn jr_backward() {
        let mut bus = TestBus::with_program(&[0x18, 0xFE]);
        let mut cpu = Z80::new();

        assert_eq!(cpu.step(&mut bus), 12);
        // 0x0000 is also the start value: without the jump, PC would be 0x0002
        assert_eq!(cpu.registers.pc, 0x0000);
    }
}
