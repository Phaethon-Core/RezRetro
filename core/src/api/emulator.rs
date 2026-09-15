// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use crate::runtime::J2meRuntime;
use std::sync::Mutex;

static EMULATOR_RUNTIME: Mutex<Option<J2meRuntime>> = Mutex::new(None);

pub fn init_emulator() -> Result<(), String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    *runtime = Some(J2meRuntime::new());
    Ok(())
}

pub fn load_game(jar_path: String) -> Result<(), String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        emu.load_jar(jar_path)
    } else {
        Err("Emulator not initialized".to_string())
    }
}

pub fn start_game() -> Result<(), String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        emu.start()
    } else {
        Err("Emulator not initialized".to_string())
    }
}

pub fn pause_game() -> Result<(), String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        emu.pause()
    } else {
        Err("Emulator not initialized".to_string())
    }
}

pub fn resume_game() -> Result<(), String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        emu.resume()
    } else {
        Err("Emulator not initialized".to_string())
    }
}

pub fn stop_game() -> Result<(), String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        emu.stop()
    } else {
        Err("Emulator not initialized".to_string())
    }
}

pub fn get_screen_width() -> i32 {
    240
}

pub fn get_screen_height() -> i32 {
    320
}

pub fn get_screen_pixels() -> Vec<u32> {
    let mut runtime = EMULATOR_RUNTIME.lock().ok();
    if let Some(ref mut opt_emu) = runtime.as_mut() {
        if let Some(ref mut emu) = opt_emu.as_mut() {
            emu.tick();
            return emu.vm.framebuffer.pixels.clone();
        }
    }
    vec![0xFF000000; 240 * 320]
}

pub fn send_key_event(key_code: i32, pressed: bool) -> Result<(), String> {
    println!("Key event: {} pressed: {}", key_code, pressed);
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        emu.handle_key(key_code, pressed);
    }
    Ok(())
}

pub fn get_next_audio_event() -> Option<String> {
    let mut runtime = EMULATOR_RUNTIME.lock().ok()?;
    if let Some(ref mut emu) = *runtime {
        if !emu.vm.audio_system.events_queue.is_empty() {
            let event = emu.vm.audio_system.events_queue.remove(0);
            match event.format {
                crate::audio::AudioFormat::Tone => {
                    let freq = u32::from_le_bytes(event.data[0..4].try_into().ok()?);
                    let dur = u32::from_le_bytes(event.data[4..8].try_into().ok()?);
                    return Some(format!("tone:{}:{}", freq, dur));
                }
                _ => return Some("midi:".to_string()),
            }
        }
    }
    None
}

pub fn install_mod(zip_path: String) -> Result<String, String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        emu.vm.mod_system.install_mod(zip_path)
    } else {
        Err("Emulator not initialized".to_string())
    }
}

pub fn toggle_mod(id: String, enable: bool) -> Result<(), String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        if enable {
            emu.vm.mod_system.enable_mod(&id)
        } else {
            emu.vm.mod_system.disable_mod(&id)
        }
    } else {
        Err("Emulator not initialized".to_string())
    }
}

pub fn save_state(filepath: String) -> Result<(), String> {
    let runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref emu) = *runtime {
        let data = emu.vm.serialize_state()?;
        std::fs::write(filepath, data).map_err(|e| e.to_string())
    } else {
        Err("Emulator not initialized".to_string())
    }
}

pub fn load_state(filepath: String) -> Result<(), String> {
    let mut runtime = EMULATOR_RUNTIME.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut emu) = *runtime {
        let data = std::fs::read(filepath).map_err(|e| e.to_string())?;
        emu.vm.deserialize_state(&data)
    } else {
        Err("Emulator not initialized".to_string())
    }
}
