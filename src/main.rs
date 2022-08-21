//! # Pico GPIO In/Out Example
//!
//! Toggles the LED based on GPIO input.
//!
//! This will control an LED on GP25 based on a button hooked up to GP15. The
//! button should cause the line to be grounded, as the input pin is pulled high
//! internally by this example. When the button is pressed, the LED will turn
//! off.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

// The macro for our start-up function
use rp_pico::entry;

// GPIO traits
use embedded_hal::digital::v2::{InputPin, OutputPin};

// Time handling traits
use embedded_time::rate::*;

// PWM
use embedded_hal::PwmPin;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

use rp_pico::hal::gpio::FunctionPwm;
use rp_pico::hal::prelude::*;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use rp_pico::hal::pac;

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// Set PWM values
const LOW: u16 = 0;
const HIGH: u16 = 25000;

/// Entry point to our bare-metal application.
///
/// The `#[entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables are initialised.
///
/// The function configures the RP2040 peripherals, then just reads the button
/// and sets the LED appropriately.
#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // The delay object lets us wait for specified amounts of time (in
    // milliseconds)
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // init PWM4
    let mut pwm_slices = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    // Configure PWM4
    let pwm6 = &mut pwm_slices.pwm6;
    pwm6.set_ph_correct();
    pwm6.enable();

    let pwm7 = &mut pwm_slices.pwm7;
    pwm7.set_ph_correct();
    pwm7.enable();

    let channelr = &mut pwm6.channel_b;
    channelr.output_to(pins.gpio13);

    let channelg = &mut pwm7.channel_a;
    channelg.output_to(pins.gpio14);

    let channelb = &mut pwm7.channel_b;
    channelb.output_to(pins.gpio15);

    // Our button input
    // let button_pin = pins.gpio15.into_pull_up_input();

    // Run forever, setting the LED according to the button
    loop {
        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(64);
            channelr.set_duty(i);
            channelg.set_duty(i);
            channelb.set_duty(i);
        }
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(64);
            channelr.set_duty(i);
            channelg.set_duty(i);
            channelb.set_duty(i);
        }

        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(64);
            // channelr.set_duty(i);
            channelg.set_duty(i);
            channelb.set_duty(i);
        }
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(64);
            // channelr.set_duty(i);
            channelg.set_duty(i);
            channelb.set_duty(i);
        }

        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(64);
            channelr.set_duty(i);
            // channelg.set_duty(i);
            channelb.set_duty(i);
        }
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(64);
            channelr.set_duty(i);
            // channelg.set_duty(i);
            channelb.set_duty(i);
        }

        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(64);
            channelr.set_duty(i);
            channelg.set_duty(i);
            // channelb.set_duty(i);
        }
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(64);
            channelr.set_duty(i);
            channelg.set_duty(i);
            // channelb.set_duty(i);
        }

        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(64);
            channelr.set_duty(i);
            // channelg.set_duty(i);
            // channelb.set_duty(i);
        }
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(64);
            channelr.set_duty(i);
            // channelg.set_duty(i);
            // channelb.set_duty(i);
        }

        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(64);
            // channelr.set_duty(i);
            channelg.set_duty(i);
            // channelb.set_duty(i);
        }
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(64);
            // channelr.set_duty(i);
            channelg.set_duty(i);
            // channelb.set_duty(i);
        }

        for i in (LOW..=HIGH).skip(100) {
            delay.delay_us(64);
            // channelr.set_duty(i);
            // channelg.set_duty(i);
            channelb.set_duty(i);
        }
        for i in (LOW..=HIGH).rev().skip(100) {
            delay.delay_us(64);
            // channelr.set_duty(i);
            // channelg.set_duty(i);
            channelb.set_duty(i);
        }
    }
}

// End of file
