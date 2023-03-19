#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

use cortex_m::delay::Delay;
use defmt::{dbg, info};
use rp_pico::{
    hal::{gpio::PinState, pwm, sio::Sio, watchdog::Watchdog, Clock},
    pac, Pins,
};

use crate::{
    clock::init_clocks,
    dvi::{DviClockPins, DviDataPins, DviSerializer},
};

mod clock;
mod dvi;

// Separate macro annotated function to make rust-analyzer fixes apply better
#[rp_pico::entry]
fn macro_entry() -> ! {
    entry();
}

fn entry() -> ! {
    info!("Program start");

    let mut peripherals = pac::Peripherals::take().unwrap();
    let core_peripherals = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
    let single_cycle_io = Sio::new(peripherals.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let clocks = init_clocks(
        peripherals.XOSC,
        peripherals.ROSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    );

    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        single_cycle_io.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let mut led_pin = pins.gpio16.into_push_pull_output_in_state(PinState::Low);

    let mut delay = Delay::new(
        core_peripherals.SYST,
        dbg!(clocks.system_clock.freq().to_Hz()),
    );

    let pwm_slices = pwm::Slices::new(peripherals.PWM, &mut peripherals.RESETS);

    let dvi = DviSerializer::new(
        peripherals.PIO0,
        &mut peripherals.RESETS,
        DviDataPins {
            red_pos: pins.gpio10,
            red_neg: pins.gpio11,
            green_pos: pins.gpio12,
            green_neg: pins.gpio13,
            blue_pos: pins.gpio14,
            blue_neg: pins.gpio15,
        },
        DviClockPins {
            clock_pos: pins.gpio8,
            clock_neg: pins.gpio9,
            pwm_slice: pwm_slices.pwm4,
        },
    );

    // dvi.enable();

    loop {
        info!("high");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("low");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
