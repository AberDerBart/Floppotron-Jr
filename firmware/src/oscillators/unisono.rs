use rp_pico::hal::pwm::{FreeRunning, Pwm0, Slice};

use crate::floppy::{Floppy, Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5};

use super::{set_pwm_note, Oscillator};

pub struct UnisonoOscillator {
    pwm_slice: Slice<Pwm0, FreeRunning>,
    floppies: (Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5),
}

impl UnisonoOscillator {
    pub fn new(
        pwm_slice: Slice<Pwm0, FreeRunning>,
        floppies: (Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5),
    ) -> Self {
        Self {
            pwm_slice,
            floppies,
        }
    }

    fn all_floppies(&mut self, f: fn(&mut dyn Floppy) -> ()) {
        f(&mut self.floppies.0);
        f(&mut self.floppies.1);
        f(&mut self.floppies.2);
        f(&mut self.floppies.3);
        f(&mut self.floppies.4);
        f(&mut self.floppies.5);
    }

    pub fn free(
        self,
    ) -> (
        Slice<Pwm0, FreeRunning>,
        (Floppy0, Floppy1, Floppy2, Floppy3, Floppy4, Floppy5),
    ) {
        (self.pwm_slice, self.floppies)
    }
}

impl Oscillator for UnisonoOscillator {
    fn stop(&mut self) {
        self.all_floppies(|f| f.set_enabled(false).unwrap());
    }

    fn set_note(&mut self, note: u8) {
        self.all_floppies(|f| f.set_enabled(true).unwrap());
        set_pwm_note(&mut self.pwm_slice, note);
    }

    fn handle_interrupt(&mut self) {
        if !self.pwm_slice.has_overflown() {
            return;
        }

        self.pwm_slice.clear_interrupt();

        self.all_floppies(|f| f.step().unwrap());
    }
}
