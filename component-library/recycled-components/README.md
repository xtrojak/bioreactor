# Recycled Components

Here, we keep track of everything that we have available from older/malfunctioning computer parts (mainly CD/DVD drives and faulty motherboards). For every part, there is a "source component" (so that we know where we got it), a pdf datasheet, and a quick overview of the specifications. Not everything can be completely identified, in such case we give the "best available approximation" of what we think the part is.

## Mosfets

#### D472 Mosfet (N) (10pcs)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./mosfet-d472.pdf)

*Characteristics:* Threshold voltage 1.2-2.5V; Drain-source max voltage 25V; Max sustained current 10A at 10V or 5A at 20V; Power dissipation 75-150W.

*Comments:* In theory it should be able to directly drive a DC pump/motor at 12V. Not entirely sure if I'd use it directly for 20V LEDs though.

#### D452 Mosfet (N) (8psc)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./mosfet-d452.pdf)

*Characteristics:* Threshold voltage 1.2-3V; Drain-source max voltage 25V; Max sustained current 2A at 10V or 1A at 20V; Power dissipation 25-50W.

*Comments:* Similar to D472, but can only survive smaller amounts of current. Hopefully should still be good for less demanding applications.

#### 9T16GH Mosfet (N) (3psc)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./mosfet-9t16gh.pdf)

*Characteristics:* Threshold voltage 0.5-1.5V; Drain-source max voltage 20V; Max sustained current 20A at 1V, or 2A at 10V; Power dissipation 25W.

*Comments:* Probably not a good idea to use this for anything with non-trivial power draw at voltages above 5V, but could be still useful for something smaller, or to drive another mosfet.

#### 3310GH Mosfet (P) (1pc)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./mosfet-3310gh.pdf)

*Characteristics:* Threshold voltage -0.5V; Drain-source max voltage -20V; Max sustained current 1-10A depedning on voltage; Power dissipation 25W;

*Comments:* Compared to N-mosfets, P-mosfets activate when there is negative voltage on their gate pin. In theory, this should be able to do the same things as N-mosfet, but in different configuration. It could be useful in combination with a P-mosfet to switch the direction of motor rotation.

#### PHB11N06LTA Mosfet (N) (1pc)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./mosfet-phb11n06lta.pdf)

*Characteristics:* Threshold voltage 1-2V; Drain-source max voltage 55V; Max sustained current 2A at 10V, 1A at 20V; Power dissipation 36W.

#### NTD80N02 Mosfet (N) (4pcs)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./mosfet-ntd80n02.pdf)

*Characteristics:* Threshold voltage 1-3V; Drain-source max voltage 24V; Max sustained current 8A at 10V; Power dissipation 75W.

#### IRLR3714 Mosfet (2pcs)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./mosfet-irlr3714.pdf)

*Characteristics:* Threshold voltage 1-3V; Drain-source max voltage 20V; Max sustained current 2A at 10V; Power dissipation 33-47W.

#### IRFR3704 Mosfet (2pcs)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./mosfet-irfr3704.pdf)

*Characteristics:* Threshold voltage 1-3V; Drain-source max voltage 20V; Max sustained current 9A at 10V; Power dissipation 62-90W.

## Voltage Regulators

#### AZ1084 Voltage Regulator (1pc)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./voltage-regulator-az1084.pdf)

*Characteristics:* Dropout voltage 1.3V; Max input voltage 12V; Adjustable output voltage 1.5-5V; Max sustained current 5A at 3V difference;

*Comments:* The adjustment is performed using an extra resistor on the third pin (example in datasheet). Not sure how useful this is for us, but could be probably useful for getting 3.3V/5V in addition to 12V on some daughter board.

#### LM1117 Voltage Regulator (2pcs)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./voltage-regulator-lm1117.pdf)

*Characteristics:* Dropout voltage 1.3V; Recommended input voltage 15V; Adjustable output voltage 1.25-13.8V; Max sustained current 0.8A;

*Comments:* We have one fixed to 3.3V, and one adjustable.

#### LM78M05 Voltage Regulator (1pc)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./voltage-regulator-lm78m05.pdf)

*Characteristics:* Dropout voltage 2.2V; Recommended input voltage 7-35V; Output voltage 5V; Max sustained current 0.5A;

#### W83310DG Voltage REgulator (1pc)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./voltage-regulator-w83310dg.pdf)

*Comments:* Is probably useless to us. It origianlly generates votage for RAM modules, which means it can convert 3.3V to a very narrow interval around 1.2V. If I understand it correctly, it's a gate driver with two integrated mosfets.

## Amplifiers

#### THS4505 Amplifier (5pcs)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./amplifier-ths4505.pdf)

*Characteristics:* Supply voltage max 16V; Current 150mA; Differential input voltage 4V;

*Comments:* These could be useful to amplify light intensity measurements though photodiodes.

#### LM358 Amplifier (2pcs)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./amplifier-lm358.pdf)

*Characteristics:* Suuply voltage max 32V; Current 50mA; Differential input volate 32V;

*Comments:* Similar to THS4505, but contains two amplifiers in one package.

#### AS358 Amplifier (2pcs)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./amplifier-as358.pdf)

*Comments:* Also contains two amplifiers in one package.

## Gate Drivers

#### NCP5359 Gate Driver (4pcs)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./ncp5359-gate-driver.pdf)

*Comments:* A gate driver is used in a buck converter to drive two mosfets which together implement a step-down conversion from a high DC voltage (say 12V) to something lower (say 5V, or 3.3V). The output voltage is regulated by a PWM signal. This is more efficient and versatile than voltage regulators, but also harder to implement. Could be fun to try to build this (example in datasheet). Unless we'd drive each LED separately at 3-4V, this isn't very useful to us, aside from maybe converting 20V down to 12V/6V for motors (but we may as well buy that solution separately). A useful example circuit is also in the NCP5393 datasheet. This is a controller which can manage up to 4 of these drivers in parallel based on the instructions from the CPU. As such, it is not useful to us (we don't need 50W on 1.\* volts) as is, but it shows how to operate these drivers.

#### TDA21101G Gate Driver (2pcs)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./tda21101g-gate-driver.pdf)

*Comments:* Similar to NCP5359, but older.

#### APW7120 Gate Driver (2pcs)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./apw7120-gate-driver.pdf)

*Comments:* Compared to previous gate drivers, this one does not need an external PWM signal. Instead, 300kHz signal is generated internally and the duty cycle is automatically adjusted such that the output voltage corresponds to the one set on the reference pin. There is also a nice example circuit on how to set it up. Maybe we could even build an adjustable power supply with this one (by tuning the reference resistor).

## Other ICs

#### 24C08WP I2C EEPROM (1pc)

*Source:* RS780M03G1-8EKSMH AMD Motherboard

*Datasheet:* [link](./eeprom-24c08wp.pdf)

*Comments:* An I2C operated EEPROM with 1kB capacity. Could be useful to store something like calibration tables or other internal information.

#### HIP6302 PWM Buck Controller (1pc)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./hip6302-buch-controller.pdf)

*Comments:* Used to control the TDA21101G drivers. Could be useful if we decided to have our own power supply for the LEDs, but the target voltage is only 1.1-1.85V, which is not enough for LEDs. But maybe there is a way to skew the target if we swap the recommended resistors for some different values.

#### LM85CIMQ Fan controller (1pc)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./lm85cimq-fan-controller.pdf)

*Comments:* Probably not useful for us. It can measure its own temperature and two external thermocouples, and then sets three PWM signals based on the measured values. It could be useful to cool the LEDs or Peltier elements, but for that we can probably get something easier to use. For temperature readings, we need something more precise anyway. It can also perform voltage monitoring, which could be actually useful to us as a safety mechanism, i.e. to see if there is a short/under-voltage and shut down the system.

#### LPC47M172 I/O Controller (1pc)

*Source:* D845GEBV2 Intel Motherboard

*Datasheet:* [link](./lpc47m172-io-controller.pdf)

*Comments:* This implements basic CPU I/O which is not provided by a chipset (serial, parallel, IR, some GPIO, Fan tachometers, PS/2 mouse/keyboard). It has a lot of fun features and a relatively straightforward communication protocol, but probably nothing that we can use unless we wanted to do something really crazy.

