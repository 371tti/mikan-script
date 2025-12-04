use crate::vm::{instruction::operations::Operations, vm::VM};


/// 制御系
impl Operations {
    /// ジャンプ
    /// ol[0]: dst register index
    /// oh: immediate offset
    #[inline(always)]
    pub fn jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let dst = ol[0] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            vm.st.pc = (*r.add(dst)).wrapping_add(offset) as usize;
        }
    }

    /// 等しい場合のジャンプ
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn eq_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) == *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// 等しくない場合のジャンプ
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn neq_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) != *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// より小さい場合のジャンプ (符号なし)
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn lt_u64_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) < *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// より小さいか等しい場合のジャンプ (符号なし)
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn lte_u64_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) <= *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// より小さい場合のジャンプ (符号付き)
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn lt_i64_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) < (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// より小さいか等しい場合のジャンプ (符号付き)
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn lte_i64_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) <= (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// より大きい場合のジャンプ (符号なし)
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn gt_u64_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) > *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// より大きいか等しい場合のジャンプ (符号なし)
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn gte_u64_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) >= *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// より大きい場合のジャンプ (符号付き)
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn gt_i64_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) > (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// より大きいか等しい場合のジャンプ (符号付き)
    /// ol[0]: addr_reg
    /// ol[1]: a
    /// ol[2]: b
    /// oh: immediate offset
    #[inline(always)]
    pub fn gte_i64_jump(vm: &mut VM) {
        let ol = vm.next_operand();
        let addr_reg = ol[0] as usize;
        let a = ol[1] as usize;
        let b = ol[2] as usize;
        let offset = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) >= (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.next_step();
            }
        }
    }

    /// 関数呼び出し
    /// call func_index
    /// set pc ( 普通は関数先頭アドレスで0 )
    #[inline(always)]
    pub fn call(vm: &mut VM) {
        let func_index = vm.next_operand_imm();
        let pc = vm.next_operand_imm();
        vm.st.call_stack.push(vm.st.pc);
        vm.st.call_stack.push(vm.st.now_call_index);
        vm.st.pc = pc as usize;
        vm.st.now_call_index = func_index as usize; 
        vm.st.now_function_ptr = vm.function_table[vm.st.now_call_index];
    }

    /// 関数リターン
    /// ret
    #[inline(always)]
    pub fn ret(vm: &mut VM) {
        vm.st.now_call_index = vm.st.call_stack.pop().expect("Call stack underflow on return");
        vm.st.pc = vm.st.call_stack.pop().unwrap();
        vm.next_step();
        vm.st.now_function_ptr = vm.function_table[vm.st.now_call_index];
    }
}