#ifndef ANALOG_OUT_H
#define ANALOG_OUT_H

#include <stdint.h>
#include <stdbool.h>

void out_init();

void set_velocity(uint8_t velocity);

void set_mod(uint8_t mod);

void set_trig(bool trig);

void set_gate(bool gate);

#endif