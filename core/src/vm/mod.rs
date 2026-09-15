// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

pub mod bytecode;
pub mod instructions;
pub mod native;

use crate::classfile::ClassFile;
use std::collections::HashMap;

pub struct Vm {
    pub classes: HashMap<String, ClassFile>,
    pub heap: Vec<VmObject>,
    pub call_stack: Vec<StackFrame>,
    pub framebuffer: crate::graphics::Framebuffer,
    pub back_buffer: crate::graphics::Framebuffer,
    pub audio_system: crate::audio::AudioSystem,
    pub mod_system: crate::mods::ModSystem,
    pub active_displayable: Option<usize>,
    pub static_fields: HashMap<String, VmValue>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum VmValue {
    Null,
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ObjectRef(usize),
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct VmObject {
    pub class_name: String,
    pub fields: HashMap<String, VmValue>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct StackFrame {
    pub class_name: String,
    pub local_variables: Vec<VmValue>,
    pub operand_stack: Vec<VmValue>,
    pub program_counter: usize,
    pub bytecode: Vec<u8>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct VmSaveState {
    pub heap: Vec<VmObject>,
    pub call_stack: Vec<StackFrame>,
}

impl StackFrame {
    pub fn new(class_name: &str, max_stack: u16, max_locals: u16, bytecode: Vec<u8>) -> Self {
        Self {
            class_name: class_name.to_string(),
            local_variables: vec![VmValue::Null; max_locals as usize],
            operand_stack: Vec::with_capacity(max_stack as usize),
            program_counter: 0,
            bytecode,
        }
    }

    pub fn pop(&mut self) -> Result<VmValue, String> {
        self.operand_stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }

    pub fn push(&mut self, value: VmValue) {
        self.operand_stack.push(value);
    }

    pub fn pop_int(&mut self) -> Result<i32, String> {
        match self.pop()? {
            VmValue::Int(v) => Ok(v),
            _ => Err("Expected Int on operand stack".to_string()),
        }
    }

    pub fn push_int(&mut self, val: i32) {
        self.push(VmValue::Int(val));
    }
}

impl Vm {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
            heap: Vec::new(),
            call_stack: Vec::new(),
            framebuffer: crate::graphics::Framebuffer::new(240, 320),
            back_buffer: crate::graphics::Framebuffer::new(240, 320),
            audio_system: crate::audio::AudioSystem::new(),
            mod_system: crate::mods::ModSystem::new(),
            active_displayable: None,
            static_fields: HashMap::new(),
        }
    }

    pub fn load_class(&mut self, name: &str, bytes: &[u8]) -> Result<(), String> {
        let class = ClassFile::parse(bytes)?;
        self.classes.insert(name.to_string(), class);
        Ok(())
    }

    pub fn execute(&mut self) -> Result<(), String> {
        while let Some(mut frame) = self.call_stack.pop() {
            let res = instructions::execute_frame(self, &mut frame);
            match res {
                Ok(instructions::FrameResult::Invoked(next_frame)) => {
                    self.call_stack.push(frame); // Save current frame
                    self.call_stack.push(next_frame); // Push invoked frame
                }
                Ok(instructions::FrameResult::Completed) => {
                    // Frame completed (returned), so we do not push it back
                }
                Ok(instructions::FrameResult::Suspended) => {
                    self.call_stack.push(frame); // Save current frame as suspended
                    break; // Suspend execution for this tick
                }
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    pub fn serialize_state(&self) -> Result<Vec<u8>, String> {
        let state = VmSaveState {
            heap: self.heap.clone(),
            call_stack: self.call_stack.clone(),
        };
        serde_json::to_vec(&state).map_err(|e| e.to_string())
    }

    pub fn deserialize_state(&mut self, data: &[u8]) -> Result<(), String> {
        let state: VmSaveState = serde_json::from_slice(data).map_err(|e| e.to_string())?;
        self.heap = state.heap;
        self.call_stack = state.call_stack;
        Ok(())
    }
}
