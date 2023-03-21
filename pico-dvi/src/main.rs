#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _; // TODO: remove if you need 5kb of space, since panicking + formatting machinery is huge

use cortex_m::delay::Delay;
use defmt::{dbg, info};
use embedded_hal::digital::v2::OutputPin;
use rp_pico::{
    hal::{gpio::PinState, pwm, sio::Sio, watchdog::Watchdog, Clock},
    pac, Pins,
};

use crate::{
    clock::init_clocks,
    dvi::serializer::{DviClockPins, DviDataPins, DviSerializer},
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

    sysinfo(&peripherals.SYSINFO);

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

    rom();
    ram_x();
    ram_y();

    loop {
        info!("high");
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        info!("low");
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}

fn sysinfo(sysinfo: &pac::SYSINFO) {
    let is_fpga = sysinfo.platform.read().fpga().bit();
    let is_asic = sysinfo.platform.read().asic().bit();
    let git_hash = sysinfo.gitref_rp2040.read().bits();
    let manufacturer = sysinfo.chip_id.read().manufacturer().bits();
    let part = sysinfo.chip_id.read().part().bits();
    let revision = sysinfo.chip_id.read().revision().bits();

    info!(
        "SYSINFO
platform:
    FPGA: {}
    ASIC: {}
gitref_rp2040: {:x}
chip_id:
    manufacturer: {:X}
    part:         {}
    revision:     {}",
        is_fpga, is_asic, git_hash, manufacturer, part, revision
    );
}

// Functions and statics are placed in rom by default
fn rom() {
    dbg!(rom as fn() as *const ());
}

// This function will be placed in ram
#[link_section = scratch!(x, ram)]
fn ram_x() {
    dbg!(module_path!(), ram_x as fn() as *const ());
}

// This function will be placed in ram
#[link_section = scratch!(y, ram)]
fn ram_y() {
    dbg!(module_path!(), ram_y as fn() as *const ());
}

#[macro_export]
macro_rules! scratch {
    (x, $fn_name:ident) => {
        concat!(".scratch_x.", file!(), ".", line!(), ".", stringify!($fn_name))
    };
    (y, $fn_name:ident) => {
        concat!(".scratch_y.", file!(), ".", line!(), ".", stringify!($fn_name))
    };
}
