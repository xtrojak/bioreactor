# Device Protocol

Here, we describe the communication mechanism that can be used to transmit information from a low-powered bioreactor controller to a more capable central hub. In general, this communication protocol should be able to work using an arbitrary reliable channel. Most of the time, this will be WiFi/Ethernet, but theoretically, USB or Bluetooth should be also feasible. Some more low-level links (like SPI or I2C) could be considered, however, in that case an additional form of error correction should be implemented, as these protocol are not necessarily reliable.

To increase data density and reduce the time spent on transmission, the protocol assumes that all messages are raw binary data (aside from certain values that are inherently UTF-8). Consequently, this document uses either the `0b1010` notation to denote a value in binary, or `0xab89` to denote values in hexadecimal encoding. The protocol is message based (with bounded message length) and is not designed to be particularly quick. The primary goal is to block the operation of the low-level device as little as possible (to decrease the chance of missing real-time deadlines for low-level devices).

### Message structure

Every message starts with two bytes (`u16`) that together indicate the length of the message (first byte contains the most significant bits). Subsequent values can range from byte-sized to 64-bit values depending on the type and contents of the message. For multi-byte values, most significant bits are sent first.

Practically, this means that if a device receives `0x12, 0x34, 0x56, 0x78` in this order, and it expects a 32-bit number, this should be interpreted as `0x12345678` (as opposed to `0x78563412` for LSB-first ordering). The bits within the individual bytes are ordered based on the assumptions of the underlying platform and we assume that the communication channel takes care of properly translating the endianness during transmission. 

Note that when the communication is buffered, then the first byte received is the one on position `0`. That is, the *least significant* byes of the buffer represent the *most significant* message bytes. This is quite standard, but can be a little surprising if you for some reason try to write down the whole message as a single number.

After the message length, next byte represents a message type code. These indicate the structure of the message and are explained later.

### Handshake

The connection is initiated by the bioreactor device. Configuration data (server address and port, network name, etc.) are either directly in the firmware or on some external medium (e.g. an SD card). The device starts the connection by sending a handshake message:

```
[0x00,0x06,0xff,0b01100100,0x$V1,0x$V2]
```

Here, `0x0006` is the length of the message (in the subsequent sections, we simply write `0xLL, 0xLL` to denote these length bytes), `0xff` is a special "handshake" command and `0b01100100` is a "magic constant" (character "d"). Finally, `$V1` is the major and `$V2` is the minor version of the protocol supported by this device. 

Now, the server should respond with a similar handshake message, sending "s" instead of "d" as the "magic constant". The server must send the same major-minor version as the device specified, by which the server confirms that it can adhere to this specification. Here, a newer server is allowed to "impersonate" an older version if a device with an older protocol tries to connect. However, it should do so in a consistent manner (i.e. it should actually respond in accordance with the older protocol).

Alternatively, the server concludes that the protocol versions are ultimately incompatible and rejects the connection. In such case, it sends a simple `[0x00, 0x03, 0x00]` reject message and closes the connection. The device then simply logs this as an error and does not attempt to reconnect unless restarted. In case of other connection issues (i.e. cases where the connection is not explicitly rejected), the device should attempt to reconnect after some amount of time has passed (ideally more than one second and less than a minute).

Once handshake is established, the device automatically enters "inactive" mode (i.e. the server can assume that after a handshake, a device is inactive).

### Inactive mode

In the inactive mode, a device is connected to the server, but it is not actively performing an experiment. In particular, the device is not doing any logging and  there should be almost no periodic tasks running (there can be some low-level safeguards still active though -- these should be mostly to prevent catastrophic failures due to, e.g., hardware malfunction). 

Consequently, the device's resources should be free to communicate with the server. In this mode, the server can request detailed information about the instruments that are connected to the device, perform tasks like calibration, or update some other global long-term settings. Similarly, it may be possible to (slowly) extract logs from the device  or update the device's firmware (this should be easier by manipulating the device's SD card, but this option may be faster when the number of devices is large since it could be automated).

Once the server learns about the available instruments and readouts, the device can be switched to the active mode.

### Active mode

When active, the device will send periodic status messages with updated readouts form individual instruments. For now, this period is fixed, but should be configureable in the future.

In this mode, the server can queue-in requests to update certain readouts. These are either applied immediately of possible (returning "done" `0x02`) or saved for later (returning "accept" `0x01` and a 16-bit "request" ID). If too many pending requests are queued, the device can also respond with "reject" (`0x00`). Part of each status message then also contains IDs of completed requests.

### Message codes

Here, we briefly list the possible message codes that appear in the protocol:

* [`0xff`] Handshake message: contains a magic byte constant and major-minor version of the protocol.
* [`0x00`] Reject: the last message was either invalid or cannot be carried our.
* [`0x01`] Accept: the last message was accepted.
* [`0x02`] Done: the last command was performed.
* [`0x12`, inactive] Instrument list: If received from the server, the device should respond with a list of connected instruments. Each instrument is described by a 16-bit ID and a 16-bit type.
* [`0x13`, inactive] Readout list: If received from the server, it should contain a 16-bit instrument ID. The device responds with a list of readouts, where each readout is again described by a 16-bit ID and a 16-bit type. Note that the readout ID should be unique also among readouts of the other instruments.
* [`0x14`, inactive] Device name: If received from the server, respond with a pre-configured name string (UTF-8).
* [`0x15`, inactive] Device info: If received from the server, respond with a pre-configured info string (UTF-8).
* [`0x16`, inactive] Instrument name: Respond with a pre-configured instrument name string (UTF-8).
* [`0x17`, inactive] Instrument info: Respond with a pre-configured instrument info string (UTF-8).
* [`0x70`, inactive] Activate.
* [`0x71`, active] Status. First, a 16-bit number giving the number of readouts, then a list of readouts in the form (type, ID, value). The length of value may differ based on type. Second, an 8-bit number giving the amount of resolved requests, followed by a list of 16-bit request IDs.
* [`0x72`, active] Update readout. Contains the 16-bit readout ID and its new value, returns either "done" or a new request ID.