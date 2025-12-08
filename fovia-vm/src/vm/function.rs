use std::{ops::Deref, pin::Pin};

use crate::vm::instruction::Instruction;

/// 関数保持
#[derive(Clone, Debug)]
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

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    #[inline(always)]
    pub fn fast_get(&self, index: usize) -> Instruction {
        unsafe {
            *self.instructions.get_unchecked(index)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FunctionPtr(pub *const Function);

impl Deref for FunctionPtr {
    type Target = Function;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.0 }
    }
}

unsafe impl Send for FunctionPtr {}
unsafe impl Sync for FunctionPtr {}