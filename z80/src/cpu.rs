//! Z80 CPU core: execution state and the fetch/execute loop.
//!
//! The CPU only talks to the outside world through [`Z80Bus`].

// TODO: remove once interrupts use iff1/iff2/halted/im
#![allow(dead_code)]

use crate::{Z80Bus, registers::Registers};

/// Maskable interrupt mode, set by the `IM 0/1/2` instructions.
///
/// Says how an INT is serviced, not whether it is accepted (that is IFF1).
#[derive(Debug, Clone, Copy, PartialEq)]
enum Im {
    /// Executes the instruction placed on the data bus by the device. Mode at reset.
    Im0,
    /// Jumps to 0x0038 (`RST 38h`). The mode used on Game Gear.
    Im1,
    /// Jumps to the address read at `(I << 8) | data bus byte`.
    Im2,
}

/// Zilog Z80 CPU.
///
/// Holds the register file and the execution state. Memory, I/O and
/// interrupt lines are provided at each step by a [`Z80Bus`] implementation.
#[derive(Debug)]
pub struct Z80 {
    /// CPU registers.
    pub(crate) registers: Registers,
    /// Maskable interrupts enabled right now.
    iff1: bool,
    /// Copy of IFF1 saved when an NMI clears it, restored by `RETN`.
    iff2: bool,
    /// Set by `HALT`: the CPU waits for an interrupt without advancing PC.
    pub(crate) halted: bool,
    /// Current maskable interrupt mode.
    im: Im,
    /// T-states spent by the current instruction. Reset by `step()`, returned at its end.
    pub(crate) cycles: u8,
}

impl Default for Z80 {
    fn default() -> Self {
        Self::new()
    }
}

impl Z80 {
    /// Returns a CPU in its power-on state: PC = 0, interrupts disabled, IM 0.
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            iff1: false,
            iff2: false,
            halted: false,
            im: Im::Im0,
            cycles: 0,
        }
    }

    pub fn c(&self) -> u8 {
        self.registers.c
    }

    pub fn e(&self) -> u8 {
        self.registers.e
    }

    pub fn pc(&self) -> u16 {
        self.registers.pc
    }

    pub fn de(&self) -> u16 {
        self.registers.de()
    }

    /// Executes one instruction and returns the number of T-states it took.
    ///
    /// The caller uses this count to advance the other components (VDP, PSG)
    /// and to keep real-time speed.
    ///
    /// # Panics
    ///
    /// Panics on an opcode that is not implemented yet, with the opcode and
    /// its address in the message.
    pub fn step<B: Z80Bus>(&mut self, bus: &mut B) -> u8 {
        self.cycles = 0;
        let opcode = self.fetch_opcode(bus);

        self.exec_base(bus, opcode);

        self.cycles
    }

    /// Reads an opcode or a prefix at PC (M1 cycle): advances PC and increments R.
    pub(crate) fn fetch_opcode<B: Z80Bus>(&mut self, bus: &mut B) -> u8 {
        let opcode = self.fetch_byte(bus);
        self.registers.inc_r();
        self.cycles += 1;
        opcode
    }

    /// Reads an operand byte at PC (`n`, `d`, half of `nn`): advances PC, R unchanged.
    /// PC wraps from 0xFFFF to 0x0000.
    pub(crate) fn fetch_byte<B: Z80Bus>(&mut self, bus: &mut B) -> u8 {
        let val = self.read_mem(bus, self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        val
    }

    pub(crate) fn fetch_word<B: Z80Bus>(&mut self, bus: &mut B) -> u16 {
        let low = self.fetch_byte(bus);
        let high = self.fetch_byte(bus);
        ((high as u16) << 8) | (low as u16)
    }

    /// Reads a byte from memory: one memory cycle, 3 T-states.
    ///
    /// The only place, with `write_mem`, that calls the bus for memory accesses.
    pub(crate) fn read_mem<B: Z80Bus>(&mut self, bus: &B, address: u16) -> u8 {
        self.cycles += 3;
        bus.read(address)
    }

    /// Writes a byte to memory: one memory cycle, 3 T-states.
    pub(crate) fn write_mem<B: Z80Bus>(&mut self, bus: &mut B, address: u16, value: u8) {
        self.cycles += 3;
        bus.write(address, value);
    }
}

#[cfg(test)]
mod tests {
    use crate::Z80;
    use crate::Z80Bus;
    use crate::test_bus::TestBus;

    #[test]
    fn nop_opcode() {
        let mut bus = TestBus::new();
        let mut cpu = Z80::new();

        assert_eq!(cpu.step(&mut bus), 4);
        assert_eq!(cpu.registers.pc, 1);
        assert_eq!(cpu.registers.r, 1);
    }

    #[test]
    #[should_panic(expected = "not implemented: opcode 0xFD at address 0x0000 not implemented")]
    fn not_implemented_opcode() {
        let mut bus = TestBus::new();
        bus.write(0x0000, 0xFD); // 0xFD is the IY prefix, implemented late
        let mut cpu = Z80::new();

        cpu.step(&mut bus);
    }

    #[test]
    fn pc_wraps_around() {
        let mut bus = TestBus::new();
        let mut cpu = Z80::new();
        cpu.registers.pc = 0xFFFF;

        assert_eq!(cpu.step(&mut bus), 4);
        assert_eq!(cpu.registers.pc, 0);
    }
}
