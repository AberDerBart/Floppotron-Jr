#include "pico/stdlib.h"

#include <stdio.h>
#include "led.h"
#include "midi.h"
#include "noteStack.h"
#include "oscillator.h"
#include "dispatcher.h"
#include "cvOutput.h"
#include "midi_ble.h"

void midi_task();

int main(void)
{
    midi_init();
    led_init();
    noteStack_init();
    oscillator_init();
    dispatcher_init();
    out_init();
    midi_ble_init();

    stdio_init_all();

    printf("init\n");

    while (1)
    {
        midi_task();
    }

    return 0;
}

void handle_midi_packet(struct midi_packet packet)
{
    switch (packet.status & 0xf0)
    {
    case MIDI_NOTE_ON:
        if (packet.b2 != 0)
        {
            noteStack_push(packet.b1, packet.b2);
            pulse_trig();
        }
        else
        {
            noteStack_rm(packet.b1);
        }
        set_velocity(noteStack_get_velocity());
        break;
    case MIDI_NOTE_OFF:
        noteStack_rm(packet.b1);
        set_velocity(noteStack_get_velocity());
        break;
    case MIDI_PITCHBEND:
        uint16_t pitchbend_val = packet.b1 | packet.b2 << 7;
        oscillators_set_pitchbend(pitchbend_val, 2);
        break;
    case MIDI_CONTROL_CHANGE:
        switch (packet.b1)
        {
        case MIDI_CC_ALL_NOTES_OFF:
            noteStack_clear();
            break;
        case MIDI_CC_MODULATION_WHEEL:
            set_mod(packet.b2);
            break;
        default:
            break;
        }
    default:
        break;
    }

    set_gate(!noteStack_is_empty());

    dispatcher_run();
}

void midi_task()
{
    struct midi_packet packet;

    if (midi_ble_try_read(&packet))
    {
        handle_midi_packet(packet);
    }
}
