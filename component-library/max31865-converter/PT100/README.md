# PT100 Temperature Sensor

See attached [PDF file](max31865-rtd-pt100-amplifier.pdf) for details on how to correctly connect PT100 to MAX31865 converter.

## 2-wire sensors with MAX31865

Note that to use a 2-wire probe with a MAX31865 board such as [this one](https://www.laskakit.cz/modul-prevodniku-pro-termoclanek-pt100--max31865--spi/), you'll have to connect a wire from the probe to *both* terminals (i.e. one wire to both negative terminals and one wire to both positive terminals). The board has solder pads to bridge the terminals permanently, but you can also achieve this using a short wire. You can read all about this in the attached [PDF instructions](max31865-rtd-pt100-amplifier.pdf), just be warned!