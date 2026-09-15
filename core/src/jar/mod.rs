// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

pub mod manifest;

use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

pub struct JarContainer {
    pub archive: ZipArchive<File>,
    pub manifest: manifest::JarManifest,
}

impl JarContainer {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
        
        let manifest = manifest::JarManifest::parse(&mut archive)?;
        
        Ok(Self { archive, manifest })
    }

    pub fn read_file(&mut self, name: &str) -> Result<Vec<u8>, String> {
        use std::io::Read;
        let mut file = self.archive.by_name(name).map_err(|e| e.to_string())?;
        let mut data = Vec::new();
        file.read_to_end(&mut data).map_err(|e| e.to_string())?;
        Ok(data)
    }
}
