# liclock
the libre chess clock

This document is a work in progress! It's basically a disorganized scratchpad right now.

## Current Consumption

The MCU (ATtiny44a @ 20MHz) and voltage regulator (MAX5035BUSA+) can be configured to run in a variety of power states. This section explores the current consuption at each.

### Symbols

* Input Leakage Current: **Iᵢₙ or Iᵢʟₖ**
* Load Capacitance: **Cʟ or Cᵢ or Cᵢₙ**
* Quiescent Current: **I<sub>Q</sub> or Q⊂⊂**
* Shutdown Current: **Iₛₕₔₙ or I⏏**

### Current Consumption

| MCU (@ 20MHZ)   | U3        | Attiny44A         | 9mA[^2]  | 11mA[^2]  | 0.4μA[^3]        | -                    |
| LEDs[^5]        | LED[1..2] | SSL-LX5093GD-5V   | 12mA     | 24mA      | -                | -                    |
| Voltage Regular | U6        | MAX5035BUSA+[^1]  | 270μA    | 440μA     | 270μA            | 10μA                 |

#### Non-MCU Components

* N₯ == Active pins (average number of active pins flowing current through a terminal)
* I₯ == Current per active pin (when a pin is active, how much current is it flowing)
* ΣI₯ ==  Sum of the current being flowed by all active pins


| Component           | Designator | PN              | Q⊂⊂typ | Q⊂⊂max | N₯  | I₯typ  | ΣI₯typ     | I₯max  | I⏏typ | Iₛₕₔₙ MAX |
| :------------------ | :--------- | :-------------- | :----- | :----- | :-- | :----- | :--------- | :----- | :----------------- | :-------- | :-------- |
| Voltage Regular     | U6         | MAX5035BUSA+    | 270μA  | 440μA  |     |        |            |        |                    |      45μA |      10μA |
| Display 1[^2]       | U1         | ICM7211MIJL     | 5μA    | 25μA   | 2   | .01μA  | 0.02μA     | 8*0*1μA    ≈   1μA |           |           |
| Display 2[^2]       | U2         | ICM7211MIJL     | 5μA    | 25μA   | 2   | .01μA  | 0.02μA     | 8*0*1μA    ≈   1μA |           |           |
| Inverter[^3]        | U4         | NL37WZ14USG     | 1μA    | 10μA   | 1.5 | 0.1μA  | 0.15μA     | 3*.5*1μA   = 1.5μA |           |           |
| Demultiplexer[^4]   | U5         | CD74ACT139M     | 8μA    | 80μA   | 2   | 2.4mA  | 4.8mA      | 4*.5*3mA   =   6mA |           |           |
| LEDs[^3]            | LED[1..2]  | SSL-LX5093GD-5V |        |        | 1   | 12mA   | 12mA       | 2*.5*12mA  = 12mA  |           |           |
| Buzzer[^6]          | LS1        | SFM-1440-1      |        |        | 0.01   | 12mA   | 12mA       | 2*.5*12mA  = 12mA  |           |           |
| ------------------- | ---------- | --------------- | ------ | ------ | --- | ------ | ---------- | ------ | ------------------ | --------- | --------- |
| TOTAL               |            |                 | 289μA  | 580μA  |     |        | 16800.17μA |         18002.5μA  |      45μA |      10μA |

[^2]: The firmware consistently drives the 6 direct pins low when not writing data to the driver (effectively always). The inverse situation will occur for the two multiplexed pins (CHIP_SELECT_1 and DATA_2), which will nearly always be held high when not writing. This'll even out to a pretty exact 2-pins-high situation.
[^3]: The LCD backplane has a 50% duty cycle. The two LEDs alternate, and the whichever is active will be driven high most of the time (~98%, maybe? These are multiplexed, but these LEDs are the "resting" option targeted by firmware after it utilizes an led-hosting mux).
[^4]: Both activation pins (G1/G2) are tied low (0%), and 50% is a wild guess for the others. This needs more measurement (pg. 3, ACT input load table).

#### MCU
0.19μA

##### I/O Pins

Every pin except VCC (1), XTAL[1..2] (2, 3), PA3 (10) and GND (14) is an I/O pin

| Pin     | Label    | Duty Cycle | Cʟ   | fₛ𝑤       | I⊂ₚ≈V⊂⊂*Cʟ*fₛ𝑤
| :------ | :------- | :--------- | :--- | :-------- | 
| 4   PB3 | data_0   |        ~0% |  5pF |  20Hz[^5] | 
| 5   PB2 | data_1   |        ~0% |  5pF |  20Hz[^5] |
| 6   PA7 | buzz     |        ~1% | 10nF |  40Hz[^6] |
| 7   PA6 | data_3   |        ~0% |  5pF |  20Hz[^5] |
| 8   PA5 | char_1   |        ~0% |  5pF |  20Hz[^5] |
| 9   PA4 | char_1   |        ~0% |  5pF |  20Hz[^5] |
| 11  PA2 | mux_a_1  |        50% | 10pF |  10Hz[^7] |
| 12  PA1 | mux_a_2  |        50% | 10pF | 120Hz[^7] |
| 13  PA0 | mux_b    |        50% | 10pF |           |

[^5]: Each clock refreshes 5 times per second, and each input flips roughly every other character (4 times per 8 characters).
[^6]: 4kHz buzzer buzzing 1% of the time
[^7]: mux_a_1 writes data into the driver latch 8 times per refresh 5 times per second (80 toggles)
[^8]: In addition to toggling char_2 (estimated above as 20Hz), mux_a_2 powers the SW3 pull-up, which is checked maybe 100 times/second? Needs more investigation!
[^9]: mux_b toggles twice per character refresh, switching between lcd drives. It will probably change a good deal to select other mux items. Needs more investigation!. Additionally, it toggles between 

##### Modes

This section describes the baseline power consumption of the MCU, i.e., the cost to hang out in a given mode and just sit there. See pg. 39 on the datasheet for more info.

| Mode       | I⊂⊂   | Description
| :--------- | :---- | :------------------------------------------------------------------------------------------------- |
| Active     | 9mA   | Default "on" state                                                                                 |
| Idle       | 3mA   | CPU stops, but keep other systems (timers, interrupts etc) remain operational.                     |
| Standby    | 80μA  | "Power-down" but keep the oscillator running, allowing the MCU to be woken up in six clock cycles. |
| Power-down | 0.4μA | Lowest functionality level of short of yanking power, nearly everything is stopped.                |

The initial build will include a physical on/off switch to toggle the voltage regulator's ON/OFF pin between GND and Vᵢₙ. This OFF state will be used in place of standby or power-down. This approach has a couple significant benefits:

* Every IC (except the regulator itself) is will be fully disabled when the device is off.
* The MCU doesn't have to meddle at all with the battery or voltage voltage regulator, saving pins as well as complication.

In a future build, liclock could be built without a physical switch - further investigation & measurement is needed



### Active Mode

TODO: Calculate pin current draw (pg. 188, voltage * load capacitance * average switching frequency of the I/O pin)

| Consumer                  | Typ   | Max   |
| :------------------------ | :---- | :---- |
| MCU in Active Mode        | 9mA   | 11mA  |
| MCU Pins                  | ?mA   | ?mA   |
| IC Quiescent Current      | 289μA | 580μA |
| LEDs (2x alterating = 1x) | 12mA  | 12mA  |

### Power-down Mode

Power-down is the MCU's deepest sleep mode; it's the next-best thing to pulling the cord.

* The MCU's consumption drops to ~0.4μA, and it can only be be woken by the watchdog or by an external interrupt.
* Voltage is still being supplied to all ICs, so quiescent current their qu but at least the LEDs are off.

### Standby

Current consumption standby mode is .


#### MCU

TODO: use features like the PRR register to disable unused features

TODO: estimate I/O (page 188)

#### Volatage Regulator Efficiency

COMING SOON
