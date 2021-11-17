from time import sleep
import board
import busio
import adafruit_pca9685

# initialize the I2C connection
i2c = busio.I2C(board.SCL, board.SDA)
pca = adafruit_pca9685.PCA9685(i2c)

# set the board's PWM frequency
pca.frequency = 60
led_channel = pca.channels[0]

# set cycle to particular value (in range 0-65535 (or 0xffff))
led_channel.duty_cycle = 0xffff
sleep(5)
