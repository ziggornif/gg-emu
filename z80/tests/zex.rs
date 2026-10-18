use z80::{Z80, Z80Bus};

const ZEXDOC: &[u8] = include_bytes!("../roms/zexdoc.com");
const ZEXALL: &[u8] = include_bytes!("../roms/zexall.com");
const LOAD_ADDRESS: usize = 0x0100;

struct CpmBus {
    mem: Vec<u8>,
}

impl CpmBus {
    pub fn new() -> Self {
        Self {
            mem: vec![0; 65536],
        }
    }

    fn load_program(&mut self, program: &[u8]) {
        let high = (LOAD_ADDRESS >> 8) as u8;
        let low = LOAD_ADDRESS as u8;

        // CP/M page zero, minimal version:
        // 0x0000: JP LOAD_ADDRESS, entry point (also where the program jumps back when done)
        self.mem[0x0000] = 0xC3;
        self.mem[0x0001] = low;
        self.mem[0x0002] = high;
        // 0x0005: BDOS entry. The harness handles the call, then this RET returns to the caller
        self.mem[0x0005] = 0xC9;
        // 0x0006-0x0007: top of memory, read by the program as its stack pointer (LD HL,(0x0006))
        self.mem[0x0006] = 0x00;
        self.mem[0x0007] = 0xF0;

        let end = LOAD_ADDRESS + program.len();
        self.mem[LOAD_ADDRESS..end].copy_from_slice(program);
    }
}

impl Z80Bus for CpmBus {
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

fn bdos(cpu: &Z80, bus: &CpmBus, output: &mut String) {
    match cpu.c() {
        2 => {
            let c = cpu.e() as char;
            output.push(c);
            print!("{c}");
        }
        9 => {
            let mut address = cpu.de();
            loop {
                let c = bus.read(address) as char;
                if c == '$' {
                    break;
                }

                output.push(c);
                print!("{c}");
                address = address.wrapping_add(1);
            }
        }
        _ => panic!("unknown function {}", cpu.c()),
    }
}

fn run(program: &[u8]) -> String {
    let mut output = String::new();
    let mut bus = CpmBus::new();
    bus.load_program(program);
    let mut cpu = Z80::new();

    loop {
        if cpu.pc() == 0x0005 {
            bdos(&cpu, &bus, &mut output);
        }
        cpu.step(&mut bus);
        if cpu.pc() == 0x0000 {
            break;
        }
    }

    output
}

#[test]
#[ignore]
fn zexdoc() {
    let output = run(ZEXDOC);
    assert!(!output.contains("ERROR"), "{output}");
}

#[test]
#[ignore]
fn zexall() {
    let output = run(ZEXALL);
    assert!(!output.contains("ERROR"), "{output}");
}
