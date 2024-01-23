#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(abi_avr_interrupt)]
#![feature(asm_experimental_arch)]
#![allow(internal_features)]
#![feature(int_roundings)]

mod chess_clock;
mod descriptor;
mod timer;

use panic_halt as _;

use avr_device::{
    attiny1627::{self, Peripherals},
    interrupt,
    interrupt::{
        CriticalSection,
        Mutex,
    },
};
use core::{
    cell::RefCell,
    ops::DerefMut,
};

use chess_clock::ChessClock;
use descriptor::{
    CHAR_POSITIONS,
    CharQuartet,
    CharPosition,
    Player, ClockMode,
};
use timer::Timer;

static mut CHESS_CLOCK: Mutex<RefCell<ChessClock>> = Mutex::new(RefCell::new(ChessClock::new()));
static mut DP: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

#[avr_device::entry]
fn main() -> ! {
    let dp = attiny1627::Peripherals::take().unwrap();

    // Set up the 32kHz oscillator (if not already enabled)
    if dp.CLKCTRL.xosc32kctrla.read().bits() & 0b0000_0001 == 0 {
        // Enable the 32kHz oscillator, even in standby
        dp.CPU.ccp.write(|w| unsafe { w.bits(0b1101_1000)}); // disable write protection
        dp.CLKCTRL.xosc32kctrla.write(|w| unsafe { w.bits(0b0000_0011)});
        // Force the 32kHz oscillator ON in all modes
        dp.CPU.ccp.write(|w| unsafe { w.bits(0b1101_1000)}); // disable write protection
        dp.CLKCTRL.osc32kctrla.write(|w| unsafe { w.bits(0b0000_0010)});
    }

    dp.RTC.clksel.write(|w| unsafe { w.bits(0b0000_0010)});

    // Enable the real time clock with a 256 prescaler
    // * 1 tick ≈ 8ms
    // * 128 ticks per second
    // * 500 second period
    dp.RTC.ctrla.write(|w| unsafe { w.bits(0b1100_0001)});
    // second-align the periodic interrupt timer
    dp.RTC.pitctrla.write(|w| unsafe { w.bits(0b0111_0001)});
    // enable periodic interrupt
    dp.RTC.pitintctrl.write(|w| unsafe { w.bits(0b0000_0001)});

    // Enable timer A
    dp.TCA0.ctrla.write(|w| unsafe { w.bits(0b0000_0001)});

    // Configure PA1..7 as output pins
    dp.PORTA.dirset.write(|w| unsafe {
        w.bits(0b1111_1110)
    });
    // Configure PB0 (low battery led), PB 4..7 (LCD control signals) as output pins
    dp.PORTB.dirset.write(|w| unsafe {
        w.bits(0b1111_0001)
    });
    // Configure PC1 (switch 1 led), PC3 (switch 2 led), and PC5 (buzzer) as output pins
    dp.PORTC.dirset.write(|w| unsafe {
        w.bits(0b0010_1010)
    });

    // Set up the three input pins attached to switches (pull-up, interrupt on falling edge)
    dp.PORTC.pin0ctrl.write(|w| unsafe { w.bits(0b0000_1011) });
    dp.PORTC.pin2ctrl.write(|w| unsafe { w.bits(0b0000_1011) });
    dp.PORTC.pin4ctrl.write(|w| unsafe { w.bits(0b0000_1011) });

    // Drive LEDs high
    // dp.PORTC.outset.write(|w| unsafe {
    //     w.bits(0b0000_1010)
    // });

    // Disable all LCD write pins
    dp.PORTA.outset.write(|w| unsafe { w.bits(0b0000_1110) });

    unsafe {
        interrupt::free(|cs| {
            DP.borrow(cs).replace(Some(dp));
        });
        interrupt::enable();
    }

    interrupt::free(|cs| unsafe {
        let ref mut chess_clock = CHESS_CLOCK.borrow(cs).borrow_mut();
        render(cs, chess_clock.get_digits(Player::A), Player::A, true);
        render(cs, chess_clock.get_digits(Player::B), Player::B, true);
    });

    unsafe {
        interrupt::enable();
    }

    loop{}
}


#[interrupt(attiny1627)]
fn RTC_PIT() {
    interrupt::free(|cs| unsafe {
        if let Some(ref mut dp) = DP.borrow(cs).borrow_mut().deref_mut() {
            dp.RTC.pitintflags.write(|w| {w.bits(0b0000_0001)});
        }
        let ref mut chess_clock = CHESS_CLOCK.borrow(cs).borrow_mut();
        if let Some(active_player) = chess_clock.tick() {
            render(cs, chess_clock.get_digits(active_player), active_player, true);
        }
    });
}

#[interrupt(attiny1627)]
fn PORTC_PORT() {
    static mut PLAYER_A_PRESSED: u16 = 0;
    static mut PLAYER_B_PRESSED: u16 = 0;
    static mut CONTROL_PRESSED: u16 = 0;

    let mut new_player_a_pressed = *PLAYER_A_PRESSED;
    let mut new_player_b_pressed = *PLAYER_B_PRESSED;
    let mut new_control_pressed = *CONTROL_PRESSED;

    let mut chars_a: Option<CharQuartet> = None;
    let mut chars_b: Option<CharQuartet> = None;

    interrupt::free(|cs| unsafe {
        if let Some(ref mut dp) = DP.borrow(cs).borrow_mut().deref_mut() {
            let ref mut chess_clock = CHESS_CLOCK.borrow(cs).borrow_mut();

            if dp.PORTC.intflags.read().pc0().bit_is_set() {
                // Player A button press detected
                dp.PORTC.intflags.modify(|r, w| { w.bits(r.bits() & 0b0000_0001)});
                if chess_clock.mode != ClockMode::TurnB {
                    let now = dp.RTC.cnt.read().bits();
                    if now < new_player_a_pressed || now - new_player_a_pressed > 25 {
                        dp.PORTC.outset.write(|w| {w.bits(0b0000_1000)});
                        dp.PORTC.outclr.write(|w| {w.bits(0b0000_0010)});
                        if chess_clock.mode == ClockMode::TurnA {
                            let inc = chess_clock.increment[Player::A.own_idx()];
                            chess_clock.timers[Player::A.own_idx()].increment(inc);
                            chars_a = Some(chess_clock.get_digits(Player::A));
                            // Buzzzzzz
                            for _ in 1..100 {
                                for _ in 1..155 {
                                    dp.PORTA.outtgl.write(|w| { w.bits(0b0000_0000) });
                                }
                                dp.PORTC.outtgl.write(|w| { w.bits(0b0010_0000) });
                            }
                        }
                        chess_clock.mode = ClockMode::TurnB;
                        new_player_a_pressed = now;
                    }
                }
            }
            if dp.PORTC.intflags.read().pc2().bit_is_set() {
                // Player B button press detected
                dp.PORTC.intflags.modify(|r, w| { w.bits(r.bits() & 0b0000_0100)});
                if chess_clock.mode != ClockMode::TurnA {
                    let now = dp.RTC.cnt.read().bits();
                    if now < new_player_b_pressed || now - new_player_b_pressed > 25 {
                        dp.PORTC.outclr.write(|w| {w.bits(0b0000_1000)});
                        dp.PORTC.outset.write(|w| {w.bits(0b0000_0010)});
                        if chess_clock.mode == ClockMode::TurnB {
                            let inc = chess_clock.increment[Player::B.own_idx()];
                            chess_clock.timers[Player::B.own_idx()].increment(inc);
                            chars_b = Some(chess_clock.get_digits(Player::B));
                            // Buzzzzzz
                            for _ in 1..100 {
                                for _ in 1..155 {
                                    dp.PORTA.outtgl.write(|w| { w.bits(0b0000_0000) });
                                }
                                dp.PORTC.outtgl.write(|w| { w.bits(0b0010_0000) });
                            }
                        }
                        chess_clock.mode = ClockMode::TurnA;
                        new_player_b_pressed = now;
                    }
                }
            }
            if dp.PORTC.intflags.read().pc4().bit_is_set() {
                // Control button press detected
                dp.PORTC.intflags.modify(|r, w| { w.bits(r.bits() & 0b0001_0000)});
                let now = dp.RTC.cnt.read().bits();
                if now < new_control_pressed || now - new_control_pressed > 25 {
                    dp.PORTC.outset.write(|w| {w.bits(0b0000_1010)});
                    if chess_clock.mode == ClockMode::Pause {
                        chess_clock.increment = [0, 0];
                        chess_clock.timers = [Timer::new(300), Timer::new(300)];
                        chars_a = Some(chess_clock.get_digits(Player::A));
                        chars_b = Some(chess_clock.get_digits(Player::B));
                    } else {
                        chess_clock.mode = ClockMode::Pause;
                    }
                    new_control_pressed = now;
                }
            }
        }

        if let Some(chars) = chars_a {
            render(cs, chars, Player::A, true);
        }
        if let Some(chars) = chars_b {
            render(cs, chars, Player::B, true);
        }
    });

    *PLAYER_A_PRESSED = new_player_a_pressed;
    *PLAYER_B_PRESSED = new_player_b_pressed;
    *CONTROL_PRESSED = new_control_pressed;
}

unsafe fn render(cs: CriticalSection<'_>, value: CharQuartet, player: Player, enable_col: bool) {
    if let Some(ref mut dp) = DP.borrow(cs).borrow_mut().deref_mut() {

        // CHARACTERS
        dp.PORTA.outclr.write(|w| unsafe { w.bits(0b0000_0010) });
        match player {
            Player::A => {
                dp.PORTA.outset.write(|w| unsafe { w.bits(0b0000_1000) });
                dp.PORTA.outclr.write(|w| unsafe { w.bits(0b0000_0100) });
            },
            Player::B => {
                dp.PORTA.outset.write(|w| unsafe { w.bits(0b0000_0100) });
                dp.PORTA.outclr.write(|w| unsafe { w.bits(0b0000_1000) });
            },
        }
        for char_position in CHAR_POSITIONS {
            let char = value.get(char_position);
            let ics_start = dp.TCA0.cnt().read().bits();

            match char_position {
                CharPosition::Second => {
                    dp.PORTB.outclr.write(|w| unsafe {w.bits(0b1100_0000)});
                },
                CharPosition::Decasecond => {
                    dp.PORTB.outclr.write(|w| unsafe {w.bits(0b1000_0000)});
                    dp.PORTB.outset.write(|w| unsafe {w.bits(0b0100_0000)});
                },
                CharPosition::Minute => {
                    dp.PORTB.outclr.write(|w| unsafe {w.bits(0b0100_0000)});
                    dp.PORTB.outset.write(|w| unsafe {w.bits(0b1000_0000)});
                },
                CharPosition::Decaminute => {
                    dp.PORTB.outset.write(|w| unsafe {w.bits(0b1100_0000)});
                },
            }

            if char & 0b0000_0001 == 1 {
                dp.PORTA.outset.write(|w| unsafe {w.bits(0b0001_0000)});
            } else {
                dp.PORTA.outclr.write(|w| unsafe {w.bits(0b0001_0000)});
            }
            if (char >> 1) & 0b0000_0001 == 1 {
                dp.PORTA.outset.write(|w| unsafe {w.bits(0b0010_0000)});
            } else {
                dp.PORTA.outclr.write(|w| unsafe {w.bits(0b0010_0000)});
            }
            if (char >> 2) & 0b0000_0001 == 1 {
                dp.PORTA.outset.write(|w| unsafe {w.bits(0b0100_0000)});
            } else {
                dp.PORTA.outclr.write(|w| unsafe {w.bits(0b0100_0000)});
            }
            if (char >> 3) & 0b0000_0001 == 1 {
                dp.PORTA.outset.write(|w| unsafe {w.bits(0b1000_0000)});
            } else {
                dp.PORTA.outclr.write(|w| unsafe {w.bits(0b1000_0000)});
            }

            // Prepare char write
            dp.PORTA.outclr.write(|w| unsafe {
                w.bits(0b0000_0010)
            });
            let tcsa_start = dp.TCA0.cnt().read().bits();
            'wait_ics: loop {
                if dp.TCA0.cnt().read().bits().wrapping_sub(tcsa_start) >= 1 {
                    break 'wait_ics;
                }
            }

            // Allocate >= 2uS between each digit place (don't wait after the final digit)
            if char_position == CharPosition::Second {
                // Finish LCD write cycle
                dp.PORTA.outset.write(|w| unsafe {
                    w.bits(0b0000_1110)
                });
            } else {
                // Latch the digit
                dp.PORTA.outset.write(|w| unsafe {
                    w.bits(0b0000_0010)
                });
                'wait_ics: loop {
                    if dp.TCA0.cnt().read().bits().wrapping_sub(ics_start) >= 5 {
                        break 'wait_ics;
                    }
                }
            }
        }

        // COLON
        let col_bit_pos = match player {
            Player::A => 0b0001_0000,
            Player::B => 0b0010_0000,
        };
        if enable_col {
            dp.PORTB.outset.write(|w| unsafe { w.bits(col_bit_pos) });
        } else {
            dp.PORTB.outclr.write(|w| unsafe { w.bits(col_bit_pos) });
        }
    }
}
