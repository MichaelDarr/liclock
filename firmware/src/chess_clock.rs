use crate::{
    descriptor::{
        ClockMode,
        Character,
        CHAR_POSITIONS,
        CharPosition,
        CharQuartet,
        Player,
    },
    timer::Timer,
};

#[derive(Clone, PartialEq)]
pub struct ChessClock {
    pub increment: [u32; 2],
    pub mode: ClockMode,
    pub timers: [Timer; 2],
}

impl ChessClock {
    pub const fn new() -> Self {
        return Self {
            increment: [2, 2],
            mode: ClockMode::Pause,
            timers: [Timer::new(420), Timer::new(420)],
        };
    }

    pub fn get_digits(&self, player: Player) -> CharQuartet {
        let mut digits = CharQuartet::new();

        let mut clock_ms = self.timers[player.own_idx()].remaining();
        let mut digit_value: u32 = 600;
        for digit_position in CHAR_POSITIONS {
            let mut clock_digit: Character = 0b0000_0000;
            while clock_ms >= digit_value {
                clock_digit += 1;
                clock_ms = clock_ms - digit_value;
            }
            digit_value = if digit_position == CharPosition::Minute {digit_value / 6} else {digit_value / 10};
            digits.set(digit_position, clock_digit);
        }

        digits
    }

    pub fn active_player(&mut self) -> Option<Player> {
        match self.mode {
            ClockMode::TurnA => Some(Player::A),
            ClockMode::TurnB => Some(Player::B),
            _ => None,
        }
    }

    pub unsafe fn tick(&mut self) -> Option<Player> {
        if let Some(active_player) = self.active_player() {
            self.timers[active_player.own_idx()].tick();
            return Some(active_player);
        }
        return None;
    }
}
