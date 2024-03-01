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
	sudo avrdude \
        -p t1627 \
        -c pkobn_updi \
        -U{eeprom,flash,fuse5,lock}:w:"./target/avr-attiny1627/debug/liclock.elf":e
