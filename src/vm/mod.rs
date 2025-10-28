use std::{
    ops::Deref, pin::Pin, sync::{Arc, RwLock}, thread::{self, JoinHandle}
};

use crate::vm::{code_manager::CodeManager, memory::Memory};

pub mod code_manager;
pub mod memory;
pub mod operations;
pub mod pre_decoder;

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
            code_manager: Arc::new(CodeManager::new()),
        }
    }

    pub fn push_and_run_threaded(&mut self, vm: VM, function: usize, use_core_affinity: bool) {
        let vm_arc = Arc::new(RwLock::new(vm));
        self.vms.push(vm_arc.clone());
        let index = self.vms.len() - 1;
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
            vm.run_function(function);
        });

        self.handles.push(handle);
    }

    pub fn wait_all(&mut self) {
        while let Some(handle) = self.handles.pop() {
            handle.join().unwrap();
        }
    }
}

/// Direct-threaded VM
pub struct VM {
    pub st: VMState,
    pub function_table: Box<[FunctionPtr]>,
    pub cm: Arc<CodeManager>,
}

pub struct FunctionPtr(*const Function);

impl Deref for FunctionPtr {
    type Target = Function;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

unsafe impl Send for FunctionPtr {}
unsafe impl Sync for FunctionPtr {}
/// Direct-threaded VM ですてすてす
impl VM {
    pub fn new() -> Self {
        VM {
            st: VMState::new(),
            function_table: Box::new([]),
            cm: Arc::new(CodeManager::new()),
        }
    }

    pub fn replace_code_manager(&mut self, cm: Arc<CodeManager>) {
        self.cm = cm;
    }

    /// 指定の関数を実行します
    #[inline(always)]
    pub fn run_function(&mut self, index: usize) {
        self.st.call_stack.push(index);
        self.st.now_call_index = *self.st.call_stack.last().unwrap_or_else(|| {
            std::process::exit(1);
        });
        loop {
            let func = &self.function_table[index];
            let ins = &func.instructions[self.st.pc];
            (ins.f)(self, ins.a, ins.b);
        }
    }
}

/// VMの状態を保持する構造体
pub struct VMState {
    /// 汎用レジスタ
    /// r0 : 0x000000 固定値レジスタ
    /// r1~r253 : 汎用レジスタ
    /// r254 : ゴミ箱レジスタ
    /// r255 : 0xFFFFFF 固定値レジスタ
    pub r: [u64; 256],
    pub mem: Memory,
    pub pc: usize,
    /// 呼び出しスタック
    /// 現在の関数インデックスを保持する
    pub call_stack: Vec<usize>,
    pub now_call_index: usize,
}

impl VMState {
    pub fn new() -> Self {
        let mut r = [0u64; 256];
        r[255] = u64::MAX;
        VMState {
            r,
            mem: Memory::new(),
            pc: 0,
            call_stack: Vec::new(),
            now_call_index: 0,
        }
    }
}

/// 命令の型定義
pub type Op = fn(&mut VM, a: u64, b: u64);

/// デコード済み命令
#[derive(Clone)]
pub struct Instruction {
    pub f: Op,
    pub a: u64,
    pub b: u64,
}

impl Instruction {
    pub fn new(f: Op, a: u64, b: u64) -> Self {
        Instruction { f, a, b }
    }
}

/// 関数保持
#[derive(Clone)]
pub struct Function {
    pub instructions: Pin<Box<[Instruction]>>,
}

impl Function {
    pub fn new(instructions: Box<[Instruction]>) -> Self {
        Function { instructions: Pin::new(instructions) }
    }

    #[inline(always)]
    pub fn pinned_ptr(&self) -> FunctionPtr {
        FunctionPtr(self as *const Function)
    }
}