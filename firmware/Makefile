.PHONY: default
default:
	make build
	make flash

.PHONY: build
build:
	cargo build -r

.PHONY: flash
flash: SHELL:=/bin/bash
flash:
	avrdude \
    -p t84a \
    -P /dev/ttyACM0 \
    -c avrisp \
    -b 250000 \
    -U{eeprom,flash,{l,h}fuse,lock}:w:"$(CURDIR)/target/avr-attiny84a/release/liclock.elf":e
