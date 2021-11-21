# IRF520 MOSFET

A small N-type transistor module for switching high-voltage or high-current devices, such as motors, pumps or LED arrays.

Note that to fully switch, the transistor requires 5V on gate (technically 2-4V, but it usually seems to be closer to 4V). So the standard 3.3V PWM/GPIO pins on Raspberry Pi or HiFive boards are typically unable to drive the transistor on their own. You need a logic voltage shifter (3.3V -> 5V) to actually use this module properly. 