pub const CLOCK_CHARACTER_COUNT: usize = 4;

// the final 4 bits are the character's hexadecimalBCD representation (0bxxxx_3210)
// See table 1: output codes in the ICM7211 datasheet
pub type Character = u8;

pub mod code_b {
    use crate::descriptor::Character;

    pub const ZERO: Character = 0b0000_0000;
    pub const ONE: Character = 0b0000_0001;
    pub const TWO: Character = 0b0000_0010;
    pub const THREE: Character = 0b0000_0011;
    pub const FOUR: Character = 0b0000_0100;
    pub const FIVE: Character = 0b0000_0101;
    pub const SIX: Character = 0b0000_0110;
    pub const SEVEN: Character = 0b0000_0111;
    pub const EIGHT: Character = 0b0000_1000;
    pub const NINE: Character = 0b0000_1001;
    pub const DASH: Character = 0b0000_1010;
    pub const E: Character = 0b0000_1011;
    pub const H: Character = 0b0000_1100;
    pub const L: Character = 0b0000_1101;
    pub const P: Character = 0b0000_1110;
    pub const BLANK: Character = 0b0000_1111;
}

// transform a single-digit number (0-9, inclusive) to a character
// if the provided number is greater than 9 (two digits), an empty character is returned
pub fn num_to_code_b(src: u8) -> Character {
    match src {
        0 => code_b::ZERO,
        1 => code_b::ONE,
        2 => code_b::TWO,
        3 => code_b::THREE,
        4 => code_b::FOUR,
        5 => code_b::FIVE,
        6 => code_b::SIX,
        7 => code_b::SEVEN,
        8 => code_b::EIGHT,
        9 => code_b::NINE,
        _ => code_b::ZERO,
    }
}

// the effect an external action (usually a button press) had on a chess clock
#[derive(Clone, PartialEq)]
pub enum ChessClockBehavior {
    Pause,
    Reset,
    Resume,
    ToggleTurn,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    pub fn opponent_idx(&self) -> usize {
        match self {
            Player::A => 1,
            Player::B => 0,
        }
    }

    pub fn own_idx(&self) -> usize {
        match self {
            Player::A => 0,
            Player::B => 1,
        }
    }
}