// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use super::{bytecode::Opcode, Vm, StackFrame, VmValue, VmObject};
use std::collections::HashMap;

#[derive(Debug)]
pub enum FrameResult {
    Completed,
    Invoked(super::StackFrame),
    Suspended,
}

pub fn execute_frame(vm: &mut Vm, frame: &mut StackFrame) -> Result<FrameResult, String> {
    let res = execute_frame_impl(vm, frame);
    if let Err(ref e) = res {
        println!("  VM Exec Error: {} at PC {} (opcode byte 0x{:02x}) in class {}", e, frame.program_counter, frame.bytecode.get(frame.program_counter).cloned().unwrap_or(0), frame.class_name);
    }
    res
}

fn execute_frame_impl(vm: &mut Vm, frame: &mut StackFrame) -> Result<FrameResult, String> {
    let max_instructions = 50000;
    let mut instruction_count = 0;

    while frame.program_counter < frame.bytecode.len() {
        instruction_count += 1;
        if instruction_count > max_instructions {
            return Ok(FrameResult::Suspended);
        }

        let opcode_byte = frame.bytecode[frame.program_counter];
        let opcode = Opcode::from_u8(opcode_byte)
            .ok_or_else(|| format!("Unknown opcode: 0x{:02x} at PC {}", opcode_byte, frame.program_counter))?;

        match opcode {
            Opcode::Nop => {
                frame.program_counter += 1;
            }
            Opcode::AconstNull => {
                frame.push(VmValue::Null);
                frame.program_counter += 1;
            }
            Opcode::IconstM1 => { frame.push_int(-1); frame.program_counter += 1; }
            Opcode::Iconst0 => { frame.push_int(0); frame.program_counter += 1; }
            Opcode::Iconst1 => { frame.push_int(1); frame.program_counter += 1; }
            Opcode::Iconst2 => { frame.push_int(2); frame.program_counter += 1; }
            Opcode::Iconst3 => { frame.push_int(3); frame.program_counter += 1; }
            Opcode::Iconst4 => { frame.push_int(4); frame.program_counter += 1; }
            Opcode::Iconst5 => { frame.push_int(5); frame.program_counter += 1; }
            Opcode::Lconst0 => { frame.push(VmValue::Long(0)); frame.program_counter += 1; }
            Opcode::Lconst1 => { frame.push(VmValue::Long(1)); frame.program_counter += 1; }
            Opcode::Fconst0 => { frame.push(VmValue::Float(0.0)); frame.program_counter += 1; }
            Opcode::Fconst1 => { frame.push(VmValue::Float(1.0)); frame.program_counter += 1; }
            Opcode::Fconst2 => { frame.push(VmValue::Float(2.0)); frame.program_counter += 1; }
            Opcode::Dconst0 => { frame.push(VmValue::Double(0.0)); frame.program_counter += 1; }
            Opcode::Dconst1 => { frame.push(VmValue::Double(1.0)); frame.program_counter += 1; }

            Opcode::Bipush => {
                let val = frame.bytecode[frame.program_counter + 1] as i8 as i32;
                frame.push_int(val);
                frame.program_counter += 2;
            }
            Opcode::Sipush => {
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let val = ((b1 << 8) | b2) as i16 as i32;
                frame.push_int(val);
                frame.program_counter += 3;
            }

            // --- LDC family ---
            Opcode::Ldc => {
                let index = frame.bytecode[frame.program_counter + 1] as u16;
                let val = resolve_ldc(vm, &frame.class_name, index)?;
                frame.push(val);
                frame.program_counter += 2;
            }
            Opcode::LdcW => {
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let index = (b1 << 8) | b2;
                let val = resolve_ldc(vm, &frame.class_name, index)?;
                frame.push(val);
                frame.program_counter += 3;
            }
            Opcode::Ldc2W => {
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let index = (b1 << 8) | b2;
                let val = resolve_ldc2w(vm, &frame.class_name, index)?;
                frame.push(val);
                frame.program_counter += 3;
            }

            // --- Load instructions ---
            Opcode::Iload | Opcode::Fload | Opcode::Aload | Opcode::Lload | Opcode::Dload => {
                let index = frame.bytecode[frame.program_counter + 1] as usize;
                let val = safe_local_get(frame, index)?;
                frame.push(val);
                frame.program_counter += 2;
            }
            Opcode::Iload0 | Opcode::Fload0 | Opcode::Aload0 | Opcode::Lload0 | Opcode::Dload0 => {
                let val = safe_local_get(frame, 0)?;
                frame.push(val);
                frame.program_counter += 1;
            }
            Opcode::Iload1 | Opcode::Fload1 | Opcode::Aload1 | Opcode::Lload1 | Opcode::Dload1 => {
                let val = safe_local_get(frame, 1)?;
                frame.push(val);
                frame.program_counter += 1;
            }
            Opcode::Iload2 | Opcode::Fload2 | Opcode::Aload2 | Opcode::Lload2 | Opcode::Dload2 => {
                let val = safe_local_get(frame, 2)?;
                frame.push(val);
                frame.program_counter += 1;
            }
            Opcode::Iload3 | Opcode::Fload3 | Opcode::Aload3 | Opcode::Lload3 | Opcode::Dload3 => {
                let val = safe_local_get(frame, 3)?;
                frame.push(val);
                frame.program_counter += 1;
            }

            // --- Store instructions ---
            Opcode::Istore | Opcode::Fstore | Opcode::Astore | Opcode::Lstore | Opcode::Dstore => {
                let index = frame.bytecode[frame.program_counter + 1] as usize;
                let val = frame.pop()?;
                safe_local_set(frame, index, val);
                frame.program_counter += 2;
            }
            Opcode::Istore0 | Opcode::Fstore0 | Opcode::Astore0 | Opcode::Lstore0 | Opcode::Dstore0 => {
                let val = frame.pop()?;
                safe_local_set(frame, 0, val);
                frame.program_counter += 1;
            }
            Opcode::Istore1 | Opcode::Fstore1 | Opcode::Astore1 | Opcode::Lstore1 | Opcode::Dstore1 => {
                let val = frame.pop()?;
                safe_local_set(frame, 1, val);
                frame.program_counter += 1;
            }
            Opcode::Istore2 | Opcode::Fstore2 | Opcode::Astore2 | Opcode::Lstore2 | Opcode::Dstore2 => {
                let val = frame.pop()?;
                safe_local_set(frame, 2, val);
                frame.program_counter += 1;
            }
            Opcode::Istore3 | Opcode::Fstore3 | Opcode::Astore3 | Opcode::Lstore3 | Opcode::Dstore3 => {
                let val = frame.pop()?;
                safe_local_set(frame, 3, val);
                frame.program_counter += 1;
            }

            // --- Array load/store ---
            Opcode::Iaload | Opcode::Aaload | Opcode::Baload | Opcode::Caload | Opcode::Saload
            | Opcode::Laload | Opcode::Faload | Opcode::Daload => {
                let _index = frame.pop_int()?;
                let _arrayref = frame.pop()?;
                frame.push_int(0);
                frame.program_counter += 1;
            }
            Opcode::Iastore | Opcode::Aastore | Opcode::Bastore | Opcode::Castore | Opcode::Sastore
            | Opcode::Lastore | Opcode::Fastore | Opcode::Dastore => {
                let _val = frame.pop()?;
                let _index = frame.pop()?;
                let _arrayref = frame.pop()?;
                frame.program_counter += 1;
            }

            // --- Stack manipulation ---
            Opcode::Pop => { frame.pop()?; frame.program_counter += 1; }
            Opcode::Pop2 => { frame.pop()?; let _ = frame.pop(); frame.program_counter += 1; }
            Opcode::Dup => {
                let val = frame.operand_stack.last()
                    .ok_or_else(|| "Stack underflow on dup".to_string())?
                    .clone();
                frame.push(val);
                frame.program_counter += 1;
            }
            Opcode::DupX1 => {
                let v1 = frame.pop()?;
                let v2 = frame.pop()?;
                frame.push(v1.clone());
                frame.push(v2);
                frame.push(v1);
                frame.program_counter += 1;
            }
            Opcode::DupX2 => {
                let v1 = frame.pop()?;
                let v2 = frame.pop()?;
                let v3 = frame.pop()?;
                frame.push(v1.clone());
                frame.push(v3);
                frame.push(v2);
                frame.push(v1);
                frame.program_counter += 1;
            }
            Opcode::Dup2 => {
                let v1 = frame.pop()?;
                let v2 = frame.pop()?;
                frame.push(v2.clone());
                frame.push(v1.clone());
                frame.push(v2);
                frame.push(v1);
                frame.program_counter += 1;
            }
            Opcode::Dup2X1 | Opcode::Dup2X2 => {
                // Simplified: treat as no-op for category-2 values
                frame.program_counter += 1;
            }
            Opcode::Swap => {
                let v1 = frame.pop()?;
                let v2 = frame.pop()?;
                frame.push(v1);
                frame.push(v2);
                frame.program_counter += 1;
            }

            // --- Integer arithmetic ---
            Opcode::Iadd => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a.wrapping_add(b)); frame.program_counter += 1; }
            Opcode::Isub => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a.wrapping_sub(b)); frame.program_counter += 1; }
            Opcode::Imul => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a.wrapping_mul(b)); frame.program_counter += 1; }
            Opcode::Idiv => {
                let b = frame.pop_int()?;
                let a = frame.pop_int()?;
                if b == 0 { return Err("ArithmeticException: / by zero".to_string()); }
                frame.push_int(a.wrapping_div(b));
                frame.program_counter += 1;
            }
            Opcode::Irem => {
                let b = frame.pop_int()?;
                let a = frame.pop_int()?;
                if b == 0 { return Err("ArithmeticException: / by zero".to_string()); }
                frame.push_int(a.wrapping_rem(b));
                frame.program_counter += 1;
            }
            Opcode::Ineg => { let a = frame.pop_int()?; frame.push_int(-a); frame.program_counter += 1; }
            Opcode::Ishl => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a << (b & 0x1f)); frame.program_counter += 1; }
            Opcode::Ishr => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a >> (b & 0x1f)); frame.program_counter += 1; }
            Opcode::Iushr => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(((a as u32) >> (b & 0x1f)) as i32); frame.program_counter += 1; }
            Opcode::Iand => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a & b); frame.program_counter += 1; }
            Opcode::Ior => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a | b); frame.program_counter += 1; }
            Opcode::Ixor => { let b = frame.pop_int()?; let a = frame.pop_int()?; frame.push_int(a ^ b); frame.program_counter += 1; }

            // --- Long arithmetic ---
            Opcode::Ladd | Opcode::Lsub | Opcode::Lmul | Opcode::Ldiv | Opcode::Lrem
            | Opcode::Lneg | Opcode::Lshl | Opcode::Lshr | Opcode::Lushr
            | Opcode::Land | Opcode::Lor | Opcode::Lxor => {
                handle_long_op(frame, opcode)?;
                frame.program_counter += 1;
            }

            // --- Float/Double arithmetic ---
            Opcode::Fadd | Opcode::Fsub | Opcode::Fmul | Opcode::Fdiv | Opcode::Frem | Opcode::Fneg => {
                handle_float_op(frame, opcode)?;
                frame.program_counter += 1;
            }
            Opcode::Dadd | Opcode::Dsub | Opcode::Dmul | Opcode::Ddiv | Opcode::Drem | Opcode::Dneg => {
                handle_double_op(frame, opcode)?;
                frame.program_counter += 1;
            }

            Opcode::Iinc => {
                let index = frame.bytecode[frame.program_counter + 1] as usize;
                let val = frame.bytecode[frame.program_counter + 2] as i8 as i32;
                if let Some(VmValue::Int(ref mut curr)) = frame.local_variables.get_mut(index) {
                    *curr += val;
                }
                frame.program_counter += 3;
            }

            // --- Type conversions ---
            Opcode::I2l => { let v = frame.pop_int()?; frame.push(VmValue::Long(v as i64)); frame.program_counter += 1; }
            Opcode::I2f => { let v = frame.pop_int()?; frame.push(VmValue::Float(v as f32)); frame.program_counter += 1; }
            Opcode::I2d => { let v = frame.pop_int()?; frame.push(VmValue::Double(v as f64)); frame.program_counter += 1; }
            Opcode::I2b => { let v = frame.pop_int()?; frame.push_int(v as i8 as i32); frame.program_counter += 1; }
            Opcode::I2c => { let v = frame.pop_int()?; frame.push_int((v as u16) as i32); frame.program_counter += 1; }
            Opcode::I2s => { let v = frame.pop_int()?; frame.push_int(v as i16 as i32); frame.program_counter += 1; }
            Opcode::L2i => { let v = pop_long(frame)?; frame.push_int(v as i32); frame.program_counter += 1; }
            Opcode::L2f => { let v = pop_long(frame)?; frame.push(VmValue::Float(v as f32)); frame.program_counter += 1; }
            Opcode::L2d => { let v = pop_long(frame)?; frame.push(VmValue::Double(v as f64)); frame.program_counter += 1; }
            Opcode::F2i => { let v = pop_float(frame)?; frame.push_int(v as i32); frame.program_counter += 1; }
            Opcode::F2l => { let v = pop_float(frame)?; frame.push(VmValue::Long(v as i64)); frame.program_counter += 1; }
            Opcode::F2d => { let v = pop_float(frame)?; frame.push(VmValue::Double(v as f64)); frame.program_counter += 1; }
            Opcode::D2i => { let v = pop_double(frame)?; frame.push_int(v as i32); frame.program_counter += 1; }
            Opcode::D2l => { let v = pop_double(frame)?; frame.push(VmValue::Long(v as i64)); frame.program_counter += 1; }
            Opcode::D2f => { let v = pop_double(frame)?; frame.push(VmValue::Float(v as f32)); frame.program_counter += 1; }

            // --- Comparisons ---
            Opcode::Lcmp => {
                let b = pop_long(frame)?;
                let a = pop_long(frame)?;
                frame.push_int(if a > b { 1 } else if a < b { -1 } else { 0 });
                frame.program_counter += 1;
            }
            Opcode::Fcmpl | Opcode::Fcmpg => {
                let b = pop_float(frame)?;
                let a = pop_float(frame)?;
                let result = if a.is_nan() || b.is_nan() {
                    if opcode == Opcode::Fcmpl { -1 } else { 1 }
                } else if a > b { 1 } else if a < b { -1 } else { 0 };
                frame.push_int(result);
                frame.program_counter += 1;
            }
            Opcode::Dcmpl | Opcode::Dcmpg => {
                let b = pop_double(frame)?;
                let a = pop_double(frame)?;
                let result = if a.is_nan() || b.is_nan() {
                    if opcode == Opcode::Dcmpl { -1 } else { 1 }
                } else if a > b { 1 } else if a < b { -1 } else { 0 };
                frame.push_int(result);
                frame.program_counter += 1;
            }

            // --- Unary int branches ---
            Opcode::Ifeq | Opcode::Ifne | Opcode::Iflt | Opcode::Ifge | Opcode::Ifgt | Opcode::Ifle => {
                let val = frame.pop_int()?;
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let cond = match opcode {
                    Opcode::Ifeq => val == 0,
                    Opcode::Ifne => val != 0,
                    Opcode::Iflt => val < 0,
                    Opcode::Ifge => val >= 0,
                    Opcode::Ifgt => val > 0,
                    Opcode::Ifle => val <= 0,
                    _ => false,
                };
                if cond {
                    let offset = ((b1 << 8) | b2) as i16 as i32;
                    frame.program_counter = (frame.program_counter as i32 + offset) as usize;
                } else {
                    frame.program_counter += 3;
                }
            }

            // --- Binary int branches ---
            Opcode::IfIcmpeq | Opcode::IfIcmpne | Opcode::IfIcmplt | Opcode::IfIcmpge
            | Opcode::IfIcmpgt | Opcode::IfIcmple => {
                let b = frame.pop_int()?;
                let a = frame.pop_int()?;
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let cond = match opcode {
                    Opcode::IfIcmpeq => a == b,
                    Opcode::IfIcmpne => a != b,
                    Opcode::IfIcmplt => a < b,
                    Opcode::IfIcmpge => a >= b,
                    Opcode::IfIcmpgt => a > b,
                    Opcode::IfIcmple => a <= b,
                    _ => false,
                };
                if cond {
                    let offset = ((b1 << 8) | b2) as i16 as i32;
                    frame.program_counter = (frame.program_counter as i32 + offset) as usize;
                } else {
                    frame.program_counter += 3;
                }
            }

            // --- Reference branches ---
            Opcode::IfAcmpeq | Opcode::IfAcmpne => {
                let b = frame.pop()?;
                let a = frame.pop()?;
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let equal = matches!((&a, &b), (VmValue::Null, VmValue::Null))
                    || matches!((&a, &b), (VmValue::ObjectRef(x), VmValue::ObjectRef(y)) if x == y);
                let cond = if opcode == Opcode::IfAcmpeq { equal } else { !equal };
                if cond {
                    let offset = ((b1 << 8) | b2) as i16 as i32;
                    frame.program_counter = (frame.program_counter as i32 + offset) as usize;
                } else {
                    frame.program_counter += 3;
                }
            }
            Opcode::Ifnull | Opcode::Ifnonnull => {
                let val = frame.pop()?;
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let is_null = matches!(val, VmValue::Null);
                let cond = if opcode == Opcode::Ifnull { is_null } else { !is_null };
                if cond {
                    let offset = ((b1 << 8) | b2) as i16 as i32;
                    frame.program_counter = (frame.program_counter as i32 + offset) as usize;
                } else {
                    frame.program_counter += 3;
                }
            }

            Opcode::Goto => {
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let offset = ((b1 << 8) | b2) as i16 as i32;
                frame.program_counter = (frame.program_counter as i32 + offset) as usize;
            }
            Opcode::GotoW => {
                let b1 = frame.bytecode[frame.program_counter + 1] as u32;
                let b2 = frame.bytecode[frame.program_counter + 2] as u32;
                let b3 = frame.bytecode[frame.program_counter + 3] as u32;
                let b4 = frame.bytecode[frame.program_counter + 4] as u32;
                let offset = ((b1 << 24) | (b2 << 16) | (b3 << 8) | b4) as i32;
                frame.program_counter = (frame.program_counter as i32 + offset) as usize;
            }

            // --- Tableswitch ---
            Opcode::Tableswitch => {
                let base_pc = frame.program_counter;
                let pad = (4 - ((base_pc + 1) % 4)) % 4;
                let mut pos = base_pc + 1 + pad;
                let default_offset = read_i32(&frame.bytecode, pos); pos += 4;
                let low = read_i32(&frame.bytecode, pos); pos += 4;
                let high = read_i32(&frame.bytecode, pos); pos += 4;
                let key = frame.pop_int()?;
                if key >= low && key <= high {
                    let entry = pos + ((key - low) as usize) * 4;
                    let offset = read_i32(&frame.bytecode, entry);
                    frame.program_counter = (base_pc as i32 + offset) as usize;
                } else {
                    frame.program_counter = (base_pc as i32 + default_offset) as usize;
                }
            }

            // --- Lookupswitch ---
            Opcode::Lookupswitch => {
                let base_pc = frame.program_counter;
                let pad = (4 - ((base_pc + 1) % 4)) % 4;
                let mut pos = base_pc + 1 + pad;
                let default_offset = read_i32(&frame.bytecode, pos); pos += 4;
                let npairs = read_i32(&frame.bytecode, pos) as usize; pos += 4;
                let key = frame.pop_int()?;
                let mut found = false;
                for _ in 0..npairs {
                    let match_val = read_i32(&frame.bytecode, pos); pos += 4;
                    let offset = read_i32(&frame.bytecode, pos); pos += 4;
                    if key == match_val {
                        frame.program_counter = (base_pc as i32 + offset) as usize;
                        found = true;
                        break;
                    }
                }
                if !found {
                    frame.program_counter = (base_pc as i32 + default_offset) as usize;
                }
            }

            // --- Returns ---
            Opcode::Return => { return Ok(FrameResult::Completed); }
            Opcode::Ireturn | Opcode::Freturn | Opcode::Areturn | Opcode::Lreturn | Opcode::Dreturn => {
                let val = frame.pop()?;
                if let Some(parent) = vm.call_stack.last_mut() {
                    parent.push(val);
                }
                return Ok(FrameResult::Completed);
            }

            // --- Field access ---
            Opcode::Getstatic => {
                let index = read_u16(&frame.bytecode, frame.program_counter + 1);
                let (class_name, field_name, descriptor) = {
                    let class = vm.classes.get(&frame.class_name)
                        .ok_or_else(|| format!("Class not found: {}", frame.class_name))?;
                    let (tc, fn_, ds) = class.constant_pool.get_field_ref(index)?;
                    (tc.to_string(), fn_.to_string(), ds.to_string())
                };

                let key = format!("{}/{}", class_name, field_name);
                let val = vm.static_fields.get(&key).cloned().unwrap_or_else(|| {
                    if descriptor.starts_with('L') || descriptor.starts_with('[') {
                        VmValue::Null
                    } else {
                        VmValue::Int(0)
                    }
                });
                frame.push(val);
                frame.program_counter += 3;
            }
            Opcode::Putstatic => {
                let index = read_u16(&frame.bytecode, frame.program_counter + 1);
                let (class_name, field_name, _descriptor) = {
                    let class = vm.classes.get(&frame.class_name)
                        .ok_or_else(|| format!("Class not found: {}", frame.class_name))?;
                    let (tc, fn_, ds) = class.constant_pool.get_field_ref(index)?;
                    (tc.to_string(), fn_.to_string(), ds.to_string())
                };

                let val = frame.pop()?;
                let key = format!("{}/{}", class_name, field_name);
                vm.static_fields.insert(key, val);
                frame.program_counter += 3;
            }
            Opcode::Getfield => {
                let index = read_u16(&frame.bytecode, frame.program_counter + 1);
                let objectref = frame.pop()?;

                let (field_name, descriptor) = {
                    let class = vm.classes.get(&frame.class_name);
                    if let Some(cf) = class {
                        resolve_field_info(&cf.constant_pool, index).unwrap_or((String::new(), String::new()))
                    } else {
                        (String::new(), String::new())
                    }
                };

                if let VmValue::ObjectRef(heap_idx) = objectref {
                    if let Some(obj) = vm.heap.get(heap_idx) {
                        let val = obj.fields.get(&field_name).cloned().unwrap_or_else(|| {
                            if descriptor.starts_with('L') || descriptor.starts_with('[') {
                                VmValue::Null
                            } else {
                                VmValue::Int(0)
                            }
                        });
                        frame.push(val);
                    } else {
                        frame.push(VmValue::Null);
                    }
                } else {
                    frame.push(VmValue::Null);
                }
                frame.program_counter += 3;
            }
            Opcode::Putfield => {
                let index = read_u16(&frame.bytecode, frame.program_counter + 1);
                let val = frame.pop()?;
                let objectref = frame.pop()?;

                let field_name = {
                    let class = vm.classes.get(&frame.class_name);
                    if let Some(cf) = class {
                        resolve_field_name(&cf.constant_pool, index).unwrap_or_default()
                    } else {
                        String::new()
                    }
                };

                if let VmValue::ObjectRef(heap_idx) = objectref {
                    if let Some(obj) = vm.heap.get_mut(heap_idx) {
                        obj.fields.insert(field_name, val);
                    }
                }
                frame.program_counter += 3;
            }

            // --- Method invocation ---
            Opcode::Invokevirtual | Opcode::Invokespecial => {
                let method_ref_index = read_u16(&frame.bytecode, frame.program_counter + 1);

                let (target_class, method_name, descriptor) = {
                    let class = vm.classes.get(&frame.class_name)
                        .ok_or_else(|| format!("Class not found: {}", frame.class_name))?;
                    let (tc, mn, ds) = class.constant_pool.get_method_ref(method_ref_index)?;
                    (tc.to_string(), mn.to_string(), ds.to_string())
                };

                if super::native::is_native_class(&target_class) {
                    super::native::invoke_override(vm, &target_class, &method_name, &descriptor, frame)?;
                    frame.program_counter += 3;
                } else if let Some(target_cf) = vm.classes.get(&target_class) {
                    let method = target_cf.methods.iter().find(|m| {
                        if let Ok(name) = target_cf.constant_pool.get_utf8(m.name_index) {
                            if let Ok(desc) = target_cf.constant_pool.get_utf8(m.descriptor_index) {
                                return name == method_name && desc == descriptor;
                            }
                        }
                        false
                    });

                    if let Some(m) = method {
                        if let Some(code) = m.get_code(&target_cf.constant_pool) {
                            let mut next_frame = StackFrame::new(&target_class, code.max_stack, code.max_locals, code.code);
                            let arg_count = parse_descriptor_args_count(&descriptor) + 1;
                            for i in (0..arg_count).rev() {
                                let arg_val = frame.pop()?;
                                if i < next_frame.local_variables.len() {
                                    next_frame.local_variables[i] = arg_val;
                                }
                            }
                            frame.program_counter += 3;
                            return Ok(FrameResult::Invoked(next_frame));
                        } else {
                            // Native or abstract method without Code — pop args and skip
                            let arg_count = parse_descriptor_args_count(&descriptor) + 1;
                            for _ in 0..arg_count { let _ = frame.pop(); }
                            if !descriptor.ends_with(")V") {
                                if descriptor.ends_with(";") || descriptor.contains("[") {
                                    frame.push(VmValue::Null);
                                } else {
                                    frame.push(VmValue::Int(0));
                                }
                            }
                            frame.program_counter += 3;
                        }
                        // Method not found — pop args and continue
                        let arg_count = parse_descriptor_args_count(&descriptor) + 1;
                        for _ in 0..arg_count { let _ = frame.pop(); }
                        if !descriptor.ends_with(")V") {
                            if descriptor.ends_with(";") || descriptor.contains("[") {
                                frame.push(VmValue::Null);
                            } else {
                                frame.push(VmValue::Int(0));
                            }
                        }
                        frame.program_counter += 3;
                    }
                } else {
                    // Class not loaded — treat as native stub
                    let arg_count = parse_descriptor_args_count(&descriptor) + 1;
                    for _ in 0..arg_count { let _ = frame.pop(); }
                    if !descriptor.ends_with(")V") {
                        if descriptor.ends_with(";") || descriptor.contains("[") {
                            frame.push(VmValue::Null);
                        } else {
                            frame.push(VmValue::Int(0));
                        }
                    }
                    frame.program_counter += 3;
                }
            }

            Opcode::Invokestatic => {
                let method_ref_index = read_u16(&frame.bytecode, frame.program_counter + 1);

                let (target_class, method_name, descriptor) = {
                    let class = vm.classes.get(&frame.class_name)
                        .ok_or_else(|| format!("Class not found: {}", frame.class_name))?;
                    let (tc, mn, ds) = class.constant_pool.get_method_ref(method_ref_index)?;
                    (tc.to_string(), mn.to_string(), ds.to_string())
                };

                if super::native::is_native_class(&target_class) {
                    super::native::invoke_override(vm, &target_class, &method_name, &descriptor, frame)?;
                    frame.program_counter += 3;
                } else if let Some(target_cf) = vm.classes.get(&target_class) {
                    let method = target_cf.methods.iter().find(|m| {
                        if let Ok(name) = target_cf.constant_pool.get_utf8(m.name_index) {
                            if let Ok(desc) = target_cf.constant_pool.get_utf8(m.descriptor_index) {
                                return name == method_name && desc == descriptor;
                            }
                        }
                        false
                    });
                    if let Some(m) = method {
                        if let Some(code) = m.get_code(&target_cf.constant_pool) {
                            let mut next_frame = StackFrame::new(&target_class, code.max_stack, code.max_locals, code.code);
                            let arg_count = parse_descriptor_args_count(&descriptor);
                            for i in (0..arg_count).rev() {
                                let arg_val = frame.pop()?;
                                if i < next_frame.local_variables.len() {
                                    next_frame.local_variables[i] = arg_val;
                                }
                            }
                            frame.program_counter += 3;
                            return Ok(FrameResult::Invoked(next_frame));
                        } else {
                            let arg_count = parse_descriptor_args_count(&descriptor);
                            for _ in 0..arg_count { let _ = frame.pop(); }
                            if !descriptor.ends_with(")V") {
                                if descriptor.ends_with(";") || descriptor.contains("[") {
                                    frame.push(VmValue::Null);
                                } else {
                                    frame.push(VmValue::Int(0));
                                }
                            }
                            frame.program_counter += 3;
                        }
                    } else {
                        let arg_count = parse_descriptor_args_count(&descriptor);
                        for _ in 0..arg_count { let _ = frame.pop(); }
                        if !descriptor.ends_with(")V") {
                            if descriptor.ends_with(";") || descriptor.contains("[") {
                                frame.push(VmValue::Null);
                            } else {
                                    frame.push(VmValue::Int(0));
                            }
                        }
                        frame.program_counter += 3;
                    }
                } else {
                    let arg_count = parse_descriptor_args_count(&descriptor);
                    for _ in 0..arg_count { let _ = frame.pop(); }
                    if !descriptor.ends_with(")V") {
                        if descriptor.ends_with(";") || descriptor.contains("[") {
                            frame.push(VmValue::Null);
                        } else {
                            frame.push(VmValue::Int(0));
                        }
                    }
                    frame.program_counter += 3;
                }
            }

            Opcode::Invokeinterface => {
                let method_ref_index = read_u16(&frame.bytecode, frame.program_counter + 1);
                let _count = frame.bytecode[frame.program_counter + 3];
                // byte 4 is always 0

                let (target_class, method_name, descriptor) = {
                    let class = vm.classes.get(&frame.class_name)
                        .ok_or_else(|| format!("Class not found: {}", frame.class_name))?;
                    // Interface methods use InterfaceMethodref, but try Methodref resolution too
                    let result = resolve_interface_method_ref(&class.constant_pool, method_ref_index);
                    match result {
                        Ok((tc, mn, ds)) => (tc.to_string(), mn.to_string(), ds.to_string()),
                        Err(_) => {
                            // Skip unresolvable interface call
                            frame.program_counter += 5;
                            continue;
                        }
                    }
                };

                if super::native::is_native_class(&target_class) {
                    super::native::invoke_override(vm, &target_class, &method_name, &descriptor, frame)?;
                } else {
                    let arg_count = parse_descriptor_args_count(&descriptor) + 1;
                    for _ in 0..arg_count { let _ = frame.pop(); }
                    if !descriptor.ends_with(")V") {
                        if descriptor.ends_with(";") || descriptor.contains("[") {
                            frame.push(VmValue::Null);
                        } else {
                            frame.push(VmValue::Int(0));
                        }
                    }
                }
                frame.program_counter += 5;
            }

            Opcode::Invokedynamic => {
                // Not used in J2ME; skip 5 bytes
                frame.program_counter += 5;
            }

            // --- Object creation ---
            Opcode::New => {
                let index = read_u16(&frame.bytecode, frame.program_counter + 1);
                let class_name = {
                    let class = vm.classes.get(&frame.class_name)
                        .ok_or_else(|| format!("Class not found: {}", frame.class_name))?;
                    class.constant_pool.get_class_name(index)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|_| "java/lang/Object".to_string())
                };
                vm.heap.push(VmObject {
                    class_name,
                    fields: HashMap::new(),
                });
                frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                frame.program_counter += 3;
            }
            Opcode::Newarray => {
                let _count = frame.pop_int()?;
                let _atype = frame.bytecode[frame.program_counter + 1];
                vm.heap.push(VmObject {
                    class_name: "[I".to_string(),
                    fields: HashMap::new(),
                });
                frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                frame.program_counter += 2;
            }
            Opcode::Anewarray => {
                let _count = frame.pop_int()?;
                let _index = read_u16(&frame.bytecode, frame.program_counter + 1);
                vm.heap.push(VmObject {
                    class_name: "[Ljava/lang/Object;".to_string(),
                    fields: HashMap::new(),
                });
                frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                frame.program_counter += 3;
            }
            Opcode::Multianewarray => {
                let _index = read_u16(&frame.bytecode, frame.program_counter + 1);
                let dims = frame.bytecode[frame.program_counter + 3] as usize;
                for _ in 0..dims { let _ = frame.pop(); }
                vm.heap.push(VmObject {
                    class_name: "[[I".to_string(),
                    fields: HashMap::new(),
                });
                frame.push(VmValue::ObjectRef(vm.heap.len() - 1));
                frame.program_counter += 4;
            }
            Opcode::Arraylength => {
                let _arrayref = frame.pop()?;
                frame.push_int(0);
                frame.program_counter += 1;
            }

            // --- Type checking ---
            Opcode::Checkcast => {
                // Leave the objectref on the stack; skip type check
                frame.program_counter += 3;
            }
            Opcode::Instanceof => {
                let val = frame.pop()?;
                let _index = read_u16(&frame.bytecode, frame.program_counter + 1);
                frame.push_int(if matches!(val, VmValue::Null) { 0 } else { 1 });
                frame.program_counter += 3;
            }

            // --- Synchronization (single-threaded no-ops) ---
            Opcode::Monitorenter => { let _ = frame.pop(); frame.program_counter += 1; }
            Opcode::Monitorexit => { let _ = frame.pop(); frame.program_counter += 1; }

            // --- Exception throwing ---
            Opcode::Athrow => {
                let _exception = frame.pop()?;
                return Err("J2ME Exception thrown (athrow)".to_string());
            }

            // --- JSR/RET (rare, legacy) ---
            Opcode::Jsr => {
                frame.push_int((frame.program_counter + 3) as i32);
                let b1 = frame.bytecode[frame.program_counter + 1] as u16;
                let b2 = frame.bytecode[frame.program_counter + 2] as u16;
                let offset = ((b1 << 8) | b2) as i16 as i32;
                frame.program_counter = (frame.program_counter as i32 + offset) as usize;
            }
            Opcode::JsrW => {
                frame.push_int((frame.program_counter + 5) as i32);
                let offset = read_i32(&frame.bytecode, frame.program_counter + 1);
                frame.program_counter = (frame.program_counter as i32 + offset) as usize;
            }
            Opcode::Ret => {
                let index = frame.bytecode[frame.program_counter + 1] as usize;
                if let VmValue::Int(addr) = safe_local_get(frame, index)? {
                    frame.program_counter = addr as usize;
                } else {
                    return Err("ret: expected return address".to_string());
                }
            }

            Opcode::Wide => {
                // Wide prefix modifies the next instruction
                let modified_opcode = frame.bytecode[frame.program_counter + 1];
                let index = read_u16(&frame.bytecode, frame.program_counter + 2) as usize;
                match modified_opcode {
                    0x15 | 0x17 | 0x19 | 0x16 | 0x18 => { // xload
                        let val = safe_local_get(frame, index)?;
                        frame.push(val);
                        frame.program_counter += 4;
                    }
                    0x36 | 0x38 | 0x3a | 0x37 | 0x39 => { // xstore
                        let val = frame.pop()?;
                        safe_local_set(frame, index, val);
                        frame.program_counter += 4;
                    }
                    0x84 => { // iinc wide
                        let inc = read_i16(&frame.bytecode, frame.program_counter + 4) as i32;
                        if let Some(VmValue::Int(ref mut v)) = frame.local_variables.get_mut(index) {
                            *v += inc;
                        }
                        frame.program_counter += 6;
                    }
                    _ => { frame.program_counter += 4; }
                }
            }

            Opcode::Breakpoint | Opcode::Impdep1 | Opcode::Impdep2 => {
                frame.program_counter += 1;
            }
        }
    }
    Ok(FrameResult::Completed)
}

// --- Helper functions ---

fn safe_local_get(frame: &StackFrame, index: usize) -> Result<VmValue, String> {
    frame.local_variables.get(index)
        .cloned()
        .ok_or_else(|| format!("Local variable index {} out of bounds (len={})", index, frame.local_variables.len()))
}

fn safe_local_set(frame: &mut StackFrame, index: usize, val: VmValue) {
    while frame.local_variables.len() <= index {
        frame.local_variables.push(VmValue::Null);
    }
    frame.local_variables[index] = val;
}

fn read_u16(bytes: &[u8], pos: usize) -> u16 {
    ((bytes[pos] as u16) << 8) | (bytes[pos + 1] as u16)
}

fn read_i16(bytes: &[u8], pos: usize) -> i16 {
    read_u16(bytes, pos) as i16
}

fn read_i32(bytes: &[u8], pos: usize) -> i32 {
    ((bytes[pos] as i32) << 24) | ((bytes[pos+1] as i32) << 16)
    | ((bytes[pos+2] as i32) << 8) | (bytes[pos+3] as i32)
}

fn pop_long(frame: &mut StackFrame) -> Result<i64, String> {
    match frame.pop()? {
        VmValue::Long(v) => Ok(v),
        VmValue::Int(v) => Ok(v as i64),
        _ => Ok(0),
    }
}

fn pop_float(frame: &mut StackFrame) -> Result<f32, String> {
    match frame.pop()? {
        VmValue::Float(v) => Ok(v),
        VmValue::Int(v) => Ok(v as f32),
        _ => Ok(0.0),
    }
}

fn pop_double(frame: &mut StackFrame) -> Result<f64, String> {
    match frame.pop()? {
        VmValue::Double(v) => Ok(v),
        VmValue::Float(v) => Ok(v as f64),
        VmValue::Int(v) => Ok(v as f64),
        _ => Ok(0.0),
    }
}

fn resolve_ldc(vm: &Vm, class_name: &str, index: u16) -> Result<VmValue, String> {
    use crate::classfile::constant_pool::Constant;
    let class = vm.classes.get(class_name)
        .ok_or_else(|| format!("Class not found for ldc: {}", class_name))?;
    match class.constant_pool.constants.get(index as usize) {
        Some(Constant::Integer(v)) => Ok(VmValue::Int(*v)),
        Some(Constant::Float(v)) => Ok(VmValue::Float(*v)),
        Some(Constant::String { string_index }) => {
            // Push a mock string reference
            let _ = string_index;
            Ok(VmValue::Int(0))
        }
        Some(Constant::Class { .. }) => Ok(VmValue::Int(0)),
        _ => Ok(VmValue::Int(0)),
    }
}

fn resolve_ldc2w(vm: &Vm, class_name: &str, index: u16) -> Result<VmValue, String> {
    use crate::classfile::constant_pool::Constant;
    let class = vm.classes.get(class_name)
        .ok_or_else(|| format!("Class not found for ldc2_w: {}", class_name))?;
    match class.constant_pool.constants.get(index as usize) {
        Some(Constant::Long(v)) => Ok(VmValue::Long(*v)),
        Some(Constant::Double(v)) => Ok(VmValue::Double(*v)),
        _ => Ok(VmValue::Long(0)),
    }
}

fn resolve_field_name(cp: &crate::classfile::constant_pool::ConstantPool, index: u16) -> Result<String, String> {
    use crate::classfile::constant_pool::Constant;
    match cp.constants.get(index as usize) {
        Some(Constant::Fieldref { name_and_type_index, .. }) => {
            let (name, _desc) = cp.get_name_and_type(*name_and_type_index)?;
            Ok(name.to_string())
        }
        _ => Err("Invalid fieldref".to_string()),
    }
}

fn resolve_field_info(cp: &crate::classfile::constant_pool::ConstantPool, index: u16) -> Result<(String, String), String> {
    use crate::classfile::constant_pool::Constant;
    match cp.constants.get(index as usize) {
        Some(Constant::Fieldref { name_and_type_index, .. }) => {
            let (name, desc) = cp.get_name_and_type(*name_and_type_index)?;
            Ok((name.to_string(), desc.to_string()))
        }
        _ => Err("Invalid fieldref".to_string()),
    }
}

fn resolve_interface_method_ref(cp: &crate::classfile::constant_pool::ConstantPool, index: u16) -> Result<(&str, &str, &str), String> {
    use crate::classfile::constant_pool::Constant;
    match cp.constants.get(index as usize) {
        Some(Constant::InterfaceMethodref { class_index, name_and_type_index }) => {
            let class_name = cp.get_class_name(*class_index)?;
            let (name, descriptor) = cp.get_name_and_type(*name_and_type_index)?;
            Ok((class_name, name, descriptor))
        }
        Some(Constant::Methodref { class_index, name_and_type_index }) => {
            let class_name = cp.get_class_name(*class_index)?;
            let (name, descriptor) = cp.get_name_and_type(*name_and_type_index)?;
            Ok((class_name, name, descriptor))
        }
        _ => Err(format!("Invalid interface method ref at index {}", index)),
    }
}

fn handle_long_op(frame: &mut StackFrame, opcode: Opcode) -> Result<(), String> {
    match opcode {
        Opcode::Lneg => { let a = pop_long(frame)?; frame.push(VmValue::Long(-a)); }
        _ => {
            let b = pop_long(frame)?;
            let a = pop_long(frame)?;
            let result = match opcode {
                Opcode::Ladd => a.wrapping_add(b),
                Opcode::Lsub => a.wrapping_sub(b),
                Opcode::Lmul => a.wrapping_mul(b),
                Opcode::Ldiv => { if b == 0 { return Err("ArithmeticException".to_string()); } a / b }
                Opcode::Lrem => { if b == 0 { return Err("ArithmeticException".to_string()); } a % b }
                Opcode::Land => a & b,
                Opcode::Lor => a | b,
                Opcode::Lxor => a ^ b,
                Opcode::Lshl => a << (b & 0x3f),
                Opcode::Lshr => a >> (b & 0x3f),
                Opcode::Lushr => ((a as u64) >> (b & 0x3f)) as i64,
                _ => 0,
            };
            frame.push(VmValue::Long(result));
        }
    }
    Ok(())
}

fn handle_float_op(frame: &mut StackFrame, opcode: Opcode) -> Result<(), String> {
    match opcode {
        Opcode::Fneg => { let a = pop_float(frame)?; frame.push(VmValue::Float(-a)); }
        _ => {
            let b = pop_float(frame)?;
            let a = pop_float(frame)?;
            let result = match opcode {
                Opcode::Fadd => a + b,
                Opcode::Fsub => a - b,
                Opcode::Fmul => a * b,
                Opcode::Fdiv => a / b,
                Opcode::Frem => a % b,
                _ => 0.0,
            };
            frame.push(VmValue::Float(result));
        }
    }
    Ok(())
}

fn handle_double_op(frame: &mut StackFrame, opcode: Opcode) -> Result<(), String> {
    match opcode {
        Opcode::Dneg => { let a = pop_double(frame)?; frame.push(VmValue::Double(-a)); }
        _ => {
            let b = pop_double(frame)?;
            let a = pop_double(frame)?;
            let result = match opcode {
                Opcode::Dadd => a + b,
                Opcode::Dsub => a - b,
                Opcode::Dmul => a * b,
                Opcode::Ddiv => a / b,
                Opcode::Drem => a % b,
                _ => 0.0,
            };
            frame.push(VmValue::Double(result));
        }
    }
    Ok(())
}

fn parse_descriptor_args_count(descriptor: &str) -> usize {
    let mut count = 0;
    let mut in_parenthesis = false;
    let mut in_object = false;
    for c in descriptor.chars() {
        if c == '(' {
            in_parenthesis = true;
            continue;
        }
        if c == ')' {
            break;
        }
        if in_parenthesis {
            if in_object {
                if c == ';' {
                    in_object = false;
                    count += 1;
                }
            } else if c == 'L' {
                in_object = true;
            } else if c == '[' {
                // array type prefix
            } else {
                count += 1;
            }
        }
    }
    count
}
