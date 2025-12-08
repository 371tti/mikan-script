use crate::vm::{instruction::operations::Operations, vm::VM};


/// レジスタ操作系
impl Operations {
    /// レジスタ間値コピー
    /// ol[0]: dst register index
    /// ol[1]: src register index
    #[inline(always)]
    pub fn mov(vm: &mut VM) {
        let ol = vm.next_operand();
        let dst = ol[0] as usize;
        let src = ol[1] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(src);
        }
        vm.next_step();
    }

    /// 即値ロード
    /// ol[0]: dst register index
    /// oh: immediate imm
    #[inline(always)]
    pub fn load_u64_immediate(vm: &mut VM) {
        let ol = vm.next_operand();
        let dst = ol[0] as usize;
        let imm = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = imm;
        }
        vm.next_step();
    }

    /// 交換
    /// ol[0]: reg_a
    /// ol[1]: reg_b
    #[inline(always)]
    pub fn swap(vm: &mut VM) {
        let ol = vm.next_operand();
        let a = ol[0] as usize;
        let b = ol[1] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let temp = *r.add(a);
            *r.add(a) = *r.add(b);
            *r.add(b) = temp;
        }
        vm.next_step();
    }
}