#ifndef OSCILLATOR_H
#define OSCILLATOR_H

#include <stdint.h>

#include "envelope.h"
#include "floppy.h"

#define N_OSCILLATORS 6

struct oscillator {
  uint8_t slice;
  struct floppy* floppy;
  uint8_t current_note;
  envelope_state_t envelope_state;
};

void oscillators_init(envelope_config_t* envelope_config);
void oscillators_set_pitchbend(uint16_t pitchbend, uint8_t scale);
void oscillators_set_envelope(envelope_config_t* envelope_config);
uint32_t oscillators_get_level();
void oscillators_force_stop_release_phase(uint8_t keep_note);

void oscillator_task();

void oscillator_stop(struct oscillator* osc);
void oscillator_force_stop(struct oscillator* osc);
void oscillator_set_note(struct oscillator* osc, uint8_t note, bool retrig);

extern struct oscillator oscillators[8];

#endif
