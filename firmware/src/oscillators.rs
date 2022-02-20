use core::{cell::RefCell, ops::DerefMut};

use cortex_m::interrupt::Mutex;
use rp_pico as bsp;

use bsp::{hal::pwm::Pwm0, pac::interrupt};
use rp_pico::hal::pwm::{FreeRunning, Slice, SliceId};

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
    Unisono(UnisonoOscillator),
}

impl OscConfiguration {
    pub fn free(
        self,
    ) -> (
        Slice<Pwm0, FreeRunning>,
        (Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5),
    ) {
        match self {
            OscConfiguration::Unisono(os) => os.free(),
        }
    }

    pub fn handle_interrupt(&mut self) {
        match self {
            OscConfiguration::Unisono(os) => os.handle_interrupt(),
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
        pwm0: Slice<Pwm0, FreeRunning>,
    ) {
        let mut osc0 = UnisonoOscillator::new(pwm0, floppies);
        osc0.set_note(41);
        self.config = Some(OscConfiguration::Unisono(osc0));
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
