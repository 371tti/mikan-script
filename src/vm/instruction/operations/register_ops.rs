use crate::vm::{instruction::operations::Operations, vm::VM};


/// レジスタ操作系
impl Operations {
    /// レジスタ間値コピー
    /// ol[0]: dst register index
    /// ol[1]: src register index
    #[inline(always)]
    pub fn mov(vm: &mut VM) {
        let ol = vm.next_operand();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(ol[0] as usize) = *r.add(ol[1] as usize);
        }
        vm.next_step();
    }

    /// 即値ロード
    /// ol[0]: dst register index
    /// oh: immediate imm
    #[inline(always)]
    pub fn load_u64_immediate(vm: &mut VM) {
        let ol = vm.next_operand();
        let imm = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(ol[0] as usize) = imm;
        }
        vm.next_step();
    }

    /// 交換
    /// ol[0]: reg_a
    /// ol[1]: reg_b
    #[inline(always)]
    pub fn swap(vm: &mut VM) {
        let ol = vm.next_operand();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let a = ol[0] as usize;
            let b = ol[1] as usize;
            let temp = *r.add(a);
            *r.add(a) = *r.add(b);
            *r.add(b) = temp;
        }
        vm.next_step();
    }
}