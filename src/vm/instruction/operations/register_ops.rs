use crate::vm::{instruction::operations::Operations, vm::VM};


/// レジスタ操作系
impl Operations {
    /// レジスタ間値コピー
    /// *dst = *src
    #[inline(always)]
    pub fn mov(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(src as usize);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 即値ロード
    /// *dst = imm
    #[inline(always)]
    pub fn load_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = imm;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 交換
    /// *reg_a, *reg_b = *reg_b, *reg_a
    #[inline(always)]
    pub fn swap(vm: &mut VM, reg_a: u64, reg_b: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let temp = *r.add(reg_a as usize);
            *r.add(reg_a as usize) = *r.add(reg_b as usize);
            *r.add(reg_b as usize) = temp;
        }
        vm.st.pc += 1; // fallthrough
    }
}