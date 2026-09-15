// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

#[derive(Debug, Clone, Default)]
pub struct JarManifest {
    pub attributes: HashMap<String, String>,
}

impl JarManifest {
    pub fn parse(archive: &mut ZipArchive<File>) -> Result<Self, String> {
        let mut file = archive
            .by_name("META-INF/MANIFEST.MF")
            .map_err(|_| "Manifest not found in JAR (META-INF/MANIFEST.MF is required)".to_string())?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;
        
        let mut attributes: HashMap<String, String> = HashMap::new();
        let mut current_key: Option<String> = None;

        for line in contents.lines() {
            if line.is_empty() {
                continue;
            }

            if line.starts_with(' ') {
                // Continuation line (wraps from previous line)
                if let Some(ref key) = current_key {
                    if let Some(val) = attributes.get_mut(key) {
                        val.push_str(&line[1..]);
                    }
                }
            } else if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_string();
                let val = line[pos + 1..].trim().to_string();
                attributes.insert(key.clone(), val);
                current_key = Some(key);
            }
        }

        Ok(Self { attributes })
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }

    pub fn midlet_name(&self) -> Option<&String> {
        self.get("MIDlet-Name")
    }

    pub fn midlet_version(&self) -> Option<&String> {
        self.get("MIDlet-Version")
    }

    pub fn midlet_vendor(&self) -> Option<&String> {
        self.get("MIDlet-Vendor")
    }

    pub fn midlet_entry_point(&self) -> Option<&String> {
        // Typically MIDlet-1 has format: "Name, Icon, ClassName"
        self.get("MIDlet-1")
    }
}
