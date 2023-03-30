#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use {defmt_rtt as _, panic_probe as _}; // Panic handler and defmt logger

#[cfg(all(not(feature = "systick"), not(feature = "rp2040")))]
compile_error!("one of `systick` or `rp2040` must be enabled");

#[cfg(all(feature = "systick", feature = "rp2040"))]
compile_error!("only one of `systick` or `rp2040` can be enabled");

#[rtic::app(device = rp_pico::hal::pac, dispatchers = [SW0_IRQ, SW1_IRQ, SW2_IRQ, SW3_IRQ, SW4_IRQ, SW5_IRQ])]
mod app {
    use rp_pico::{
        hal::{self, rom_data},
        pac,
    };
    use rtic_monotonics::Monotonic;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(context: init::Context) -> (Shared, Local) {
        // Soft-reset does not release the hardware spinlocks
        // Release them now to avoid a deadlock after debug or watchdog reset
        unsafe {
            hal::sio::spinlock_reset();
        }

        let rom_version = rom_data::rom_version_number();
        let git_revision = rom_data::git_revision();
        let copyright = rom_data::copyright_string();

        defmt::info!(
            "RP2040-B{=u8} (ROM {=u32:x}) {=str}",
            rom_version,
            git_revision,
            copyright
        );

        let core_peripherals: pac::CorePeripherals = context.core;
        let mut peripherals: pac::Peripherals = context.device;

        let mut watchdog = hal::Watchdog::new(peripherals.WATCHDOG);
        let _clocks = hal::clocks::init_clocks_and_plls(
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

        #[cfg(feature = "systick")]
        rtic_monotonics::systick::Systick::start(
            core_peripherals.SYST,
            clocks.system_clock.freq().to_Hz(),
            rtic_monotonics::create_systick_token!(),
        );

        #[cfg(feature = "rp2040")]
        rtic_monotonics::rp2040::Timer::start(
            peripherals.TIMER,
            &mut peripherals.RESETS,
            rtic_monotonics::create_rp2040_monotonic_token!(),
        );

        one_second::spawn().unwrap();
        three_seconds::spawn().unwrap();

        (Shared {}, Local {})
    }

    #[idle]
    fn idle(context: idle::Context) -> ! {
        loop {
            cortex_m::asm::wfe();
        }
    }

    #[cfg(feature = "systick")]
    type Delay = rtic_monotonics::systick::Systick;

    #[cfg(feature = "rp2040")]
    type Delay = rtic_monotonics::rp2040::Timer;

    defmt::timestamp!(
        "{:us}",
        <Delay as rtic_monotonics::Monotonic>::now()
            .duration_since_epoch()
            .to_micros()
    );

    type Duration = <Delay as Monotonic>::Duration;

    #[task]
    async fn three_seconds(context: three_seconds::Context) {
        let mut delay_until = Delay::now();

        loop {
            delay_until += Duration::millis(3000);

            Delay::delay_until(delay_until).await;

            defmt::info!("Three Second");
        }
    }

    #[task]
    async fn one_second(context: one_second::Context) {
        let mut delay_until = Delay::now();

        loop {
            delay_until += Duration::millis(1000);

            Delay::delay_until(delay_until).await;

            defmt::info!("One Second");
        }
    }
}
