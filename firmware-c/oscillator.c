#include "oscillator.h"
#include "pico/stdlib.h"
#include "hardware/pwm.h"
#include "hardware/irq.h"

#include "noteDict.h"
#include "noteStack.h"

#include <stdio.h>

#define PB_RANGE 0x2000
#define PB_BITMASK (PB_RANGE - 1)
#define PB_CENTER 0x2000

const uint8_t GPIOS_STEP[N_OSCILLATORS] = {16, 18};
const uint8_t GPIOS_DIR[N_OSCILLATORS] = {7, 17};

struct oscillator oscillators[8];

void oscillator_step(struct oscillator *osc);
struct oscillator oscillator_new(uint8_t slice, struct floppy *floppy);

// pitchbend is the pitchbend up scaled in 13 bits (like the poistive half of the MIDI pitchbend value)
uint16_t pitchbend = 0;
// not negative pitchbend or pitchbend > 1 halfstep, adjust the note
int8_t note_offset = 0;

void on_pwm_wrap()
{
  uint8_t irq_mask = pwm_get_irq_status_mask();

  for (uint8_t i = 0; i < N_OSCILLATORS; i++)
  {
    if (irq_mask & 1 << i)
    {
      oscillator_step(oscillators + i);
      pwm_clear_irq(i);
    }
  }
}

void oscillator_init()
{
  irq_set_exclusive_handler(PWM_IRQ_WRAP, on_pwm_wrap);

  pwm_set_irq_enabled(0xff, false);
  irq_set_enabled(PWM_IRQ_WRAP, true);
  floppy_init();

  for (uint8_t i = 0; i < N_OSCILLATORS; i++)
  {
    oscillators[i] = oscillator_new(i, &floppies[i]);
  }
}

struct oscillator oscillator_new(uint8_t slice, struct floppy *floppy)
{
  pwm_set_irq_enabled(slice, true);

  struct oscillator osc = {
    slice : slice,
    floppy : floppy,
    current_note : NO_NOTE,
  };
  return osc;
}

void oscillator_free(struct oscillator osc)
{
  oscillator_by_index_stop(osc.slice);
  pwm_set_irq_enabled(osc.slice, false);
}

void oscillator_by_index_stop(uint8_t slice)
{
  pwm_set_enabled(slice, false);
  oscillators[slice].current_note = NO_NOTE;
  floppy_enable(oscillators[slice].floppy, false);
}

void oscillator_by_index_set_note(uint8_t slice, uint8_t note)
{
  if (note == NO_NOTE)
  {
    oscillator_by_index_stop(slice);
  }
  floppy_enable(oscillators[slice].floppy, true);
  oscillators[slice].current_note = note;

  note += note_offset;

  if (note < 0)
  {
    note = 0;
  }
  if (note > 127)
  {
    note = 127;
  }

  // set clock divider
  pwm_set_clkdiv_int_frac(slice, noteDict[note].clk_div, 0);

  // calculate period
  uint16_t min_period = noteDict[note].wrap_pb_up;
  uint16_t max_period = noteDict[note].wrap;
  uint16_t period_diff = max_period - min_period;

  uint16_t period = max_period - (((uint32_t)period_diff * pitchbend) >> 13);
  pwm_set_wrap(slice, period);
  pwm_set_chan_level(slice, 0, period / 2);

  // enable oscillator
  pwm_set_enabled(slice, true);
}

void oscillator_step(struct oscillator *osc)
{
  floppy_step(osc->floppy);
}

void oscillator_set_pitchbend(uint16_t pb, uint8_t scale)
{
  int16_t scaled_pb = ((int16_t)pb - PB_CENTER) * scale;

  pitchbend = scaled_pb & PB_BITMASK;

  if (scaled_pb >= 0)
  {
    note_offset = scaled_pb / PB_RANGE;
  }
  else
  {
    note_offset = -((-scaled_pb - scale) / PB_RANGE) - 1;
  }

  // update playing oscillators
  for (size_t i = 0; i < N_OSCILLATORS; i++)
  {
    if (oscillators[i].current_note != NO_NOTE)
    {
      oscillator_by_index_set_note(i, oscillators[i].current_note);
    }
  }
}
