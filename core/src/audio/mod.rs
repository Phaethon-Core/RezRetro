// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

pub enum AudioFormat {
    Tone,
    Midi,
    Wav,
}

pub struct AudioEvent {
    pub format: AudioFormat,
    pub data: Vec<u8>,
}

pub struct AudioSystem {
    pub events_queue: Vec<AudioEvent>,
    pub volume: u8,
}

impl AudioSystem {
    pub fn new() -> Self {
        Self {
            events_queue: Vec::new(),
            volume: 80,
        }
    }

    pub fn play_tone(&mut self, frequency: u32, duration_ms: u32) {
        let mut data = Vec::new();
        data.extend_from_slice(&frequency.to_le_bytes());
        data.extend_from_slice(&duration_ms.to_le_bytes());
        
        self.events_queue.push(AudioEvent {
            format: AudioFormat::Tone,
            data,
        });
    }

    pub fn play_midi(&mut self, midi_data: Vec<u8>) {
        self.events_queue.push(AudioEvent {
            format: AudioFormat::Midi,
            data: midi_data,
        });
    }

    pub fn set_volume(&mut self, volume: u8) {
        self.volume = volume.min(100);
    }

    pub fn clear_queue(&mut self) {
        self.events_queue.clear();
    }
}
