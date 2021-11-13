# HiFive1 RevB Blink Leds Example

This example is adapted from [this repository](https://github.com/riscv-rust/riscv-rust-quickstart). For a more detailed information about the code, please read their documentation.

However, in general, you'll:
 - Need to have installed [Rust](https://rust-lang.org) compiler.
 - Install the RISC-V target using `rustup target add riscv32imac-unknown-none-elf`.
 - Have the RISC-V GNU toolchain added to `PATH`. Version for Ubuntu is [here](https://static.dev.sifive.com/dev-tools/riscv64-unknown-elf-gcc-8.1.0-2019.01.0-x86_64-linux-ubuntu14.tar.gz), or if you're unsure how to install, see the getting started guide.
 - Also, to use the toolchain, you may need to install `sudo apt-get install libncurses5`.
 - Start the programmer server on your computer:

```
JLinkGDBServer -device FE310 -if JTAG -speed 4000 -port 3333 -nogui
```

 - Use `cargo build` and `cargo run` to build the project and to send it to the device (assuming it is connected and you have the J-Link software installed and started as in the command above).
 - After you exit your program (`Ctrl+C`), you end up in `(gdb)` console. Press `Ctrl+D` to exit this console.