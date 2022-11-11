//! Blinks the LED on a Pico board
//!
//! This will blink an LED attached to GP25, which is the pin the Pico uses for the on-board LED.
#![no_std]
#![no_main]

use core::ops::DerefMut;

use bsp::entry;
use cortex_m::interrupt;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use floppotron_jr::{
    deactivate_slice_ints,
    floppy::{Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5},
    listen_to_midi,
    oscillators::{OscConfiguration, OSCILLATORS},
};
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

    let slices = (
        slices.pwm0,
        slices.pwm1,
        slices.pwm2,
        slices.pwm3,
        slices.pwm4,
        slices.pwm5,
    );

    info!("unmasked");

    let osc_config = OscConfiguration::new_single(slices, floppies);
    info!("config initialized");

    interrupt::free(|cs| {
        OSCILLATORS
            .borrow(cs)
            .borrow_mut()
            .deref_mut()
            .init_with_config(osc_config)
    });

    let uart_pins = (
        pins.gpio0.into_mode::<bsp::hal::gpio::FunctionUart>(),
        pins.gpio1.into_mode::<bsp::hal::gpio::FunctionUart>(),
    );
    let uart0 = bsp::hal::uart::UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS);

    info!("freq: {}", clocks.peripheral_clock.freq().0,);

    let led_pin = pins.led.into_push_pull_output();

    // enable PWM interrupt
    unsafe {
        info!("unmask");
        pac::NVIC::unmask(pac::Interrupt::PWM_IRQ_WRAP); // infinite loop?
        info!("enable");
        interrupt::enable(); // infinite loop?
    }

    listen_to_midi(uart0, led_pin);
}

// End of file
