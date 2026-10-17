// TODO
#![allow(dead_code)]

use crate::{Z80Bus, registers::Registers};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Im {
    Im0,
    Im1,
    Im2,
}

#[derive(Debug)]
pub struct Z80 {
    registers: Registers,
    iff1: bool,
    iff2: bool,
    halted: bool,
    im: Im,
}

impl Default for Z80 {
    fn default() -> Self {
        Self::new()
    }
}

impl Z80 {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            iff1: false,
            iff2: false,
            halted: false,
            im: Im::Im0,
        }
    }

    pub fn step<B: Z80Bus>(&mut self, bus: &mut B) -> u8 {
        let opcode = self.fetch_opcode(bus);
        match opcode {
            0x00 => 4,
            _ => unimplemented!(
                "opcode {:#04X} at address {:#06X} not implemented",
                opcode,
                self.registers.pc.wrapping_sub(1)
            ),
        }
    }

    pub(crate) fn fetch_opcode<B: Z80Bus>(&mut self, bus: &mut B) -> u8 {
        let opcode = self.fetch_byte(bus);
        self.registers.inc_r();
        opcode
    }

    pub(crate) fn fetch_byte<B: Z80Bus>(&mut self, bus: &mut B) -> u8 {
        let val = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }
}

#[cfg(test)]
mod tests {
    use crate::Z80Bus;

    struct TestBus {
        mem: Vec<u8>,
    }

    impl TestBus {
        pub fn new() -> Self {
            Self {
                mem: vec![0; 65536],
            }
        }
    }

    impl Z80Bus for TestBus {
        fn read(&self, address: u16) -> u8 {
            self.mem[usize::from(address)]
        }

        fn write(&mut self, address: u16, value: u8) {
            self.mem[usize::from(address)] = value;
        }

        fn io_read(&mut self, _port: u16) -> u8 {
            0xFF
        }

        fn io_write(&mut self, _port: u16, _value: u8) {}
    }
}
