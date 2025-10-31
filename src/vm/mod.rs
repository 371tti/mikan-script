use std::{
    path::PathBuf, sync::{Arc, RwLock}, thread::{self, JoinHandle}
};

use crate::vm::{code_manager::CodeManager, vm::VM};

pub mod code_manager;
pub mod memory;
pub mod operations;
pub mod pre_decoder;
pub mod vm;
pub mod function;

pub struct VMPool {
    pub vms: Vec<Arc<RwLock<VM>>>,
    handles: Vec<JoinHandle<()>>,
    pub code_manager: CodeManager,
}

impl VMPool {
    pub fn new() -> Self {
        VMPool {
            vms: Vec::new(),
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

    pub fn push_and_run_threaded(&mut self, mut vm: VM, use_core_affinity: bool) {
        let index = self.vms.len();
        vm.vm_id = index as u64;
        vm.cm = self.code_manager.clone_shared();
        let vm_arc = Arc::new(RwLock::new(vm));
        self.vms.push(vm_arc.clone());

        let handle = thread::spawn(move || {
            if use_core_affinity {
                if let Some(cores) = core_affinity::get_core_ids() {
                    let core = &cores[index % cores.len()];
                    core_affinity::set_for_current(*core);
                }
            }
            let mut vm = vm_arc.write().unwrap();
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