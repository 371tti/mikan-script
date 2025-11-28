use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

use crate::vm::{instruction::operations::Operations, vm::VM};


/// メモリ操作
impl Operations {
    /// u64ロード
    /// *result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn load_u64(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = *(addr as *const u64);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// u32ロード
    /// *result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn load_u32(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = *(addr as *const u32) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// u16ロード
    /// *result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn load_u16(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = *(addr as *const u16) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// u8ロード
    /// *result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn load_u8(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = *(addr as *const u8) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// u64ストア
    /// * (heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn store_u64(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut u64) = *r.add(src_reg);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// u32ストア
    /// * (heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn store_u32(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut u32) = *r.add(src_reg) as u32;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// u16ストア
    /// * (heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn store_u16(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut u16) = *r.add(src_reg) as u16;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// u8ストア
    /// * (heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn store_u8(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut u8) = *r.add(src_reg) as u8;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u64 ロード
    /// *result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset)
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_load_u64(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU64;
            *r.add(result_reg) = (*atomic_ptr).load(Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u64 ストア
    /// atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_store_u64(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            (*atomic_ptr).store(*r.add(src_reg), Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u64 加算
    /// *result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_add_u64(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            *r.add(result_reg) = (*atomic_ptr).fetch_add(*r.add(src_reg), Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u64 減算
    /// *result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_sub_u64(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            *r.add(result_reg) = (*atomic_ptr).fetch_sub(*r.add(src_reg), Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u32 ロード
    /// *result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset)
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_load_u32(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU32;
            *r.add(result_reg) = (*atomic_ptr).load(Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u32 ストア
    /// atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_store_u32(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            (*atomic_ptr).store(*r.add(src_reg) as u32, Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u32 加算
    /// *result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_add_u32(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            *r.add(result_reg) =
                (*atomic_ptr).fetch_add(*r.add(src_reg) as u32, Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u32 減算
    /// *result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_sub_u32(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            *r.add(result_reg) =
                (*atomic_ptr).fetch_sub(*r.add(src_reg) as u32, Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u16 ロード
    /// *result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset)
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_load_u16(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU16;
            *r.add(result_reg) = (*atomic_ptr).load(Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u16 ストア
    /// atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_store_u16(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            (*atomic_ptr).store(*r.add(src_reg) as u16, Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u16 加算
    /// *result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_add_u16(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            *r.add(result_reg) =
                (*atomic_ptr).fetch_add(*r.add(src_reg) as u16, Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u16 減算
    /// *result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_sub_u16(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            *r.add(result_reg) =
                (*atomic_ptr).fetch_sub(*r.add(src_reg) as u16, Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u8 ロード
    /// *result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset)
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_load_u8(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU8;
            *r.add(result_reg) = (*atomic_ptr).load(Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u8 ストア
    /// atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_store_u8(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            (*atomic_ptr).store(*r.add(src_reg) as u8, Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u8 加算
    /// *result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_add_u8(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            *r.add(result_reg) =
                (*atomic_ptr).fetch_add(*r.add(src_reg) as u8, Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic u8 減算
    /// *result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_sub_u8(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            *r.add(result_reg) =
                (*atomic_ptr).fetch_sub(*r.add(src_reg) as u8, Ordering::SeqCst) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// i8ロード（符号拡張）
    /// *result_reg = (*(heep_ptr(*id_reg) + *addr_reg + offset) as i8) as i64
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn load_i8(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = (*(addr as *const i8) as i64) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// i16ロード（符号拡張）
    /// *result_reg = (*(heep_ptr(*id_reg) + *addr_reg + offset) as i16) as i64
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn load_i16(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = (*(addr as *const i16) as i64) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// i32ロード（符号拡張）
    /// *result_reg = (*(heep_ptr(*id_reg) + *addr_reg + offset) as i32) as i64
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn load_i32(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = (*(addr as *const i32) as i64) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// i64ロード（符号拡張）
    /// *result_reg = (*(heep_ptr(*id_reg) + *addr_reg + offset) as i64) as u64
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn load_i64(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *r.add(result_reg) = (*(addr as *const i64) as i64) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// i8ストア（符号拡張）
    /// * (heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg as i8
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn store_i8(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut i8) = *r.add(src_reg) as i8;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// i16ストア（符号拡張）
    /// * (heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg as i16
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn store_i16(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut i16) = *r.add(src_reg) as i16;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// i32ストア（符号拡張）
    /// * (heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg as i32
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn store_i32(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut i32) = *r.add(src_reg) as i32;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// i64ストア（符号拡張）
    /// * (heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg as i64
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn store_i64(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            *(addr as *mut i64) = *r.add(src_reg) as i64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i8 ロード（符号拡張）
    /// *result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset) as i8 as i64
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_load_i8(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU8;
            *r.add(result_reg) = ((*atomic_ptr).load(Ordering::SeqCst) as i8) as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i16 ロード（符号拡張）
    /// *result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset) as i16 as i64
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_load_i16(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU16;
            *r.add(result_reg) = ((*atomic_ptr).load(Ordering::SeqCst) as i16) as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i32 ロード（符号拡張）
    /// *result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset) as i32 as i64
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_load_i32(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU32;
            *r.add(result_reg) = ((*atomic_ptr).load(Ordering::SeqCst) as i32) as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i64 ロード（符号拡張）
    /// *result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset) as i64 as u64
    /// idr_ptr_res: [ id_reg(8bit) | addr_reg(8bit) | result_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_load_i64(vm: &mut VM, idr_ptr_res: u64, offset: u64) {
        let id_reg = ((idr_ptr_res >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_res >> 8) & 0xFF) as usize;
        let result_reg = (idr_ptr_res & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *const AtomicU64;
            *r.add(result_reg) = ((*atomic_ptr).load(Ordering::SeqCst) as i64) as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i8 ストア（符号拡張）
    /// atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_store_i8(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            (*atomic_ptr).store(*r.add(src_reg) as i8 as u8, Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i16 ストア（符号拡張）
    /// atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_store_i16(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            (*atomic_ptr).store(*r.add(src_reg) as i16 as u16, Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i32 ストア（符号拡張）
    /// atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_store_i32(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            (*atomic_ptr).store(*r.add(src_reg) as i32 as u32, Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i64 ストア（符号拡張）
    /// atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)
    /// idr_ptr_src: [ id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_store_i64(vm: &mut VM, idr_ptr_src: u64, offset: u64) {
        let id_reg = ((idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            (*atomic_ptr).store(*r.add(src_reg) as i64 as u64, Ordering::SeqCst);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i8 加算（符号拡張）
    /// *result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg) as i8 as i64
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_add_i8(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            *r.add(result_reg) = (*atomic_ptr)
                .fetch_add(*r.add(src_reg) as i8 as u8, Ordering::SeqCst)
                as i8 as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i16 加算（符号拡張）
    /// *result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg) as i16 as i64
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_add_i16(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            *r.add(result_reg) = (*atomic_ptr)
                .fetch_add(*r.add(src_reg) as i16 as u16, Ordering::SeqCst)
                as i16 as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i32 加算（符号拡張）
    /// *result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg) as i32 as i64
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_add_i32(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            *r.add(result_reg) = (*atomic_ptr)
                .fetch_add(*r.add(src_reg) as i32 as u32, Ordering::SeqCst)
                as i32 as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i64 加算（符号拡張）
    /// *result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg) as i64 as u64
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_add_i64(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            *r.add(result_reg) = (*atomic_ptr)
                .fetch_add(*r.add(src_reg) as i64 as u64, Ordering::SeqCst)
                as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i8 減算（符号拡張）
    /// *result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg) as i8 as i64
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_sub_i8(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU8;
            *r.add(result_reg) = (*atomic_ptr)
                .fetch_sub(*r.add(src_reg) as i8 as u8, Ordering::SeqCst)
                as i8 as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i16 減算（符号拡張）
    /// *result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg) as i16 as i64
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_sub_i16(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU16;
            *r.add(result_reg) = (*atomic_ptr)
                .fetch_sub(*r.add(src_reg) as i16 as u16, Ordering::SeqCst)
                as i16 as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i32 減算（符号拡張）
    /// *result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg) as i32 as i64
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_sub_i32(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU32;
            *r.add(result_reg) = (*atomic_ptr)
                .fetch_sub(*r.add(src_reg) as i32 as u32, Ordering::SeqCst)
                as i32 as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// atomic i64 減算（符号拡張）
    /// *result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg) as i64 as u64
    /// idr_ptr_src: [ result_reg(8bit) | id_reg(8bit) | addr_reg(8bit) | src_reg(8bit) ]
    #[inline(always)]
    pub fn atomic_sub_i64(vm: &mut VM, res_idr_ptr_src: u64, offset: u64) {
        let result_reg = ((res_idr_ptr_src >> 24) & 0xFF) as usize;
        let id_reg = ((res_idr_ptr_src >> 16) & 0xFF) as usize;
        let addr_reg = ((res_idr_ptr_src >> 8) & 0xFF) as usize;
        let src_reg = (res_idr_ptr_src & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let heep_ptr = vm.st.mem.head_ptr(*r.add(id_reg));
            let addr = ((*r.add(addr_reg)).wrapping_add(offset) as usize).wrapping_add(heep_ptr);
            let atomic_ptr = addr as *mut AtomicU64;
            *r.add(result_reg) = (*atomic_ptr)
                .fetch_sub(*r.add(src_reg) as i64 as u64, Ordering::SeqCst)
                as i64 as u64;
        }
        vm.st.pc += 1; // fallthrough
    }
}
