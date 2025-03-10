#pragma once
#include <stdbool.h>
#include <stdint.h>

typedef struct {
  uint8_t attack_time;
  uint8_t decay_time;
  uint8_t sustain_level;
  uint8_t release_time;
} envelope_config_t;

typedef enum {
  ATTACK,
  DECAY,
  SUSTAIN,
  RELEASE,
  OFF,
} envelope_phase_t;

typedef struct {
  uint32_t attack_rate;
  uint32_t decay_rate;
  uint32_t sustain_level;
  uint32_t release_rate;

  envelope_phase_t phase;
  uint32_t level;
} envelope_state_t;

void envelope_task();

envelope_state_t envelope_state_default();

void envelope_config_apply(envelope_state_t* state, envelope_config_t* config);

void envelope_progress(envelope_state_t* state);
void envelope_trigger(envelope_state_t* state);
void envelope_stop(envelope_state_t* state);
void envelope_force_stop(envelope_state_t* state);
