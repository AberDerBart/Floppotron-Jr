TARGET = build/floppotron.elf

.PHONY: flash clean distclean debug

$TARGET: *.c *.h CMakeLists.txt
	mkdir -p build
	cd build
	cmake -B build -DCMAKE_BUILD_TYPE=Debug
	make -C build

flash: ${TARGET}
	sudo openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000" -c "program ${TARGET} verify reset exit"

clean:
	make -C build clean

distclean:
	rm -rf build

debug-openocd: ${TARGET}
	sudo openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000"

debug-gdb: ${TARGET}
	arm-none-eabi-gdb build/keytar.elf -x ./gdbinit

serial:
	minicom -b 115200 -o -D /dev/ttyACM0
