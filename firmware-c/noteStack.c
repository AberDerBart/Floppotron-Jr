#include "noteStack.h"

#include <stdio.h>
struct stackNote;

struct stackNote {
  struct stackNote *up;
  struct stackNote *down;
  uint8_t value;
  uint8_t velocity;
};

struct stackNote noteStack[128];

struct stackNote *top = NULL;

void noteStack_init() {
  for (int i = 0; i < 128; i++) {
    noteStack[i].down = NULL;
    noteStack[i].up = NULL;
    noteStack[i].value = i;
  }
}

void noteStack_push(uint8_t noteVal, uint8_t velocity) {
  if (noteVal >= 128) {
    return;
  }
  noteStack_rm(noteVal);

  struct stackNote *note = &noteStack[noteVal];

  note->velocity = velocity;

  note->down = top;
  if (top) {
    top->up = note;
  }
  top = note;
}

void noteStack_rm(uint8_t noteVal) {
  if (noteVal >= 128) {
    return;
  }

  struct stackNote *note = &noteStack[noteVal];

  struct stackNote *up = note->up;
  struct stackNote *down = note->down;

  if (up) {
    up->down = down;
  }
  if (down) {
    down->up = up;
  }

  note->up = NULL;
  note->down = NULL;

  if (top == note) {
    top = down;
  }
}

void noteStack_clear() {
  while (top) {
    struct stackNote *next = top->down;
    top->up = NULL;
    top->down = NULL;
    top = next;
  }
}

size_t noteStack_getTop(uint8_t *res, size_t n) {
  size_t read = 0;
  struct stackNote *currentNote = top;

  while (read <= n && currentNote) {
    res[read] = currentNote->value;
    currentNote = currentNote->down;
    read++;
  }

  return read;
}

bool noteStack_is_empty() { return top == NULL; }

uint8_t noteStack_get_velocity() {
  if (!top) {
    return 0;
  }

  return top->velocity;
}
