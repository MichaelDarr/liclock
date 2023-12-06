use crate::EEPROM;

use avr_device::interrupt;
use core::ops::DerefMut;
use crate::{
    ChessClock,
    descriptor::Player,
};

// Clock configuration is encoded in 128 bits (16 bytes) for storage in the EEPROM
// ┍━━━━━━━┯━━━━━━━━┳━━━━━━━┯━━━━━━━┯━━━━━━━┯━━━━━━━┯━━━━━━━┯━━━━━━━┯━━━━━━━┯━━━━━━━┑
// │Address│  Name  ┃ Bit 7 │ Bit 6 │ Bit 5 │ Bit 4 │ Bit 3 │ Bit 2 │ Bit 1 │ Bit 0 │
// ┝━━━━━━━┿━━━━━━━━╋━━━━━━━┷━━━━━━━┷━━━━━━━┷━━━━━━━┷━━━━━━━┷━━━━━━━┷━━━━━━━┷━━━━━━━┥
// │  0x0  │ DURAH  ┃         Player A: Total Duration, Seconds - High Byte         │
// ├───────┼────────╂───────────────────────────────────────────────────────────────┤
// │  0x1  │ DURAL  ┃         Player A: Total Duration, Seconds - Low Byte          │
// ├───────┼────────╂───────────────────────────────────────────────────────────────┤
// │  0x2  │ DURBH  ┃         Player B: Total Duration, Seconds - High Byte         │
// ├───────┼────────╂───────────────────────────────────────────────────────────────┤
// │  0x3  │ DURBL  ┃         Player B: Total Duration, Seconds - Low Byte          │
// ├───────┼────────╂───────────────────────────────────────────────────────────────┤
// │  0x4  │  INCA  ┃                   Player A: Increment, Seconds                │
// ├───────┼────────╂───────────────────────────────────────────────────────────────┤
// │  0x5  │  INCB  ┃                   Player B: Increment, Seconds                │
// ├───────┼────────╂───────────────────────────────────────────────────────────────┤
// │  0x6  │  DLYA  ┃                Player A: Delay, Seconds                       │
// ├───────┼────────╂───────────────────────────────────────────────────────────────┤
// │  0x7  │  DLYB  ┃                Player B: Delay, Seconds                       │
// ├───────┼────────╂───────┬───────┬───────────────┬───────┬───────────────────────┤
// │  0x8  │  BEEP  ┃   -   │   -   │   Beep Tone   │   -   │      Beep Volume      │
// ├───────┼────────╂───────┼───────┼───────┬───────┼───────┼───────┬───────┬───────┤
// │  0x9  │Reserved┃   -   │   -   │   -   │   -   │   -   │   -   │   -   │   -   │
// ├───────┼────────╂───────┼───────┼───────┼───────┼───────┼───────┼───────┼───────┤
// │  0xa  │Reserved┃   -   │   -   │   -   │   -   │   -   │   -   │   -   │   -   │
// ├───────┼────────╂───────┼───────┼───────┼───────┼───────┼───────┼───────┼───────┤
// │  0xb  │Reserved┃   -   │   -   │   -   │   -   │   -   │   -   │   -   │   -   │
// ├───────┼────────╂───────┼───────┼───────┼───────┼───────┼───────┼───────┼───────┤
// │  0xc  │Reserved┃   -   │   -   │   -   │   -   │   -   │   -   │   -   │   -   │
// ├───────┼────────╂───────┼───────┼───────┼───────┼───────┼───────┼───────┼───────┤
// │  0xd  │Reserved┃   -   │   -   │   -   │   -   │   -   │   -   │   -   │   -   │
// ├───────┼────────╂───────┼───────┼───────┼───────┼───────┼───────┼───────┼───────┤
// │  0xe  │Reserved┃   -   │   -   │   -   │   -   │   -   │   -   │   -   │   -   │
// ├───────┼────────╂───────┼───────┼───────┼───────┼───────┼───────┼───────┼───────┤
// │  0xf  │Reserved┃   -   │   -   │   -   │   -   │   -   │   -   │   -   │   -   │
// └───────┴────────┸───────┴───────┴───────┴───────┴───────┴───────┴───────┴───────┘
pub struct ChessClockConfig {
  data: [u8; 16],
}

impl From<ChessClock> for ChessClockConfig {
    fn from(chess_clock: ChessClock) -> Self {
        let mut data = [0_u8; 16];

        for player in [Player::A, Player::B] {
            let player_duration = chess_clock.timers[player.own_idx()].duration_millis / 1000;
            let high_byte_offset = player.own_idx() * 2;
            data[high_byte_offset] = (player_duration >> 8) as u8;
            data[high_byte_offset + 1] = ((player_duration << 8) >> 8) as u8;

            data[4 + player.own_idx()] = (chess_clock.increment_millis[player.own_idx()] / 1000) as u8;
            data[6 + player.own_idx()] = (chess_clock.delay_millis[player.own_idx()] / 1000) as u8;
        }
        data[8] = chess_clock.beep_volume | (chess_clock.beep_tone << 4);

        return ChessClockConfig{data};
    }
}

impl Into<ChessClock> for ChessClockConfig {
    fn into(self) -> ChessClock {
        return ChessClock::new(Some(self));
    }
}

impl ChessClockConfig {
    // Create a new chess clock config
    // If `offset` is `None`, the config is completely blank (all 0's)
    // Otherwise, the config is populated with the 64 bits of info at `offset` within the EEPROM.
    pub unsafe fn new(offset: Option<u16>) -> Self {
        match offset {
            Some(config_offset) => {
                return interrupt::free(|cs| {
                    if let Some(ref mut eeprom) = EEPROM.borrow(cs).borrow_mut().deref_mut() {
                        let mut data = [0_u8; 16];
                        if eeprom.read(config_offset, &mut data).is_err() {
                            // TODO => handle?
                            panic!("failed to read eeprom")
                        }
                        return ChessClockConfig{data};
                    } else {
                        // TODO => handle?
                        panic!("failed to get eeprom")
                    }
                });
            },
            None => {
                return ChessClockConfig{
                    data: [0_u8; 16],
                }
            }
        }
    }

    pub unsafe fn persist(&self, offset: u16) {
        interrupt::free(|cs| {
            if let Some(ref mut eeprom) = EEPROM.borrow(cs).borrow_mut().deref_mut() {
                if eeprom.write(offset, self.data.clone().as_mut()).is_err() {
                    // TODO => handle?
                    panic!("failed to write eeprom")
                }
            } else {
                // TODO => handle?
                panic!("failed to get eeprom")
            }
        });

    }

    pub const fn beep_tone(&self) -> u8 {
        return (self.data[8] << 2) >> 6;
    }

    pub const fn beep_volume(&self) -> u8 {
        return (self.data[8] << 5) >> 5;
    }

    pub const fn delay_millis(&self, player: Player) -> u32 {
        return( (self.data[6 + player.own_idx()]) as u32) * 1000;
    }

    pub const fn duration_millis(&self, player: Player) -> u32 {
        let high_byte_offset = player.own_idx()*2;
        return (((self.data[high_byte_offset] as u32) << 8) | (self.data[high_byte_offset + 1] as u32)) * 1000;
    }

    pub const fn increment_millis(&self, player: Player) -> u32 {
        return (self.data[4 + player.own_idx()] as u32)* 1000
    }
}
