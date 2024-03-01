# liclock firmware

## Fuse Bytes

[ATtiny84A, Page 165](https://ww1.microchip.com/downloads/en/DeviceDoc/ATtiny24A-84A-84A-DataSheet-DS40002269A.pdf)

### Low Byte

**0b1111_1111**

* No clock division
* Clock output disabled
* Crystal oscillator, 8.0MHz or higher
* Slowly rising power

```sh
avrdude \
  -p t84a \
  -P /dev/ttyACM0 \
  -c avrisp \
  -b 19200 \
  -U lfuse:w:0xff:m
```

### High Byte

**0b1101_0101** (default `0b1101_1111`/`df`)

* Keep reset pin enabled
* No watchdog timer
* Preserve EEPROM memory through chip erase
* Enable brown-out detector, 2.7V

```sh
avrdude \
  -p t84a \
  -P /dev/ttyACM0 \
  -c avrisp \
  -b 19200 \
  -U hfuse:w:0xd5:m
```

### Sanity Check

To output the clock on pin 5, program CKOUT

```sh
avrdude \
  -p t84a \
  -P /dev/ttyACM0 \
  -c avrisp \
  -b 19200 \
  -U lfuse:w:0xbf:m
```

### v0.2 -> v0.3

# MCU Pinout Changes

| Pin      | v0.2           | v0.3        |
| :------- | :------------- | :---------- |
| 11 | PB5 | COL_2          | x           |
| 12 | PB4 | COL_1          | x           |
| 17 | PC0 | SW1_ACTIVE     | SW1_LED     |
| 18 | PC1 | SW1_LED_ENABLE | SW_READ     |
| 19 | PC2 | SW2_ACTIVE     | SW2_LED     |
| 20 | PC3 | SW1_LED_ENABLE | x           |
| 21 | PC4 | SW3_ACTIVE     | BUZZ        |
| 22 | PC5 | BUZZ           | x           |
