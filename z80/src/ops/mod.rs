use crate::{Z80, Z80Bus};

mod base;

impl Z80 {
    pub(crate) fn read_r<B: Z80Bus>(&mut self, bus: &B, n: u8) -> u8 {
        match n {
            0 => self.registers.b,
            1 => self.registers.c,
            2 => self.registers.d,
            3 => self.registers.e,
            4 => self.registers.h,
            5 => self.registers.l,
            6 => self.read_mem(bus, self.registers.hl()),
            7 => self.registers.a,
            _ => unreachable!("invalid register index {n}"),
        }
    }
    pub(crate) fn write_r<B: Z80Bus>(&mut self, bus: &mut B, n: u8, value: u8) {
        match n {
            0 => self.registers.b = value,
            1 => self.registers.c = value,
            2 => self.registers.d = value,
            3 => self.registers.e = value,
            4 => self.registers.h = value,
            5 => self.registers.l = value,
            6 => self.write_mem(bus, self.registers.hl(), value),
            7 => self.registers.a = value,
            _ => unreachable!("invalid register index {n}"),
        }
    }

    pub(crate) fn unimplemented_opcode(&self, opcode: u8) -> ! {
        unimplemented!(
            "opcode {:#04X} at address {:#06X} not implemented",
            opcode,
            self.registers.pc.wrapping_sub(1)
        )
    }
}
