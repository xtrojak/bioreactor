# HiFive1 RevB Blink Leds Example

This example is adapted from [this repository](https://github.com/riscv-rust/riscv-rust-quickstart). For a more detailed information about the code, please read their documentation.

However, in general, you'll:
 - Need to have installed [Rust](https://rust-lang.org) compiler.
 - Install the RISC-V target using `rustup target add riscv32imac-unknown-none-elf`.
 - Have the RISC-V GNU toolchain added to `PATH`. An update to date version of the toolchain can be obtained [here](https://www.sifive.com/software).
 - Have a J-Link OB Debugger installed (available [here](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack)).
 - Also, to use the toolchain, you may need to install `sudo apt-get install libncurses5` on linux.
 - To see the device output, connect to the device through terminal:

```bash
# On linux, this should work:
sudo screen /dev/ttyACM0 115200
# On macOS, the device name will be probably different.
# But look for tty.usbmodemXY devices:
sudo screen /dev/tty.usbmodem1234567 115200
```

 - Start the programmer server on your computer:

```
JLinkGDBServer -device FE310 -if JTAG -speed 4000 -port 3333 -nogui
```

 - Now use `cargo build` and `cargo run` to build the project and to send it to the device. Assuming you have performed the actions above, you should see the LED blinking (with different colours) and the device log should show a `Blink!` message every time this happens.
 - After you exit your program (`Ctrl+C`), you end up in `(gdb)` console. Press `Ctrl+D` to exit this console.