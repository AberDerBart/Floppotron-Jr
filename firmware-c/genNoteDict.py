import math

main_clock = 125000000 

def calcPeriod(note):
    return main_clock / (math.pow(2, (note - 69) / 12.) * 880.)

def genNote(note):
    period = calcPeriod(note)
    periodPbUp = calcPeriod(note + 1)

    clockDiv = math.ceil(period/65536)
    clockDivPbUp = math.ceil(period/65536)

    wrap = round(period / clockDiv)

    wrapPbUp = round(periodPbUp / clockDivPbUp)


    return '{' + '{}, {}, {}'.format(wrap, clockDiv, wrapPbUp) + '}'

def noteName(note):
    names = ['A', 'A#', 'B', 'C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#']

    nameIndex = (note - 21) % 12
    octave = (note - 12) // 12

    return names[nameIndex] + str(octave)


def genNoteDict():
    noteDict = "struct pwmSetting noteDict[] = {\n"

    notes = ['\t' + genNote(note) + ', // ' + noteName(note) for note in range(128)]

    noteDict += '\n'.join(notes)
    noteDict += '\n};'

    return noteDict

if __name__ == '__main__':
    print('#include <stdint.h>')
    print()
    print('struct pwmSetting {')
    print('\tuint16_t wrap;')
    print('\tuint8_t  clk_div;')
    print('\tuint16_t wrap_pb_up;')
    print('};')
    print()
    print(genNoteDict())

