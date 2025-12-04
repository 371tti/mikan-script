pub mod operations;
use crate::vm::VM;

pub type OpPtr = fn(&mut VM);

/// 命令の型定義
/// もしくはただのu64ラッパーとして機能
#[derive(Clone, Copy, Debug)]
#[repr(transparent)] // 変更: 内部の fn ポインタと同じレイアウトにする
pub struct Op(u64);

impl Op {
    #[inline(always)]
    fn or(self) -> u64 {
        self.0
    }

    #[inline(always)]
    fn oc(self) -> OpPtr {
        unsafe { core::mem::transmute::<Op, OpPtr>(self) }
    }

    #[inline(always)]
    fn ol(self) -> [u8; 8] {
        self.0.to_ne_bytes()
    }
}

impl From<OpPtr> for Op {
    fn from(f: OpPtr) -> Self {
        Op(unsafe { std::mem::transmute(f) })
    }
}

impl From<[u8; 8]> for Op {
    fn from(b: [u8; 8]) -> Self {
        Op(u64::from_ne_bytes(b))
    }
}

/// デコード済み命令
pub type Instruction = Op;

impl Instruction {
    /// 1ワード命令を作成します
    pub fn new_1w_op(op: OpPtr) -> Self {
        Instruction::from(op)
    }

    /// 2ワード命令を作成します
    pub fn new_2w_op(op: OpPtr, ol: [u8; 8]) -> [Self; 2] {
        [
            Instruction::from(op),
            Instruction::from(ol),
        ]
    }

    /// 3ワード命令を作成します
    pub fn new_3w_op(op: OpPtr, ol: [u8; 8], oh: [u8; 8]) -> [Self; 3] {
        [
            Instruction::from(op),
            Instruction::from(ol),
            Instruction::from(oh),
        ]
    }

    /// 関数ポインタとして取得します
    #[inline(always)]
    pub fn as_fn(&self) -> OpPtr {
        (*self).oc()
    }

    /// オペランドとして取得します
    #[inline(always)]
    pub fn as_ol(&self) -> [u8; 8] {
        (*self).ol()
    }

    /// 即値オペランドとして取得します
    #[inline(always)]
    pub fn as_imm(&self) -> u64 {
        (*self).or()
    }
}