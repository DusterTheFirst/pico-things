#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

// Board support package
use rp_pico::{self as bsp};

use bsp::{
    hal::{gpio::PinState, sio::Sio, watchdog::Watchdog, Clock},
    pac, Pins,
};
use cortex_m::delay::Delay;
use defmt::{dbg, info};

use crate::clock::init_clocks;

mod clock;

// Separate macro annotated function to make rust-analyzer fixes apply better
#[bsp::entry]
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

    let mut led_pin = pins.gpio15.into_push_pull_output_in_state(PinState::Low);

    let mut delay = Delay::new(
        core_peripherals.SYST,
        dbg!(clocks.system_clock.freq().to_Hz()),
    );

    loop {
        info!("high");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("low");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}

struct DviPhy {}

impl DviPhy {
    pub fn new() {
        let program_a = pio_proc::pio_file!("src/test.pio", select_program("a"));
        let program_b = pio_proc::pio_file!("src/test.pio", select_program("b"));

        let mut program_c = pio::Assembler::<{ pio::RP2040_MAX_PROGRAM_SIZE }>::new();
    }
}
