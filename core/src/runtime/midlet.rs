// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MidletState {
    Created,
    Active,
    Paused,
    Destroyed,
}

pub struct Midlet {
    pub name: String,
    pub class_name: String,
    pub state: MidletState,
}

impl Midlet {
    pub fn new(name: &str, class_name: &str) -> Self {
        Self {
            name: name.to_string(),
            class_name: class_name.to_string(),
            state: MidletState::Created,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if self.state == MidletState::Destroyed {
            return Err("Cannot start a destroyed MIDlet".to_string());
        }
        self.state = MidletState::Active;
        println!("MIDlet {} started (Active)", self.name);
        Ok(())
    }

    pub fn pause(&mut self) -> Result<(), String> {
        if self.state != MidletState::Active {
            return Err("Can only pause an active MIDlet".to_string());
        }
        self.state = MidletState::Paused;
        println!("MIDlet {} paused", self.name);
        Ok(())
    }

    pub fn resume(&mut self) -> Result<(), String> {
        if self.state != MidletState::Paused {
            return Err("Can only resume a paused MIDlet".to_string());
        }
        self.state = MidletState::Active;
        println!("MIDlet {} resumed (Active)", self.name);
        Ok(())
    }

    pub fn destroy(&mut self, unconditional: bool) -> Result<(), String> {
        if self.state == MidletState::Destroyed {
            return Ok(());
        }
        if !unconditional && self.state == MidletState::Paused {
            // Some MIDlets might request not to be destroyed if they can, but we simplify for MVP
        }
        self.state = MidletState::Destroyed;
        println!("MIDlet {} destroyed", self.name);
        Ok(())
    }
}
