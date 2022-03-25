use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use rp_pico as bsp;

use bsp::{
    hal::pwm::{Pwm0, Pwm1, Pwm2, Pwm3, Pwm4, Pwm5},
    pac::interrupt,
};
use rp_pico::hal::pwm::{FreeRunning, Slice, SliceId};

use crate::{
    floppy::{Floppies, Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5},
    note_dict::NOTE_DICT,
};
use cortex_m::interrupt as cortex_interrupt;

pub mod inverse;
pub mod single;
pub mod unisono;

use self::{inverse::InverseOscillator, single::SingleOscillator, unisono::UnisonoOscillator};

pub fn set_pwm_note<SID: SliceId>(pwm_slice: &mut Slice<SID, FreeRunning>, note: u8) {
    if let Some(pwm_setting) = NOTE_DICT.get(note as usize) {
        pwm_slice.set_div_int(pwm_setting.div_int);
        pwm_slice.set_top(pwm_setting.top);
        pwm_slice.enable();
        pwm_slice.enable_interrupt();
    } else {
        pwm_slice.disable();
    }
}

trait Oscillator {
    fn stop(&mut self);
    fn set_note(&mut self, note: u8);
    fn handle_interrupt(&mut self);
}

pub enum OscConfiguration {
    Single(
        (
            SingleOscillator<Floppy0, Pwm0>,
            SingleOscillator<Floppy1, Pwm1>,
            SingleOscillator<Floppy2, Pwm2>,
            SingleOscillator<Floppy3, Pwm3>,
            SingleOscillator<Floppy4, Pwm4>,
            SingleOscillator<Floppy5, Pwm5>,
        ),
    ),
    Unisono(
        (
            UnisonoOscillator,
            (
                Slice<Pwm1, FreeRunning>,
                Slice<Pwm2, FreeRunning>,
                Slice<Pwm3, FreeRunning>,
                Slice<Pwm4, FreeRunning>,
                Slice<Pwm5, FreeRunning>,
            ),
        ),
    ),
    Inverse(
        (
            InverseOscillator<Pwm0, Floppy0, Floppy1>,
            InverseOscillator<Pwm1, Floppy2, Floppy3>,
            InverseOscillator<Pwm2, Floppy4, Floppy5>,
        ),
        (
            Slice<Pwm3, FreeRunning>,
            Slice<Pwm4, FreeRunning>,
            Slice<Pwm5, FreeRunning>,
        ),
    ),
}

type OscSlices = (
    Slice<Pwm0, FreeRunning>,
    Slice<Pwm1, FreeRunning>,
    Slice<Pwm2, FreeRunning>,
    Slice<Pwm3, FreeRunning>,
    Slice<Pwm4, FreeRunning>,
    Slice<Pwm5, FreeRunning>,
);

impl OscConfiguration {
    pub fn free(self) -> (OscSlices, Floppies) {
        match self {
            OscConfiguration::Unisono((os, (s1, s2, s3, s4, s5))) => {
                let (s0, floppies) = os.free();
                return ((s0, s1, s2, s3, s4, s5), floppies);
            }
            OscConfiguration::Single((os0, os1, os2, os3, os4, os5)) => {
                let (f0, s0) = os0.free();
                let (f1, s1) = os1.free();
                let (f2, s2) = os2.free();
                let (f3, s3) = os3.free();
                let (f4, s4) = os4.free();
                let (f5, s5) = os5.free();

                return ((s0, s1, s2, s3, s4, s5), (f0, f1, f2, f3, f4, f5));
            }
            OscConfiguration::Inverse((os0, os1, os2), (s3, s4, s5)) => {
                let (s0, (f0, f1)) = os0.free();
                let (s1, (f2, f3)) = os1.free();
                let (s2, (f4, f5)) = os2.free();

                return ((s0, s1, s2, s3, s4, s5), (f0, f1, f2, f3, f4, f5));
            }
        }
    }

    pub fn handle_interrupt(&mut self) {
        match self {
            OscConfiguration::Unisono((os, _)) => os.handle_interrupt(),
            OscConfiguration::Single((f0, f1, f2, f3, f4, f5)) => {
                f0.handle_interrupt();
                f1.handle_interrupt();
                f2.handle_interrupt();
                f3.handle_interrupt();
                f4.handle_interrupt();
                f5.handle_interrupt();
            }
            OscConfiguration::Inverse((os0, os1, os2), _) => {
                os0.handle_interrupt();
                os1.handle_interrupt();
                os2.handle_interrupt();
            }
        }
    }

    pub fn new_single(slices: OscSlices, floppies: Floppies) -> Self {
        Self::Single((
            SingleOscillator::new(slices.0, floppies.0),
            SingleOscillator::new(slices.1, floppies.1),
            SingleOscillator::new(slices.2, floppies.2),
            SingleOscillator::new(slices.3, floppies.3),
            SingleOscillator::new(slices.4, floppies.4),
            SingleOscillator::new(slices.5, floppies.5),
        ))
    }

    pub fn new_unisono(slices: OscSlices, floppies: Floppies) -> Self {
        Self::Unisono((
            UnisonoOscillator::new(slices.0, floppies),
            (slices.1, slices.2, slices.3, slices.4, slices.5),
        ))
    }

    pub fn new_inverse(slices: OscSlices, floppies: Floppies) -> Self {
        Self::Inverse(
            (
                InverseOscillator::new(slices.0, (floppies.0, floppies.1)),
                InverseOscillator::new(slices.1, (floppies.2, floppies.3)),
                InverseOscillator::new(slices.2, (floppies.4, floppies.5)),
            ),
            (slices.3, slices.4, slices.5),
        )
    }
}

pub struct Oscillators {
    config: Option<OscConfiguration>,
}

impl Oscillators {
    pub fn handle_interrupt(&mut self) {
        if let Some(config) = &mut self.config {
            config.handle_interrupt();
        }
    }

    pub fn init(&mut self, floppies: Floppies, slices: OscSlices) {
        self.config = Some(OscConfiguration::new_single(slices, floppies))
    }

    pub fn to_single(&mut self) {
        if let Some(config) = self.config.take() {
            let (slices, floppies) = config.free();
            self.config = Some(OscConfiguration::new_single(slices, floppies))
        }
    }

    pub fn to_unisono(&mut self) {
        if let Some(config) = self.config.take() {
            let (slices, floppies) = config.free();
            self.config = Some(OscConfiguration::new_unisono(slices, floppies))
        }
    }

    pub fn to_inverse(&mut self) {
        if let Some(config) = self.config.take() {
            let (slices, floppies) = config.free();
            self.config = Some(OscConfiguration::new_inverse(slices, floppies))
        }
    }
}

pub static OSCILLATORS: cortex_interrupt::Mutex<RefCell<Oscillators>> =
    Mutex::new(RefCell::new(Oscillators { config: None }));

#[interrupt]
fn PWM_IRQ_WRAP() {
    cortex_interrupt::free(|cs| {
        OSCILLATORS
            .borrow(cs)
            .borrow_mut()
            .deref_mut()
            .handle_interrupt()
    })
}
