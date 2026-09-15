// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

pub mod midlet;

use crate::jar::JarContainer;
use crate::vm::{Vm, StackFrame, VmValue, VmObject};
use midlet::Midlet;
use std::path::Path;
use std::collections::HashMap;
use zip::ZipArchive;

pub struct J2meRuntime {
    pub vm: Vm,
    pub active_midlet: Option<Midlet>,
    pub jar: Option<JarContainer>,
    pub frame_count: u32,
    pub jar_files: Vec<String>,
    pub midlet_vendor: String,
    pub midlet_version: String,
    pub entry_class: String,
    pub initialized: bool,
    pub boot_complete: bool,
    pub execution_error: Option<String>,
    pub key_queue: Vec<(i32, bool)>,
}

impl J2meRuntime {
    pub fn new() -> Self {
        Self {
            vm: Vm::new(),
            active_midlet: None,
            jar: None,
            frame_count: 0,
            jar_files: Vec::new(),
            midlet_vendor: "Unknown Vendor".to_string(),
            midlet_version: "1.0.0".to_string(),
            entry_class: String::new(),
            initialized: false,
            boot_complete: false,
            execution_error: None,
            key_queue: Vec::new(),
        }
    }

    pub fn load_jar<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let jar = JarContainer::load(&path)?;

        let midlet_name = jar.manifest.midlet_name()
            .cloned()
            .unwrap_or_else(|| "Unknown J2ME Game".to_string());

        let midlet_vendor = jar.manifest.midlet_vendor()
            .cloned()
            .unwrap_or_else(|| "Unknown Vendor".to_string());

        let midlet_version = jar.manifest.midlet_version()
            .cloned()
            .unwrap_or_else(|| "1.0.0".to_string());

        let midlet_entry = jar.manifest.midlet_entry_point()
            .cloned()
            .ok_or_else(|| "No MIDlet entry point found in manifest (MIDlet-1 is required)".to_string())?;

        let parts: Vec<&str> = midlet_entry.split(',').collect();
        let class_name = if parts.len() >= 3 {
            parts[2].trim().to_string()
        } else if parts.len() == 1 {
            parts[0].trim().to_string()
        } else {
            return Err("Invalid MIDlet-1 entry point format".to_string());
        };

        // Convert dot notation to slash: com.example.MyClass -> com/example/MyClass
        let class_name = class_name.replace('.', "/");

        // --- Extract and load all .class files from the JAR ---
        let mut class_count = 0;
        let mut jar_file_names = Vec::new();

        if let Ok(file) = std::fs::File::open(&path) {
            if let Ok(mut archive) = ZipArchive::new(file) {
                for i in 0..archive.len() {
                    let (name, data) = {
                        let mut entry = archive.by_index(i).map_err(|e| e.to_string())?;
                        let name = entry.name().to_string();
                        let mut data = Vec::new();
                        use std::io::Read;
                        entry.read_to_end(&mut data).map_err(|e| e.to_string())?;
                        (name, data)
                    };

                    jar_file_names.push(name.clone());

                    if name.ends_with(".class") {
                        // Convert path to class name: com/example/MyClass.class -> com/example/MyClass
                        let vm_class_name = name.trim_end_matches(".class").to_string();
                        match self.vm.load_class(&vm_class_name, &data) {
                            Ok(()) => {
                                class_count += 1;
                                println!("  Loaded class: {}", vm_class_name);
                            }
                            Err(e) => {
                                println!("  WARN: Failed to parse class {}: {}", vm_class_name, e);
                            }
                        }
                    }
                }
            }
        }

        println!("Loaded {} classes from JAR", class_count);

        let midlet = Midlet::new(&midlet_name, &class_name);
        self.active_midlet = Some(midlet);
        self.jar = Some(jar);
        self.jar_files = jar_file_names;
        self.midlet_vendor = midlet_vendor;
        self.midlet_version = midlet_version;
        self.entry_class = class_name.clone();
        self.initialized = false;
        self.boot_complete = false;
        self.execution_error = None;
        self.frame_count = 0;

        println!("Successfully loaded JAR: {} with entry class {}", midlet_name, class_name);
        Ok(())
    }

    fn initialize_midlet(&mut self) {
        if self.initialized { return; }
        self.initialized = true;

        let entry_class = self.entry_class.clone();

        // Create the MIDlet object on the heap
        self.vm.heap.push(VmObject {
            class_name: entry_class.clone(),
            fields: HashMap::new(),
        });
        let midlet_ref = self.vm.heap.len() - 1;

        // Run <init> constructor if it exists
        self.invoke_method_on_class(&entry_class, "<init>", "()V", Some(midlet_ref));

        // Run <clinit> (class initializer) if it exists
        self.invoke_method_on_class(&entry_class, "<clinit>", "()V", None);

        // Run startApp() lifecycle method
        self.invoke_method_on_class(&entry_class, "startApp", "()V", Some(midlet_ref));

        println!("MIDlet initialized: {}", entry_class);
    }

    fn invoke_method_on_class(
        &mut self,
        class_name: &str,
        method_name: &str,
        descriptor: &str,
        this_ref: Option<usize>,
    ) {
        if let Some(cf) = self.vm.classes.get(class_name) {
            let method = cf.methods.iter().find(|m| {
                if let Ok(name) = cf.constant_pool.get_utf8(m.name_index) {
                    if let Ok(desc) = cf.constant_pool.get_utf8(m.descriptor_index) {
                        return name == method_name && desc == descriptor;
                    }
                }
                false
            });

            if let Some(m) = method {
                if let Some(code) = m.get_code(&cf.constant_pool) {
                    let mut frame = StackFrame::new(
                        class_name,
                        code.max_stack,
                        code.max_locals,
                        code.code,
                    );
                    // Set local[0] = this reference if instance method
                    if let Some(ref_idx) = this_ref {
                        if !frame.local_variables.is_empty() {
                            frame.local_variables[0] = VmValue::ObjectRef(ref_idx);
                        }
                    }

                    self.vm.call_stack.push(frame);
                    match self.vm.execute() {
                        Ok(()) => {
                            println!("  Executed {}::{}{}", class_name, method_name, descriptor);
                        }
                        Err(e) => {
                            println!("  WARN: Error executing {}::{}: {}", class_name, method_name, e);
                        }
                    }
                } else {
                    println!("  {}::{} has no Code attribute (native/abstract)", class_name, method_name);
                }
            } else {
                println!("  Method {}::{}{} not found", class_name, method_name, descriptor);
            }
        } else {
            println!("  Class {} not loaded", class_name);
        }
    }

    fn try_invoke_paint(&mut self) {
        if let Some(canvas_idx) = self.vm.active_displayable {
            let paint_class = self.vm.heap[canvas_idx].class_name.clone();

            if let Some(cf) = self.vm.classes.get(&paint_class) {
                let method = cf.methods.iter().find(|m| {
                    if let Ok(name) = cf.constant_pool.get_utf8(m.name_index) {
                        return name == "paint";
                    }
                    false
                });

                if let Some(m) = method {
                    if let Some(code) = m.get_code(&cf.constant_pool) {
                        let mut frame = StackFrame::new(
                            &paint_class,
                            code.max_stack,
                            code.max_locals,
                            code.code,
                        );
                        // local[0] = this (canvas object)
                        frame.local_variables[0] = VmValue::ObjectRef(canvas_idx);
                        
                        // Push a Graphics object onto the heap
                        self.vm.heap.push(VmObject {
                            class_name: "javax/microedition/lcdui/Graphics".to_string(),
                            fields: HashMap::new(),
                        });
                        if frame.local_variables.len() > 1 {
                            frame.local_variables[1] = VmValue::ObjectRef(self.vm.heap.len() - 1);
                        }

                        self.vm.call_stack.push(frame);
                        match self.vm.execute() {
                            Ok(()) => {}
                            Err(e) => {
                                if self.execution_error.is_none() {
                                    println!("  paint() error: {}", e);
                                    self.execution_error = Some(e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn try_invoke_run(&mut self) {
        let run_class = self.find_class_with_method("run", "()V");
        if let Some(class_name) = run_class {
            self.invoke_method_on_class(&class_name, "run", "()V", Some(0));
        }
    }

    fn find_class_with_method(&self, method_name: &str, _descriptor: &str) -> Option<String> {
        // First check entry class
        if let Some(cf) = self.vm.classes.get(&self.entry_class) {
            for m in &cf.methods {
                if let Ok(name) = cf.constant_pool.get_utf8(m.name_index) {
                    if name == method_name {
                        return Some(self.entry_class.clone());
                    }
                }
            }
        }

        // Search all loaded classes
        for (class_name, cf) in &self.vm.classes {
            for m in &cf.methods {
                if let Ok(name) = cf.constant_pool.get_utf8(m.name_index) {
                    if name == method_name {
                        return Some(class_name.clone());
                    }
                }
            }
        }
        None
    }

    pub fn handle_key(&mut self, key_code: i32, pressed: bool) {
        self.key_queue.push((key_code, pressed));

        if let Some(canvas_idx) = self.vm.active_displayable {
            let canvas_class = self.vm.heap[canvas_idx].class_name.clone();
            let method = if pressed { "keyPressed" } else { "keyReleased" };

            if let Some(cf) = self.vm.classes.get(&canvas_class) {
                let found = cf.methods.iter().find(|m| {
                    if let Ok(name) = cf.constant_pool.get_utf8(m.name_index) {
                        return name == method;
                    }
                    false
                });

                if let Some(m) = found {
                    if let Some(code) = m.get_code(&cf.constant_pool) {
                        let mut frame = StackFrame::new(
                            &canvas_class,
                            code.max_stack,
                            code.max_locals,
                            code.code,
                        );
                        if !frame.local_variables.is_empty() {
                            frame.local_variables[0] = VmValue::ObjectRef(canvas_idx); // this
                        }
                        if frame.local_variables.len() > 1 {
                            frame.local_variables[1] = VmValue::Int(key_code); // keyCode arg
                        }

                        self.vm.call_stack.push(frame);
                        if let Err(e) = self.vm.execute() {
                            println!("  {}() error: {}", method, e);
                        }
                    }
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.frame_count += 1;

        let active_name = self.active_midlet.as_ref()
            .map(|m| m.name.clone())
            .unwrap_or_else(|| "RessuRetro".to_string());

        // --- BOOT SEQUENCE (first 60 frames) ---
        if !self.boot_complete {
            self.vm.framebuffer.clear(0xFF0A0A0F);
            self.vm.framebuffer.draw_string(10, 15, "RESSURETRO JVM v0.1.0", 0xFF00D4FF);
            self.vm.framebuffer.draw_line(10, 26, 230, 26, 0xFF1C1C28);

            self.vm.framebuffer.draw_string(10, 36, &format!("JAR: {}", active_name), 0xFFE8E8ED);
            self.vm.framebuffer.draw_string(10, 48, &format!("VENDOR: {}", self.midlet_vendor), 0xFF71717A);
            self.vm.framebuffer.draw_string(10, 60, &format!("VERSION: {}", self.midlet_version), 0xFF71717A);

            self.vm.framebuffer.draw_string(10, 80, "LOADING JAR CLASSFILES...", 0xFFE8E8ED);
            let mut y = 92;
            let class_files: Vec<&String> = self.jar_files.iter()
                .filter(|f| f.ends_with(".class"))
                .take(6)
                .collect();
            for file in &class_files {
                let display_name = if file.len() > 28 { &file[file.len() - 28..] } else { file };
                self.vm.framebuffer.draw_string(10, y, &format!("+ {}", display_name), 0xFF008B99);
                y += 12;
            }

            let total_classes = self.vm.classes.len();
            self.vm.framebuffer.draw_string(10, y + 4,
                &format!("{} classes loaded", total_classes), 0xFF00D4FF);

            if self.frame_count > 15 {
                self.vm.framebuffer.draw_string(10, 200, "RESOLVING CONSTANT POOL... OK", 0xFF00D4FF);
            }
            if self.frame_count > 30 {
                self.vm.framebuffer.draw_string(10, 212, "INSTANTIATING MIDLET CLASS... OK", 0xFF00D4FF);
            }
            if self.frame_count > 45 {
                self.vm.framebuffer.draw_string(10, 224,
                    &format!("ENTRY: {}", self.entry_class), 0xFFE8E8ED);
            }

            let progress = (self.frame_count as i32 * 220) / 60;
            self.vm.framebuffer.draw_rect(10, 260, 220, 14, 0xFF1C1C28);
            self.vm.framebuffer.fill_rect(12, 262, progress.min(216), 10, 0xFF00D4FF);

            if self.frame_count >= 60 {
                self.boot_complete = true;
                self.initialize_midlet();
            }
            return;
        }

        // --- GAME EXECUTION ---
        // 1. Progress any background threads / main loops on the stack
        let _ = self.vm.execute();

        // 2. Periodic screen repaint
        self.try_invoke_paint();

        // 3. Fallback: if no thread or background frame is active, call run() as a loop starter
        if self.vm.call_stack.is_empty() {
            self.try_invoke_run();
        }

        // If there was an execution error and framebuffer is empty, show the error
        if let Some(ref error) = self.execution_error {
            if self.frame_count % 60 < 2 {
                // Only show on first occurrence
                self.vm.framebuffer.fill_rect(0, 280, 240, 40, 0xFF0D0D19);
                self.vm.framebuffer.draw_string(4, 285, &format!("VM: {}", &error[..error.len().min(35)]), 0xFFFF4B6B);
            }
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        if let Some(ref mut midlet) = self.active_midlet {
            midlet.start()
        } else {
            Err("No active MIDlet loaded".to_string())
        }
    }

    pub fn pause(&mut self) -> Result<(), String> {
        if let Some(ref mut midlet) = self.active_midlet {
            midlet.pause()
        } else {
            Err("No active MIDlet running".to_string())
        }
    }

    pub fn resume(&mut self) -> Result<(), String> {
        if let Some(ref mut midlet) = self.active_midlet {
            midlet.resume()
        } else {
            Err("No active MIDlet running".to_string())
        }
    }

    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(ref mut midlet) = self.active_midlet {
            midlet.destroy(true)
        } else {
            Err("No active MIDlet running".to_string())
        }
    }
}
