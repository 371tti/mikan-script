use std::{
    path::PathBuf, thread::{self, JoinHandle}
};

use crate::vm::{code_manager::CodeManager, vm::VM};

pub mod code_manager;
pub mod memory;
pub mod instruction;
pub mod pre_decoder;
pub mod vm;
pub mod function;

pub struct VMPool {
    pub vm_num: u64,
    handles: Vec<JoinHandle<()>>,
    pub code_manager: CodeManager,
}

impl VMPool {
    pub fn new() -> Self {
        VMPool {
            vm_num: 0,
            handles: Vec::new(),
            code_manager: CodeManager::new("none".into()),
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.code_manager = CodeManager::new(PathBuf::from(path));
    }

    pub fn run(&mut self) {
        let vm = VM::new();
        self.push_and_run_threaded(vm,false);
    }

    pub fn run_with_core_affinity(&mut self) {
        let vm = VM::new();
        self.push_and_run_threaded(vm,true);
    }

    pub fn push_and_run_threaded(&mut self, mut vm: VM, use_core_affinity: bool) {
        let index = self.vm_num;
        vm.vm_id = index;
        vm.cm = self.code_manager.clone_shared();
        self.vm_num += 1;


        let handle = thread::spawn(move || {
            if use_core_affinity {
                if let Some(cores) = core_affinity::get_core_ids() {
                    let core = &cores[(index as usize) % cores.len()];
                    core_affinity::set_for_current(*core);
                }
            }
            vm.run();
        });

        self.handles.push(handle);
    }

    pub fn wait_all(&mut self) {
        while let Some(handle) = self.handles.pop() {
            handle.join().unwrap();
        }
    }
}