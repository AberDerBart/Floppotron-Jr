#ifndef NOTESTACK_H
#define NOTESTACK_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#define NO_NOTE 255

void noteStack_init();
void noteStack_push(uint8_t note);
void noteStack_rm(uint8_t note);

size_t noteStack_getTop(uint8_t* res, size_t n);

#endif
