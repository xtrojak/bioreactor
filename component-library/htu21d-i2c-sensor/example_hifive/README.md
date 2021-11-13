# HTU21D I2C Sensor

This is a Rust example code that uses I2C to read temperature from the HTU21D sensor. If you've successfully executed the basic LED example for the `HiFive Rev B` board, this example should be easy to test.

 - Connect the sensor to the board (SCL, SDA, GND to their respective pins, VIN to 3.3V, but 5V should be also ok).
 - Connect the board to your computer and in two separate terminal windows, start the debugger interface...
 
    ```
    JLinkGDBServer -device FE310 -if JTAG -speed 4000 -port 3333 -nogui
    ```
    ... and the output log ...

    ```
    sudo screen /dev/ttyACM0 115200
    ```
   
 - `cargo run` this project. You should see a console output similar to this:

    ```
   Sensor reset command success.
   - Measure command sent.
   - Raw measurement: 01101000 11100000 10100011.
   - Status flags ok.
   - Checksum ok.
   - Converted measurement: 26848.
     ======== Temperature: 25.149147 ========
   - Measure command sent.
   - Raw measurement: 01101100 00001000 01011000.
   - Status flags ok.
   - Checksum ok.
   - Converted measurement: 27656.
     ======== Temperature: 27.315987 ========
   - Measure command sent.
   - Raw measurement: 01101101 01111000 01010100.
   - Status flags ok.
   - Checksum ok.
   - Converted measurement: 28024.
     ======== Temperature: 28.302864 ========
   - Measure command sent.
   - Raw measurement: 01101011 10001000 10001100.
   - Status flags ok.
   - Checksum ok.
   - Converted measurement: 27528.
     ======== Temperature: 26.972725 ========
   - Measure command sent.
   - Raw measurement: 01101010 10000000 11000001.
   - Status flags ok.
   - Checksum ok.
   - Converted measurement: 27264.
     ======== Temperature: 26.264748 ========
       ```
