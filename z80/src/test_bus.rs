//! Minimal bus for unit tests: 64 KiB of flat RAM, no peripherals.

use crate::Z80Bus;

/// 64 KiB of RAM; I/O reads return 0xFF (open bus), I/O writes are ignored.
pub(crate) struct TestBus {
    mem: Vec<u8>,
}

impl TestBus {
    pub fn new() -> Self {
        Self {
            mem: vec![0; 65536],
        }
    }

    pub(crate) fn with_program(program: &[u8]) -> Self {
        let mut bus = Self::new();
        bus.mem[0..program.len()].copy_from_slice(program);
        bus
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
