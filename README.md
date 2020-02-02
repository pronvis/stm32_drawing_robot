# Build and load

1. to build&load main app: `./build_and_load.sh blink_release.bin`
2. to build&load example: `./build_and_load.sh -e blink blink_debug.bin`

### Detailed instruction
to upload look here: https://rust-embedded.github.io/book/start/hardware.html
- in one terminal open `openocd`
- in another one `gdb -q target/thumbv7m-none-eabi/debug/examples/blink`

in gdb:
- `target remote :3333`
- `load`
- `continue`

or just `gdb -x openocd.gdb target/thumbv7m-none-eabi/debug/examples/hello` then enter and `continue`

# Utils to install
- `brew install stlink`, so to have `st-flash` to upload code without openocd & gdb