# HTU21D I2C Sensor

This is a Python example code that uses I2C to read temperature and humidity from the HTU21D sensor. This code was tested on `Raspberry Pi 4` model `B`, but in theory should work on any board running Python with I2C interface.

This example is based on [CircuitPython tutorial](https://learn.adafruit.com/adafruit-htu21d-f-temperature-humidity-sensor/python-circuitpython)⹁ which also includes description of wiring for RPI.

Installation requires to enable I2C on RPI and installing respective library:

```
sudo pip3 install adafruit-circuitpython-htu21d
```

Then, you can use the installed library to directly read temperature and humidity from the sensor:

```python
import board
from adafruit_htu21d import HTU21D

i2c = board.I2C()  # uses board.SCL and board.SDA
sensor = HTU21D(i2c)

print("Temperature: %0.1f °C" % sensor.temperature)
print("Humidity: %0.1f %%" % sensor.relative_humidity)
```

Which should print something like this:

```
Temperature: 22.0 °C
Humidity: 48.9 %
```
