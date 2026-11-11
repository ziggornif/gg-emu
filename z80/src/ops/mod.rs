//! Instruction execution, split by opcode prefix, and helpers shared by all prefixes.

use crate::{
    Z80, Z80Bus,
    registers::{FLAG_C, FLAG_PV, FLAG_S, FLAG_Z},
};

mod base;
mod ed;

impl Z80 {
    /// Reads the operand at index `n` of the r[] table: B, C, D, E, H, L, (HL), A.
    ///
    /// Index 6 reads memory at the address in HL (one memory cycle, 3 T-states).
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

    /// Writes `value` to the operand at index `n` of the r[] table.
    ///
    /// Index 6 writes memory at the address in HL (one memory cycle, 3 T-states).
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

    /// Reads the register pair at index `p` of the rp[] table: BC, DE, HL, SP.
    pub(crate) fn read_rp(&self, p: u8) -> u16 {
        match p {
            0 => self.registers.bc(),
            1 => self.registers.de(),
            2 => self.registers.hl(),
            3 => self.registers.sp,
            _ => unreachable!("invalid register pair index {p}"),
        }
    }

    /// Writes `value` to the register pair at index `p` of the rp[] table: BC, DE, HL, SP.
    pub(crate) fn write_rp(&mut self, p: u8, value: u16) {
        match p {
            0 => self.registers.set_bc(value),
            1 => self.registers.set_de(value),
            2 => self.registers.set_hl(value),
            3 => self.registers.sp = value,
            _ => unreachable!("invalid register pair index {p}"),
        }
    }

    /// Reads the register pair at index `p` of the rp2[] table: BC, DE, HL, AF.
    pub(crate) fn read_rp2(&self, p: u8) -> u16 {
        match p {
            0 => self.registers.bc(),
            1 => self.registers.de(),
            2 => self.registers.hl(),
            3 => self.registers.af(),
            _ => unreachable!("invalid register pair index {p}"),
        }
    }

    /// Writes `value` to the register pair at index `p` of the rp2[] table: BC, DE, HL, AF.
    pub(crate) fn write_rp2(&mut self, p: u8, value: u16) {
        match p {
            0 => self.registers.set_bc(value),
            1 => self.registers.set_de(value),
            2 => self.registers.set_hl(value),
            3 => self.registers.set_af(value),
            _ => unreachable!("invalid register pair index {p}"),
        }
    }

    /// Evaluates condition cc[y] against the current flags.
    ///
    /// Pairs share a flag (NZ/Z, NC/C, PO/PE, P/M): even y tests for 0, odd y for 1.
    pub(crate) fn check_cc(&self, y: u8) -> bool {
        match y {
            0 => !self.registers.flag(FLAG_Z),
            1 => self.registers.flag(FLAG_Z),
            2 => !self.registers.flag(FLAG_C),
            3 => self.registers.flag(FLAG_C),
            4 => !self.registers.flag(FLAG_PV),
            5 => self.registers.flag(FLAG_PV),
            6 => !self.registers.flag(FLAG_S),
            7 => self.registers.flag(FLAG_S),
            _ => unreachable!("invalid condition index {y}"),
        }
    }

    /// Panics on an opcode that is not implemented yet, with the opcode and its address.
    ///
    /// The address is PC − 1 for an unprefixed opcode, PC − 2 for a prefixed one
    /// (not valid for DD CB / FD CB, where the opcode comes after a displacement).
    pub(crate) fn unimplemented_opcode(&self, prefix: Option<u8>, opcode: u8) -> ! {
        match prefix {
            Some(prefix) => unimplemented!(
                "opcode {:#04X} {:#04X} at address {:#06X} not implemented",
                prefix,
                opcode,
                self.registers.pc.wrapping_sub(2)
            ),
            None => unimplemented!(
                "opcode {:#04X} at address {:#06X} not implemented",
                opcode,
                self.registers.pc.wrapping_sub(1)
            ),
        }
    }
}

/// Opcode split into its x/y/z/p/q fields (see z80.info/decoding.htm).
#[derive(Debug, Clone, Copy)]
struct OpcodeFields {
    x: u8,
    y: u8,
    z: u8,
    p: u8,
    q: u8,
}

impl OpcodeFields {
    fn decode(opcode: u8) -> Self {
        let x = opcode >> 6;
        let y = (opcode >> 3) & 0x07;
        let z = opcode & 0x07;
        let p = y >> 1;
        let q = y & 1;

        Self { x, y, z, p, q }
    }
}
