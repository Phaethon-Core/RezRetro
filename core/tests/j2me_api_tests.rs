// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use ressu_core::vm::{Vm, StackFrame, VmValue};

#[test]
fn test_graphics_draw_rect() {
    let mut vm = Vm::new();
    let mut frame = StackFrame::new("javax/microedition/lcdui/Graphics", 5, 0, vec![]);
    
    frame.push_int(10); // x
    frame.push_int(20); // y
    frame.push_int(30); // width
    frame.push_int(40); // height
    
    let res = ressu_core::vm::native::invoke_override(
        &mut vm,
        "javax/microedition/lcdui/Graphics",
        "drawRect",
        "(IIII)V",
        &mut frame
    );
    
    assert!(res.is_ok());
    let index = (20 * 240 + 10) as usize;
    assert_eq!(vm.framebuffer.pixels[index], 0xFFFFFFFF);
}

#[test]
fn test_rms_record_store() {
    let mut vm = Vm::new();
    let mut frame = StackFrame::new("TestGame", 5, 0, vec![]);
    
    frame.push(VmValue::Null); // name
    frame.push_int(1); // create
    
    let res = ressu_core::vm::native::invoke_override(
        &mut vm,
        "javax/microedition/rms/RecordStore",
        "openRecordStore",
        "(Ljava/lang/String;Z)Ljavax/microedition/rms/RecordStore;",
        &mut frame
    );
    
    assert!(res.is_ok());
    let return_val = frame.operand_stack.last().unwrap();
    if let VmValue::ObjectRef(heap_idx) = return_val {
        assert_eq!(vm.heap[*heap_idx].class_name, "javax/microedition/rms/RecordStore");
    } else {
        panic!("Expected ObjectRef returned");
    }
}
