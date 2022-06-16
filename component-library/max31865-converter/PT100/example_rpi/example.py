'''
Inspired by 
https://learn.adafruit.com/adafruit-max31865-rtd-pt100-amplifier/python-circuitpython
'''

import board
import digitalio
import adafruit_max31865

spi = board.SPI()
cs = digitalio.DigitalInOut(board.D5)
sensor = adafruit_max31865.MAX31865(spi, cs, wires=2)

print('Temperature: {0:0.3f}C'.format(sensor.temperature))
print('Resistance: {0:0.3f} Ohms'.format(sensor.resistance))
