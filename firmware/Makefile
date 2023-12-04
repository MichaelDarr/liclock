.PHONY: default
default:
	make build
	make flash

.PHONY: build
build:
	cargo build

.PHONY: flash
flash: SHELL:=/bin/bash
flash:
	avrdude \
    -p t84a \
    -P /dev/ttyACM1 \
    -c avrisp \
    -b 19200 \
    -U{eeprom,flash,{l,h}fuse,lock}:w:"$(CURDIR)/target/avr-attiny84a/debug/liclock.elf":e
