# `own notes`
to upload look here: https://rust-embedded.github.io/book/start/hardware.html
- in one terminal open `openocd`
- in another one `gdb -q target/thumbv7m-none-eabi/debug/examples/blink`

in gdb:
- `target remote :3333`
- `load`
- `continue`

or just `gdb -x openocd.gdb target/thumbv7m-none-eabi/debug/examples/hello` then enter and `continue`
