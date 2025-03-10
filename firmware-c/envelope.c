#include "envelope.h"

void envelope_task() {}

envelope_state_t envelope_state_default() {
  envelope_state_t state = {
    attack_rate : UINT32_MAX,
    decay_rate : UINT32_MAX,
    sustain_level : UINT32_MAX,
    release_rate : UINT32_MAX,
    phase : OFF,
    level : 0,
  };

  return state;
}

void envelope_config_apply(envelope_state_t* state, envelope_config_t* config) {
  state->sustain_level = config->sustain_level << (32 - 7);

  if (config->attack_time >= 127) {
    state->attack_rate = UINT32_MAX;
  } else {
    state->attack_rate = UINT32_MAX / config->attack_time * 127;
  }

  if (config->decay_time >= 127) {
    state->decay_rate = UINT32_MAX;
  } else {
    state->decay_rate = UINT32_MAX / config->decay_time * 127;
  }

  if (config->release_time >= 127) {
    state->release_rate = UINT32_MAX;
  } else {
    state->release_rate = UINT32_MAX / config->release_time * 127;
  }
}

void envelope_progress(envelope_state_t* state) {
  while (true) {
    switch (state->phase) {
      case ATTACK:
        // No attack, skip to decay
        if (state->attack_rate == UINT32_MAX) {
          state->level = UINT32_MAX;
          state->phase = DECAY;
          continue;
        }

        // End of attack reached, go to max level, switch to release
        if (UINT32_MAX - state->attack_rate > state->level) {
          state->level = UINT32_MAX;
          state->phase = DECAY;
          return;
        }

        // Increase level and stay in attack
        state->level += state->attack_rate;
        return;
      case DECAY:
        // No decay, skip to sustain
        if (state->decay_rate == UINT32_MAX) {
          state->level = state->sustain_level;
          state->phase = SUSTAIN;
          return;
        }

        // End of decay reached, go to max level, switch to release
        if (state->level < state->sustain_level ||
            state->decay_rate > state->level - state->sustain_level) {
          state->level = state->sustain_level;
          state->phase = SUSTAIN;
          return;
        }

        // Decrease level and stay in sustain
        state->level -= state->decay_rate;
        return;
      case SUSTAIN:
        // Sustain current level
        return;
      case RELEASE:
        // End of release reached, switch to off
        if (state->decay_rate > state->level) {
          state->phase = OFF;
          state->level = 0;
          return;
        }

        // Decrease level and stay in release
        state->level -= state->decay_rate;
        return;
      default:
        // Make sure we are off
        state->level = 0;
        return;
    }
  }
}

void envelope_trigger(envelope_state_t* state) {
  state->phase = ATTACK;
  if (state->attack_rate != UINT32_MAX) {
    return;
  }

  state->level = UINT32_MAX;
  state->phase = DECAY;

  if (state->decay_rate != UINT32_MAX) {
    return;
  }

  state->level = state->sustain_level;
  state->phase = SUSTAIN;
}

void envelope_stop(envelope_state_t* state) {
  if (state->release_rate == UINT32_MAX) {
    envelope_force_stop(state);
    return;
  }
  state->phase = RELEASE;
}

void envelope_force_stop(envelope_state_t* state) {
  state->phase = OFF;
  state->level = 0;
}
