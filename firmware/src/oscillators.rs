use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use defmt::info;
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
    fn get_note(&self) -> Option<u8>;
    fn get_age(&self) -> u8;
    fn set_age(&mut self, age: u8);
}

type OscSlices = (
    Slice<Pwm0, FreeRunning>,
    Slice<Pwm1, FreeRunning>,
    Slice<Pwm2, FreeRunning>,
    Slice<Pwm3, FreeRunning>,
    Slice<Pwm4, FreeRunning>,
    Slice<Pwm5, FreeRunning>,
);

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
        UnisonoOscillator,
        (
            Slice<Pwm1, FreeRunning>,
            Slice<Pwm2, FreeRunning>,
            Slice<Pwm3, FreeRunning>,
            Slice<Pwm4, FreeRunning>,
            Slice<Pwm5, FreeRunning>,
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

impl OscConfiguration {
    pub fn free(self) -> (OscSlices, Floppies) {
        match self {
            OscConfiguration::Unisono(os, (s1, s2, s3, s4, s5)) => {
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

    fn for_each<F: Fn(&mut dyn Oscillator)>(&mut self, func: F) {
        self.find(|osc| {
            func(osc);
            false
        });
    }

    fn find<F: Fn(&mut dyn Oscillator) -> bool>(&mut self, func: F) -> Option<&mut dyn Oscillator> {
        match self {
            OscConfiguration::Single(oss) => {
                if func(&mut oss.0) {
                    return Some(&mut oss.0);
                }
                if func(&mut oss.1) {
                    return Some(&mut oss.1);
                }
                if func(&mut oss.2) {
                    return Some(&mut oss.2);
                }
                if func(&mut oss.3) {
                    return Some(&mut oss.3);
                }
                if func(&mut oss.4) {
                    return Some(&mut oss.4);
                }
                if func(&mut oss.5) {
                    return Some(&mut oss.5);
                }
                None
            }
            OscConfiguration::Unisono(os, _) => {
                if func(os) {
                    return Some(os);
                }
                return None;
            }
            OscConfiguration::Inverse(oss, _) => {
                if func(&mut oss.0) {
                    return Some(&mut oss.0);
                }
                if func(&mut oss.1) {
                    return Some(&mut oss.1);
                }
                if func(&mut oss.2) {
                    return Some(&mut oss.2);
                }
                None
            }
        }
    }

    pub fn oscillator_count(&self) -> u8 {
        match self {
            OscConfiguration::Single(_) => 6,
            OscConfiguration::Unisono(_, _) => 1,
            OscConfiguration::Inverse(_, _) => 3,
        }
    }

    pub fn stop_note(&mut self, note: u8) {
        info!("stopping note");
        if let Some(active_osc) = self.find(|osc| osc.get_note() == Some(note)) {
            let active_age = active_osc.get_age();
            active_osc.stop();

            self.for_each(|osc| {
                if let Some(osc_note) = osc.get_note() {
                    let age = osc.get_age();
                    if osc_note == note {
                        osc.set_age(0);
                    } else if age > active_age {
                        osc.set_age(age - 1);
                    }
                }
            })
        }
    }

    pub fn play_note(&mut self, note: u8) {
        info!("playing note {}", note);
        if let Some(active_osc) = self.find(|osc| osc.get_note() == Some(note)) {
            // retrigger
            let active_age = active_osc.get_age();
            active_osc.set_note(note);

            self.for_each(|osc| {
                if let Some(osc_note) = osc.get_note() {
                    let age = osc.get_age();
                    if age < active_age {
                        osc.set_age(age + 1);
                    } else if osc_note == note {
                        osc.set_age(0);
                    }
                }
            });
            return;
        }

        if let Some(free_osc) = self.find(|osc| osc.get_note() == None) {
            free_osc.set_note(note);

            self.for_each(|osc| {
                if let Some(osc_note) = osc.get_note() {
                    if osc_note == note {
                        osc.set_age(0);
                    } else {
                        osc.set_age(osc.get_age() + 1);
                    }
                }
            });
            return;
        }

        let osc_count = self.oscillator_count();
        if let Some(oldest_osc) = self.find(|osc| osc.get_age() >= osc_count) {
            oldest_osc.set_note(note);

            self.for_each(|osc| {
                if let Some(osc_note) = osc.get_note() {
                    if osc_note == note {
                        osc.set_age(0);
                    } else {
                        osc.set_age(osc.get_age() + 1);
                    }
                }
            })
        }
    }

    pub fn handle_interrupt(&mut self) {
        self.for_each(|os| os.handle_interrupt());
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
        Self::Unisono(
            UnisonoOscillator::new(slices.0, floppies),
            (slices.1, slices.2, slices.3, slices.4, slices.5),
        )
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
    pub config: Option<OscConfiguration>,
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

    pub fn init_with_config(&mut self, config: OscConfiguration) {
        self.config = Some(config);
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
    });
}
