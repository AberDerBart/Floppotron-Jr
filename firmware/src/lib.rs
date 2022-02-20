#![no_main]
#![no_std]

use rp_pico::hal::pwm::Slices;
pub mod floppy;
pub mod note_dict;
pub mod oscillators;

pub fn deactivate_slice_ints(slices: &mut Slices) {
    slices.pwm0.disable_interrupt();
    slices.pwm1.disable_interrupt();
    slices.pwm2.disable_interrupt();
    slices.pwm3.disable_interrupt();
    slices.pwm4.disable_interrupt();
    slices.pwm5.disable_interrupt();
    slices.pwm6.disable_interrupt();
    slices.pwm7.disable_interrupt();
}
