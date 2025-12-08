use crate::vm::{instruction::operations::Operations, vm::VM};


/// 論理演算
impl Operations {
    /// 64bit論理積
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst & *src
    #[inline(always)]
    pub fn and_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) & *r.add(src);
        }
        vm.next_step();
    }

    /// 64bit論理積
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = *dst & imm
    #[inline(always)]
    pub fn and_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) & imm;
        }
        vm.next_step();
    }

    /// 64bit論理和
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst | *src
    #[inline(always)]
    pub fn or_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) | *r.add(src);
        }
        vm.next_step();
    }

    /// 64bit論理和
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = *dst | imm
    #[inline(always)]
    pub fn or_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) | imm;
        }
        vm.next_step();
    }

    /// 64bit排他的論理和
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst ^ *src
    #[inline(always)]
    pub fn xor_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) ^ *r.add(src);
        }
        vm.next_step();
    }

    /// 64bit排他的論理和
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = *dst ^ imm
    #[inline(always)]
    pub fn xor_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) ^ imm;
        }
        vm.next_step();
    }

    /// 64bit論理否定
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = !*src
    #[inline(always)]
    pub fn not_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = !*r.add(src);
        }
        vm.next_step();
    }

    /// 64bit論理左シフト
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst << *src
    #[inline(always)]
    pub fn shl_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) << (*r.add(src) as u32);
        }
        vm.next_step();
    }

    /// 64bit論理左シフト
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = *dst << imm
    #[inline(always)]
    pub fn shl_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) << (imm as u32);
        }
        vm.next_step();
    }

    /// 64bit算術左シフト
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst << *src
    #[inline(always)]
    pub fn shl_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = ((*r.add(dst) as i64) << (*r.add(src) as usize as u32)) as u64;
        }
        vm.next_step();
    }

    /// 64bit算術左シフト
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = *dst << imm
    #[inline(always)]
    pub fn shl_i64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = ((*r.add(dst) as i64) << (imm as u32)) as u64;
        }
        vm.next_step();
    }

    /// 64bit論理右シフト
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst >> *src
    #[inline(always)]
    pub fn shr_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) >> (*r.add(src) as u32);
        }
        vm.next_step();
    }

    /// 64bit論理右シフト
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = *dst >> imm
    #[inline(always)]
    pub fn shr_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = *r.add(dst) >> (imm as u32);
        }
        vm.next_step();
    }

    /// 64bit算術右シフト
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = *dst >> *src
    #[inline(always)]
    pub fn shr_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = ((*r.add(dst) as i64) >> (*r.add(src) as u32)) as u64;
        }
        vm.next_step();
    }

    /// 64bit算術右シフト
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = *dst >> imm
    #[inline(always)]
    pub fn shr_i64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = ((*r.add(dst) as i64) >> (imm as u32)) as u64;
        }
        vm.next_step();
    }

    /// 64bit論理左ローテート
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = rol(*dst, *src)
    #[inline(always)]
    pub fn rol_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst);
            let shift = (*r.add(src) & 0b111_1111) as u32;
            *r.add(dst) = value.rotate_left(shift);
        }
        vm.next_step();
    }

    /// 64bit論理左ローテート
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = rol(*dst, imm)
    #[inline(always)]
    pub fn rol_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst);
            let shift = (imm & 0b111_1111) as u32;
            *r.add(dst) = value.rotate_left(shift);
        }
        vm.next_step();
    }

    /// 64bit算術左ローテート
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = rol(*dst, *src)
    #[inline(always)]
    pub fn rol_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst) as i64;
            let shift = (*r.add(src) & 0b111_1111) as u32;
            *r.add(dst) = value.rotate_left(shift) as u64;
        }
        vm.next_step();
    }

    /// 64bit算術左ローテート
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = rol(*dst, imm)
    #[inline(always)]
    pub fn rol_i64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst) as i64;
            let shift = (imm & 0b111_1111) as u32;
            *r.add(dst) = value.rotate_left(shift) as u64;
        }
        vm.next_step();
    }

    /// 64bit論理右ローテート
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = ror(*dst, *src)
    #[inline(always)]
    pub fn ror_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst);
            let shift = (*r.add(src) & 0b111_1111) as u32;
            *r.add(dst) = value.rotate_right(shift);
        }
        vm.next_step();
    }

    /// 64bit論理右ローテート
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = ror(*dst, imm)
    #[inline(always)]
    pub fn ror_u64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst);
            let shift = (imm & 0b111_1111) as u32;
            *r.add(dst) = value.rotate_right(shift);
        }
        vm.next_step();
    }

    /// 64bit算術右ローテート
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = ror(*dst, *src)
    #[inline(always)]
    pub fn ror_i64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst) as i64;
            let shift = (*r.add(src) & 0b111_1111) as u32;
            *r.add(dst) = value.rotate_right(shift) as u64;
        }
        vm.next_step();
    }

    /// 64bit算術右ローテート
    /// 3 word instruction
    /// ol[0]: dst register index
    /// imm: immediate value (u64)
    /// *dst = ror(*dst, imm)
    #[inline(always)]
    pub fn ror_i64_immediate(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            
            let imm = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let value = *r.add(dst) as i64;
            let shift = (imm & 0b111_1111) as u32;
            *r.add(dst) = value.rotate_right(shift) as u64;
        }
        vm.next_step();
    }

    /// bitcount 1
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = count_ones(*src)
    #[inline(always)]
    pub fn count_ones_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = (*r.add(src)).count_ones() as u64;
        }
        vm.next_step();
    }

    /// bitcount 0
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = count_zeros(*src)
    #[inline(always)]
    pub fn count_zeros_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = (*r.add(src)).count_zeros() as u64;
        }
        vm.next_step();
    }

    /// trailing zeros
    /// 2 word instruction
    /// ol[0]: dst register index
    /// ol[1]: src register index
    /// *dst = trailing_zeros(*src)
    #[inline(always)]
    pub fn trailing_zeros_u64(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let dst = ol[0] as usize;
            let src = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst) = (*r.add(src)).trailing_zeros() as u64;
        }
        vm.next_step();
    }
}