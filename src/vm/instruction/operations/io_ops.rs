use crate::vm::{instruction::operations::Operations, vm::VM};


/// IO操作
impl Operations {
    /// 整数の出力
    /// 2 word instruction
    /// ol[0]: src register index
    // #[inline(always)]
    pub fn print_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let src = ol[0] as usize;
            let r = vm.st.r.as_mut_ptr();
            println!("{}", *r.add(src));
        }
        vm.next_step();
    }

    /// allocate memory
    /// allocate *size + add_size, store id in *id_res_reg
    /// ol[0]: size reg
    /// ol[1]: id res reg
    /// oh: immediate add_size
    // #[inline(always)]
    pub fn alloc(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let size_reg = ol[0] as usize;
            let id_res_reg = ol[1] as usize;
            let add_size = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let size = (*r.add(size_reg)).wrapping_add(add_size) as usize;
            let id = vm.st.mem.alloc_heep(size);
            *r.add(id_res_reg) = id;
        }
        vm.next_step();
    }

    /// reallocate memory
    /// 2 word instruction
    /// ol[0]: size reg
    /// ol[1]: id reg
    // #[inline(always)]
    pub fn realloc(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let size_reg = ol[0] as usize;
            let id_reg = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let size = *r.add(size_reg) as usize;
            let id = *r.add(id_reg);
            vm.st.mem.realloc_heep(id, size);
        }
        vm.next_step();
    }

    /// deallocate memory
    /// 2 word instruction
    /// ol[0]: id reg
    // #[inline(always)]
    pub fn dealloc(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let id_reg = ol[0] as usize;
            let r = vm.st.r.as_mut_ptr();
            let id = *r.add(id_reg);
            vm.st.mem.dealloc_heep(id);
        }
        vm.next_step();
    }
}