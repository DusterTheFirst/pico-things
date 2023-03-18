#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

// Board support package
use rp_pico::{self as bsp, hal::pio::PIOBuilder};

use bsp::{
    hal::{gpio::PinState, pio, prelude::*, sio::Sio, watchdog::Watchdog, Clock},
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

    let (mut pio, state_machine_red, state_machine_green, state_machine_blue, _) =
        peripherals.PIO0.split(&mut peripherals.RESETS);

    let dvi_output_program = pio_proc::pio_file!("src/dvi_differential.pio");

    let installed_program = pio.install(&dvi_output_program.program).unwrap();
    let (state_machine_red, _, red_tx) = PIOBuilder::from_program(installed_program)
        .side_set_pin_base(10)
        .clock_divisor_fixed_point(1, 1)
        .autopull(true)
        .buffers(pio::Buffers::OnlyRx)
        .pull_threshold(8)
        .build(state_machine_red);
    // TODO: DMA and 2 other state machines

    loop {
        info!("high");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("low");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}
