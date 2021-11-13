import board
from adafruit_htu21d import HTU21D

i2c = board.I2C()  # uses board.SCL and board.SDA
sensor = HTU21D(i2c)

print("Temperature: %0.1f °C" % sensor.temperature)
print("Humidity: %0.1f %%" % sensor.relative_humidity)
