#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]
#![allow(internal_features)]
#![feature(int_roundings)]
#![feature(cell_update)]

mod chess_clock;
mod descriptor;
mod timer;

use panic_halt as _;

use avr_device::{
    attiny84a,
    interrupt,
    interrupt::Mutex,
};
use chess_clock::ChessClock;
use core::{
    cell::RefCell,
    ops::DerefMut,
};
use descriptor::Player;

pub static mut CHESS_CLOCK: Mutex<RefCell<ChessClock>> = Mutex::new(RefCell::new(ChessClock::new()));
pub static mut PORTA: Mutex<RefCell<Option<attiny84a::PORTA>>> = Mutex::new(RefCell::new(None));
pub static mut PORTB: Mutex<RefCell<Option<attiny84a::PORTB>>> = Mutex::new(RefCell::new(None));
pub static mut TC0: Mutex<RefCell<Option<attiny84a::TC0>>> = Mutex::new(RefCell::new(None));

#[avr_device::entry]
fn main() -> ! {
    let dp = attiny84a::Peripherals::take().unwrap();

    unsafe {
        // TC0: Normal mode, no prescaler
        // * TOP = 0xFF
        // * Update of OCRx at Immediate
        // * TOV flag set on MAX
        dp.TC0.tccr0b.write(|w| w.bits(0b0000_0001));
        // Place the `A` output compare register at TOP
        dp.TC0.ocr0a.write(|w| w.bits(255));
        // enable `TIM0_COMPA` interrupt
        dp.TC0.timsk0.write(|w| w.bits(0b0000_0010));

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
            PORTA.borrow(cs).replace(Some(dp.PORTA));
            PORTB.borrow(cs).replace(Some(dp.PORTB));
            TC0.borrow(cs).replace(Some(dp.TC0));
            let ref mut chess_clock = CHESS_CLOCK.borrow(cs).borrow_mut();
            let chess_clock_ref = chess_clock.deref_mut();
            chess_clock_ref.render_full();
        });
        interrupt::enable();
    }

    loop{}
}

#[interrupt(attiny84a)]
fn TIM0_COMPA() {
    interrupt::free(|cs| {
        unsafe {
            let ref mut chess_clock = CHESS_CLOCK.borrow(cs).borrow_mut();
            let chess_clock_ref = chess_clock.deref_mut();
            
            if let Some(ref mut tc0) = TC0.borrow(cs).borrow_mut().deref_mut() {
                if let Some(ref mut porta) = PORTA.borrow(cs).borrow_mut().deref_mut() {
                    // b high, a high, mux1_enable high: enable sw3
                    porta.porta.modify(|r, w| w.bits(
                        r.bits() | 0b0000_0111
                    ));

                    let mut start_time = tc0.tcnt0.read().bits();
                    'wait_sw1: loop {
                        if tc0.tcnt0.read().bits().wrapping_sub(start_time) >= 4 {
                            chess_clock_ref.record_keystate(None, porta.pina.read().pa3().bit_is_clear());
                            break 'wait_sw1;
                        }
                    }

                    // a low: enable sw1
                    porta.porta.modify(|r, w| w.bits(
                        r.bits() & 0b1111_1101
                    ));

                    start_time = tc0.tcnt0.read().bits();
                    'wait_sw1: loop {
                        if tc0.tcnt0.read().bits().wrapping_sub(start_time) >= 4 {
                            chess_clock_ref.record_keystate(Some(Player::A), porta.pina.read().pa3().bit_is_clear());
                            break 'wait_sw1;
                        }
                    }

                    // mux1_enable low: enable sw2
                    porta.porta.modify(|r, w| w.bits(
                        r.bits() & 0b1111_1110
                    ));

                    start_time = tc0.tcnt0.read().bits();
                    'wait_sw2: loop {
                        if tc0.tcnt0.read().bits().wrapping_sub(start_time) >= 4 {
                            chess_clock_ref.record_keystate(Some(Player::B), porta.pina.read().pa3().bit_is_clear());
                            break 'wait_sw2;
                        }
                    }

                    match chess_clock_ref.get_target() {
                        Some(Player::A) => {
                            // Drive ~{led_1} low
                            // * mux_2 enabled
                            // * a low
                            // * b low
                            porta.porta.write(|w| w.bits(
                                0b1111_1001
                            ));
                        },
                        Some(Player::B) => {
                            // Drive ~{led_2} low
                            // * mux_2 enabled
                            // * a high
                            // * b low
                            porta.porta.write(|w| w.bits(
                                0b1111_1011
                            ));
                        },
                        None => {
                            // All mux outputs high:
                            // * mux_1 enabled
                            // * a high
                            // * b high
                            porta.porta.write(|w| w.bits(
                                0b1111_1110
                            ));
                        },
                    }
                }
            }
        }
    });
}

#[interrupt(attiny84a)]
fn TIM1_OVF() {
    interrupt::free(|cs| {
        unsafe {
            let ref mut chess_clock = CHESS_CLOCK.borrow(cs).borrow_mut();
            let chess_clock_ref = chess_clock.deref_mut();
            chess_clock_ref.tick();
        }
    });
}
