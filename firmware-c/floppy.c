#include "floppy.h"

#include <stddef.h>

#include "led.h"
#include "pico/stdlib.h"

struct floppy floppies[] = {
    FLOPPY_DEFAULTS(18, 17, 16), FLOPPY_DEFAULTS(14, 15, 19),
    FLOPPY_DEFAULTS(12, 13, 20), FLOPPY_DEFAULTS(10, 11, 21),
    FLOPPY_DEFAULTS(8, 9, 22),   FLOPPY_DEFAULTS(5, 6, 7),
};

void floppy_init() {
  // init GPIO
  for (size_t i = 0; i < N_FLOPPIES; i++) {
    gpio_init(floppies[i].gpio_step);
    gpio_set_dir(floppies[i].gpio_step, true);
    gpio_init(floppies[i].gpio_dir);
    gpio_set_dir(floppies[i].gpio_dir, true);
    gpio_init(floppies[i].gpio_enable);
    gpio_set_dir(floppies[i].gpio_enable, true);

    gpio_put(floppies[i].gpio_enable, false);
  }
}

void floppy_step(struct floppy *floppy) {
  if (floppy->step_state) {
    floppy->counter++;
    if (floppy->counter >= 80) {
      floppy->counter = 0;
      floppy->dir = !floppy->dir;
      gpio_put(floppy->gpio_dir, floppy->dir);
    }
  }

  floppy->step_state = !floppy->step_state;
  gpio_put(floppy->gpio_step, floppy->step_state);
}

void floppy_enable(struct floppy *floppy, bool enable) {
  gpio_put(floppy->gpio_enable, enable);
}
