# gb_emulator

# Screenshots

# Tests

## Blargg's tests
**cpu_instrs**

- [x] 01-special.gb
- [x] 02-interrupts.gb
- [x] 03-op sp,hl.gb
- [x] 04-op r,imm.gb
- [x] 05-op rp.gb
- [x] 06-ld r,r.gb
- [x] 07-jr,jp,call,ret,rst.gb
- [x] 08-misc instrs.gb
- [x] 09-op r,r.gb
- [x] 10-bit ops.gb
- [x] 11-op a,(hl).gb

**instr_timing**

- [ ] instr_timing.gb

# Resources

* [Pan Docs](http://bgb.bircd.org/pandocs.html)
* [Gameboy CPU instruction set](https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html)
* [Gameboy Development Manual](https://archive.org/details/GameBoyProgManVer1.1)
* [The Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI)

# Dependencies

* [winit](https://github.com/rust-windowing/winit): 0.27.5
* [pixels](https://github.com/parasyte/pixels): 0.9.0