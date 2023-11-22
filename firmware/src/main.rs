#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]

mod chess_clock;
mod descriptor;
mod switch;
mod timer;

use panic_halt as _;

use avr_device::{
    attiny44a,
    interrupt,
    interrupt::Mutex,
};
use chess_clock::ChessClock;
use core::{
    cell::RefCell,
    ops::DerefMut,
};
use descriptor::Player;

static CHESS_CLOCK: Mutex<RefCell<ChessClock>> = Mutex::new(RefCell::new(ChessClock::new()));

#[avr_device::entry]
fn main() -> ! {
    let dp = attiny44a::Peripherals::take().unwrap();

    unsafe {
        // TC0: Normal mode, no prescaler
        // * TOP = 0xFF
        // * Update of OCRx at Immediate
        // * TOV flag set on MAX

        dp.TC0.tccr0b.write(|w| w.bits(0b0000_0001));

        // Configure pins:
        // PA0 | mux1_enable | output
        // PA1 | mux_a       | output
        // PA2 | mux_b       | output
        // PA3 | sw_pull_up  | input
        // PA4 | digit_2     | output
        // PA5 | digit_1     | output
        // PA6 | data_2      | output
        // PA7 | data_1      | output
        dp.PORTA.ddra.write(|w| w.bits(0b1111_0111));
        // PB2 | data_0      | output
        // PB3 | RESET       | floating (eventually disable reset, become data_3)
        dp.PORTB.ddrb.write(|w| w.bits(0b0000_0100));

        // Disable mux1, use sw_pull_up as a pull-up, default b (chip select 2) high
        dp.PORTA.porta.write(|w| w.bits(0b0000_1101));

        // | AVR Setting                    | Symbol           | Value         |
        // | :----------------------------- | :--------------- | :------------ |
        // | System Frequency Clock         | Fclk_I/0         | 20MHz         |
        // | Clock Cycle Duration           | CK               | 50nS      1CK |
        // | Timer/Clock 0 Cycle Duration   | TC0              | 50nS      0CK |
        // | Timer/Clock 1 Cycle Duration   | TC1 (clkI/O/256) | 12800nS 256CK |
        //
        // ## TC1 -> Seconds:
        // 78125CK1 == 1S == 0->15625 x 5
        // Count using fifth-of-second interval (200ms per interval)
        //
        // | Variable                       | Symbol   | Value          | Spec             | Description
        // | :----------------------------- | :------- | :------------- | :--------------- | :----------
        // | Data Setup Time                | tDSM     | 10μS     200CK | min: 100nS   2CK | WR high between digit transfer and double-low select active
        // | Chip Select Active Pulse Width | tCSA     | 200nS      4CK | typ: 200nS   4CK | Both low
        // | Data Hold Time                 | tDHM     | >250nS    >5CK | min: 10nS  0.2CK | Holding period between data transfer trigger and completion
        // | Inter-Chip Select Time         | tICS     | 10.2μS   204CK | min: 2μS    20CK | WR cycle duration (tICS = tDSM + tCSA)


        // TC1: Fast PWM (mode 14)
        // * TOP = OCR1A
        // * Update of 1CRx at TOP
        // * TOV flag set on TOP
 
        // Set WGM11 & WGM10
        dp.TC1.tccr1a.write(|w| w.bits(0b0000_0011));
        // Set WGM12 & WGM13, prescale 256x
        dp.TC1.tccr1b.write(|w| w.bits(0b0001_1100));
        // tick the chess clock every 200ms
        // (0 -> ocr1a) = 4000000CK = 200000000nS = 200 mS
        dp.TC1.ocr1a.write(|w| w.bits(15624));
        // enable `TIM1_OVF` interrupt
        dp.TC1.timsk1.write(|w| w.bits(0b0000_0001));
    }

    unsafe {
        interrupt::free(|cs| {
            CHESS_CLOCK.borrow(cs).borrow_mut().deref_mut().register_action(Some(Player::A));
        });
        interrupt::enable();
    }

    loop{
        interrupt::free(|cs| {
            unsafe {
                // convert PA3 into an output
                dp.PORTA.ddra.modify(|r, w| w.bits(
                    r.bits() | 0b0000_1000
                ));
                for player in [Player::A, Player::B] {
                    let digits = CHESS_CLOCK.borrow(cs).borrow_mut().deref_mut().get_digits(player);
                    match player {
                        Player::A => {
                            // prepare to send to clock 1, a low, b low
                            dp.PORTA.porta.write(|w| w.bits(0b0000_1001));
                        },
                        Player::B => {
                            // prepare to send to clock 2, a high, b low
                            dp.PORTA.porta.write(|w| w.bits(0b0000_1011));
                        },
                    }
                    for digit_idx in 0..4 {
                        let start_time = dp.TC0.tcnt0.read().bits();
                        let digit = digits[3-digit_idx];

                        match digit_idx {
                            0 => {
                                // PA4 low, PA5 low
                                dp.PORTA.porta.modify(|r, w| w.bits(
                                    r.bits() & 0b1100_1111
                                ));
                            },
                            1 => {
                                // PA4 high, PA5 low
                                dp.PORTA.porta.modify(|r, w| w.bits(
                                    (r.bits() & 0b1101_1111) | 0b0001_0000
                                ));
                            },
                            2 => {
                                // PA4 low, PA5 high
                                dp.PORTA.porta.modify(|r, w| w.bits(
                                    (r.bits() & 0b1110_1111) | 0b0010_0000
                                ));
                            },
                            3 => {
                                // PA4 high, PA5 high
                                dp.PORTA.porta.modify(|r, w| w.bits(
                                    r.bits() | 0b0011_0000
                                ));
                            },
                            _ => {},
                        }

                        if digit & 0b0000_0001 == 1 {
                            // PB2 high
                            dp.PORTB.portb.modify(|r, w| w.bits(
                                r.bits() | 0b0000_0100
                            ));
                        } else {
                            // PB2 low
                            dp.PORTB.portb.modify(|r, w| w.bits(
                                r.bits() & 0b1111_1011
                            ));
                        }
                        if (digit >> 1) & 0b0000_0001 == 1 {
                            // PA7 high
                            dp.PORTA.porta.modify(|r, w| w.bits(
                                r.bits() | 0b1000_0000
                            ));
                        } else {
                            // PA7 low
                            dp.PORTA.porta.modify(|r, w| w.bits(
                                r.bits() & 0b0111_1111
                            ));
                        }
                        if (digit >> 2) & 0b0000_0001 == 1 {
                            // PA6 high
                            dp.PORTA.porta.modify(|r, w| w.bits(
                                r.bits() | 0b0100_0000
                            ));
                        } else {
                            // PA6 low
                            dp.PORTA.porta.modify(|r, w| w.bits(
                                r.bits() & 0b1011_1111
                            ));
                        }
                        if (digit >> 3) & 0b0000_0001 == 1 {
                            // PA3 high
                            dp.PORTA.porta.modify(|r, w| w.bits(
                                r.bits() | 0b0000_1000
                            ));
                        } else {
                            // PA3 low
                            dp.PORTA.porta.modify(|r, w| w.bits(
                                r.bits() & 0b1111_0111
                            ));
                        }

                        // pulse mux_1_enable
                        dp.PORTA.porta.modify(|r, w| w.bits(
                            r.bits() & 0b1111_1110
                        ));
                        dp.PORTA.porta.modify(|r, w| w.bits(
                            r.bits() | 0b0000_0001
                        ));

                        // Wait at least 2uS between each digit (min tICS)
                        if digit_idx != 3 {
                            'wait_ics: loop {
                                if dp.TC0.tcnt0.read().bits().wrapping_sub(start_time) >= 50 {
                                    break 'wait_ics;
                                }
                            }
                        }
                    }
                }
                // revert PA3 back to a pull-up input, driving high for the transition (see datasheet page 61, section 10.1.3)
                // Additionally, pull b high to end the write
                dp.PORTA.porta.modify(|r, w| w.bits(
                    r.bits() | 0b0000_1100
                ));
                dp.PORTA.ddra.modify(|r, w| w.bits(
                    r.bits() & 0b1111_0111
                ));
            }
        });

        let cycle_time = dp.TC0.tcnt0.read().bits();
        'wait_cycle: loop {
            if dp.TC0.tcnt0.read().bits().wrapping_sub(cycle_time) >= 200 {
                break 'wait_cycle;
            }
        }
    }
}

#[interrupt(attiny44a)]
fn TIM1_OVF() {
    interrupt::free(|cs| {
        CHESS_CLOCK.borrow(cs).borrow_mut().deref_mut().tick();
    });
}
