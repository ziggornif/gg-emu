//! Z80 register file: storage, pair/half accessors, flags and exchanges.
//!
//! Plain state only: no instruction logic lives here.

// TODO: remove this when flags will be used
#![allow(dead_code)]
/// Sign: copy of bit 7 of the result.
pub(crate) const FLAG_S: u8 = 0b1000_0000;
/// Zero: set when the result is 0.
pub(crate) const FLAG_Z: u8 = 0b0100_0000;
/// Undocumented: usually a copy of bit 5 of the result.
pub(crate) const FLAG_Y: u8 = 0b0010_0000;
/// Half-carry: carry from bit 3 to bit 4 (borrow for subtractions). Used by DAA.
pub(crate) const FLAG_H: u8 = 0b0001_0000;
/// Undocumented: usually a copy of bit 3 of the result.
pub(crate) const FLAG_X: u8 = 0b0000_1000;
/// Parity (logical ops, rotations) or signed overflow (arithmetic).
pub(crate) const FLAG_PV: u8 = 0b0000_0100;
/// Subtract: set when the last operation was a subtraction. Used by DAA.
pub(crate) const FLAG_N: u8 = 0b0000_0010;
/// Carry: carry out of bit 7 (borrow for subtractions).
pub(crate) const FLAG_C: u8 = 0b0000_0001;

/// Z80 register file.
///
/// Registers are stored in the shape they are mostly used in:
/// 8-bit registers as `u8` (pairs rebuilt by accessors), address
/// registers as `u16` (halves extracted by accessors).
///
/// Reset values follow the Zilog manual (PC, I, R = 0) and Sean Young's
/// measurements (AF, SP = 0xFFFF). Other registers are undefined on real
/// hardware and set to 0 here for determinism.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Registers {
    /// Accumulator
    pub(crate) a: u8,
    pub(crate) b: u8,
    pub(crate) c: u8,
    pub(crate) d: u8,
    pub(crate) e: u8,
    /// Flags, see `FLAG_*`. All 8 bits are meaningful, including X and Y.
    pub(crate) f: u8,
    pub(crate) h: u8,
    pub(crate) l: u8,
    /// Interrupt vector base (high byte of the IM 2 table address).
    pub(crate) i: u8,
    /// Memory refresh counter. Only the low 7 bits are incremented, see [`Registers::inc_r`].
    pub(crate) r: u8,

    /// Index registers, used as base for `(IX+d)` / `(IY+d)` addressing.
    pub(crate) ix: u16,
    pub(crate) iy: u16,
    /// Stack pointer.
    pub(crate) sp: u16,
    /// Program counter.
    pub(crate) pc: u16,

    /// Alternate set (AF', BC', DE', HL'), stored as pairs since it is only
    /// exchanged with the main set, never accessed byte by byte.
    pub(crate) af_alt: u16,
    pub(crate) bc_alt: u16,
    pub(crate) de_alt: u16,
    pub(crate) hl_alt: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            a: 0xFF,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0xFF,
            h: 0x00,
            l: 0x00,
            i: 0x00,
            r: 0x00,
            ix: 0x0000,
            iy: 0x0000,
            sp: 0xFFFF,
            pc: 0x0000,
            af_alt: 0x0000,
            bc_alt: 0x0000,
            de_alt: 0x0000,
            hl_alt: 0x0000,
        }
    }

    /// Returns `true` if the flag selected by `mask` is set.
    pub(crate) fn flag(&self, mask: u8) -> bool {
        (self.f & mask) != 0
    }

    /// Sets (`true`) or clears (`false`) the flag selected by `mask`,
    /// leaving the other bits of F untouched.
    pub(crate) fn set_flag(&mut self, mask: u8, value: bool) {
        if value {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }
    }

    /// Returns the AF pair (A high byte, F low byte).
    pub(crate) fn af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f as u16)
    }

    /// Writes the AF pair.
    pub(crate) fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = value as u8;
    }

    /// Returns the BC pair (B high byte, C low byte).
    pub(crate) fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    /// Writes the BC pair.
    pub(crate) fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = value as u8;
    }

    /// Returns the DE pair (D high byte, E low byte).
    pub(crate) fn de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    /// Writes the DE pair.
    pub(crate) fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = value as u8;
    }

    /// Returns the HL pair (H high byte, L low byte).
    pub(crate) fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    /// Writes the HL pair.
    pub(crate) fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = value as u8;
    }

    /// High byte of IX (undocumented IXH register).
    pub(crate) fn ixh(&self) -> u8 {
        (self.ix >> 8) as u8
    }

    /// Replaces the high byte of IX, keeping the low byte.
    pub(crate) fn set_ixh(&mut self, value: u8) {
        self.ix = ((value as u16) << 8) | (self.ix & 0x00FF);
    }

    /// Low byte of IX (undocumented IXL register).
    pub(crate) fn ixl(&self) -> u8 {
        self.ix as u8
    }

    /// Replaces the low byte of IX, keeping the high byte.
    pub(crate) fn set_ixl(&mut self, value: u8) {
        self.ix = (value as u16) | (self.ix & 0xFF00);
    }

    /// High byte of IY (undocumented IYH register).
    pub(crate) fn iyh(&self) -> u8 {
        (self.iy >> 8) as u8
    }

    /// Replaces the high byte of IY, keeping the low byte.
    pub(crate) fn set_iyh(&mut self, value: u8) {
        self.iy = ((value as u16) << 8) | (self.iy & 0x00FF);
    }

    /// Low byte of IY (undocumented IYL register).
    pub(crate) fn iyl(&self) -> u8 {
        self.iy as u8
    }

    /// Replaces the low byte of IY, keeping the high byte.
    pub(crate) fn set_iyl(&mut self, value: u8) {
        self.iy = (value as u16) | (self.iy & 0xFF00);
    }

    /// Exchanges AF with AF' (`EX AF,AF'`).
    pub(crate) fn ex_af(&mut self) {
        let tmp_af = self.af();
        self.set_af(self.af_alt);
        self.af_alt = tmp_af;
    }

    /// Exchanges BC, DE and HL with BC', DE' and HL' (`EXX`). AF is not affected.
    pub(crate) fn exx(&mut self) {
        let tmp_bc = self.bc();
        self.set_bc(self.bc_alt);
        self.bc_alt = tmp_bc;

        let tmp_de = self.de();
        self.set_de(self.de_alt);
        self.de_alt = tmp_de;

        let tmp_hl = self.hl();
        self.set_hl(self.hl_alt);
        self.hl_alt = tmp_hl;
    }

    /// Increments the low 7 bits of R, wrapping from 0x7F to 0x00.
    /// Bit 7 is preserved: it only changes through `LD R,A`.
    /// Called once per opcode fetch (M1 cycle).
    pub(crate) fn inc_r(&mut self) {
        let bit7 = self.r & 0x80;
        let low7 = self.r & 0x7F;
        let low7_inc = low7.wrapping_add(1) & 0x7F;
        self.r = bit7 | low7_inc;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values() {
        let registers = Registers::new();
        assert_eq!(registers.pc, 0);
        assert_eq!(registers.i, 0);
        assert_eq!(registers.r, 0);
        assert_eq!(registers.af(), 0xFFFF);
        assert_eq!(registers.sp, 0xFFFF);
    }

    #[test]
    fn bc_pair() {
        let mut registers = Registers::new();
        registers.set_bc(0x1234);
        assert_eq!(registers.b, 0x12);
        assert_eq!(registers.c, 0x34);
        assert_eq!(registers.bc(), 0x1234);
    }

    #[test]
    fn de_pair() {
        let mut registers = Registers::new();
        registers.set_de(0x5678);
        assert_eq!(registers.d, 0x56);
        assert_eq!(registers.e, 0x78);
        assert_eq!(registers.de(), 0x5678);
    }

    #[test]
    fn hl_pair() {
        let mut registers = Registers::new();
        registers.set_hl(0x9ABC);
        assert_eq!(registers.h, 0x9A);
        assert_eq!(registers.l, 0xBC);
        assert_eq!(registers.hl(), 0x9ABC);
    }

    #[test]
    fn af_pair() {
        let mut registers = Registers::new();
        registers.set_af(0x125A);
        assert_eq!(registers.a, 0x12);
        assert_eq!(registers.f, 0x5A);
        assert_eq!(registers.af(), 0x125A);
    }

    #[test]
    fn pairs_are_independent() {
        let mut registers = Registers::new();
        registers.set_bc(0x1234);
        registers.set_de(0x5678);
        registers.set_hl(0x9ABC);
        registers.set_af(0x125A);
        assert_eq!(registers.bc(), 0x1234);
        assert_eq!(registers.de(), 0x5678);
        assert_eq!(registers.hl(), 0x9ABC);
        assert_eq!(registers.af(), 0x125A);
    }

    #[test]
    fn set_ixh() {
        let mut registers = Registers::new();
        registers.ix = 0x1234;
        registers.set_ixh(0x56);
        assert_eq!(registers.ixh(), 0x56);
        assert_eq!(registers.ix, 0x5634);
    }

    #[test]
    fn set_ixl() {
        let mut registers = Registers::new();
        registers.ix = 0x1234;
        registers.set_ixl(0x56);
        assert_eq!(registers.ixl(), 0x56);
        assert_eq!(registers.ix, 0x1256);
    }

    #[test]
    fn set_iyh() {
        let mut registers = Registers::new();
        registers.iy = 0x1234;
        registers.set_iyh(0x56);
        assert_eq!(registers.iyh(), 0x56);
        assert_eq!(registers.iy, 0x5634);
    }

    #[test]
    fn set_iyl() {
        let mut registers = Registers::new();
        registers.iy = 0x1234;
        registers.set_iyl(0x56);
        assert_eq!(registers.iyl(), 0x56);
        assert_eq!(registers.iy, 0x1256);
    }

    #[test]
    fn set_flag_s() {
        let mut registers = Registers::new();
        assert!(registers.flag(FLAG_S));
        registers.set_flag(FLAG_S, false);
        assert!(!registers.flag(FLAG_S));
        assert_eq!(registers.f, 0x7F);
        registers.set_flag(FLAG_S, true);
        assert!(registers.flag(FLAG_S));
        assert_eq!(registers.f, 0xFF);
    }

    #[test]
    fn set_flag_z() {
        let mut registers = Registers::new();
        assert!(registers.flag(FLAG_Z));
        registers.set_flag(FLAG_Z, false);
        assert!(!registers.flag(FLAG_Z));
        assert_eq!(registers.f, 0xBF);
        registers.set_flag(FLAG_Z, true);
        assert!(registers.flag(FLAG_Z));
        assert_eq!(registers.f, 0xFF);
    }

    #[test]
    fn set_flag_y() {
        let mut registers = Registers::new();
        assert!(registers.flag(FLAG_Y));
        registers.set_flag(FLAG_Y, false);
        assert!(!registers.flag(FLAG_Y));
        assert_eq!(registers.f, 0xDF);
        registers.set_flag(FLAG_Y, true);
        assert!(registers.flag(FLAG_Y));
        assert_eq!(registers.f, 0xFF);
    }

    #[test]
    fn set_flag_h() {
        let mut registers = Registers::new();
        assert!(registers.flag(FLAG_H));
        registers.set_flag(FLAG_H, false);
        assert!(!registers.flag(FLAG_H));
        assert_eq!(registers.f, 0xEF);
        registers.set_flag(FLAG_H, true);
        assert!(registers.flag(FLAG_H));
        assert_eq!(registers.f, 0xFF);
    }

    #[test]
    fn set_flag_x() {
        let mut registers = Registers::new();
        assert!(registers.flag(FLAG_X));
        registers.set_flag(FLAG_X, false);
        assert!(!registers.flag(FLAG_X));
        assert_eq!(registers.f, 0xF7);
        registers.set_flag(FLAG_X, true);
        assert!(registers.flag(FLAG_X));
        assert_eq!(registers.f, 0xFF);
    }

    #[test]
    fn set_flag_pv() {
        let mut registers = Registers::new();
        assert!(registers.flag(FLAG_PV));
        registers.set_flag(FLAG_PV, false);
        assert!(!registers.flag(FLAG_PV));
        assert_eq!(registers.f, 0xFB);
        registers.set_flag(FLAG_PV, true);
        assert!(registers.flag(FLAG_PV));
        assert_eq!(registers.f, 0xFF);
    }

    #[test]
    fn set_flag_n() {
        let mut registers = Registers::new();
        assert!(registers.flag(FLAG_N));
        registers.set_flag(FLAG_N, false);
        assert!(!registers.flag(FLAG_N));
        assert_eq!(registers.f, 0xFD);
        registers.set_flag(FLAG_N, true);
        assert!(registers.flag(FLAG_N));
        assert_eq!(registers.f, 0xFF);
    }

    #[test]
    fn set_flag_c() {
        let mut registers = Registers::new();
        assert!(registers.flag(FLAG_C));
        registers.set_flag(FLAG_C, false);
        assert!(!registers.flag(FLAG_C));
        assert_eq!(registers.f, 0xFE);
        registers.set_flag(FLAG_C, true);
        assert!(registers.flag(FLAG_C));
        assert_eq!(registers.f, 0xFF);
    }

    #[test]
    fn ex_af() {
        let mut registers = Registers::new();
        assert_eq!(registers.af(), 0xFFFF);
        registers.af_alt = 0x1234;
        registers.ex_af();
        assert_eq!(registers.af(), 0x1234);
        assert_eq!(registers.af_alt, 0xFFFF);
        registers.ex_af();
        assert_eq!(registers.af(), 0xFFFF);
        assert_eq!(registers.af_alt, 0x1234);
    }

    #[test]
    fn exx() {
        let mut registers = Registers::new();
        registers.set_bc(0x1122);
        registers.set_de(0x3344);
        registers.set_hl(0x5566);

        registers.bc_alt = 0x1234;
        registers.de_alt = 0x5678;
        registers.hl_alt = 0x9ABC;
        registers.exx();

        assert_eq!(registers.bc(), 0x1234);
        assert_eq!(registers.bc_alt, 0x1122);
        assert_eq!(registers.de(), 0x5678);
        assert_eq!(registers.de_alt, 0x3344);
        assert_eq!(registers.hl(), 0x9ABC);
        assert_eq!(registers.hl_alt, 0x5566);
        assert_eq!(registers.af(), 0xFFFF);
        assert_eq!(registers.af_alt, 0x0000);

        registers.exx();

        assert_eq!(registers.bc(), 0x1122);
        assert_eq!(registers.bc_alt, 0x1234);
        assert_eq!(registers.de(), 0x3344);
        assert_eq!(registers.de_alt, 0x5678);
        assert_eq!(registers.hl(), 0x5566);
        assert_eq!(registers.hl_alt, 0x9ABC);
        assert_eq!(registers.af(), 0xFFFF);
        assert_eq!(registers.af_alt, 0x0000);
    }

    #[test]
    fn inc_r() {
        let mut registers = Registers::new();
        registers.r = 0x05;
        registers.inc_r();
        assert_eq!(registers.r, 0x06);

        registers.r = 0x7F;
        registers.inc_r();
        assert_eq!(registers.r, 0x00);

        registers.r = 0xFF;
        registers.inc_r();
        assert_eq!(registers.r, 0x80);
    }
}
