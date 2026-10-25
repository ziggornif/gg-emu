use crate::registers::{FLAG_H, FLAG_PV, FLAG_S, FLAG_X, FLAG_Y, FLAG_Z};

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

#[cfg(test)]
mod tests {
    use crate::{
        alu::logic_flags,
        registers::{FLAG_H, FLAG_PV, FLAG_S, FLAG_X, FLAG_Y, FLAG_Z},
    };

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
}
