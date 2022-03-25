//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use core::ops::DerefMut;

use cortex_m::interrupt;
use cortex_m_rt::entry;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use embedded_time::fixed_point::FixedPoint;
use floppotron::{
    deactivate_slice_ints,
    floppy::{Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5},
    oscillators::OSCILLATORS,
};
use panic_probe as _;

// Provide an alias for our BSP so we can switch targets quickly.
// Uncomment the BSP you included in Cargo.toml, the rest of the code does not need to change.
use rp_pico as bsp;
// use sparkfun_pro_micro_rp2040 as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
};

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut slices = bsp::hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

    slices.pwm0.set_div_int(255);
    slices.pwm0.set_top(65465);
    slices.pwm0.enable();

    // let step_pin = pins.gpio26.into_push_pull_output();
    // let dir_pin = pins.gpio27.into_push_pull_output();
    // let en_pin = pins.gpio28.into_push_pull_output();

    let floppy0 = Floppy0::new(
        pins.gpio26.into_push_pull_output(),
        pins.gpio27.into_push_pull_output(),
        pins.gpio28.into_push_pull_output(),
    );

    let floppy1 = Floppy1::new(
        pins.gpio7.into_push_pull_output(),
        pins.gpio6.into_push_pull_output(),
        pins.gpio5.into_push_pull_output(),
    );

    let floppy2 = Floppy2::new(
        pins.gpio20.into_push_pull_output(),
        pins.gpio21.into_push_pull_output(),
        pins.gpio22.into_push_pull_output(),
    );

    let floppy3 = Floppy3::new(
        pins.gpio11.into_push_pull_output(),
        pins.gpio10.into_push_pull_output(),
        pins.gpio9.into_push_pull_output(),
    );

    let floppy4 = Floppy4::new(
        pins.gpio16.into_push_pull_output(),
        pins.gpio17.into_push_pull_output(),
        pins.gpio18.into_push_pull_output(),
    );

    let floppy5 = Floppy5::new(
        pins.gpio15.into_push_pull_output(),
        pins.gpio14.into_push_pull_output(),
        pins.gpio13.into_push_pull_output(),
    );

    let floppies = (floppy0, floppy1, floppy2, floppy3, floppy4, floppy5);

    deactivate_slice_ints(&mut slices);

    interrupt::free(|cs| {
        OSCILLATORS
            .borrow(cs)
            .borrow_mut()
            .deref_mut()
            .init(floppies, slices)
    });

    // enable PWM interrupt
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::PWM_IRQ_WRAP);
        interrupt::enable();
    }

    let mut led_pin = pins.led.into_push_pull_output();

    loop {
        info!("on!");
        led_pin.set_high().unwrap();
        delay.delay_ms(5);
        info!("off!");
        led_pin.set_low().unwrap();
        delay.delay_ms(5);
    }
}

// End of file
