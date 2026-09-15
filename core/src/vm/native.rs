// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use super::{StackFrame, Vm, VmValue, VmObject};
use std::collections::HashMap;

const NATIVE_CLASSES: &[&str] = &[
    "java/lang/Object",
    "java/lang/Class",
    "java/lang/String",
    "java/lang/StringBuilder",
    "java/lang/StringBuffer",
    "java/lang/System",
    "java/lang/Math",
    "java/lang/Integer",
    "java/lang/Long",
    "java/lang/Float",
    "java/lang/Double",
    "java/lang/Boolean",
    "java/lang/Byte",
    "java/lang/Short",
    "java/lang/Character",
    "java/lang/Thread",
    "java/lang/Runnable",
    "java/lang/Runtime",
    "java/lang/Throwable",
    "java/lang/Exception",
    "java/lang/RuntimeException",
    "java/lang/NullPointerException",
    "java/lang/ArrayIndexOutOfBoundsException",
    "java/lang/IllegalArgumentException",
    "java/io/InputStream",
    "java/io/OutputStream",
    "java/io/ByteArrayInputStream",
    "java/io/ByteArrayOutputStream",
    "java/io/DataInputStream",
    "java/io/DataOutputStream",
    "java/io/PrintStream",
    "java/util/Vector",
    "java/util/Hashtable",
    "java/util/Random",
    "java/util/Timer",
    "java/util/TimerTask",
    "javax/microedition/midlet/MIDlet",
    "javax/microedition/lcdui/Display",
    "javax/microedition/lcdui/Displayable",
    "javax/microedition/lcdui/Canvas",
    "javax/microedition/lcdui/Graphics",
    "javax/microedition/lcdui/Image",
    "javax/microedition/lcdui/Font",
    "javax/microedition/lcdui/Form",
    "javax/microedition/lcdui/Alert",
    "javax/microedition/lcdui/List",
    "javax/microedition/lcdui/TextBox",
    "javax/microedition/lcdui/Command",
    "javax/microedition/lcdui/CommandListener",
    "javax/microedition/lcdui/game/GameCanvas",
    "javax/microedition/lcdui/game/Sprite",
    "javax/microedition/lcdui/game/TiledLayer",
    "javax/microedition/lcdui/game/LayerManager",
    "javax/microedition/rms/RecordStore",
    "javax/microedition/media/Manager",
    "javax/microedition/media/Player",
    "javax/microedition/media/control/VolumeControl",
    "javax/microedition/io/Connector",
    "com/nokia/mid/ui/DeviceControl",
    "com/nokia/mid/ui/DirectGraphics",
    "com/samsung/util/Vibration",
];

pub fn is_native_class(class_name: &str) -> bool {
    NATIVE_CLASSES.iter().any(|&c| c == class_name)
}

pub fn invoke_override(
    vm: &mut Vm,
    class_name: &str,
    method_name: &str,
    descriptor: &str,
    frame: &mut StackFrame,
) -> Result<(), String> {
    match class_name {
        "javax/microedition/lcdui/Graphics" => handle_graphics_call(vm, method_name, frame),
        "javax/microedition/lcdui/game/GameCanvas" => {
            match method_name {
                "flushGraphics" => {
                    vm.framebuffer.pixels = vm.back_buffer.pixels.clone();
                }
                "getKeyStates" => {
                    frame.push_int(0);
                }
                "getGraphics" => {
                    frame.push(VmValue::Int(0));
                }
                _ => {}
            }
            Ok(())
        }
        "javax/microedition/rms/RecordStore" => handle_rms_call(vm, method_name, frame),
        "javax/microedition/media/Manager" => {
            if method_name == "playTone" {
                let _volume = frame.pop_int()?;
                let duration = frame.pop_int()?;
                let note = frame.pop_int()?;
                let frequency = (440.0 * 2.0_f32.powf((note - 69) as f32 / 12.0)) as u32;
                vm.audio_system.play_tone(frequency, duration as u32);
            } else {
                consume_args_for_descriptor(frame, descriptor, method_name != "<init>");
            }
            Ok(())
        }
        "javax/microedition/lcdui/Display" => {
            match method_name {
                "getDisplay" => {
                    let _ = frame.pop(); // MIDlet arg
                    vm.heap.push(VmObject {
                        class_name: "javax/microedition/lcdui/Display".to_string(),
                        fields: HashMap::new(),
                    });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                "setCurrent" => {
                    let displayable = frame.pop()?;
                    let _this = frame.pop()?;
                    if let VmValue::ObjectRef(idx) = displayable {
                        vm.active_displayable = Some(idx);
                        println!("Display.setCurrent: set active displayable to heap index {}", idx);
                    }
                }
                "getWidth" | "getHeight" => {
                    let _ = frame.pop(); // this
                    frame.push_int(if method_name == "getWidth" { 240 } else { 320 });
                }
                _ => {
                    consume_args_for_descriptor(frame, descriptor, true);
                }
            }
            Ok(())
        }
        "javax/microedition/lcdui/Canvas" => {
            match method_name {
                "getWidth" => { let _ = frame.pop(); frame.push_int(240); }
                "getHeight" => { let _ = frame.pop(); frame.push_int(320); }
                "repaint" => { let _ = frame.pop(); }
                "serviceRepaints" => { let _ = frame.pop(); }
                "setFullScreenMode" => { let _ = frame.pop(); let _ = frame.pop(); }
                "isDoubleBuffered" => { let _ = frame.pop(); frame.push_int(1); }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "javax/microedition/lcdui/Image" => {
            match method_name {
                "createImage" => {
                    consume_args_for_descriptor(frame, descriptor, false);
                    vm.heap.push(VmObject {
                        class_name: "javax/microedition/lcdui/Image".to_string(),
                        fields: HashMap::new(),
                    });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                "getWidth" => { let _ = frame.pop(); frame.push_int(16); }
                "getHeight" => { let _ = frame.pop(); frame.push_int(16); }
                "getGraphics" => {
                    let _ = frame.pop();
                    vm.heap.push(VmObject {
                        class_name: "javax/microedition/lcdui/Graphics".to_string(),
                        fields: HashMap::new(),
                    });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "javax/microedition/lcdui/Font" => {
            match method_name {
                "getFont" | "getDefaultFont" => {
                    consume_args_for_descriptor(frame, descriptor, false);
                    vm.heap.push(VmObject {
                        class_name: "javax/microedition/lcdui/Font".to_string(),
                        fields: HashMap::new(),
                    });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                "getHeight" | "stringWidth" | "charWidth" | "getBaselinePosition" => {
                    consume_args_for_descriptor(frame, descriptor, true);
                    frame.push_int(8);
                }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "javax/microedition/midlet/MIDlet" => {
            match method_name {
                "getAppProperty" => {
                    let _ = frame.pop(); // key
                    let _ = frame.pop(); // this
                    frame.push(VmValue::Null);
                }
                "notifyDestroyed" | "notifyPaused" | "resumeRequest" => {
                    let _ = frame.pop(); // this
                }
                "<init>" => {
                    let _ = frame.pop(); // this
                }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "java/lang/Object" => {
            match method_name {
                "<init>" => { let _ = frame.pop(); }
                "getClass" => { let _ = frame.pop(); frame.push(VmValue::Int(0)); }
                "hashCode" => { let _ = frame.pop(); frame.push_int(42); }
                "equals" => { let _ = frame.pop(); let _ = frame.pop(); frame.push_int(0); }
                "toString" => { let _ = frame.pop(); frame.push(VmValue::Int(0)); }
                "notify" | "notifyAll" | "wait" => { let _ = frame.pop(); }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "java/lang/Class" => {
            match method_name {
                "getResourceAsStream" => {
                    let _name_ref = frame.pop()?; // resource name
                    let _ = frame.pop(); // this (Class object, may not exist in tests)
                    vm.heap.push(VmObject {
                        class_name: "java/io/InputStream".to_string(),
                        fields: HashMap::new(),
                    });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                "forName" => {
                    let _ = frame.pop();
                    vm.heap.push(VmObject {
                        class_name: "java/lang/Class".to_string(),
                        fields: HashMap::new(),
                    });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                "getName" => { let _ = frame.pop(); frame.push(VmValue::Int(0)); }
                "newInstance" => {
                    let _ = frame.pop();
                    vm.heap.push(VmObject {
                        class_name: "java/lang/Object".to_string(),
                        fields: HashMap::new(),
                    });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "java/lang/System" => {
            match method_name {
                "currentTimeMillis" => {
                    frame.push(VmValue::Long(0));
                }
                "arraycopy" => {
                    for _ in 0..5 { let _ = frame.pop(); }
                }
                "gc" => {}
                "exit" => { let _ = frame.pop(); }
                _ => { consume_args_for_descriptor(frame, descriptor, false); }
            }
            Ok(())
        }
        "java/lang/Math" => {
            match method_name {
                "abs" => {
                    if descriptor.contains("I") {
                        let v = frame.pop_int()?;
                        frame.push_int(v.abs());
                    } else {
                        let v = frame.pop_int()?;
                        frame.push_int(v.abs());
                    }
                }
                "min" => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a.min(b)); }
                "max" => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a.max(b)); }
                "random" => { frame.push(VmValue::Double(0.5)); }
                "sqrt" => { let v = frame.pop_int()?; frame.push(VmValue::Double((v as f64).sqrt())); }
                "sin" | "cos" | "tan" => { let _ = frame.pop(); frame.push(VmValue::Double(0.0)); }
                _ => { consume_args_for_descriptor(frame, descriptor, false); }
            }
            Ok(())
        }
        "java/lang/Thread" => {
            match method_name {
                "sleep" => { let _ = frame.pop(); }
                "start" => {
                    let this_val = frame.pop()?;
                    if let VmValue::ObjectRef(this_idx) = this_val {
                        if let Some(obj) = vm.heap.get(this_idx) {
                            let target = obj.fields.get("target").cloned().unwrap_or(VmValue::Null);
                            if let VmValue::ObjectRef(target_idx) = target {
                                let class_name = vm.heap[target_idx].class_name.clone();
                                if let Some(cf) = vm.classes.get(&class_name) {
                                    let run_method = cf.methods.iter().find(|m| {
                                        if let Ok(name) = cf.constant_pool.get_utf8(m.name_index) {
                                            return name == "run";
                                        }
                                        false
                                    });
                                    if let Some(m) = run_method {
                                        if let Some(code) = m.get_code(&cf.constant_pool) {
                                            let mut next_frame = StackFrame::new(&class_name, code.max_stack, code.max_locals, code.code.clone());
                                            if !next_frame.local_variables.is_empty() {
                                                next_frame.local_variables[0] = VmValue::ObjectRef(target_idx);
                                            }
                                            vm.call_stack.push(next_frame);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                "run" | "interrupt" | "join" => { let _ = frame.pop(); }
                "<init>" => {
                    if descriptor.contains("Runnable") {
                        let target = frame.pop()?;
                        let this_val = frame.pop()?;
                        if let VmValue::ObjectRef(this_idx) = this_val {
                            if let Some(obj) = vm.heap.get_mut(this_idx) {
                                obj.fields.insert("target".to_string(), target);
                            }
                        }
                    } else {
                        consume_args_for_descriptor(frame, descriptor, true);
                    }
                }
                "currentThread" => {
                    vm.heap.push(VmObject { class_name: "java/lang/Thread".to_string(), fields: HashMap::new() });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "java/lang/String" | "java/lang/StringBuilder" | "java/lang/StringBuffer" => {
            match method_name {
                "<init>" => { consume_args_for_descriptor(frame, descriptor, true); }
                "length" => { let _ = frame.pop(); frame.push_int(0); }
                "charAt" => { let _ = frame.pop(); let _ = frame.pop(); frame.push_int(0); }
                "toString" | "substring" | "concat" | "trim" | "toLowerCase" | "toUpperCase"
                | "valueOf" | "replace" | "intern" => {
                    consume_args_for_descriptor(frame, descriptor, method_name != "valueOf");
                    frame.push(VmValue::Null);
                }
                "append" => {
                    let _ = frame.pop(); // arg
                    // return this (already on stack conceptually)
                    let this_val = frame.operand_stack.last().cloned().unwrap_or(VmValue::Null);
                    let _ = this_val;
                }
                "equals" | "equalsIgnoreCase" | "startsWith" | "endsWith" | "contains" => {
                    let _ = frame.pop(); let _ = frame.pop();
                    frame.push_int(0);
                }
                "indexOf" | "lastIndexOf" | "compareTo" => {
                    consume_args_for_descriptor(frame, descriptor, true);
                    frame.push_int(-1);
                }
                "getBytes" | "toCharArray" => {
                    let _ = frame.pop();
                    vm.heap.push(VmObject { class_name: "[B".to_string(), fields: HashMap::new() });
                    frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "java/lang/Integer" => {
            match method_name {
                "parseInt" => { let _ = frame.pop(); frame.push_int(0); }
                "valueOf" => { let _ = frame.pop(); frame.push(VmValue::Null); }
                "toString" => { let _ = frame.pop(); frame.push(VmValue::Null); }
                "intValue" => { let _ = frame.pop(); frame.push_int(0); }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "java/util/Random" => {
            match method_name {
                "<init>" => { consume_args_for_descriptor(frame, descriptor, true); }
                "nextInt" => {
                    if descriptor == "(I)I" {
                        let bound = frame.pop_int()?;
                        let _ = frame.pop(); // this
                        let pseudo = (vm.heap.len() as i32 * 1103515245 + 12345) % bound.max(1);
                        frame.push_int(pseudo.abs());
                    } else {
                        let _ = frame.pop();
                        frame.push_int(42);
                    }
                }
                "nextBoolean" => { let _ = frame.pop(); frame.push_int(0); }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "java/util/Vector" | "java/util/Hashtable" => {
            match method_name {
                "<init>" => { consume_args_for_descriptor(frame, descriptor, true); }
                "size" => { let _ = frame.pop(); frame.push_int(0); }
                "isEmpty" => { let _ = frame.pop(); frame.push_int(1); }
                "elementAt" | "get" => { let _ = frame.pop(); let _ = frame.pop(); frame.push(VmValue::Null); }
                "addElement" | "add" | "put" | "remove" | "removeElement"
                | "removeElementAt" | "removeAllElements" | "clear"
                | "insertElementAt" | "setElementAt" | "set" => {
                    consume_args_for_descriptor(frame, descriptor, true);
                }
                "contains" | "containsKey" => { let _ = frame.pop(); let _ = frame.pop(); frame.push_int(0); }
                _ => { consume_args_for_descriptor(frame, descriptor, true); }
            }
            Ok(())
        }
        "com/nokia/mid/ui/DeviceControl" | "com/nokia/mid/ui/DirectGraphics" => {
            consume_args_for_descriptor(frame, descriptor, true);
            Ok(())
        }
        "com/samsung/util/Vibration" => {
            consume_args_for_descriptor(frame, descriptor, true);
            Ok(())
        }
        _ => {
            // Unknown native class — consume args safely
            consume_args_for_descriptor(frame, descriptor, true);
            if !descriptor.ends_with(")V") {
                if descriptor.ends_with(";") || descriptor.contains("[") {
                    frame.push(VmValue::Null);
                } else {
                    frame.push(VmValue::Int(0));
                }
            }
            Ok(())
        }
    }
}

fn consume_args_for_descriptor(frame: &mut StackFrame, descriptor: &str, has_this: bool) {
    let arg_count = count_descriptor_args(descriptor) + if has_this { 1 } else { 0 };
    for _ in 0..arg_count {
        let _ = frame.pop();
    }
}

fn count_descriptor_args(descriptor: &str) -> usize {
    let mut count = 0;
    let mut in_paren = false;
    let mut in_obj = false;
    for c in descriptor.chars() {
        if c == '(' { in_paren = true; continue; }
        if c == ')' { break; }
        if in_paren {
            if in_obj {
                if c == ';' { in_obj = false; count += 1; }
            } else if c == 'L' { in_obj = true; }
            else if c == '[' { /* array prefix */ }
            else { count += 1; }
        }
    }
    count
}

fn handle_graphics_call(vm: &mut Vm, method_name: &str, frame: &mut StackFrame) -> Result<(), String> {
    match method_name {
        "drawRect" => {
            let h = frame.pop_int()?; let w = frame.pop_int()?;
            let y = frame.pop_int()?; let x = frame.pop_int()?;
            let _ = frame.pop(); // this
            vm.framebuffer.draw_rect(x, y, w, h, 0xFFFFFFFF);
        }
        "fillRect" => {
            let h = frame.pop_int()?; let w = frame.pop_int()?;
            let y = frame.pop_int()?; let x = frame.pop_int()?;
            let this_val = frame.pop()?;
            let mut color = 0xFFFFFFFF;
            if let VmValue::ObjectRef(idx) = this_val {
                if let Some(obj) = vm.heap.get(idx) {
                    if let Some(VmValue::Int(c)) = obj.fields.get("color") {
                        color = *c as u32;
                    }
                }
            }
            vm.framebuffer.fill_rect(x, y, w, h, color);
        }
        "drawLine" => {
            let y2 = frame.pop_int()?; let x2 = frame.pop_int()?;
            let y1 = frame.pop_int()?; let x1 = frame.pop_int()?;
            let _ = frame.pop();
            vm.framebuffer.draw_line(x1, y1, x2, y2, 0xFFFFFFFF);
        }
        "drawString" | "drawSubstring" => {
            let _anchor = frame.pop_int()?;
            let y = frame.pop_int()?;
            let x = frame.pop_int()?;
            let _str_ref = frame.pop()?;
            let _ = frame.pop(); // this
            vm.framebuffer.draw_string(x, y, "TEXT", 0xFFFFFFFF);
        }
        "drawImage" => {
            let _anchor = frame.pop_int()?;
            let y = frame.pop_int()?;
            let x = frame.pop_int()?;
            let _img_ref = frame.pop()?;
            let _ = frame.pop(); // this
            vm.framebuffer.fill_rect(x, y, 16, 16, 0xFF808080);
        }
        "setColor" => {
            let argb = if frame.operand_stack.len() >= 4 {
                let b = frame.pop_int()?;
                let g = frame.pop_int()?;
                let r = frame.pop_int()?;
                0xFF000000 | ((r as u32 & 0xFF) << 16) | ((g as u32 & 0xFF) << 8) | (b as u32 & 0xFF)
            } else {
                let color = frame.pop_int()?;
                0xFF000000 | (color as u32)
            };
            let this_val = frame.pop()?;
            if let VmValue::ObjectRef(idx) = this_val {
                if let Some(obj) = vm.heap.get_mut(idx) {
                    obj.fields.insert("color".to_string(), VmValue::Int(argb as i32));
                }
            }
        }
        "setFont" | "setClip" | "clipRect" | "translate" => {
            // Consume all args silently
            while frame.operand_stack.len() > 0 {
                let _ = frame.pop();
            }
        }
        "getClipX" | "getClipY" => { let _ = frame.pop(); frame.push_int(0); }
        "getClipWidth" => { let _ = frame.pop(); frame.push_int(240); }
        "getClipHeight" => { let _ = frame.pop(); frame.push_int(320); }
        "getColor" => { let _ = frame.pop(); frame.push_int(0xFFFFFF); }
        "getFont" => {
            let _ = frame.pop();
            vm.heap.push(VmObject {
                class_name: "javax/microedition/lcdui/Font".to_string(),
                fields: HashMap::new(),
            });
            frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
        }
        "<init>" => { let _ = frame.pop(); }
        _ => {
            // Unknown Graphics method — consume this
            let _ = frame.pop();
        }
    }
    Ok(())
}

fn handle_rms_call(vm: &mut Vm, method_name: &str, frame: &mut StackFrame) -> Result<(), String> {
    match method_name {
        "openRecordStore" => {
            let _create = frame.pop()?;
            let _name = frame.pop()?;
            vm.heap.push(VmObject {
                class_name: "javax/microedition/rms/RecordStore".to_string(),
                fields: HashMap::new(),
            });
            frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
        }
        "closeRecordStore" => { let _ = frame.pop(); }
        "addRecord" => {
            let _ = frame.pop(); let _ = frame.pop();
            let _ = frame.pop(); let _ = frame.pop();
            frame.push_int(1);
        }
        "getRecord" => {
            let _ = frame.pop(); let _ = frame.pop();
            vm.heap.push(VmObject { class_name: "[B".to_string(), fields: HashMap::new() });
            frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
        }
        "getNumRecords" => { let _ = frame.pop(); frame.push_int(0); }
        "deleteRecordStore" => { let _ = frame.pop(); }
        _ => { let _ = frame.pop(); }
    }
    Ok(())
}
