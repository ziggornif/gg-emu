use crate::Z80Bus;

pub(crate) struct TestBus {
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
