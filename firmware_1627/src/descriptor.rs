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

#[derive(Clone, Copy, PartialEq)]
pub enum CharPosition {
    Decaminute,
    Minute,
    Decasecond,
    Second,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ClockMode {
    TurnA,
    TurnB,
    Pause,
}

#[derive(Clone, Copy, PartialEq)]
pub struct CharQuartet(u16);

pub const CHAR_POSITIONS: [CharPosition; 4] = [
    CharPosition::Decaminute,
    CharPosition::Minute,
    CharPosition::Decasecond,
    CharPosition::Second,
];

fn digit_quartet_position_index(char_position: CharPosition) -> u8 {
    match char_position {
        CharPosition::Decaminute => 12,
        CharPosition::Minute => 8,
        CharPosition::Decasecond => 4,
        CharPosition::Second => 0,
    }
}

impl CharQuartet {
    // Create 4 new blank digits
    pub fn new() -> CharQuartet {
        CharQuartet(0b1111_1111_1111_1111)
    }

    pub fn get(self, char_position: CharPosition) -> u8 {
        (self.0 >> digit_quartet_position_index(char_position)) as u8 & 0b0000_1111
    }

    pub fn set(&mut self, char_position: CharPosition, character: Character) {
        let digit_idx = digit_quartet_position_index(char_position);
        //          generate a mask to clear the old 4 bits     ⟶  set new 4 bits in their place 
        self.0 = (self.0 & !(0b0000_0000_0000_1111 << digit_idx)) | ((character as u16) << digit_idx);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    A,
    B,
}

impl Player {
    pub const fn opponent(&self) -> Player {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    pub const fn opponent_idx(&self) -> usize {
        match self {
            Player::A => 1,
            Player::B => 0,
        }
    }

    pub const fn own_idx(&self) -> usize {
        match self {
            Player::A => 0,
            Player::B => 1,
        }
    }
}
