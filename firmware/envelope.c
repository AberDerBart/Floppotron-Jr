#include "envelope.h"

#include <pico/time.h>

envelope_config_t envelope_config_default() {
  envelope_config_t config = {
    attack_time : 0,
    decay_time : 0,
    release_time : 0,
    sustain_level : 127,
    max_time_ms : 5000,
  };

  return config;
}

envelope_state_t envelope_state_default() {
  envelope_state_t state = {
    attack_rate : UINT32_MAX,
    decay_rate : UINT32_MAX,
    sustain_level : UINT32_MAX,
    release_rate : UINT32_MAX,
    phase : ENVELOPE_PHASE_OFF,
    level : 0,
  };

  return state;
}

void envelope_config_apply(envelope_state_t* state, envelope_config_t* config) {
  state->sustain_level = config->sustain_level << (32 - 7);

  if (config->attack_time) {
    state->attack_rate =
        UINT32_MAX / (config->attack_time * config->max_time_ms / 127);
  } else {
    state->attack_rate = UINT32_MAX;
  }

  if (config->decay_time) {
    state->decay_rate =
        UINT32_MAX / (config->decay_time * config->max_time_ms / 127);
  } else {
    state->decay_rate = UINT32_MAX;
  }

  if (config->release_time) {
    state->release_rate =
        UINT32_MAX / (config->release_time * config->max_time_ms / 127);
  } else {
    state->release_rate = UINT32_MAX;
  }

  state->sustain_level = (UINT32_MAX / 127) * config->sustain_level;

  state->last_tick_time_ms = to_ms_since_boot(get_absolute_time());
}

bool increase_to_limit(uint32_t* value, uint32_t add, uint32_t limit) {
  if (add > limit - *value) {
    *value = limit;
    return true;
  }

  *value += add;
  return false;
}

bool decrease_to_limit(uint32_t* value, uint32_t sub, uint32_t limit) {
  if (sub > *value - limit) {
    *value = limit;
    return true;
  }

  *value -= sub;
  return false;
}

uint32_t mult_limited(uint32_t a, uint32_t b) {
  if (!a || !b) {
    return 0;
  }
  uint32_t result = a * b;
  if (result < a || result < b) {
    return UINT32_MAX;
  }
  return result;
}

envelope_phase_t envelope_progress_ms(envelope_state_t* state, uint32_t dt_ms) {
  switch (state->phase) {
    case ENVELOPE_PHASE_ATTACK:
      if (state->attack_rate != UINT32_MAX) {
        if (increase_to_limit(&(state->level),
                              mult_limited(state->attack_rate, dt_ms),
                              UINT32_MAX)) {
          state->phase = ENVELOPE_PHASE_DECAY;
        }
        break;
      }

      // If attack is immediate, set level to max and continue to
      // ENVELOPE_PHASE_DECAY
      state->level = UINT32_MAX;
      state->phase = ENVELOPE_PHASE_DECAY;
      __attribute__((fallthrough));
    case ENVELOPE_PHASE_DECAY:
      if (state->decay_rate != UINT32_MAX) {
        if (decrease_to_limit(&(state->level),
                              mult_limited(state->decay_rate, dt_ms),
                              state->sustain_level)) {
          state->phase = ENVELOPE_PHASE_SUSTAIN;
        }
        break;
      }

      // If decay is immediate, set sustain level and continue to
      // ENVELOPE_PHASE_SUSTAIN
      state->level = state->sustain_level;
      state->phase = ENVELOPE_PHASE_SUSTAIN;
      __attribute__((fallthrough));
    case ENVELOPE_PHASE_SUSTAIN:
      // Sustain current level
      break;
    case ENVELOPE_PHASE_RELEASE:
      if (state->release_rate != UINT32_MAX) {
        if (decrease_to_limit(&(state->level),
                              mult_limited(state->release_rate, dt_ms), 0)) {
          state->phase = ENVELOPE_PHASE_OFF;
        }
        break;
      }

      // If release is immediate, set sustain level and continue to
      // ENVELOPE_PHASE_OFF
      state->level = 0;
      state->phase = ENVELOPE_PHASE_OFF;
      __attribute__((fallthrough));
    default:
      // Make sure we are off
      state->level = 0;
      break;
  }

  state->last_tick_time_ms += dt_ms;
  return state->phase;
}

envelope_phase_t envelope_progress(envelope_state_t* state) {
  uint32_t now = to_ms_since_boot(get_absolute_time());

  return envelope_progress_ms(state, now - state->last_tick_time_ms);
}

void envelope_trigger(envelope_state_t* state) {
  state->last_tick_time_ms = to_ms_since_boot(get_absolute_time());
  state->phase = ENVELOPE_PHASE_ATTACK;
  envelope_progress_ms(state, 0);
}

void envelope_stop(envelope_state_t* state) {
  state->phase = ENVELOPE_PHASE_RELEASE;
  envelope_progress_ms(state, 0);
}

void envelope_force_stop(envelope_state_t* state) {
  state->phase = ENVELOPE_PHASE_OFF;
  state->level = 0;
}
