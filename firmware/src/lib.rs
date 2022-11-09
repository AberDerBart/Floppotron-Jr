#![no_main]
#![no_std]

pub mod floppy;
pub mod note_dict;
pub mod oscillators;

use cortex_m::interrupt::{self, CriticalSection};
use defmt::info;
use embedded_hal::digital::v2::OutputPin;
use midi_port::MidiMessage;
use oscillators::OSCILLATORS;
use rp_pico::hal::{
    pwm::Slices,
    uart::{self, UartConfig, UartDevice, UartPeripheral, ValidUartPinout},
};

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

pub fn listen_to_midi<D: UartDevice, P: ValidUartPinout<D>, IP: OutputPin>(
    uart: UartPeripheral<uart::Disabled, D, P>,
    mut p: IP,
) -> ! {
    let mut config = UartConfig::default();
    config.baudrate = embedded_time::rate::Baud(31250);
    config.data_bits = uart::DataBits::Eight;
    config.stop_bits = uart::StopBits::One;
    config.parity = None;

    info!("listening");
    let uart = uart
        .enable(config, embedded_time::rate::Hertz(125000000))
        .unwrap();

    let mut midi_in = midi_port::MidiInPort::new(uart);

    loop {
        midi_in.poll_uart();
        if let Some(msg) = midi_in.get_message() {
            interrupt::free(|cs| handle_midi_message(cs, msg, &mut p));
        }
    }
}

pub fn handle_midi_message<IP: OutputPin>(
    cs: &CriticalSection,
    msg: MidiMessage,
    indicator_pin: &mut IP,
) {
    let mut oscs = OSCILLATORS.borrow(cs).borrow_mut();
    match msg {
        midi_port::MidiMessage::NoteOn {
            channel,
            note,
            velocity: 0,
        } => {
            info!("note on event (0): {} {}", channel, note);
            indicator_pin.set_low().unwrap_or(());
            oscs.config.as_mut().unwrap().stop_note(note)
        }
        midi_port::MidiMessage::NoteOn {
            channel,
            note,
            velocity,
        } => {
            info!("note on event: {} {} {}", channel, note, velocity);
            indicator_pin.set_high().unwrap_or(());
            oscs.config.as_mut().unwrap().play_note(note)
        }
        midi_port::MidiMessage::NoteOff {
            channel,
            note,
            velocity,
        } => {
            info!("note off event: {} {} {}", channel, note, velocity);
            indicator_pin.set_low().unwrap_or(());
            oscs.config.as_mut().unwrap().stop_note(note)
        }
        midi_port::MidiMessage::ProgramChange {
            channel: _,
            program,
        } => match program {
            0 => oscs.to_single(),
            1 => oscs.to_inverse(),
            2 => oscs.to_unisono(),
            _ => (),
        },
        // TODO
        midi_port::MidiMessage::PitchBendChange { channel, value } => {
            info!("Pitchbend {} {}", channel, value);
        }
        _ => (),
    }
}
