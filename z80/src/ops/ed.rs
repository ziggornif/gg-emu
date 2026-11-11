//! ED-prefixed opcodes: block transfers, 16-bit ADC/SBC, I/O, interrupt modes...

use crate::{
    Z80, Z80Bus,
    ops::OpcodeFields,
    registers::{FLAG_H, FLAG_N, FLAG_PV},
};

impl Z80 {
    pub(crate) fn exec_ed<B: Z80Bus>(&mut self, bus: &mut B, opcode: u8) {
        let OpcodeFields { x, y, z, .. } = OpcodeFields::decode(opcode);

        match x {
            2 => match (y, z) {
                (6, 0) => {
                    // LDIR
                    let val = self.read_mem(bus, self.registers.hl());
                    self.write_mem(bus, self.registers.de(), val);

                    self.registers.set_hl(self.registers.hl().wrapping_add(1));
                    self.registers.set_de(self.registers.de().wrapping_add(1));
                    self.registers.set_bc(self.registers.bc().wrapping_sub(1));

                    // internal: HL/DE/BC update
                    self.cycles += 2;

                    self.registers.set_flag(FLAG_H, false);
                    self.registers.set_flag(FLAG_N, false);

                    if self.registers.bc() != 0 {
                        self.registers.pc = self.registers.pc.wrapping_sub(2);
                        self.registers.set_flag(FLAG_PV, true);

                        // repeat: PC reload
                        self.cycles += 5;
                    } else {
                        self.registers.set_flag(FLAG_PV, false);
                    }
                }
                _ => self.unimplemented_opcode(Some(0xED), opcode),
            },
            _ => self.unimplemented_opcode(Some(0xED), opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Z80;
    use crate::Z80Bus;
    use crate::test_bus::TestBus;

    #[test]
    fn ldir() {
        let mut bus = TestBus::with_program(&[0xED, 0xB0]);
        let mut cpu = Z80::new();

        cpu.registers.set_hl(0x1000);
        cpu.registers.set_de(0x2000);
        cpu.registers.set_bc(0x0002);
        cpu.registers.f = 0xFF;

        bus.write(0x1000, 0xAB);
        bus.write(0x1001, 0xCD);
        bus.write(0x2000, 0x12);
        bus.write(0x2001, 0x34);

        // first iter
        assert_eq!(cpu.step(&mut bus), 21);
        assert_eq!(cpu.registers.hl(), 0x1001);
        assert_eq!(cpu.registers.de(), 0x2001);
        assert_eq!(cpu.registers.bc(), 0x0001);
        assert_eq!(cpu.registers.f, 0xED);
        assert_eq!(bus.read(0x2000), 0xAB);
        assert_eq!(bus.read(0x2001), 0x34);
        assert_eq!(cpu.registers.pc, 0x0000);

        // second iter
        assert_eq!(cpu.step(&mut bus), 16);
        assert_eq!(cpu.registers.hl(), 0x1002);
        assert_eq!(cpu.registers.de(), 0x2002);
        assert_eq!(cpu.registers.bc(), 0x0000);
        assert_eq!(cpu.registers.f, 0xE9);
        assert_eq!(bus.read(0x2000), 0xAB);
        assert_eq!(bus.read(0x2001), 0xCD);
        assert_eq!(cpu.registers.pc, 0x0002);
    }
}
