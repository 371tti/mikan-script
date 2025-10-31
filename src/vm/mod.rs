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
    pub code_manager: Arc<CodeManager>,
}

impl VMPool {
    pub fn new() -> Self {
        VMPool {
            vms: Vec::new(),
            handles: Vec::new(),
            code_manager: Arc::new(CodeManager::new("none".into())),
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.code_manager = Arc::new(CodeManager::new(PathBuf::from(path)));
    }

    pub fn push_and_run_threaded(&mut self, mut vm: VM, use_core_affinity: bool) {
        let index = self.vms.len();
        vm.vm_id = index as u64;
        let vm_arc = Arc::new(RwLock::new(vm));
        self.vms.push(vm_arc.clone());
        let function_table = self.code_manager.get_decoded();

        let handle = thread::spawn(move || {
            if use_core_affinity {
                if let Some(cores) = core_affinity::get_core_ids() {
                    let core = &cores[index % cores.len()];
                    core_affinity::set_for_current(*core);
                }
            }
            let mut vm = vm_arc.write().unwrap();
            vm.function_table = function_table;
            vm.run_function();
        });

        self.handles.push(handle);
    }

    pub fn wait_all(&mut self) {
        while let Some(handle) = self.handles.pop() {
            handle.join().unwrap();
        }
    }
}