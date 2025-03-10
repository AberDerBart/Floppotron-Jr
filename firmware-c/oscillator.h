#ifndef OSCILLATOR_H
#define OSCILLATOR_H

#include <stdint.h>

#include "envelope.h"
#include "floppy.h"

#define N_OSCILLATORS 6

void oscillators_init(envelope_config_t* envelope_config);
void oscillators_set_pitchbend(uint16_t pitchbend, uint8_t scale);
void oscillators_set_envelope(envelope_config_t* envelope_config);
uint32_t oscillators_get_level();

void oscillator_task();

void oscillator_by_index_set_note(uint8_t slice, uint8_t note, bool retrig);
void oscillator_by_index_force_stop(uint8_t slice);

struct oscillator {
  uint8_t slice;
  struct floppy* floppy;
  uint8_t current_note;
  envelope_state_t envelope_state;
};

extern struct oscillator oscillators[8];

#endif
