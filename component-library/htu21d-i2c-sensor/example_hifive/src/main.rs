#![no_std]
#![no_main]

extern crate panic_halt;

use core::ops::Not;
use crc8_rs::has_valid_crc8;
use hifive1::hal::i2c::{I2c, Speed};
use hifive1::hal::prelude::*;
use hifive1::hal::DeviceResources;
use hifive1::{pin, sprintln};
use hifive1::hal::delay::Sleep;
use riscv_rt::entry;

mod htu21d {
    pub const ADDRESS: u8 = 0x40;
    pub const CMD_MEASURE_TEMPERATURE: u8 = 0xE3;
    pub const CMD_MEASURE_HUMIDITY: u8 = 0xE5;
    pub const CMD_MEASURE_TEMPERATURE_ASYNC: u8 = 0xF3;
    pub const CMD_MEASURE_HUMIDITY_ASYNC: u8 = 0xF5;
    pub const CMD_READ_REGISTER: u8 = 0xE6;
    pub const CMD_WRITE_REGISTER: u8 = 0xE7;
    pub const CMD_RESET: u8 = 0xFE;

    pub const STATUS_MASK: u8 = 0b11;
    pub const STATUS_TEMPERATURE: u8 = 0b00;
    pub const STATUS_HUMIDITY: u8 = 0b01;
}

fn abort() {
    sprintln!("Abort.");
    loop {}
}

#[entry]
fn main() -> ! {
    let dr = DeviceResources::take().unwrap();
    let p = dr.peripherals;
    let pins = dr.pins;

    // Configure clocks
    let clocks = hifive1::clock::configure(p.PRCI, p.AONCLK, 100.mhz().into());

    // Configure UART for stdout
    hifive1::stdout::configure(
        p.UART0,
        pin!(pins, uart0_tx),
        pin!(pins, uart0_rx),
        115_200.bps(),
        clocks,
    );

    // Configure I2C
    let sda = pin!(pins, i2c0_sda).into_iof0();
    let scl = pin!(pins, i2c0_scl).into_iof0();
    let mut i2c = I2c::new(p.I2C0, sda, scl, Speed::Normal, clocks);

    let mut sleep = Sleep::new(dr.core_peripherals.clint.mtimecmp, clocks);

    // First, reset the sensor in case it is in some undefined state.
    let reset_result = i2c.write(htu21d::ADDRESS, &[htu21d::CMD_RESET]);
    if let Err(error) = reset_result {
        sprintln!("Cannot reset sensor: {:?}", error);
        abort();
    } else {
        sprintln!("Sensor reset command success.");
    }

    // The sensor should take 15ms to reset.
    sleep.delay_ms(20);

    let measure_command = i2c.write(htu21d::ADDRESS, &[htu21d::CMD_MEASURE_TEMPERATURE_ASYNC]);
    if let Err(error) = measure_command {
        sprintln!("Error sending measurement command: {:?}", error);
        abort();
    } else {
        sprintln!("Measure temperature command sent.");
    }

    // The sensor can take up-to 50ms to perform the measurement when in 14-bit mode.
    sleep.delay_ms(60);

    let mut result: [u8; 3] = [0; 3];
    let read_command = i2c.read(htu21d::ADDRESS, &mut result);
    if let Err(error) = read_command {
        sprintln!("Error receiving measurement: {:?}", error);
        abort();
    } else {
        sprintln!("Measurement raw result: {:08b} {:08b} {:08b}.", result[0], result[1], result[2]);
    }

    let status = result[1] & htu21d::STATUS_MASK;
    if status != htu21d::STATUS_TEMPERATURE {
        sprintln!("Error. Status flags do not match expected values.");
        abort();
    }

    let measurement: u16 = (u16::from(result[0])  << 8) + (u16::from(result[1] & htu21d::STATUS_MASK.not()));

    sprintln!("Measurement: {}", measurement);

    let temperature = -46.85 + 175.75f32 * (measurement as f32) / ((1 << 16) as f32);

    sprintln!("Temperature: {}", temperature);

    loop {}
}