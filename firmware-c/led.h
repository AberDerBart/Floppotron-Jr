#ifndef LED_H
#define LED_H
#include <stdint.h>

void led_init();
void led_set(uint8_t new_state);

#endif