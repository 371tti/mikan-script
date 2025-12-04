pub mod operations;

pub type Operations = operations::Operations;

use crate::vm::VM;

/// 命令の型定義
pub type Op = fn(&mut VM);

/// デコード済み命令
#[derive(Clone)]
#[repr(align(8))]
pub struct Instruction {
    /// 以下を含む可能性があります
    /// - 命令ポインタ
    /// - オペランド(8バイト)
    /// - JIT済み関数ポインタ
    pub code: [u8; 8],
}

impl Instruction {
    /// 1ワード命令を作成します
    pub fn new_1w_op(op: Op) -> Self {
        let ptr = op as *const ();
        let addr = ptr as usize as u64;
        Instruction {
            code: addr.to_le_bytes(),
        }
    }

    /// 2ワード命令を作成します
    pub fn new_2w_op(op: Op, ol: [u8; 8]) -> [Self; 2] {
        let ptr = op as *const ();
        let addr = ptr as usize as u64;
        [
            Instruction {
                code: addr.to_le_bytes(),
            },
            Instruction { code: ol },
        ]
    }

    /// 3ワード命令を作成します
    pub fn new_3w_op(op: Op, ol: [u8; 8], oh: [u8; 8]) -> [Self; 3] {
        let ptr = op as *const ();
        let addr = ptr as usize as u64;
        [
            Instruction {
                code: addr.to_le_bytes(),
            },
            Instruction { code: ol },
            Instruction { code: oh },
        ]
    }

    /// 関数ポインタとして取得します
    #[inline(always)]
    pub fn as_fn(&self) -> Op {
        let ptr = u64::from_le_bytes(self.code) as usize as *const ();
        let func: Op = unsafe { std::mem::transmute(ptr) };
        func
    }

    /// オペランドとして取得します
    #[inline(always)]
    pub fn as_ol(&self) -> &[u8; 8] {
        &self.code
    }

    /// 即値オペランドとして取得します
    #[inline(always)]
    pub fn as_imm(&self) -> u64 {
        u64::from_ne_bytes(self.code)
    }
}