use crate::registers::{FLAG_C, FLAG_H, FLAG_PV, FLAG_S, FLAG_X, FLAG_Y, FLAG_Z};

/// Flags after a logical operation (AND, XOR, OR) producing `result`.
///
/// S, Y, X copy bits 7, 5, 3 of the result; Z if the result is 0;
/// P/V is the parity (set when the number of 1 bits is even);
/// N and C are always reset. H is set by AND only.
#[must_use]
pub(crate) fn logic_flags(result: u8, half_carry: bool) -> u8 {
    let sign_xy = result & (FLAG_S | FLAG_Y | FLAG_X);
    let zero = if result == 0 { FLAG_Z } else { 0 };
    let half = if half_carry { FLAG_H } else { 0 };
    let parity = if result.count_ones().is_multiple_of(2) {
        FLAG_PV
    } else {
        0
    };

    sign_xy | zero | half | parity
}

/// 16-bit addition (ADD HL/IX/IY, rr): returns `(result, flags)`.
///
/// S, Z and P/V are preserved from `flags`; H is the carry from bit 11,
/// C the carry from bit 15, N is reset; Y and X copy bits 13 and 11 of the result.
#[must_use]
pub(crate) fn add16(lhs: u16, rhs: u16, flags: u8) -> (u16, u8) {
    let (result, overflow) = lhs.overflowing_add(rhs);
    let f = flags & (FLAG_S | FLAG_Z | FLAG_PV);
    let carry = if overflow { FLAG_C } else { 0 };
    let half = if ((lhs & 0x0FFF) + (rhs & 0x0FFF)) & 0x1000 != 0 {
        FLAG_H
    } else {
        0
    };
    let yx = ((result >> 8) as u8) & (FLAG_Y | FLAG_X);

    (result, f | yx | half | carry)
}

/// RLCA: rotates `a` left, bit 7 goes to both bit 0 and C. Returns `(result, flags)`.
///
/// S, Z and P/V are preserved from `flags`; H and N are reset; Y and X copy bits 5 and 3 of the result.
#[must_use]
pub(crate) fn rlca(a: u8, flags: u8) -> (u8, u8) {
    let f = flags & (FLAG_S | FLAG_Z | FLAG_PV);
    let result = a.rotate_left(1);
    let carry = a >> 7;
    let yx = result & (FLAG_Y | FLAG_X);

    (result, f | yx | carry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_result() {
        assert_eq!(logic_flags(0x00, false), FLAG_Z | FLAG_PV);
    }

    #[test]
    fn sign_xy_and_half_carry() {
        assert_eq!(logic_flags(0xA8, true), FLAG_S | FLAG_Y | FLAG_X | FLAG_H);
    }

    #[test]
    fn odd_parity() {
        assert_eq!(logic_flags(0x01, false), 0);
    }

    #[test]
    fn even_parity() {
        assert_eq!(logic_flags(0x03, false), FLAG_PV);
    }

    #[test]
    fn add16_yx() {
        assert_eq!(add16(0x2000, 0x0800, 0), (0x2800, FLAG_Y | FLAG_X));
    }

    #[test]
    fn add16_half_carry() {
        assert_eq!(add16(0x0FFF, 0x0001, 0), (0x1000, FLAG_H));
    }

    #[test]
    fn add16_carry() {
        assert_eq!(add16(0x8000, 0x8000, 0), (0x0000, FLAG_C));
    }

    #[test]
    fn add16_preserves_szpv() {
        assert_eq!(
            add16(0x0001, 0x0002, 0xFF),
            (0x0003, FLAG_S | FLAG_Z | FLAG_PV)
        );
    }

    #[test]
    fn rlca_carry() {
        assert_eq!(rlca(0x9C, 0xFF), (0x39, 0xED));
    }

    #[test]
    fn rlca_no_carry() {
        assert_eq!(rlca(0x6C, 0xFF), (0xD8, 0xCC));
    }
}
