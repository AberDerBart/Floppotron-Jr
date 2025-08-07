#ifndef NOTESTACK_H
#define NOTESTACK_H

#include <oscillator.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#define NO_NOTE 255

void noteStack_init();

void noteStack_push(uint8_t note, uint8_t velocity);
void noteStack_rm(uint8_t note);
void noteStack_clear();

void noteStack_set_oscillator(uint8_t note, struct oscillator* osc);

size_t noteStack_getTop(uint8_t* res, size_t n);

bool noteStack_is_empty();

uint8_t noteStack_get_velocity();

#endif
