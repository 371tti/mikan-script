use crate::vm::{instruction::operations::Operations, vm::VM};


/// 整数演算
impl Operations {
    /// 64bit符号なし整数加算
    /// *dst = *dst + *src
    #[inline(always)]
    pub fn add_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = (*r.add(dst)).wrapping_add(*r.add(src));
        }
        vm.next_step();
    }

    /// 64bit符号なし整数加算
    /// *dst = *dst + imm
    #[inline(always)]
    pub fn add_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            *r.add(dst) = (*r.add(dst)).wrapping_add(imm);
        }
        vm.next_step();
    }

    /// 64bit符号付き整数加算
    /// *dst = *dst + *src
    #[inline(always)]
    pub fn add_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = ((*r.add(dst) as i64).wrapping_add(*r.add(src) as i64)) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号付き整数加算
    /// *dst = *dst + imm
    #[inline(always)]
    pub fn add_i64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            *r.add(dst) = ((*r.add(dst) as i64).wrapping_add(imm as i64)) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号なし整数減算
    /// *dst = *dst - *src
    #[inline(always)]
    pub fn sub_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = (*r.add(dst)).wrapping_sub(*r.add(src));
        }
        vm.next_step();
    }

    /// 64bit符号なし整数減算
    /// *dst = *dst - imm
    #[inline(always)]
    pub fn sub_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            *r.add(dst) = (*r.add(dst)).wrapping_sub(imm);
        }
        vm.next_step();
    }

    /// 64bit符号付き整数減算
    /// *dst = *dst - *src
    #[inline(always)]
    pub fn sub_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = ((*r.add(dst) as i64).wrapping_sub(*r.add(src) as i64)) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号付き整数減算
    /// *dst = *dst - imm
    #[inline(always)]
    pub fn sub_i64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            *r.add(dst) = ((*r.add(dst) as i64).wrapping_sub(imm as i64)) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号なし整数乗算
    /// *dst = *dst * *src
    #[inline(always)]
    pub fn mul_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = (*r.add(dst)).wrapping_mul(*r.add(src));
        }
        vm.next_step();
    }

    /// 64bit符号なし整数乗算
    /// *dst = *dst * imm
    #[inline(always)]
    pub fn mul_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            *r.add(dst) = (*r.add(dst)).wrapping_mul(imm);
        }
        vm.next_step();
    }

    /// 64bit符号付き整数乗算
    /// *dst = *dst * *src
    #[inline(always)]
    pub fn mul_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = ((*r.add(dst) as i64).wrapping_mul(*r.add(src) as i64)) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号付き整数乗算
    /// *dst = *dst * imm
    #[inline(always)]
    pub fn mul_i64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            *r.add(dst) = ((*r.add(dst) as i64).wrapping_mul(imm as i64)) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号なし整数除算
    /// *dst = *dst / *src
    #[inline(always)]
    pub fn div_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = *r.add(dst) / *r.add(src);
        }
        vm.next_step();
    }

    /// 64bit符号なし整数除算
    /// *dst = *dst / imm
    #[inline(always)]
    pub fn div_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            *r.add(dst) = (*r.add(dst)) / imm;
        }
        vm.next_step();
    }

    /// 64bit符号付き整数除算
    /// *dst = *dst / *src
    #[inline(always)]
    pub fn div_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = (*r.add(dst) as i64 / *r.add(src) as i64) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号付き整数除算
    /// *dst = *dst / imm
    #[inline(always)]
    pub fn div_i64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            *r.add(dst) = (*r.add(dst) as i64 / imm as i64) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号付き整数絶対値
    /// *dst = abs(*src)
    #[inline(always)]
    pub fn abs(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = (*r.add(src) as i64).wrapping_abs() as u64;
        }
        vm.next_step();
    }

    /// 64bit符号付き整数剰余
    /// *dst = *dst % *src
    #[inline(always)]
    pub fn mod_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = ((*r.add(dst) as i64).wrapping_rem(*r.add(src) as i64)) as u64;
        }
        vm.next_step();
    }

    /// 64bit符号付き 符号反転
    /// *dst = -(*src)
    #[inline(always)]
    pub fn neg_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = (-(*r.add(src) as i64)).wrapping_abs() as u64;
        }
        vm.next_step();
    }

    /// 64bit符号なし整数浮動小数点数変換
    /// *dst = (*src as f64)
    #[inline(always)]
    pub fn u64_to_f64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = f64::to_bits(*r.add(src) as u64 as f64);
        }
        vm.next_step();
    }

    /// 64bit符号あり整数浮動小数点数変換
    /// *dst = (*src as i64) as f64
    #[inline(always)]
    pub fn i64_to_f64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let r = vm.st.r.as_mut_ptr();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            *r.add(dst) = f64::to_bits(*r.add(src) as i64 as f64);
        }
        vm.next_step();
    }
}