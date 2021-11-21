# PCA9685 16-channel 12-bit PWM controller

A controller board which allows us to generate a PWM signal without relying on the main CPU. It offers 16 channels at a shared 24-1526Hz frequency with the ability to adjust the duty cycle for each channel. Communication happens through I2C and the board continues to generate the PWM signals without the need for external inputs.

The current per channel is limited to 10-25mA, which isn't sufficient for anything beyond small LEDs. To drive larger appliences, you need a mosfet that will actually swtich the high-current power rail (see section 10 of the datasheet for examples showing a LED array).

The repository also contains the schematics and PCB layout of the whole controller board for reference.