import smbus
import glob
from MCP342x import MCP342x

# find available buses, select the correct one
glob.glob(prefix + '*')

# for example, assume it is '/dev/i2c-1'
# then its ID is 1
bus = smbus.SMBus(1)

# choose the connected channel
addr68_ch0 = MCP342x(bus, 0x68, channel=0, resolution=18)

# do the measurement
MCP342x.convert_and_read(addr68_ch0)
