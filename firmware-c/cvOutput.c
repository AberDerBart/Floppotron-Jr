#include "cvOutput.h"

#include <stdbool.h>

#include "hardware/pwm.h"
#include "pico/stdlib.h"

#define VELOCITY_PIN 2
#define MOD_PIN 3

#define GATE_PIN 26
#define TRIG_PIN 4

#define A_OUT_PWM_SLICE 1

#define VELOCITY_CHAN PWM_CHAN_B
#define MOD_CHAN PWM_CHAN_A

#define TRIG_ALARM_NUM 0
#define TRIG_DURATION_US 5000

static void reset_trig() { gpio_put(TRIG_PIN, false); }

void out_init() {
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

  // init alarm for turning off trigger signal
  hardware_alarm_claim(TRIG_ALARM_NUM);
  hardware_alarm_set_callback(TRIG_ALARM_NUM, reset_trig);
}

void set_velocity(uint8_t velocity) {
  const uint16_t level = velocity << 9;
  pwm_set_chan_level(A_OUT_PWM_SLICE, VELOCITY_CHAN, level);
}

void set_mod(uint8_t mod) {
  const uint16_t level = mod << 9;
  pwm_set_chan_level(A_OUT_PWM_SLICE, MOD_CHAN, level);
}

void pulse_trig() {
  absolute_time_t current_time = time_us_64();
  hardware_alarm_set_target(TRIG_ALARM_NUM, current_time + TRIG_DURATION_US);
  gpio_put(TRIG_PIN, true);
}

void set_gate(bool gate) { gpio_put(GATE_PIN, gate); }