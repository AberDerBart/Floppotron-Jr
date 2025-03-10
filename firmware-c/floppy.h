#ifndef FLOPPY_H
#define FLOPPY_H

#include <stdbool.h>
#include <stdint.h>

struct floppy {
  uint8_t gpio_step;
  uint8_t gpio_dir;
  uint8_t gpio_enable;
  uint8_t counter;
  bool dir;
  bool step_state;
};

#define N_FLOPPIES 6

#define FLOPPY_DEFAULTS(gpio_step, gpio_dir, gpio_enable) \
  {gpio_step, gpio_dir, gpio_enable, 0, false, false}

extern struct floppy floppies[];

void floppy_init();

void floppy_enable(struct floppy *floppy, bool enable);

void floppy_step(struct floppy *floppy);

#endif
