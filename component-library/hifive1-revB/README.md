# HiFive1 Rev B  (Freedom E310-G002)

This is a development board using the [Freedom E310-G002 microcontroller from SiFive](https://www.sifive.com/boards/hifive1-rev-b). E310-G002 is a RISC-V microcontroller with the SPI, UART, and I2C interfaces, plus three PWM generators and several general purpose GPIO. The CPU is running at ~350MHz and has access to 4MB of flash for program storage (it can be also used for data but that is rather cumbersome to implement). Additionally, the board also contains an ESP32-SOLO-1 controller with WiFi/BT functionality which communicates with the main CPU using SPI.

 - The [getting started guide](./hifive1b-getting-started-guide_v1.2.pdf) has basic notes on how to set-up development environment for this board.
 - For more information about the CPU itself, refer to the [SoC manual](./fe310-g002-manual-v1p1.pdf).
 - Schematics of the board are included [here](./hifive1-b01-schematics.pdf), and assuming SiFive's website is still alive, the CAD files for the board can be obtained there as well (not included in this repository though).
 - We also include the [datasheet](./esp32-solo-1_datasheet_en.pdf) for the ESP32. To communicate with the device, you need to follow the [ESP-AT](https://github.com/espressif/esp-at) protocol (specification at the time of writing is archived [here](./esp-at-user-guide-en-latest.pdf)).

> WARNING: The board comes pre-loaded with a "recovery" bootloader which allows it to avoid executing the program in case the program is faulty (and would e.g. shut down the board immediately, which disables flashing of new code). The getting started guide notes how to set up your tool-chain such that this bootloader is preserved. Our examples will also make sure the bootloader may not be corrupted. Make sure you consider this if you want to try writing custom flashing scripts.

## Connecting to the board

This section is a loose copy of the process as described in the getting started guide. We include it mainly for quick reference. Also, this text focuses on Linux, for other operating systems see the getting started guide.

 - You'll need [Segger J-Link tools](https://www.segger.com/downloads/jlink/#J-LinkSoftwareAndDocumentationPack) to program the device. The package is not included in this repository, because it is quite big (30+ MB), so hopefully it will not disappear from the internet any time soon.
 - Connect the device using a USB cable.
 - Execute `sudo screen /dev/ttyACM0 115200` to access the output log from the device (input is not supported, also, you may need to install `screen` first).
 - Depending on the loaded program, you may need to press `reset` to actually see some output in the console. 
 - Note that the output is shared with the ESP32 chip, so if you initially see some "garbage" text, it an initialization sequence of the ESP32.
 - For more information on how to execute your code on the device, continue in the example Rust project.