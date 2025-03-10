#pragma once
#include <stdbool.h>
#include <stdint.h>

typedef struct {
  uint8_t attack_time;
  uint8_t decay_time;
  uint8_t sustain_level;
  uint8_t release_time;

  uint16_t max_time_ms;
} envelope_config_t;

typedef enum {
  ENVELOPE_PHASE_ATTACK,
  ENVELOPE_PHASE_DECAY,
  ENVELOPE_PHASE_SUSTAIN,
  ENVELOPE_PHASE_RELEASE,
  ENVELOPE_PHASE_OFF,
} envelope_phase_t;

typedef struct {
  uint32_t attack_rate;
  uint32_t decay_rate;
  uint32_t sustain_level;
  uint32_t release_rate;

  envelope_phase_t phase;
  uint32_t level;

  uint32_t last_tick_time_ms;
} envelope_state_t;

envelope_state_t envelope_state_default();

void envelope_config_apply(envelope_state_t* state, envelope_config_t* config);

envelope_phase_t envelope_progress(envelope_state_t* state);
void envelope_trigger(envelope_state_t* state);
void envelope_stop(envelope_state_t* state);
void envelope_force_stop(envelope_state_t* state);
