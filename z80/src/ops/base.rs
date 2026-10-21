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

        match x {
            0 => match (y, z) {
                (0, 0) => { /* NOP */ }
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
}
