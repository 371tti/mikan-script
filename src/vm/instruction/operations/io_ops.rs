use crate::vm::{instruction::operations::Operations, vm::{VM, state_flag}};


/// IO操作
impl Operations {
    /// 整数の出力
    /// print_u64 *src
    #[inline(always)]
    pub fn print_u64(vm: &mut VM, src: u64, _: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            println!("{}", *r.add(src as usize));
        }
        vm.st.pc += 1; // fallthrough
    }

    /// allocate memory
    /// allocate *size + add_size, store id in *id_res_reg
    /// size_idr: [ size_reg(8bit) | id_res_reg(8bit) ]
    #[inline(always)]
    pub fn alloc(vm: &mut VM, size_idr: u64, add_size: u64) {
        let size_reg = ((size_idr >> 8) & 0xFF) as usize;
        let id_res_reg = (size_idr & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let size = (*r.add(size_reg)).wrapping_add(add_size) as usize;
            let id = vm.st.mem.alloc_heep(size);
            *r.add(id_res_reg) = id;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// reallocate memory
    /// reallocate *size for *id
    #[inline(always)]
    pub fn realloc(vm: &mut VM, size: u64, id: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let size = *r.add(size as usize) as usize;
            let id = *r.add(id as usize);
            vm.st.mem.realloc_heep(id, size);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// deallocate memory
    /// deallocate *id
    #[inline(always)]
    pub fn dealloc(vm: &mut VM, id: u64, _: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let id = *r.add(id as usize);
            vm.st.mem.dealloc_heep(id);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// read file to memory
    /// heep_id path_ptr path_size 

    /// プログラム終了
    /// exit with code *code_reg
    #[inline(always)]
    pub fn exit(vm: &mut VM, code_reg: u64, _: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let code = *r.add(code_reg as usize);
            vm.st.r[0] = code; // return code
            vm.st.state_flag |= state_flag::PAUSE;
        }
    }
}