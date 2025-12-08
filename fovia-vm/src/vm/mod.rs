use std::{
    path::PathBuf, sync::{Arc, Mutex, atomic::AtomicU64}, thread::{self, JoinHandle}
};

use crate::vm::{code_manager::CodeManager, io::Reactor, memory::{Memory, MemoryManager}, vm::{PrimaryReactor, VM}};

pub mod code_manager;
pub mod memory;
pub mod instruction;
pub mod pre_decoder;
pub mod vm;
pub mod function;
pub mod io;

const INIT_MEMORY_LEN: usize = 32;

#[derive(Debug)]
pub struct VMPool {
    pub vm_num: AtomicU64,
    handles: Mutex<Vec<JoinHandle<()>>>,
    pub code_manager: CodeManager,
    pub memory: Arc<Memory>,
    pub reactor: Arc<PrimaryReactor>,
}

impl VMPool {
    pub fn new() -> Self {
        let reactor = Arc::new(PrimaryReactor::new().unwrap());
        VMPool {
            vm_num: AtomicU64::new(0),
            handles: Mutex::new(Vec::new()),
            code_manager: CodeManager::new("none".into()),
            memory: Arc::new(Memory::with_capacity(INIT_MEMORY_LEN)),
            reactor,
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.code_manager = CodeManager::new(PathBuf::from(path));
    }

    pub fn run(self: &Arc<Self>) {
        let vm = VM::new(self.reactor.clone());
        self.push_and_run_threaded(vm,false);
    }

    pub fn push_and_run_threaded(self: &Arc<Self>, mut vm: VM, use_core_affinity: bool) -> u64 {
        let index = self.vm_num.load(std::sync::atomic::Ordering::SeqCst);
        vm.set_id(index);
        vm.replace_code_manager(self.code_manager.clone_shared());
        vm.replace_memory(self.memory.clone());
        vm.set_pool(self.clone());
        self.vm_num.fetch_add(1, std::sync::atomic::Ordering::SeqCst);


        let handle = thread::spawn(move || {
            if use_core_affinity {
                core_affinity(index);
            }
            vm.run();
            // ここでVMが終了
        });

        self.handles.lock().unwrap().push(handle);
        index
    }

    pub fn wait_all(self: &Arc<Self>) {
        let mut op_handle = self.handles.lock().unwrap().pop();
        while let Some(handle) = op_handle {
            handle.join().unwrap();
            op_handle = self.handles.lock().unwrap().pop();
        }
    }
}

#[cfg(target_os = "windows")]
pub fn core_affinity(vm_index: u64) {
    use windows::Win32::System::Threading::{GetCurrentThread, SetThreadAffinityMask};
    unsafe {
        let mask = 1 << (vm_index as usize % 64); // 64コアまで対応
        SetThreadAffinityMask(GetCurrentThread(), mask);
    }
}