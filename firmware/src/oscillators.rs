use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use rp_pico as bsp;

use bsp::{
    hal::pwm::{Pwm0, Pwm1, Pwm2, Pwm3, Pwm4, Pwm5, Slices},
    pac::interrupt,
};
use rp_pico::hal::pwm::{FreeRunning, Slice, SliceId};

pub mod inverse;
pub mod unisono;

use crate::{
    floppy::{Floppy, Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5},
    note_dict::NOTE_DICT,
};
use cortex_m::interrupt as cortex_interrupt;

use self::unisono::UnisonoOscillator;

trait Oscillator {
    fn stop(&mut self);
    fn set_note(&mut self, note: u8);
    fn handle_interrupt(&mut self);
}

pub struct SingleOscillator<F, SID>
where
    SID: SliceId,
    F: Floppy,
{
    floppy: F,
    pwm_slice: Slice<SID, FreeRunning>,
}

impl<F, SID> SingleOscillator<F, SID>
where
    SID: SliceId,
    F: Floppy,
{
    pub fn new(mut pwm: Slice<SID, FreeRunning>, floppy: F) -> Self {
        pwm.enable_interrupt();
        Self {
            pwm_slice: pwm,
            floppy,
        }
    }

    pub fn step(&mut self) {
        self.floppy.step().unwrap();
    }

    pub fn free(self) -> (F, Slice<SID, FreeRunning>) {
        (self.floppy, self.pwm_slice)
    }
}

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

impl<F, SID> Oscillator for SingleOscillator<F, SID>
where
    SID: SliceId,
    F: Floppy,
{
    fn stop(&mut self) {
        self.floppy.set_enabled(false).unwrap();
        self.pwm_slice.disable();
    }

    fn set_note(&mut self, note: u8) {
        self.floppy.set_enabled(true).unwrap();
        set_pwm_note(&mut self.pwm_slice, note);
    }

    fn handle_interrupt(&mut self) {
        if !self.pwm_slice.has_overflown() {
            return;
        }

        self.pwm_slice.clear_interrupt();
        self.floppy.step().unwrap();
    }
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
}

impl OscConfiguration {
    pub fn free(
        self,
    ) -> (
        (
            Slice<Pwm0, FreeRunning>,
            Slice<Pwm1, FreeRunning>,
            Slice<Pwm2, FreeRunning>,
            Slice<Pwm3, FreeRunning>,
            Slice<Pwm4, FreeRunning>,
            Slice<Pwm5, FreeRunning>,
        ),
        (Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5),
    ) {
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
        }
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

    pub fn init(
        &mut self,
        floppies: (Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5),
        slices: Slices,
    ) {
        let osc0 = SingleOscillator::new(slices.pwm0, floppies.0);
        let osc1 = SingleOscillator::new(slices.pwm1, floppies.1);
        let osc2 = SingleOscillator::new(slices.pwm2, floppies.2);
        let osc3 = SingleOscillator::new(slices.pwm3, floppies.3);
        let osc4 = SingleOscillator::new(slices.pwm4, floppies.4);
        let osc5 = SingleOscillator::new(slices.pwm5, floppies.5);

        self.config = Some(OscConfiguration::Single((
            osc0, osc1, osc2, osc3, osc4, osc5,
        )));
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
