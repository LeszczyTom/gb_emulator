# gb_emulator
WIP
# Screenshots
Main gameboy screen with debugging widgets:
![maps](https://user-images.githubusercontent.com/37774352/209233969-15c2b751-456a-4c2f-b4ab-78003500e9eb.gif)
![misc](https://user-images.githubusercontent.com/37774352/209233975-f489c1ad-8f1b-4d2a-8b16-cd6f6c971266.gif)
![memory_dump](https://user-images.githubusercontent.com/37774352/209233972-e53d4f13-2f36-472f-8571-feeb80d2cbb5.gif)

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
