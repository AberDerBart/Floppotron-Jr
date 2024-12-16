#ifndef OSCILLATOR_H
#define OSCILLATOR_H

#include <stdint.h>

#include "floppy.h"

#define N_OSCILLATORS 6

void oscillator_init();
void oscillator_set_note(uint8_t slice, uint8_t note);
void oscillator_stop(uint8_t slice);
void oscillator_set_pitchbend(uint16_t pitchbend, uint8_t scale);

struct oscillator {
    uint8_t slice;
    struct floppy* floppy;
    uint8_t current_note;
};

extern struct oscillator oscillators[8];

#endif
