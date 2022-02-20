use core::{convert::Infallible, fmt::Debug};
use embedded_hal::digital::v2::{OutputPin, PinState};
use rp_pico::hal::gpio::{bank0::*, Output, Pin, PushPull};

enum FloppyDirection {
    Forward,
    Backward,
}

pub trait Floppy {
    fn set_enabled(&mut self, enabled: bool) -> Result<(), FloppyError>;
    fn step(&mut self) -> Result<(), FloppyError>;
}

pub struct FloppyImpl<S, D, E>
where
    S: OutputPin<Error = Infallible>,
    D: OutputPin<Error = Infallible>,
    E: OutputPin<Error = Infallible>,
{
    pin_step: S,
    pin_dir: D,
    pin_en: E,

    enabled: bool,
    track_index: u8,
    step_state: PinState,
    dir: FloppyDirection,
}

#[derive(Debug)]
pub enum FloppyError {
    Disabled,
}

impl<S, D, E> FloppyImpl<S, D, E>
where
    S: OutputPin<Error = Infallible>,
    D: OutputPin<Error = Infallible>,
    E: OutputPin<Error = Infallible>,
{
    pub fn new(pin_step: S, pin_dir: D, pin_en: E) -> Self {
        Self {
            pin_step,
            pin_dir,
            pin_en,

            enabled: false,
            track_index: 80,
            step_state: PinState::Low,
            dir: FloppyDirection::Backward,
        }
    }
}

impl<S, D, E> Floppy for FloppyImpl<S, D, E>
where
    S: OutputPin<Error = Infallible>,
    D: OutputPin<Error = Infallible>,
    E: OutputPin<Error = Infallible>,
{
    fn set_enabled(&mut self, enabled: bool) -> Result<(), FloppyError> {
        self.pin_en
            .set_state(match enabled {
                true => PinState::High,
                false => PinState::Low,
            })
            .unwrap();
        self.enabled = enabled;
        Ok(())
    }

    fn step(&mut self) -> Result<(), FloppyError> {
        if !self.enabled {
            return Err(FloppyError::Disabled);
        }

        self.step_state = !self.step_state;

        if self.step_state == PinState::Low {
            match self.dir {
                FloppyDirection::Forward => {
                    self.track_index += 1;
                    if self.track_index >= 79 {
                        self.dir = FloppyDirection::Backward;
                        self.pin_dir.set_low().unwrap();
                    }
                }
                FloppyDirection::Backward => {
                    self.track_index -= 1;
                    if self.track_index <= 0 {
                        self.dir = FloppyDirection::Forward;
                        self.pin_dir.set_high().unwrap();
                    }
                }
            }
        }

        self.pin_step.set_state(self.step_state).unwrap();
        Ok(())
    }
}

pub type Floppy0 = FloppyImpl<
    Pin<Gpio26, Output<PushPull>>,
    Pin<Gpio27, Output<PushPull>>,
    Pin<Gpio28, Output<PushPull>>,
>;

pub type Floppy1 = FloppyImpl<
    Pin<Gpio7, Output<PushPull>>,
    Pin<Gpio6, Output<PushPull>>,
    Pin<Gpio5, Output<PushPull>>,
>;

pub type Floppy2 = FloppyImpl<
    Pin<Gpio20, Output<PushPull>>,
    Pin<Gpio21, Output<PushPull>>,
    Pin<Gpio22, Output<PushPull>>,
>;

pub type Floppy3 = FloppyImpl<
    Pin<Gpio11, Output<PushPull>>,
    Pin<Gpio10, Output<PushPull>>,
    Pin<Gpio9, Output<PushPull>>,
>;

pub type Floppy4 = FloppyImpl<
    Pin<Gpio16, Output<PushPull>>,
    Pin<Gpio17, Output<PushPull>>,
    Pin<Gpio18, Output<PushPull>>,
>;

pub type Floppy5 = FloppyImpl<
    Pin<Gpio15, Output<PushPull>>,
    Pin<Gpio14, Output<PushPull>>,
    Pin<Gpio13, Output<PushPull>>,
>;

pub struct Floppies {
    pub floppy0: Option<Floppy0>,
}

// pub static FLOPPIES: Mutex<RefCell<Option<Floppy0>>> = Mutex::new(RefCell::new(None));
