#ifndef _MIDI_H_
#define _MIDI_H_

#include <stdbool.h>
#include <stdint.h>

#define MIDI_NOTE_ON 0x90
#define MIDI_NOTE_OFF 0x80
#define MIDI_PITCHBEND 0xe0
#define MIDI_CONTROL_CHANGE 0xb0

#define MIDI_CC_MODULATION_WHEEL 1
#define MIDI_CC_ALL_SOUND_OFF 120
#define MIDI_CC_ALL_NOTES_OFF 123

#define MIDI_CC_SOUND_CONTROLLER_3_RELEASE 72
#define MIDI_CC_SOUND_CONTROLLER_4_ATTACK 73
#define MIDI_CC_SOUND_CONTROLLER_6_DECAY 75
#define MIDI_CC_SOUND_CONTROLLER_7_SUSTAIN 76

void midi_init();

struct midi_packet {
  uint8_t status;
  uint8_t b1;
  uint8_t b2;
};

void midi_write(uint8_t cmd, uint8_t b1, uint8_t b2);
struct midi_packet midi_read();

bool midi_is_status_byte(uint8_t byte);

#endif
