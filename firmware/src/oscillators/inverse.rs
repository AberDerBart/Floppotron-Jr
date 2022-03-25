use rp_pico::hal::pwm::{FreeRunning, Slice, SliceId};

use crate::floppy::Floppy;

use super::{set_pwm_note, Oscillator};

pub struct InverseOscillator<S, F0, F1>
where
    S: SliceId,
    F0: Floppy,
    F1: Floppy,
{
    pwm_slice: Slice<S, FreeRunning>,
    floppies: (F0, F1),
}

impl<S, F0, F1> InverseOscillator<S, F0, F1>
where
    S: SliceId,
    F0: Floppy,
    F1: Floppy,
{
    pub fn new(pwm_slice: Slice<S, FreeRunning>, floppies: (F0, F1)) -> Self {
        Self {
            pwm_slice,
            floppies,
        }
    }

    pub fn free(self) -> (Slice<S, FreeRunning>, (F0, F1)) {
        (self.pwm_slice, self.floppies)
    }
}

impl<S, F0, F1> Oscillator for InverseOscillator<S, F0, F1>
where
    S: SliceId,
    F0: Floppy,
    F1: Floppy,
{
    fn stop(&mut self) {
        self.floppies.0.set_enabled(false).unwrap();
        self.floppies.1.set_enabled(false).unwrap();
    }

    fn set_note(&mut self, note: u8) {
        self.floppies.0.set_enabled(true).unwrap();
        self.floppies.1.set_enabled(true).unwrap();
        set_pwm_note(&mut self.pwm_slice, note);
    }

    fn handle_interrupt(&mut self) {
        if !self.pwm_slice.has_overflown() {
            return;
        }

        self.pwm_slice.clear_interrupt();

        let is_inverse = self.floppies.0.get_dir() != self.floppies.1.get_dir();

        self.floppies.0.step().unwrap();

        if is_inverse {
            self.floppies.1.step().unwrap();
        }
    }
}
