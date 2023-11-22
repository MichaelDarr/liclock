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
    -p t44a \
    -P /dev/ttyACM0 \
    -c avrisp \
    -b 19200 \
    -U{eeprom,flash,{l,h}fuse,lock}:w:"$(CURDIR)/target/avr-attiny44a/debug/liclock.elf":e
