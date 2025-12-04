use crate::vm::{instruction::operations::Operations, vm::VM};



/// 浮動小数点演算
impl Operations {
    /// 64bit浮動小数点加算
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst + *src
    // #[inline(always)]
    pub fn add_f64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let result =
                f64::from_bits(*r.add(dst)) + f64::from_bits(*r.add(src));
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点加算
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64, f64 bits)
    /// *dst = *dst + imm
    // #[inline(always)]
    pub fn add_f64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(dst)) + f64::from_bits(imm);
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点減算
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst - *src
    // #[inline(always)]
    pub fn sub_f64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let result =
                f64::from_bits(*r.add(dst)) - f64::from_bits(*r.add(src));
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点減算
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64, f64 bits)
    /// *dst = *dst - imm
    // #[inline(always)]
    pub fn sub_f64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(dst)) - f64::from_bits(imm);
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点乗算
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst * *src
    // #[inline(always)]
    pub fn mul_f64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let result =
                f64::from_bits(*r.add(dst)) * f64::from_bits(*r.add(src));
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点乗算
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64, f64 bits)
    /// *dst = *dst * imm
    // #[inline(always)]
    pub fn mul_f64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(dst)) * f64::from_bits(imm);
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点除算
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst / *src
    // #[inline(always)]
    pub fn div_f64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let result =
                f64::from_bits(*r.add(dst)) / f64::from_bits(*r.add(src));
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点除算
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64, f64 bits)
    /// *dst = *dst / imm
    // #[inline(always)]
    pub fn div_f64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(dst)) / f64::from_bits(imm);
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点絶対値
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = abs(*src)
    // #[inline(always)]
    pub fn abs_f64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(src)).abs();
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点符号反転
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = -(*src)
    // #[inline(always)]
    pub fn neg_f64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let result = -f64::from_bits(*r.add(src));
            *r.add(dst) = result.to_bits();
        }
        vm.next_step();
    }

    /// 64bit浮動小数点整数変換
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    // #[inline(always)]
    pub fn to_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = f64::from_bits(*r.add(src)) as i64 as u64;
        }
        vm.next_step();
    }
}
