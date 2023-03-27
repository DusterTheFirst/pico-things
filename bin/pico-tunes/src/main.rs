#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)] // Needed for rtic 2.0

use {defmt_rtt as _, panic_probe as _}; // Panic handler and defmt logger

mod music;

#[rtic::app(device = rp_pico::hal::pac, dispatchers = [SW0_IRQ, SW1_IRQ, SW2_IRQ, SW3_IRQ, SW4_IRQ, SW5_IRQ])]
mod app {
    use defmt::info;
    use embedded_hal::{digital::v2::PinState, prelude::*};
    use futures_util::future::join;
    use rp_pico::{
        hal::{
            clocks,
            gpio::{
                bank0::{Gpio15, Gpio16, Gpio17},
                Output, OutputDriveStrength, OutputSlewRate, Pin, PushPull,
            },
            pwm, rom_data, Sio, Watchdog,
        },
        pac,
    };
    use rtic_monotonics::systick::Systick;

    use crate::music::songs::{NUMB_LITTLE_BUG, THE_GOOD_LIFE};

    defmt::timestamp!(
        "{=u32:us}",
        <Systick as rtic_monotonics::Monotonic>::now()
            .duration_since_epoch()
            .to_micros()
    );

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        pwm_left: pwm::Slice<pwm::Pwm4, pwm::FreeRunning>,
        pwm_right: pwm::Slice<pwm::Pwm5, pwm::FreeRunning>,
    }

    #[init]
    fn init(context: init::Context) -> (Shared, Local) {
        // Soft-reset does not release the hardware spinlocks
        // Release them now to avoid a deadlock after debug or watchdog reset
        unsafe {
            rp_pico::hal::sio::spinlock_reset();
        }

        let rom_version = rom_data::rom_version_number();
        let git_revision = rom_data::git_revision();
        let copyright = rom_data::copyright_string();

        info!(
            "RP2040-B{=u8} (ROM {=u32:x}) {=str}",
            rom_version, git_revision, copyright
        );

        let core_peripherals: pac::CorePeripherals = context.core;
        let mut peripherals: pac::Peripherals = context.device;

        let mut watchdog = Watchdog::new(peripherals.WATCHDOG);
        let clocks = clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            peripherals.XOSC,
            peripherals.CLOCKS,
            peripherals.PLL_SYS,
            peripherals.PLL_USB,
            &mut peripherals.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        let single_cycle_io = Sio::new(peripherals.SIO);
        let pins = rp_pico::Pins::new(
            peripherals.IO_BANK0,
            peripherals.PADS_BANK0,
            single_cycle_io.gpio_bank0,
            &mut peripherals.RESETS,
        );

        Systick::start(
            core_peripherals.SYST,
            125_000_000,
            rtic_monotonics::make_systick_handler!(),
        );

        let pwm_slices = pwm::Slices::new(peripherals.PWM, &mut peripherals.RESETS);

        play_song::spawn().unwrap();

        (
            Shared {},
            Local {
                pwm_left: {
                    let mut pwm_left = pwm_slices.pwm4;
                    pwm_left.disable();
                    pwm_left.output_to(pins.gpio9);
                    pwm_left.set_ph_correct();
                    pwm_left.set_div_int(255);
                    pwm_left.set_div_frac(16);

                    pwm_left
                },
                pwm_right: {
                    let mut pwm_right = pwm_slices.pwm5;
                    pwm_right.disable();
                    pwm_right.output_to(pins.gpio10);
                    pwm_right.set_ph_correct();
                    pwm_right.set_div_int(255);
                    pwm_right.set_div_frac(16);

                    pwm_right
                },
            },
        )
    }

    #[idle]
    fn idle(context: idle::Context) -> ! {
        loop {
            // core::hint::spin_loop();
            // Now Wait For Interrupt is used instead of a busy-wait loop
            // to allow MCU to sleep between interrupts
            // https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Instruction-Details/Alphabetical-list-of-instructions/WFI
            rtic::export::wfi()
        }
    }

    #[task(local = [pwm_left, pwm_right], priority = 1)]
    async fn play_song(context: play_song::Context) {
        let play_song::LocalResources {
            pwm_left,
            pwm_right,
            ..
        } = context.local;

        // let song = &THE_GOOD_LIFE;
        let song = &NUMB_LITTLE_BUG;

        info!("Playing \"{=str}\" @ {=u16} bpm", song.title, song.bpm);

        pwm_left.enable();
        let mut set_left_top = |top| {
            if top == 0 {
                pwm_left.channel_b.set_duty(0);
            } else {
                pwm_left.set_top(top);
                pwm_left.channel_b.set_duty(top / 2);
            }
        };
        let left_future = crate::music::play_notes(&mut set_left_top, song.bpm, song.tracks[0]);

        pwm_right.enable();
        let mut set_right_top = |top| {
            if top == 0 {
                pwm_right.channel_a.set_duty(0);
            } else {
                pwm_right.set_top(top);
                pwm_right.channel_a.set_duty(top / 2);
            }
        };
        let right_future = crate::music::play_notes(&mut set_right_top, song.bpm, song.tracks[1]);

        join(left_future, right_future).await;
        pwm_left.disable();
        pwm_right.disable();

        info!("Done...");
    }
}
