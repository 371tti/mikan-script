use crate::vm::{instruction::operations::Operations, vm::VM};



/// 浮動小数点演算
impl Operations {
    /// 64bit浮動小数点加算
    /// *dst = *dst + *src
    #[inline(always)]
    pub fn add_f64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result =
                f64::from_bits(*r.add(dst as usize)) + f64::from_bits(*r.add(src as usize));
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点加算
    /// *dst = *dst + imm
    #[inline(always)]
    pub fn add_f64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(dst as usize)) + f64::from_bits(imm);
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点減算
    /// *dst = *dst - *src
    #[inline(always)]
    pub fn sub_f64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result =
                f64::from_bits(*r.add(dst as usize)) - f64::from_bits(*r.add(src as usize));
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点減算
    /// *dst = *dst - imm
    #[inline(always)]
    pub fn sub_f64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(dst as usize)) - f64::from_bits(imm);
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点乗算
    /// *dst = *dst * *src
    #[inline(always)]
    pub fn mul_f64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result =
                f64::from_bits(*r.add(dst as usize)) * f64::from_bits(*r.add(src as usize));
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点乗算
    /// *dst = *dst * imm
    #[inline(always)]
    pub fn mul_f64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(dst as usize)) * f64::from_bits(imm);
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点除算
    /// *dst = *dst / *src
    #[inline(always)]
    pub fn div_f64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result =
                f64::from_bits(*r.add(dst as usize)) / f64::from_bits(*r.add(src as usize));
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点除算
    /// *dst = *dst / imm
    #[inline(always)]
    pub fn div_f64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(dst as usize)) / f64::from_bits(imm);
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点絶対値
    /// *dst = abs(*src)
    #[inline(always)]
    pub fn abs_f64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result = f64::from_bits(*r.add(src as usize)).abs();
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点符号反転
    /// *dst = -(*src)
    #[inline(always)]
    pub fn neg_f64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let result = -f64::from_bits(*r.add(src as usize));
            *r.add(dst as usize) = result.to_bits();
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit浮動小数点整数変換
    #[inline(always)]
    pub fn to_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = f64::from_bits(*r.add(src as usize)) as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }
}
