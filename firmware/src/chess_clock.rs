use crate::timer::Timer;
use crate::descriptor::{
    char,
    Character,
    ChessClockBehavior,
    CLOCK_CHARACTER_COUNT,
    num_to_char,
    Player,
};

#[derive(Copy, Clone)]
pub struct ChessClock {
    increment_millis: [u32; 2],
    pending_updates: [bool; 2],
    target: Option<Player>,
    timers: [Timer; 2],
}

impl ChessClock {
    pub const fn new() -> Self {
        Self {
            increment_millis: [
                2000,
                2000,
            ],
            pending_updates: [false, false],
            target: None,
            timers: [
                Timer::new(870000, 200),
                Timer::new(870000, 200),
            ],
        }
    }

    pub fn get_digits(&self, player: Player) -> [Character; CLOCK_CHARACTER_COUNT] {
        let mut digits = [char::BLANK; CLOCK_CHARACTER_COUNT];

        let mut clock_ms = self.timers[player.own_idx()].remaining();
        let mut digit_value: u32 = 600000;

        let mut clock_digit: u8;
        for digit_idx in 0..CLOCK_CHARACTER_COUNT {
            clock_digit = 0;
            while clock_ms >= digit_value {
                clock_digit += 1;
                clock_ms = clock_ms - digit_value;
            }
            digit_value = if digit_idx == 1 {digit_value / 6} else {digit_value / 10};
            if clock_digit != 0 || digit_idx != 0 {
                digits[digit_idx] = num_to_char(clock_digit);
            }
        }
        digits
    }

    pub fn register_action(&mut self, actor: Option<Player>) -> Option<ChessClockBehavior> {
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

    pub fn tick(&mut self) {
        for i in 0..2 {
            self.timers[i].tick();
        }
    }
}
