// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

pub struct RecordStore {
    pub name: String,
    pub records: Vec<Option<Vec<u8>>>,
}

impl RecordStore {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, data: &[u8]) -> usize {
        self.records.push(Some(data.to_vec()));
        self.records.len()
    }

    pub fn get_record(&self, id: usize) -> Option<&[u8]> {
        if id > 0 && id <= self.records.len() {
            self.records[id - 1].as_deref()
        } else {
            None
        }
    }

    pub fn update_record(&mut self, id: usize, data: &[u8]) -> Result<(), String> {
        if id > 0 && id <= self.records.len() {
            self.records[id - 1] = Some(data.to_vec());
            Ok(())
        } else {
            Err("Invalid record ID".to_string())
        }
    }

    pub fn delete_record(&mut self, id: usize) -> Result<(), String> {
        if id > 0 && id <= self.records.len() {
            self.records[id - 1] = None;
            Ok(())
        } else {
            Err("Invalid record ID".to_string())
        }
    }

    pub fn save_to_dir<P: AsRef<Path>>(&self, dir: P) -> io::Result<()> {
        let path = dir.as_ref().join(format!("{}.rms", self.name));
        let mut file = File::create(path)?;
        
        let count = self.records.len() as u32;
        file.write_all(&count.to_le_bytes())?;
        
        for record in &self.records {
            if let Some(ref data) = record {
                file.write_all(&[1])?;
                let len = data.len() as u32;
                file.write_all(&len.to_le_bytes())?;
                file.write_all(data)?;
            } else {
                file.write_all(&[0])?;
            }
        }
        
        Ok(())
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut file = File::open(&path)?;
        let name = path.as_ref()
            .file_stem()
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid path"))?
            .to_string_lossy()
            .into_owned();
        
        let mut records = Vec::new();
        let mut count_buf = [0; 4];
        if file.read_exact(&mut count_buf).is_ok() {
            let count = u32::from_le_bytes(count_buf);
            for _ in 0..count {
                let mut active_buf = [0; 1];
                file.read_exact(&mut active_buf)?;
                if active_buf[0] == 1 {
                    let mut len_buf = [0; 4];
                    file.read_exact(&mut len_buf)?;
                    let len = u32::from_le_bytes(len_buf) as usize;
                    let mut data = vec![0; len];
                    file.read_exact(&mut data)?;
                    records.push(Some(data));
                } else {
                    records.push(None);
                }
            }
        }
        
        Ok(Self { name, records })
    }

    pub fn open(game_id: &str, store_name: &str, create: bool) -> io::Result<Self> {
        let dir = Path::new("data/saves").join(game_id);
        fs::create_dir_all(&dir)?;
        let path = dir.join(format!("{}.rms", store_name));
        if path.exists() {
            Self::load_from_file(&path)
        } else if create {
            Ok(Self::new(store_name))
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Record store not found"))
        }
    }
}
