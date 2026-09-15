// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum J2meKey {
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
    Fire = 5,
    Key1 = 6,
    Key2 = 7,
    Key3 = 8,
    Key4 = 9,
    Key5 = 10,
    Key6 = 11,
    Key7 = 12,
    Key8 = 13,
    Key9 = 14,
    Key0 = 15,
    Star = 16,
    Hash = 17,
    SoftLeft = 18,
    SoftRight = 19,
}

pub struct InputState {
    pub key_states: u32,
}

impl InputState {
    pub fn new() -> Self {
        Self { key_states: 0 }
    }

    pub fn set_key_state(&mut self, key: J2meKey, pressed: bool) {
        let bit = 1 << (key as u32);
        if pressed {
            self.key_states |= bit;
        } else {
            self.key_states &= !bit;
        }
    }

    pub fn is_key_pressed(&self, key: J2meKey) -> bool {
        let bit = 1 << (key as u32);
        (self.key_states & bit) != 0
    }
}
