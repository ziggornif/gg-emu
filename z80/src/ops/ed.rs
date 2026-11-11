//! ED-prefixed opcodes: block transfers, 16-bit ADC/SBC, I/O, interrupt modes...

use crate::{Z80, Z80Bus};

impl Z80 {
    pub(crate) fn exec_ed<B: Z80Bus>(&mut self, bus: &mut B, opcode: u8) {
        match opcode {
            _ => self.unimplemented_opcode(Some(0xED), opcode),
        }
    }
}
