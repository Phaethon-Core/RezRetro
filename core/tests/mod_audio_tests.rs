// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use ressu_core::vm::{Vm, StackFrame, VmValue};

#[test]
fn test_audio_play_tone() {
    let mut vm = Vm::new();
    let mut frame = StackFrame::new("javax/microedition/media/Manager", 5, 0, vec![]);
    
    frame.push_int(60); // middle C (C4, approx 261 Hz)
    frame.push_int(500);
    frame.push_int(100);
    
    let res = ressu_core::vm::native::invoke_override(
        &mut vm,
        "javax/microedition/media/Manager",
        "playTone",
        "(III)V",
        &mut frame
    );
    
    assert!(res.is_ok());
    assert_eq!(vm.audio_system.events_queue.len(), 1);
    
    let event = &vm.audio_system.events_queue[0];
    let freq = u32::from_le_bytes(event.data[0..4].try_into().unwrap());
    let dur = u32::from_le_bytes(event.data[4..8].try_into().unwrap());
    
    assert_eq!(freq, 261);
    assert_eq!(dur, 500);
}

#[test]
fn test_mod_override_resources() {
    let mut vm = Vm::new();
    
    let asset_path = "assets/background.png";
    let mock_data = vec![1, 2, 3, 4];
    vm.mod_system.overrides.insert(asset_path.to_string(), mock_data);
    
    let mut frame = StackFrame::new("java/lang/Class", 2, 0, vec![]);
    frame.push(VmValue::Null);
    
    let res = ressu_core::vm::native::invoke_override(
        &mut vm,
        "java/lang/Class",
        "getResourceAsStream",
        "(Ljava/lang/String;)Ljava/io/InputStream;",
        &mut frame
    );
    
    assert!(res.is_ok());
    let return_val = frame.operand_stack.last().unwrap();
    if let VmValue::ObjectRef(heap_idx) = return_val {
        assert_eq!(vm.heap[*heap_idx].class_name, "java/io/InputStream");
    } else {
        panic!("Expected ObjectRef returned");
    }
}
