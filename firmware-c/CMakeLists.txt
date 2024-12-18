cmake_minimum_required(VERSION 3.13)

set(PICO_BOARD pico_w)



# initialize pico-sdk from GIT
# (note this can come from environment, CMake cache etc)
set(PICO_SDK_FETCH_FROM_GIT on)

# pico_sdk_import.cmake is a single file copied from this SDK
# note: this must happen before project()
include(pico_sdk_import.cmake)

project(my_project)

# initialize the Raspberry Pi Pico SDK
pico_sdk_init()

# generate the note dict
add_custom_command(OUTPUT "noteDict.h" COMMAND python ${CMAKE_CURRENT_LIST_DIR}/genNoteDict.py > ${CMAKE_CURRENT_LIST_DIR}/noteDict.h)

# rest of your project
add_executable(floppotron main.c midi.h midi.c led.h led.c oscillator.h oscillator.c noteDict.h noteStack.c noteStack.h dispatcher.c dispatcher.h floppy.c floppy.h cvOutput.c cvOutput.h)

target_include_directories(floppotron PRIVATE ${CMAKE_CURRENT_LIST_DIR})

target_link_libraries(floppotron pico_stdlib hardware_pwm)
pico_add_extra_outputs(floppotron)

pico_enable_stdio_usb(floppotron 1)
pico_enable_stdio_uart(floppotron 0)

