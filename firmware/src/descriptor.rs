use avr_device::attiny84a;
use avr_hal_generic::eeprom;

use attiny_hal;

// the final 4 bits are the character's hexadecimalBCD representation (0bxxxx_3210)
// See table 1: output codes in the ICM7211 datasheet
pub type Character = u8;

pub type Eeprom = eeprom::Eeprom<attiny_hal::Attiny, attiny84a::EEPROM>;

pub mod code_b {
    use crate::descriptor::Character;

    pub const ZERO: Character = 0b0000_0000;
    pub const _ONE: Character = 0b0000_0001;
    pub const _TWO: Character = 0b0000_0010;
    pub const _THREE: Character = 0b0000_0011;
    pub const _FOUR: Character = 0b0000_0100;
    pub const FIVE: Character = 0b0000_0101;
    pub const _SIX: Character = 0b0000_0110;
    pub const _SEVEN: Character = 0b0000_0111;
    pub const EIGHT: Character = 0b0000_1000;
    pub const _NINE: Character = 0b0000_1001;
    pub const _DASH: Character = 0b0000_1010;
    pub const E: Character = 0b0000_1011;
    pub const H: Character = 0b0000_1100;
    pub const L: Character = 0b0000_1101;
    pub const P: Character = 0b0000_1110;
    pub const BLANK: Character = 0b0000_1111;
}


#[derive(Clone, Copy, PartialEq)]
pub enum ClockMode {
    Play,
    SetDecaminute,
    SetMinute,
    SetDecasecond,
    SetSecond,
    SetDecasecondInterval,
    SetSecondInterval,
    SetBeepTone,
    SetBeepVolume,
}

#[derive(Clone, Copy, PartialEq)]
pub enum DigitPosition {
    Decaminute,
    Minute,
    Decasecond,
    Second,
}

#[derive(Clone, Copy, PartialEq)]
pub struct DigitQuartet(u16);

pub const DIGIT_POSITIONS: [DigitPosition; 4] = [
    DigitPosition::Decaminute,
    DigitPosition::Minute,
    DigitPosition::Decasecond,
    DigitPosition::Second,
];

fn digit_quartet_position_index(digit_position: DigitPosition) -> u8 {
    match digit_position {
        DigitPosition::Decaminute => 12,
        DigitPosition::Minute => 8,
        DigitPosition::Decasecond => 4,
        DigitPosition::Second => 0,
    }
}

impl DigitQuartet {
    // Create 4 new blank digits
    pub fn new() -> DigitQuartet {
        DigitQuartet(0b1111_1111_1111_1111)
    }

    pub fn get(self, digit_position: DigitPosition) -> u8 {
        (self.0 >> digit_quartet_position_index(digit_position)) as u8 & 0b0000_1111
    }

    pub fn set(&mut self, digit_position: DigitPosition, character: Character) {
        let digit_idx = digit_quartet_position_index(digit_position);
        //          generate a mask to clear the old 4 bits     ⟶  set new 4 bits in their place 
        self.0 = (self.0 & !(0b0000_0000_0000_1111 << digit_idx)) | ((character as u16) << digit_idx);
    }
}

// the effect an external action (usually a button press) had on a chess clock
#[derive(Clone, PartialEq)]
pub enum ChessClockBehavior {
    ChangeMode,
    CycleProfile,
    EditTime,
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
