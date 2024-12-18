#include "dispatcher.h"
#include "oscillator.h"
#include "noteStack.h"

#include <stdio.h>

void dispatcher_init() {}

void dispatcher_run()
{
    uint8_t notes[N_OSCILLATORS];
    size_t n_notes = noteStack_getTop(notes, N_OSCILLATORS);

    for (int i_osc = 0; i_osc < N_OSCILLATORS; i_osc++)
    {
        struct oscillator *osc = &oscillators[i_osc];
        if (osc->current_note != NO_NOTE)
        {
            bool noteActive = false;
            for (int i_note = 0; i_note < n_notes; i_note++)
            {
                if (osc->current_note == notes[i_note])
                {
                    // mark note as already being played
                    notes[i_note] = NO_NOTE;
                    noteActive = true;
                    break;
                }
            }
            if (!noteActive)
            {
                oscillator_by_index_stop(i_osc);
            }
        }
    }

    for (int i_note = 0; i_note < n_notes; i_note++)
    {
        if (notes[i_note] != NO_NOTE)
        {
            for (int i_osc = 0; i_osc < N_OSCILLATORS; i_osc++)
            {
                struct oscillator *osc = &oscillators[i_osc];
                if (osc->current_note == NO_NOTE)
                {
                    oscillator_by_index_set_note(i_osc, notes[i_note]);
                    break;
                }
            }
        }
    }
}
