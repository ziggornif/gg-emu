use crate::{Z80, Z80Bus};

impl Z80 {
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
                    self.halted = true;
                    panic!("HALT not implemented (needs interrupts)");
                }
                (_, _) => {
                    /* LD r[y], r[z] */
                    let val = self.read_r(bus, z);
                    self.write_r(bus, y, val);
                }
            },
            _ => self.unimplemented_opcode(opcode),
        }
    }
}
