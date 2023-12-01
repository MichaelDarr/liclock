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
