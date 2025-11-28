pub mod operations;

pub type Operations = operations::Operations;

use crate::vm::VM;

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




