// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Phaethon

use ressu_core::vm::{Vm, StackFrame, VmValue};

#[test]
fn test_basic_arithmetic() {
    let mut vm = Vm::new();
    let bytecode = vec![
        0x10, 10,   // bipush 10
        0x10, 20,   // bipush 20
        0x60,       // iadd
        0xac,       // ireturn
    ];

    let parent_frame = StackFrame::new("TestClass", 2, 0, vec![0xb1]);
    let mut child_frame = StackFrame::new("TestClass", 2, 0, bytecode);
    
    vm.call_stack.push(parent_frame);
    
    let res = ressu_core::vm::instructions::execute_frame(&mut vm, &mut child_frame);
    assert!(res.is_ok());
    
    let parent = vm.call_stack.first().unwrap();
    assert_eq!(parent.operand_stack.last(), Some(&VmValue::Int(30)));
}

#[test]
fn test_loop_iteration() {
    let mut vm = Vm::new();
    let bytecode = vec![
        0x03,       // iconst_0
        0x3b,       // istore_0
        0x03,       // iconst_0
        0x3c,       // istore_1
        
        0x1a,       // iload_0
        0x10, 5,    // bipush 5
        0xa2, 0x00, 13, // if_icmpge offset 13 (PC 7 + 13 = 20 -> iload_1)
        
        0x1b,       // iload_1
        0x1a,       // iload_0
        0x60,       // iadd
        0x3c,       // istore_1
        
        0x84, 0, 1, // iinc 0, 1
        0xa7, 0xff, 243, // goto offset -13 (PC 17 - 13 = 4 -> iload_0)
        
        0x1b,       // iload_1
        0xac,       // ireturn
    ];

    let parent_frame = StackFrame::new("TestClass", 2, 0, vec![0xb1]);
    let mut child_frame = StackFrame::new("TestClass", 3, 2, bytecode);
    
    vm.call_stack.push(parent_frame);
    
    let res = ressu_core::vm::instructions::execute_frame(&mut vm, &mut child_frame);
    assert!(res.is_ok());
    
    let parent = vm.call_stack.first().unwrap();
    assert_eq!(parent.operand_stack.last(), Some(&VmValue::Int(10)));
}
