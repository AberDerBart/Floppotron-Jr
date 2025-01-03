#include <stdint.h>
#include <stdbool.h>
#include "hardware/uart.h"
#include "hardware/gpio.h"
#include "midi.h"

#define UART_ID uart0
#define BAUD_RATE 31250
#define MIDI_OUT_PIN 0
#define MIDI_IN_PIN 1

void midi_init()
{
    uart_init(UART_ID, BAUD_RATE);
    gpio_set_function(MIDI_OUT_PIN, GPIO_FUNC_UART);
    gpio_set_function(MIDI_IN_PIN, GPIO_FUNC_UART);
}

void midi_write(uint8_t cmd, uint8_t b1, uint8_t b2){
    uart_putc(UART_ID, cmd);
    uart_putc(UART_ID, b1);
    uart_putc(UART_ID, b2);
}

bool midi_is_status_byte(uint8_t byte)
{
    return (byte & 0x80) == 0x80;
}

struct midi_packet midi_read(){
    struct midi_packet packet;

    packet.status = 0;

    while(!is_status(packet.status)){
        packet.status = uart_getc(UART_ID);
    }

    packet.b1 = uart_getc(UART_ID);
    packet.b2 = uart_getc(UART_ID);
    
    return packet;
}
