use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

use crate::vm::{instruction::operations::Operations, vm::VM};


/// メモリ操作
impl Operations {
    /// u64ロード
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: result_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn load_u64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = *(addr as *const u64);
        }
        vm.next_step();
    }

    /// u32ロード
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: result_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn load_u32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = *(addr as *const u32) as u64;
        }
        vm.next_step();
    }

    /// u16ロード
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: result_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn load_u16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = *(addr as *const u16) as u64;
        }
        vm.next_step();
    }

    /// u8ロード
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: result_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn load_u8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = *(addr as *const u8) as u64;
        }
        vm.next_step();
    }

    /// u64ストア
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: src_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn store_u64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut u64) = *r.add(src_reg);
        }
        vm.next_step();
    }

    /// u32ストア
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: src_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn store_u32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut u32) = *r.add(src_reg) as u32;
        }
        vm.next_step();
    }

    /// u16ストア
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: src_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn store_u16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut u16) = *r.add(src_reg) as u16;
        }
        vm.next_step();
    }

    /// u8ストア
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: src_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn store_u8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut u8) = *r.add(src_reg) as u8;
        }
        vm.next_step();
    }

    /// atomic u64 ロード
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: result_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_load_u64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU64;
            *r.add(result_reg) = (*atomic_ptr).load(Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic u64 ストア
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: src_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_store_u64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            (*atomic_ptr).store(*r.add(src_reg), Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic u64 加算（結果を result_reg に格納）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_add_u64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg), Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic u64 減算（結果を result_reg に格納）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_sub_u64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg), Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic u32 ロード
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: result_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_load_u32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU32;
            *r.add(result_reg) = (*atomic_ptr).load(Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// atomic u32 ストア
    /// ol[0]: id_reg
    /// ol[1]: addr_reg
    /// ol[2]: src_reg
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_store_u32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            (*atomic_ptr).store(*r.add(src_reg) as u32, Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic u32 加算
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_add_u32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg) as u32, Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// atomic u32 減算
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_sub_u32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg) as u32, Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// atomic u16 ロード
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_load_u16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU16;
            *r.add(result_reg) = (*atomic_ptr).load(Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// atomic u16 ストア
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_store_u16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            (*atomic_ptr).store(*r.add(src_reg) as u16, Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic u16 加算
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_add_u16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg) as u16, Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// atomic u16 減算
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_sub_u16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg) as u16, Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// atomic u8 ロード
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_load_u8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU8;
            *r.add(result_reg) = (*atomic_ptr).load(Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// atomic u8 ストア
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_store_u8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            (*atomic_ptr).store(*r.add(src_reg) as u8, Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic u8 加算
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_add_u8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg) as u8, Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// atomic u8 減算
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_sub_u8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg) as u8, Ordering::SeqCst) as u64;
        }
        vm.next_step();
    }

    /// i8ロード（符号拡張）
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn load_i8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = (*(addr as *const i8) as i64) as u64;
        }
        vm.next_step();
    }

    /// i16ロード（符号拡張）
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn load_i16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = (*(addr as *const i16) as i64) as u64;
        }
        vm.next_step();
    }

    /// i32ロード（符号拡張）
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn load_i32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = (*(addr as *const i32) as i64) as u64;
        }
        vm.next_step();
    }

    /// i64ロード（符号拡張）
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn load_i64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = (*(addr as *const i64) as i64) as u64;
        }
        vm.next_step();
    }

    /// i8ストア（符号拡張）
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn store_i8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut i8) = *r.add(src_reg) as i8;
        }
        vm.next_step();
    }

    /// i16ストア（符号拡張）
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn store_i16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut i16) = *r.add(src_reg) as i16;
        }
        vm.next_step();
    }

    /// i32ストア（符号拡張）
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn store_i32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut i32) = *r.add(src_reg) as i32;
        }
        vm.next_step();
    }

    /// i64ストア（符号拡張）
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn store_i64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut i64) = *r.add(src_reg) as i64;
        }
        vm.next_step();
    }

    /// atomic i8 ロード（符号拡張）
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_load_i8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU8;
            *r.add(result_reg) = ((*atomic_ptr).load(Ordering::SeqCst) as i8) as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i16 ロード（符号拡張）
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_load_i16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU16;
            *r.add(result_reg) = ((*atomic_ptr).load(Ordering::SeqCst) as i16) as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i32 ロード（符号拡張）
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_load_i32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU32;
            *r.add(result_reg) = ((*atomic_ptr).load(Ordering::SeqCst) as i32) as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i64 ロード（符号拡張）
    /// ol: [ id_reg | addr_reg | result_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_load_i64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let result_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU64;
            *r.add(result_reg) = ((*atomic_ptr).load(Ordering::SeqCst) as i64) as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i8 ストア（符号拡張）
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_store_i8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            (*atomic_ptr).store(*r.add(src_reg) as i8 as u8, Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic i16 ストア（符号拡張）
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_store_i16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            (*atomic_ptr).store(*r.add(src_reg) as i16 as u16, Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic i32 ストア（符号拡張）
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_store_i32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            (*atomic_ptr).store(*r.add(src_reg) as i32 as u32, Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic i64 ストア（符号拡張）
    /// ol: [ id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_store_i64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let id_reg = ol[0] as usize;
        let addr_reg = ol[1] as usize;
        let src_reg = ol[2] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            (*atomic_ptr).store(*r.add(src_reg) as i64 as u64, Ordering::SeqCst);
        }
        vm.next_step();
    }

    /// atomic i8 加算（符号拡張）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_add_i8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg) as i8 as u8, Ordering::SeqCst)
                as i8 as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i16 加算（符号拡張）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_add_i16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg) as i16 as u16, Ordering::SeqCst)
                as i16 as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i32 加算（符号拡張）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_add_i32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg) as i32 as u32, Ordering::SeqCst)
                as i32 as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i64 加算（符号拡張）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_add_i64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg) as i64 as u64, Ordering::SeqCst)
                as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i8 減算（符号拡張）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_sub_i8(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg) as i8 as u8, Ordering::SeqCst)
                as i8 as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i16 減算（符号拡張）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_sub_i16(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg) as i16 as u16, Ordering::SeqCst)
                as i16 as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i32 減算（符号拡張）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_sub_i32(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg) as i32 as u32, Ordering::SeqCst)
                as i32 as i64 as u64;
        }
        vm.next_step();
    }

    /// atomic i64 減算（符号拡張）
    /// ol: [ result_reg | id_reg | addr_reg | src_reg ]
    /// oh: immediate offset
    #[inline(always)]
    pub fn atomic_sub_i64(vm: &mut VM) {
        let ol = vm.next_operand();
        let offset = vm.next_operand_imm();
        let result_reg = ol[0] as usize;
        let id_reg = ol[1] as usize;
        let addr_reg = ol[2] as usize;
        let src_reg = ol[3] as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg) as i64 as u64, Ordering::SeqCst)
                as i64 as u64;
        }
        vm.next_step();
    }
}
