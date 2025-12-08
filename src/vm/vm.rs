use std::{path::PathBuf, sync::Arc};

use crate::vm::{VMPool, code_manager::CodeManager, function::FunctionPtr, instruction::Instruction, io::{IoEngine, windows::IocpReactor}, memory::{Memory, MemoryManager}};

const REGISTER_NUM: usize = 256;
const INIT_MEMORY_LEN: usize = 16;

#[cfg(target_os = "windows")]
pub type PrimaryReactor = IocpReactor;

/// Direct-threaded VM
/// 関数ポインタ配列から命令を実行し続ける状態機械
#[repr(align(64))]
#[derive(Clone, Debug)]
pub struct VM {
    /// VMの状態
    pub st: VMState,
    /// 関数テーブル
    pub function_table: Box<[FunctionPtr]>,
    /// コードマネージャ
    pub cm: CodeManager,
    /// VMのID
    pub vm_id: u64,
    /// IOエンジン
    pub io: IoEngine<PrimaryReactor>,
    /// VMPoolへの参照
    pub pool: Option<Arc<VMPool>>,
}

impl VM {
    pub fn new(reactor: Arc<PrimaryReactor>) -> Self {
        VM {
            st: VMState::new(),
            function_table: Box::new([]),
            cm: CodeManager::new("none".into()),
            vm_id: 0,
            io: IoEngine::new(reactor),
            pool: None,
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
    pub fn replace_memory(&mut self, mem: Arc<Memory>) {
        self.st.mem = mem;
    }

    /// VMPoolへの参照を設定します
    pub fn set_pool(&mut self, pool: Arc<VMPool>) {
        self.pool = Some(pool);
    }

    pub fn set_id(&mut self, id: u64) {
        self.vm_id = id;
    }

    /// 指定の関数を実行します
    #[inline(always)]
    pub fn run(mut self) -> Self {
        // コードマネージャから関数テーブルを取得
        self.function_table = self.cm.get_decoded();

        self.st.now_function_ptr = self.function_table[self.st.now_call_index];
        // ループ-アンローリング(/・ω・)/
        loop {
            if self.st.state_flag & state_flag::PAUSE != 0 {
                break;
            }
            self.st.state_flag = 0;

            while self.st.state_flag == 0 {
                // アンローリング x16
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
                let ins = self.read_instruction(); ins.as_fn()(&mut self);
            }
        }

        self
    }

    /// 現在の命令を読み取ります
    #[inline(always)]
    pub fn read_instruction(&mut self) -> Instruction {
        self.st.now_function_ptr.fast_get(self.st.pc)
    }

    /// 次のオペランドを読み取ります
    #[inline(always)]
    pub fn next_operand(&mut self) -> [u8; 8] {
        unsafe {
            self.st.pc = self.st.pc.unchecked_add(1)
        }
        self.read_instruction().as_ol()
    }

    /// 次の即値オペランドを読み取ります
    #[inline(always)]
    pub fn next_operand_imm(&mut self) -> u64 {
        unsafe {
            self.st.pc = self.st.pc.unchecked_add(1)
        }
        self.read_instruction().as_imm()
    }

    /// 次のステップに進みます
    #[inline(always)]
    pub fn next_step(&mut self) {
        unsafe {
            self.st.pc = self.st.pc.unchecked_add(1)
        }
    }
}

/// VMの状態を保持する構造体
#[repr(align(64))]
#[derive(Clone, Debug)]
pub struct VMState {
    /// 汎用レジスタ
    /// r0 : 0x000000 固定値レジスタ
    /// r1~r253 : 汎用レジスタ
    /// r254 : ゴミ箱レジスタ
    /// r255 : 0xFFFFFF 固定値レジスタ
    pub r: [u64; REGISTER_NUM],
    pub now_function_ptr: FunctionPtr,
    pub pc: usize,
    pub now_call_index: usize,
    pub mem: Arc<Memory>,
    /// 呼び出しスタック
    /// 現在の関数インデックスを保持する
    pub call_stack: Vec<usize>,

    /// 1 << 0 : 停止フラグ
    /// 1 << 1 : コールサイクルフラグ
    pub state_flag: u8,
}

impl VMState {
    pub fn new() -> Self {
        let mut r = [0u64; REGISTER_NUM];
        r[0] = 0;
        r[REGISTER_NUM - 1] = u64::MAX;
        VMState {
            r,
            mem: Arc::new(Memory::with_capacity(INIT_MEMORY_LEN)),
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
    // pub const IN_CALL: u8 = 0b0000_0010;
}