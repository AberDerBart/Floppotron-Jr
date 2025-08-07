#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "midi.h"

void midi_ble_init();

void midi_ble_write(uint8_t len, uint8_t *msg);

bool midi_ble_try_read(struct midi_packet *packet);
