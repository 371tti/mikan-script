use crate::vm::{instruction::operations::Operations, vm::VM};


/// 制御系
impl Operations {
    /// ジャンプ
    /// pc = *dst + offset
    #[inline(always)]
    pub fn jump(vm: &mut VM, dst: u64, offset: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            vm.st.pc = (*r.add(dst as usize)).wrapping_add(offset) as usize;
        }
    }

    /// 等しい場合のジャンプ
    /// if *a == *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn eq_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) == *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// 等しくない場合のジャンプ
    /// if *a != *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn neq_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) != *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より小さい場合のジャンプ (符号なし)
    /// if *a < *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn lt_u64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) < *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より小さいか等しい場合のジャンプ (符号なし)
    /// if *a <= *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn lte_u64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) <= *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より小さい場合のジャンプ (符号付き)
    /// if *a < *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn lt_i64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) < (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より小さいか等しい場合のジャンプ (符号付き)
    /// if *a <= *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn lte_i64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) <= (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より大きい場合のジャンプ (符号なし)
    /// if *a > *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn gt_u64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) > *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より大きいか等しい場合のジャンプ (符号なし)
    /// if *a >= *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn gte_u64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) >= *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より大きい場合のジャンプ (符号付き)
    /// if *a > *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn gt_i64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) > (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より大きいか等しい場合のジャンプ (符号付き)
    /// if *a >= *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn gte_i64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) >= (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// 関数呼び出し
    /// call func_index
    /// set pc ( 普通は関数先頭アドレスで0 )
    #[inline(always)]
    pub fn call(vm: &mut VM, func_index: u64, pc: u64) {
        vm.st.call_stack.push(vm.st.pc);
        vm.st.call_stack.push(vm.st.now_call_index);
        vm.st.pc = pc as usize;
        vm.st.now_call_index = func_index as usize; 
        vm.st.now_function_ptr = vm.function_table[vm.st.now_call_index];
    }

    /// 関数リターン
    /// ret
    #[inline(always)]
    pub fn ret(vm: &mut VM, _: u64, _: u64) {
        vm.st.now_call_index = vm.st.call_stack.pop().expect("Call stack underflow on return");
        vm.st.pc = vm.st.call_stack.pop().unwrap() + 1;
        vm.st.now_function_ptr = vm.function_table[vm.st.now_call_index];
    }
}