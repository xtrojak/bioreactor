# HTU21D Temperature and Humidity Sensor

This is a simple sensor that connects to the master device via I2C and can relatively precisely measure both temperature (accuracy 0.3 Celsius) and humidity (accuracy 2%).

The I2C communication is described in the included [datasheet](HTU21D-datasheet.pdf). In the `example_hifive` folder, we provide a simple program in Rust which shows how to read temperature and humidity data from the sensor. 