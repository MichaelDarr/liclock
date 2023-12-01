use avr_device::interrupt;
use crate::timer::{
    LONGPRESS_CONFIRMATION_REPORTS,
    REQUIRED_CONFIRMATION_REPORTS,
    Timer,
};
use crate::{
    PORTA,
    PORTB,
    TC0,
};
use crate::descriptor::{
    Character,
    ChessClockBehavior,
    CLOCK_CHARACTER_COUNT,
    code_b,
    num_to_code_b,
    Player,
};
use core::ops::DerefMut;

pub struct ChessClock {
    ctrl_pressed_confirmations: Option<u8>,
    increment_millis: [u32; 2],
    target: Option<Player>,
    timers: [Timer; 2],
    requires_refresh: bool,
}

impl ChessClock {
    pub const fn new() -> Self {
        Self {
            ctrl_pressed_confirmations: None,
            increment_millis: [
                5000,
                5000,
            ],
            requires_refresh: false,
            target: None,
            timers: [
                Timer::new(900000, 200),
                Timer::new(900000, 200),
            ],
        }
    }

    pub fn get_target(&self) -> Option<Player> {
        return self.target;
    }

    pub unsafe fn render(&self, player: Player) {
        interrupt::free(|cs| {
            if let Some(ref mut porta) = PORTA.borrow(cs).borrow_mut().deref_mut() {
                // convert PA3 into an output
                porta.ddra.modify(|r, w| w.bits(
                    r.bits() | 0b0000_1000
                ));
                if let Some(ref mut portb) = PORTB.borrow(cs).borrow_mut().deref_mut() {
                    if let Some(ref mut tc0) = TC0.borrow(cs).borrow_mut().deref_mut() {
                        let digits = self.get_digits(player);
                        match player {
                            Player::A => {
                                // prepare to send to clock 1, a low, b low
                                porta.porta.write(|w| w.bits(0b0000_0001));
                            },
                            Player::B => {
                                // prepare to send to clock 2, a high, b low
                                porta.porta.write(|w| w.bits(0b0000_0011));
                            },
                        }
                        for digit_idx in 0..4 {
                            let tics_start = tc0.tcnt0.read().bits();
                            let digit = digits[3-digit_idx];
            
                            match digit_idx {
                                0 => {
                                    // PA4 low, PA5 low
                                    porta.porta.modify(|r, w| w.bits(
                                        r.bits() & 0b1100_1111
                                    ));
                                },
                                1 => {
                                    // PA4 high, PA5 low
                                    porta.porta.modify(|r, w| w.bits(
                                        (r.bits() & 0b1101_1111) | 0b0001_0000
                                    ));
                                },
                                2 => {
                                    // PA4 low, PA5 high
                                    porta.porta.modify(|r, w| w.bits(
                                        (r.bits() & 0b1110_1111) | 0b0010_0000
                                    ));
                                },
                                3 => {
                                    // PA4 high, PA5 high
                                    porta.porta.modify(|r, w| w.bits(
                                        r.bits() | 0b0011_0000
                                    ));
                                },
                                _ => {},
                            }
            
                            if digit & 0b0000_0001 == 1 {
                                // PB2 high
                                portb.portb.modify(|r, w| w.bits(
                                    r.bits() | 0b0000_0100
                                ));
                            } else {
                                // PB2 low
                                portb.portb.modify(|r, w| w.bits(
                                    r.bits() & 0b1111_1011
                                ));
                            }
                            if (digit >> 1) & 0b0000_0001 == 1 {
                                // PA7 high
                                porta.porta.modify(|r, w| w.bits(
                                    r.bits() | 0b1000_0000
                                ));
                            } else {
                                // PA7 low
                                porta.porta.modify(|r, w| w.bits(
                                    r.bits() & 0b0111_1111
                                ));
                            }
                            if (digit >> 2) & 0b0000_0001 == 1 {
                                // PA6 high
                                porta.porta.modify(|r, w| w.bits(
                                    r.bits() | 0b0100_0000
                                ));
                            } else {
                                // PA6 low
                                porta.porta.modify(|r, w| w.bits(
                                    r.bits() & 0b1011_1111
                                ));
                            }
                            if (digit >> 3) & 0b0000_0001 == 1 {
                                // PA3 high
                                porta.porta.modify(|r, w| w.bits(
                                    r.bits() | 0b0000_1000
                                ));
                            } else {
                                // PA3 low
                                porta.porta.modify(|r, w| w.bits(
                                    r.bits() & 0b1111_0111
                                ));
                            }
            
                            // pulse mux_1_enable for at least 200ns (5 clock cycles)
                            let tcsa_start = tc0.tcnt0.read().bits();
                            porta.porta.modify(|r, w| w.bits(
                                r.bits() & 0b1111_1110
                            ));
                            'wait_ics: loop {
                                if tc0.tcnt0.read().bits().wrapping_sub(tcsa_start) >= 3 {
                                    break 'wait_ics;
                                }
                            }
                            porta.porta.modify(|r, w| w.bits(
                                r.bits() | 0b0000_0001
                            ));
            
                            // Allocate >= 2uS between each digit place
                            if digit_idx != 3 {
                                'wait_ics: loop {
                                    if tc0.tcnt0.read().bits().wrapping_sub(tics_start) >= 38 {
                                        break 'wait_ics;
                                    }
                                }
                            }       
                        }
                    }
                }
                // revert PA3 back to a pull-up input, driving high for the transition (see datasheet page 61, section 10.1.3)
                // Additionally, pull b high to end the write
                porta.porta.modify(|r, w| w.bits(
                    r.bits() | 0b0000_1100
                ));
                porta.ddra.modify(|r, w| w.bits(
                    r.bits() & 0b1111_0111
                ));
            }
        });
    }

    pub fn record_keystate(&mut self, actor: Option<Player>, active: bool) {
        match actor {
            Some(player) => {
                if self.timers[player.own_idx()].report(active) {
                    match self.register_action(actor, false) {
                        Some(ChessClockBehavior::ToggleTurn) => {
                            self.requires_refresh = true;
                        },
                        _ => {},
                    }
                }
            },
            None => {
                match self.ctrl_pressed_confirmations {
                    Some(confirmations) => {
                        if active {
                            if confirmations < 255 {
                                self.ctrl_pressed_confirmations = Some(confirmations + 1);
                            }
                        } else {
                            if confirmations >= LONGPRESS_CONFIRMATION_REPORTS {
                                self.register_action(None, true);
                            } else if confirmations > REQUIRED_CONFIRMATION_REPORTS {
                                self.register_action(None, false);
                            }
                            self.ctrl_pressed_confirmations = None;
                        }
                    },
                    None => {
                        if active {
                            self.ctrl_pressed_confirmations = Some(1);
                        } else {
                            self.ctrl_pressed_confirmations = None;
                        }
                    },
                }
            },
        }
    }

    pub unsafe fn render_full(&self) {
        for player in [Player::A, Player::B] {
            self.render(player);
        }
    }

    pub fn get_digits(&self, player: Player) -> [Character; CLOCK_CHARACTER_COUNT] {
        let mut digits: [Character; 4] = [code_b::BLANK; CLOCK_CHARACTER_COUNT];

        let mut clock_ms = self.timers[player.own_idx()].remaining();
        let mut digit_value: u32 = 600000;

        for digit_idx in 0..CLOCK_CHARACTER_COUNT {
            let mut clock_digit: u8 = 0;
            while clock_ms >= digit_value {
                clock_digit += 1;
                clock_ms = clock_ms - digit_value;
            }
            digit_value = if digit_idx == 1 {digit_value / 6} else {digit_value / 10};
            if clock_digit != 0 || digit_idx != 0 {
                digits[digit_idx] = num_to_code_b(clock_digit);
            }
        }
        digits
    }

    // register_action registers an external action taken upon the clock.
    // A true `apply_mod` value indicates the presence of an action modifier, like a long press instead of a short one.
    pub fn register_action(&mut self, actor: Option<Player>, apply_mod: bool) -> Option<ChessClockBehavior> {
        match actor {
            // player action
            Some(player) => {
                match &self.target {
                    Some(target) => {
                        // if it's the actors turn and time is still on their clock, switch turns
                        if *target == player && !self.timers[player.own_idx()].is_expired() {
                            self.target = Some(player.opponent());
                            self.timers[player.own_idx()].halt();
                            self.timers[player.own_idx()].increment(self.increment_millis[player.own_idx()]);
                            self.timers[player.opponent_idx()].run();
                            return Some(ChessClockBehavior::ToggleTurn);
                        }
                    }
                    // resume
                    None => {
                        self.target = Some(player.opponent());
                        self.timers[player.opponent_idx()].run();
                        return Some(ChessClockBehavior::Resume);
                    }
                }
            },
            // control action
            None => {
                match &self.target {
                    // pause
                    Some(_) => {
                        for i in 0..2 {
                            self.timers[i].halt();
                        }
                        self.target = None;
                        return Some(ChessClockBehavior::Pause);
                    }
                    // reset
                    None => {
                        for i in 0..2 {
                            self.timers[i].reset();
                        }
                        return Some(ChessClockBehavior::Reset);
                    }
                }
            }
        }

        // nop
        return None;
    }

    pub unsafe fn tick(&mut self) {
        for player in [Player::A, Player::B] {
            if self.timers[player.own_idx()].tick() && !self.requires_refresh {
                self.render(player);
            }
        }
        if self.requires_refresh {
            self.render_full();
        }
    }
}
