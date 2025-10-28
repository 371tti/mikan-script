use std::sync::RwLock;

use crate::vm::{operations::Operations, Function, FunctionPtr, Instruction};

pub struct CodeManager {
    pub functions: RwLock<Vec<Function>>,
}

impl CodeManager {
    pub fn new() -> Self {
        let functions = RwLock::new(Vec::new());
        CodeManager { functions }
    }

    pub fn get_decoded(&self) -> Box<[FunctionPtr]> {
        self.functions.read().unwrap().iter()
            .map(|f| f.pinned_ptr())
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub fn get_decode(&self, _func_id: u64, _deep: u64) -> Box<[FunctionPtr]> {
        unimplemented!()
    }

    pub fn set_test(&self) {
        self.functions.write().unwrap().push(Function::new(Box::new([
            Instruction::new(Operations::load_u64_immediate, 2, 1000000000),
            Instruction::new(Operations::add_u64_immediate, 1, 1),
            Instruction::new(Operations::lt_u64_jump, 0x000102, 1),
            Instruction::new(Operations::print_u64, 1, 0),
            Instruction::new(Operations::exit, 0, 0),
        ])));
    }

    pub fn set_test2(&self) {
        self.functions.write().unwrap().push(Function::new(Box::new([
            Instruction::new(Operations::alloc, 3, 8),
            Instruction::new(Operations::add_u64_immediate, 1, 1),
            Instruction::new(Operations::load_u64_immediate, 2, 1000000000),
            Instruction::new(Operations::store_u64, 0x00030000, 0),
            Instruction::new(Operations::atomic_add_u64, 0x08030001, 0),
            Instruction::new(Operations::atomic_load_u64, 0x00030004, 0),
            Instruction::new(Operations::lt_u64_jump, 0x000402, 4),
            Instruction::new(Operations::print_u64, 4, 0),
            Instruction::new(Operations::exit, 0, 0),
        ])));
    }
}