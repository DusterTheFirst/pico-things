#![no_std]
#![no_main]

use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

// Board support package
use rp_pico as bsp;

use bsp::{
    hal::{
        clocks::{self, init_clocks_and_plls},
        gpio::PinState,
        watchdog::Watchdog,
        Clock, self,
    },
    pac, Pins,
};
use cortex_m::delay::Delay;
use defmt::info;

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
    let single_cycle_io = hal::sio::Sio::new(peripherals.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    const EXTERNAL_CRYSTAL_OSCILLATOR_FREQ_HZ: u32 = 12_000_000;
    let clocks = match init_clocks_and_plls(
        EXTERNAL_CRYSTAL_OSCILLATOR_FREQ_HZ,
        peripherals.XOSC,
        peripherals.CLOCKS,
        peripherals.PLL_SYS,
        peripherals.PLL_USB,
        &mut peripherals.RESETS,
        &mut watchdog,
    ) {
        Ok(clocks) => clocks,
        Err(clocks::InitError::ClockError(error)) => panic!("{:?}", error),
        Err(clocks::InitError::PllError(error)) => panic!("{:?}", error),
        Err(clocks::InitError::XoscErr(error)) => panic!("{:?}", error),
    };

    let pins = Pins::new(
        peripherals.IO_BANK0,
        peripherals.PADS_BANK0,
        single_cycle_io.gpio_bank0,
        &mut peripherals.RESETS,
    );

    let mut led_pin = pins.gpio0.into_push_pull_output_in_state(PinState::Low);

    let mut delay = Delay::new(core_peripherals.SYST, clocks.system_clock.freq().to_Hz());

    loop {
        info!("high");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("off");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}

struct DviPHY {

}

impl DviPHY {
    pub fn new() {
        pio_proc::pio_file!("src/test.pio");
        // pio::Assembler::new().
    }
}
