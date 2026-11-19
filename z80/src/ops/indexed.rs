//! DD/FD-prefixed opcodes: HL replaced by IX/IY, (HL) by (IX+d)/(IY+d).

use crate::{
    Z80, Z80Bus,
    ops::{Index, OpcodeFields},
};

impl Z80 {
    /// Reads IX or IY, as selected by the DD/FD prefix.
    // TODO: remove once PUSH IX/IY uses it
    #[allow(dead_code)]
    fn read_index(&self, index: Index) -> u16 {
        match index {
            Index::Ix => self.registers.ix,
            Index::Iy => self.registers.iy,
        }
    }

    /// Writes IX or IY, as selected by the DD/FD prefix.
    fn write_index(&mut self, index: Index, value: u16) {
        match index {
            Index::Ix => self.registers.ix = value,
            Index::Iy => self.registers.iy = value,
        }
    }

    /// Executes a DD/FD-prefixed opcode, already fetched after the prefix.
    pub(super) fn exec_indexed<B: Z80Bus>(&mut self, bus: &mut B, opcode: u8, index: Index) {
        let OpcodeFields { x, y, z, .. } = OpcodeFields::decode(opcode);

        match x {
            3 => match (y, z) {
                (4, 1) => {
                    // POP IX/IY
                    let val = self.pop_word(bus);
                    self.write_index(index, val);
                }
                _ => self.unimplemented_opcode(Some(index.prefix()), opcode),
            },
            _ => self.unimplemented_opcode(Some(index.prefix()), opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Z80;
    use crate::Z80Bus;
    use crate::test_bus::TestBus;

    #[test]
    fn pop_ix() {
        let mut bus = TestBus::with_program(&[0xDD, 0xE1]);
        let mut cpu = Z80::new();
        cpu.registers.sp = 0x8000;
        bus.write(0x8000, 0xCD);
        bus.write(0x8001, 0xAB);

        assert_eq!(cpu.step(&mut bus), 14);
        assert_eq!(cpu.registers.ix, 0xABCD);
        assert_eq!(cpu.registers.iy, 0x0000);
        assert_eq!(cpu.registers.pc, 0x0002);
        assert_eq!(cpu.registers.sp, 0x8002);
    }

    #[test]
    fn pop_iy() {
        let mut bus = TestBus::with_program(&[0xFD, 0xE1]);
        let mut cpu = Z80::new();
        cpu.registers.sp = 0x8000;
        bus.write(0x8000, 0xCD);
        bus.write(0x8001, 0xAB);

        assert_eq!(cpu.step(&mut bus), 14);
        assert_eq!(cpu.registers.iy, 0xABCD);
        assert_eq!(cpu.registers.ix, 0x0000);
        assert_eq!(cpu.registers.pc, 0x0002);
        assert_eq!(cpu.registers.sp, 0x8002);
    }
}
