use rp_pico::hal::pwm::{FreeRunning, Slice, SliceId};

use crate::floppy::Floppy;

use super::{set_pwm_note, Oscillator};

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

    pub fn free(mut self) -> (F, Slice<SID, FreeRunning>) {
        self.stop();
        (self.floppy, self.pwm_slice)
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
