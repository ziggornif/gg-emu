# Z80 instruction exercisers

`zexdoc` and `zexall` test the Z80 instruction set by comparing CRCs of
instruction results with values measured on real hardware.

| File | Content |
| --- | --- |
| `zexdoc.com` / `zexall.com` | CP/M binaries, loaded at 0x0100 by the test harness |
| `zexdoc.z80` / `zexall.z80` | assembly sources, to see what each test group exercises |

- `zexdoc` checks documented flags only.
- `zexall` also checks the undocumented X and Y flags (bits 3 and 5 of F).

**Author:** Frank D. Cringle, 1994.
**License:** GNU General Public License, version 2 or later (see the header of the `.z80` files).
These files are test programs run by the emulator; they are not part of the `z80` crate.
**Source:** taken from https://github.com/anotherlin/z80emu (`testfiles/`).

Run with `cargo test -p z80 --test zex zexdoc -- --ignored --nocapture`.
