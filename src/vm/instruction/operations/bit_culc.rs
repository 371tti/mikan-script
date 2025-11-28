use crate::vm::{instruction::operations::Operations, vm::VM};


/// 論理演算
impl Operations {
    /// 64bit論理積
    /// *dst = *dst & *src
    #[inline(always)]
    pub fn and_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) & *r.add(src as usize);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理積
    /// *dst = *dst & imm
    #[inline(always)]
    pub fn and_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) & imm;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理和
    /// *dst = *dst | *src
    #[inline(always)]
    pub fn or_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) | *r.add(src as usize);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理和
    /// *dst = *dst | imm
    #[inline(always)]
    pub fn or_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) | imm;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit排他的論理和
    /// *dst = *dst ^ *src
    #[inline(always)]
    pub fn xor_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) ^ *r.add(src as usize);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit排他的論理和
    /// *dst = *dst ^ imm
    #[inline(always)]
    pub fn xor_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) ^ imm;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理否定
    /// *dst = !*src
    #[inline(always)]
    pub fn not_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = !*r.add(src as usize);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理左シフト
    /// *dst = *dst << *src
    #[inline(always)]
    pub fn shl_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) << (*r.add(src as usize) as u32);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理左シフト
    /// *dst = *dst << imm
    #[inline(always)]
    pub fn shl_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) << (imm as u32);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit算術左シフト
    /// *dst = *dst << *src
    #[inline(always)]
    pub fn shl_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) =
                ((*r.add(dst as usize) as i64) << (*r.add(src as usize) as u32)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit算術左シフト
    /// *dst = *dst << imm
    #[inline(always)]
    pub fn shl_i64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = ((*r.add(dst as usize) as i64) << (imm as u32)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理右シフト
    /// *dst = *dst >> *src
    #[inline(always)]
    pub fn shr_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) >> (*r.add(src as usize) as u32);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理右シフト
    /// *dst = *dst >> imm
    #[inline(always)]
    pub fn shr_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) >> (imm as u32);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit算術右シフト
    /// *dst = *dst >> *src
    #[inline(always)]
    pub fn shr_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) =
                ((*r.add(dst as usize) as i64) >> (*r.add(src as usize) as u32)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit算術右シフト
    /// *dst = *dst >> imm
    #[inline(always)]
    pub fn shr_i64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = ((*r.add(dst as usize) as i64) >> (imm as u32)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理左ローテート
    /// *dst = rol(*dst, *src)
    #[inline(always)]
    pub fn rol_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst as usize);
            let shift = (*r.add(src as usize) & 0b111_1111) as u32;
            *r.add(dst as usize) = value.rotate_left(shift);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理左ローテート
    /// *dst = rol(*dst, imm)
    #[inline(always)]
    pub fn rol_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst as usize);
            let shift = (imm & 0b111_1111) as u32;
            *r.add(dst as usize) = value.rotate_left(shift);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit算術左ローテート
    /// *dst = rol(*dst, *src)
    #[inline(always)]
    pub fn rol_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst as usize) as i64;
            let shift = (*r.add(src as usize) & 0b111_1111) as u32;
            *r.add(dst as usize) = value.rotate_left(shift) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit算術左ローテート
    /// *dst = rol(*dst, imm)
    #[inline(always)]
    pub fn rol_i64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst as usize) as i64;
            let shift = (imm & 0b111_1111) as u32;
            *r.add(dst as usize) = value.rotate_left(shift) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理右ローテート
    /// *dst = ror(*dst, *src)
    #[inline(always)]
    pub fn ror_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst as usize);
            let shift = (*r.add(src as usize) & 0b111_1111) as u32;
            *r.add(dst as usize) = value.rotate_right(shift);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit論理右ローテート
    /// *dst = ror(*dst, imm)
    #[inline(always)]
    pub fn ror_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst as usize);
            let shift = (imm & 0b111_1111) as u32;
            *r.add(dst as usize) = value.rotate_right(shift);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit算術右ローテート
    /// *dst = ror(*dst, *src)
    #[inline(always)]
    pub fn ror_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst as usize) as i64;
            let shift = (*r.add(src as usize) & 0b111_1111) as u32;
            *r.add(dst as usize) = value.rotate_right(shift) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit算術右ローテート
    /// *dst = ror(*dst, imm)
    #[inline(always)]
    pub fn ror_i64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst as usize) as i64;
            let shift = (imm & 0b111_1111) as u32;
            *r.add(dst as usize) = value.rotate_right(shift) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// bitcount 1
    /// *dst = count_ones(*src)
    #[inline(always)]
    pub fn count_ones_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(src as usize)).count_ones() as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// bitcount 0
    /// *dst = count_zeros(*src)
    #[inline(always)]
    pub fn count_zeros_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(src as usize)).count_zeros() as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// trailing zeros
    /// *dst = trailing_zeros(*src)
    #[inline(always)]
    pub fn trailing_zeros_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(src as usize)).trailing_zeros() as u64;
        }
        vm.st.pc += 1; // fallthrough
    }
}