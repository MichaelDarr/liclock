# liclock
the libre chess clock

This document is a work in progress! It's basically a disorganized scratchpad right now.

## Electrical Characteristics

In active mode (counting down), the prototype draws ~0.7mA. This data [indicates a ~1 year battery life](https://www.digikey.com/en/resources/conversion-calculators/conversion-calculator-battery-life), assuming the clock is on and counting 24/7. This figure accounts for the following components:

1. MCU (ATtiny84a @ 20MHz, active mode)
2. 4-channel inverter (two channels are very active: the inverse backplane and toggle mux enable)
3. 2:4 channel demux (many active channels)
4. 1 LED (indicating the active player)
5. 2 LCD drivers (ICM7211AM)*
6. 2 four-digit seven-segment LCD displays[^1]
7. 5V buck regulator powered by 6 AAA batteries (source somewhere between 5V and 10V, rated for 40V) 

[^1] The clocks never count down simultaniously, one screen/driver pair is idle 

User actions trigger behavior which increases current draw to ~1.5mA (±0.5mA). These include buzzer activation, eeprom access, and switch activation.
