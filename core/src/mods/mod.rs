// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: Option<String>,
    pub target_game: String,
}

pub struct InstalledMod {
    pub manifest: ModManifest,
    pub zip_path: PathBuf,
    pub enabled: bool,
}

pub struct ModSystem {
    pub installed_mods: HashMap<String, InstalledMod>,
    pub overrides: HashMap<String, Vec<u8>>,
}

impl ModSystem {
    pub fn new() -> Self {
        Self {
            installed_mods: HashMap::new(),
            overrides: HashMap::new(),
        }
    }

    pub fn load_mod_manifest<P: AsRef<Path>>(zip_path: P) -> Result<ModManifest, String> {
        let file = File::open(&zip_path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        let mut manifest_file = archive
            .by_name("manifest.json")
            .map_err(|_| "manifest.json not found in mod archive".to_string())?;
            
        let mut manifest_str = String::new();
        manifest_file.read_to_string(&mut manifest_str).map_err(|e| e.to_string())?;
        
        let manifest: ModManifest = serde_json::from_str(&manifest_str).map_err(|e| e.to_string())?;
        Ok(manifest)
    }

    pub fn install_mod<P: AsRef<Path>>(&mut self, zip_path: P) -> Result<String, String> {
        let manifest = Self::load_mod_manifest(&zip_path)?;
        let id = manifest.id.clone();
        
        self.installed_mods.insert(
            id.clone(),
            InstalledMod {
                manifest,
                zip_path: zip_path.as_ref().to_path_buf(),
                enabled: false,
            },
        );
        
        Ok(id)
    }

    pub fn enable_mod(&mut self, id: &str) -> Result<(), String> {
        let m = self.installed_mods.get_mut(id).ok_or_else(|| "Mod not found".to_string())?;
        if m.enabled {
            return Ok(());
        }
        
        let file = File::open(&m.zip_path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
            if file.name().starts_with("assets/") {
                let mut data = Vec::new();
                file.read_to_end(&mut data).map_err(|e| e.to_string())?;
                let asset_path = file.name()["assets/".len()..].to_string();
                self.overrides.insert(asset_path, data);
            }
        }
        
        m.enabled = true;
        Ok(())
    }

    pub fn disable_mod(&mut self, id: &str) -> Result<(), String> {
        let m = self.installed_mods.get_mut(id).ok_or_else(|| "Mod not found".to_string())?;
        m.enabled = false;
        
        self.overrides.clear();
        
        let enabled_mods: Vec<PathBuf> = self.installed_mods.values()
            .filter(|md| md.enabled)
            .map(|md| md.zip_path.clone())
            .collect();
            
        for path in enabled_mods {
            let file = File::open(path).map_err(|e| e.to_string())?;
            let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
                if file.name().starts_with("assets/") {
                    let mut data = Vec::new();
                    file.read_to_end(&mut data).map_err(|e| e.to_string())?;
                    let asset_path = file.name()["assets/".len()..].to_string();
                    self.overrides.insert(asset_path, data);
                }
            }
        }
        
        Ok(())
    }

    pub fn check_override(&self, path: &str) -> Option<&Vec<u8>> {
        self.overrides.get(path)
    }
}
