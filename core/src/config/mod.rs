// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceProfile {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub vendor: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmulatorConfig {
    pub volume: u8,
    pub device_profile: DeviceProfile,
    pub keyboard_mappings: HashMap<String, u32>,
}

impl Default for EmulatorConfig {
    fn default() -> Self {
        let mut mappings = HashMap::new();
        mappings.insert("W".to_string(), 1);      // Up
        mappings.insert("S".to_string(), 2);      // Down
        mappings.insert("A".to_string(), 3);      // Left
        mappings.insert("D".to_string(), 4);      // Right
        mappings.insert("Return".to_string(), 5); // Fire
        mappings.insert("Num1".to_string(), 6);
        mappings.insert("Num2".to_string(), 7);
        mappings.insert("Num3".to_string(), 8);
        mappings.insert("Num4".to_string(), 9);
        mappings.insert("Num5".to_string(), 10);
        mappings.insert("Num6".to_string(), 11);
        mappings.insert("Num7".to_string(), 12);
        mappings.insert("Num8".to_string(), 13);
        mappings.insert("Num9".to_string(), 14);
        mappings.insert("Num0".to_string(), 15);
        mappings.insert("Q".to_string(), 16);     // Star
        mappings.insert("E".to_string(), 17);     // Hash
        mappings.insert("F1".to_string(), 18);    // SoftLeft
        mappings.insert("F2".to_string(), 19);    // SoftRight

        Self {
            volume: 80,
            device_profile: DeviceProfile {
                name: "Nokia 240x320".to_string(),
                width: 240,
                height: 320,
                vendor: "Nokia".to_string(),
            },
            keyboard_mappings: mappings,
        }
    }
}
