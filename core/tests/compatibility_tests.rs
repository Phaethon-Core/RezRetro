// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use ressu_core::vm::{Vm, StackFrame, VmValue};

#[test]
fn test_nokia_vibra_override() {
    let mut vm = Vm::new();
    let mut frame = StackFrame::new("com/nokia/mid/ui/DeviceControl", 2, 0, vec![]);
    
    frame.push_int(100);
    
    let res = ressu_core::vm::native::invoke_override(
        &mut vm,
        "com/nokia/mid/ui/DeviceControl",
        "startVibra",
        "(I)V",
        &mut frame
    );
    
    assert!(res.is_ok());
}

#[test]
fn test_vm_save_load_states() {
    let mut vm = Vm::new();
    
    vm.heap.push(ressu_core::vm::VmObject {
        class_name: "TestObject".to_string(),
        fields: std::collections::HashMap::new(),
    });
    
    let mut frame = StackFrame::new("TestClass", 5, 2, vec![1, 2, 3]);
    frame.push_int(99);
    vm.call_stack.push(frame);
    
    let serialized = vm.serialize_state();
    assert!(serialized.is_ok());
    
    let mut vm2 = Vm::new();
    let load_res = vm2.deserialize_state(&serialized.unwrap());
    assert!(load_res.is_ok());
    
    assert_eq!(vm2.heap.len(), 1);
    assert_eq!(vm2.heap[0].class_name, "TestObject");
    
    assert_eq!(vm2.call_stack.len(), 1);
    assert_eq!(vm2.call_stack[0].class_name, "TestClass");
    assert_eq!(vm2.call_stack[0].operand_stack.last(), Some(&VmValue::Int(99)));
}
