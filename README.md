# gg-emu

A Sega Game Gear emulator written in Rust, built step by step as a learning project.

> **Status: early work in progress.** Only the Z80 CPU is being written. Nothing runs a game yet.

## Workspace

| Crate | Role | State |
| --- | --- | --- |
| `z80` | Zilog Z80 CPU core, independent of the Game Gear | in progress |
| `sn76489` | SN76489 sound chip (PSG) | placeholder |
| `gg-core` | Game Gear machine: memory map, VDP, I/O, wiring of the chips | placeholder |
| `gg-app` | Desktop front-end (window, input, audio) | placeholder |

The design is hexagonal at the workspace level: `z80` is the core and knows nothing about the Game Gear.
It talks to the outside world only through the `Z80Bus` trait (memory and I/O ports), which each machine
implements as an adapter. `gg-core` will depend on `z80`, never the other way around.

## Z80 core

The CPU executes one instruction per `step()` and returns the number of T-states it took.

Done so far:

- registers, flags and the alternate register set;
- opcode decoding by x/y/z/p/q fields;
- a large part of the unprefixed instruction set: loads, jumps, calls, stack, logical ops,
  `CP`, `ADD HL,rr`, `INC`/`DEC`;
- the ED prefix dispatch, with `LDIR`;
- cycle counting on every instruction.

Not done yet: the rest of the arithmetic ALU, the CB/DD/FD prefixes, interrupts and `HALT`.

## Testing

Each instruction has unit tests (flags, cycles, PC). The CPU is also checked against **zexdoc**,
the classic Z80 instruction exerciser, run as a CP/M program by an integration test
(see [`z80/roms/README.md`](z80/roms/README.md)). Progress is measured by how far zexdoc gets
before it reaches an unimplemented opcode.

```sh
make check    # cargo fmt + clippy (warnings as errors)
make test     # unit tests
make zexdoc   # run the zexdoc exerciser (ignored by default)
```

CI runs fmt, clippy, tests, `cargo audit` and coverage on every push.

## License

MIT. The zexdoc/zexall test programs in `z80/roms/` are GPL-2.0-or-later, by Frank D. Cringle.
