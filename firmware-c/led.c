#include "led.h"
#include "pico/stdlib.h"

const uint LED_PIN = 25;

static uint8_t state = 0;

void led_init()
{
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, GPIO_OUT);
    led_set(false);
}

void led_set(uint8_t new_state)
{
    state = new_state;
    gpio_put(LED_PIN, state);
}
