use std::path::PathBuf;

use crate::vm::{code_manager::CodeManager, function::FunctionPtr, memory::Memory};

/// Direct-threaded VM
/// 関数ポインタ配列から命令を実行し続ける状態機械
///
pub struct VM {
    /// VMの状態
    pub st: VMState,
    /// 関数テーブル
    pub function_table: Box<[FunctionPtr]>,
    /// コードマネージャ
    pub cm: CodeManager,
    /// VMのID
    pub vm_id: u64,
}

impl VM {
    pub fn new() -> Self {
        VM {
            st: VMState::new(),
            function_table: Box::new([]),
            cm: CodeManager::new("none".into()),
            vm_id: 0,
        }
    }

    /// バイトコードのパスを設定します
    pub fn set_path(&mut self, path: String) {
        self.cm = CodeManager::new(PathBuf::from(path));
    }

    /// コードマネージャを差し替えます
    pub fn replace_code_manager(&mut self, cm: CodeManager) {
        self.cm = cm;
    }

    /// メモリを差し替えます
    pub fn replace_memory(&mut self, mem: Memory) {
        self.st.mem = mem;
    }

    /// 指定の関数を実行します
    pub fn run(&mut self) {
        // コードマネージャから関数テーブルを取得
        self.function_table = self.cm.get_decoded();

        self.st.now_function_ptr = self.function_table[self.st.now_call_index];
        // ループ-アンローリング(/・ω・)/ www (パフォーマンス10%向上確認済み)
        loop {
            if self.st.state_flag & state_flag::PAUSE != 0 {
                break;
            }
            self.st.state_flag = 0;
            loop {
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                let ins = &self.st.now_function_ptr.instructions[self.st.pc];
                (ins.f)(self, ins.a, ins.b);
                if self.st.state_flag != 0 {
                    break;
                }
            }
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
    /// 呼び出しスタック
    /// 現在の関数インデックスを保持する
    pub call_stack: Vec<usize>,
    pub mem: Memory,
    pub now_function_ptr: FunctionPtr,
    pub pc: usize,
    pub now_call_index: usize,

    /// 1 << 0 : 停止フラグ
    /// 1 << 1 : コールサイクルフラグ
    pub state_flag: u8,
}

impl VMState {
    pub fn new() -> Self {
        let mut r = [0u64; 256];
        r[255] = u64::MAX;
        VMState {
            r,
            mem: Memory::new(),
            now_function_ptr: FunctionPtr(std::ptr::null()),
            pc: 0,
            call_stack: Vec::new(),
            now_call_index: 0,

            state_flag: 0,
        }
    }
}

pub mod state_flag {
    pub const PAUSE: u8 = 0b0000_0001;
    pub const IN_CALL: u8 = 0b0000_0010;
}