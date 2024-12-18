#include "cvOutput.h"
#include "hardware/pwm.h"

#include "pico/stdlib.h"
#include <stdbool.h>

#define A_OUT_PWM_SLICE 1

#define VELOCITY_CHAN PWM_CHAN_B
#define MOD_CHAN PWM_CHAN_A

#define VELOCITY_PIN 2
#define MOD_PIN 3

#define GATE_PIN 26
#define TRIG_PIN 4

void out_init()
{
    gpio_init(GATE_PIN);
    gpio_set_dir(GATE_PIN, true);

    gpio_init(TRIG_PIN);
    gpio_set_dir(TRIG_PIN, true);

    // init PWM for velocity and mod
    pwm_set_irq_enabled(A_OUT_PWM_SLICE, false);

    pwm_set_wrap(A_OUT_PWM_SLICE, UINT16_MAX);

    set_velocity(0);
    set_mod(0);

    gpio_set_function(VELOCITY_PIN, GPIO_FUNC_PWM);
    gpio_set_function(MOD_PIN, GPIO_FUNC_PWM);

    pwm_set_enabled(A_OUT_PWM_SLICE, true);
}

void set_velocity(uint8_t velocity)
{
    const uint16_t level = velocity << 9;
    pwm_set_chan_level(A_OUT_PWM_SLICE, VELOCITY_CHAN, level);
}

void set_mod(uint8_t mod)
{
    const uint16_t level = mod << 9;
    pwm_set_chan_level(A_OUT_PWM_SLICE, MOD_CHAN, level);
}

void set_trig(bool trig)
{
    gpio_put(TRIG_PIN, trig);
}

void set_gate(bool gate)
{
    gpio_put(GATE_PIN, gate);
}