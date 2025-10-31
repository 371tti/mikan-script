use std::sync::Arc;

use crate::vm::{code_manager::CodeManager, function::FunctionPtr, memory::Memory};

/// Direct-threaded VM
pub struct VM {
    pub st: VMState,
    pub function_table: Box<[FunctionPtr]>,
    pub cm: Arc<CodeManager>,
    pub vm_id: u64,
}

/// Direct-threaded VM ですてすてす
impl VM {
    pub fn new() -> Self {
        VM {
            st: VMState::new(),
            function_table: Box::new([]),
            cm: Arc::new(CodeManager::new("none".into())),
            vm_id: 0,
        }
    }

    pub fn replace_code_manager(&mut self, cm: Arc<CodeManager>) {
        self.cm = cm;
    }

    /// 指定の関数を実行します
    pub fn run_function(&mut self) {
        loop {
            let func = &self.function_table[self.st.now_call_index];
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