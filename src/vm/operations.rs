use std::sync::atomic::{AtomicU8, AtomicU16, AtomicU32, AtomicU64, Ordering};

use crate::vm::{VM, vm::state_flag};

pub struct Operations;

/// 命令の型定義
pub type Op = fn(&mut VM, a: u64, b: u64);

/// デコード済み命令
#[derive(Clone)]
pub struct Instruction {
    pub f: Op,
    pub a: u64,
    pub b: u64,
}

impl Instruction {
    pub fn new(f: Op, a: u64, b: u64) -> Self {
        Instruction { f, a, b }
    }
}

/// 整数演算
impl Operations {
    /// 64bit符号なし整数加算
    /// *dst = *dst + *src
    #[inline(always)]
    pub fn add_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(dst as usize)).wrapping_add(*r.add(src as usize));
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号なし整数加算
    /// *dst = *dst + imm
    #[inline(always)]
    pub fn add_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(dst as usize)).wrapping_add(imm);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数加算
    /// *dst = *dst + *src
    #[inline(always)]
    pub fn add_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) =
                ((*r.add(dst as usize) as i64).wrapping_add(*r.add(src as usize) as i64)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数加算
    /// *dst = *dst + imm
    #[inline(always)]
    pub fn add_i64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = ((*r.add(dst as usize) as i64).wrapping_add(imm as i64)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号なし整数減算
    /// *dst = *dst - *src
    #[inline(always)]
    pub fn sub_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(dst as usize)).wrapping_sub(*r.add(src as usize));
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号なし整数減算
    /// *dst = *dst - imm
    #[inline(always)]
    pub fn sub_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(dst as usize)).wrapping_sub(imm);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数減算
    /// *dst = *dst - *src
    #[inline(always)]
    pub fn sub_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) =
                ((*r.add(dst as usize) as i64).wrapping_sub(*r.add(src as usize) as i64)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数減算
    /// *dst = *dst - imm
    #[inline(always)]
    pub fn sub_i64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = ((*r.add(dst as usize) as i64).wrapping_sub(imm as i64)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号なし整数乗算
    /// *dst = *dst * *src
    #[inline(always)]
    pub fn mul_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(dst as usize)).wrapping_mul(*r.add(src as usize));
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号なし整数乗算
    /// *dst = *dst * imm
    #[inline(always)]
    pub fn mul_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(dst as usize)).wrapping_mul(imm);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数乗算
    /// *dst = *dst * *src
    #[inline(always)]
    pub fn mul_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) =
                ((*r.add(dst as usize) as i64).wrapping_mul(*r.add(src as usize) as i64)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数乗算
    /// *dst = *dst * imm
    #[inline(always)]
    pub fn mul_i64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = ((*r.add(dst as usize) as i64).wrapping_mul(imm as i64)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号なし整数除算
    /// *dst = *dst / *src
    #[inline(always)]
    pub fn div_u64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = *r.add(dst as usize) / *r.add(src as usize);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号なし整数除算
    /// *dst = *dst / imm
    #[inline(always)]
    pub fn div_u64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(dst as usize)) / imm;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数除算
    /// *dst = *dst / *src
    #[inline(always)]
    pub fn div_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) =
                (*r.add(dst as usize) as i64 / *r.add(src as usize) as i64) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数除算
    /// *dst = *dst / imm
    #[inline(always)]
    pub fn div_i64_immediate(vm: &mut VM, dst: u64, imm: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(dst as usize) as i64 / imm as i64) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数絶対値
    /// *dst = abs(*src)
    #[inline(always)]
    pub fn abs(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (*r.add(src as usize) as i64).wrapping_abs() as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き整数剰余
    /// *dst = *dst % *src
    #[inline(always)]
    pub fn mod_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) =
                ((*r.add(dst as usize) as i64).wrapping_rem(*r.add(src as usize) as i64)) as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号付き 符号反転
    /// *dst = -(*src)
    #[inline(always)]
    pub fn neg_i64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = (-(*r.add(src as usize) as i64)).wrapping_abs() as u64;
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号なし整数浮動小数点数変換
    /// *dst = (*src as f64)
    #[inline(always)]
    pub fn u64_to_f64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = f64::to_bits(*r.add(src as usize) as u64 as f64);
        }
        vm.st.pc += 1; // fallthrough
    }

    /// 64bit符号あり整数浮動小数点数変換
    /// *dst = (*src as i64) as f64
    #[inline(always)]
    pub fn i64_to_f64(vm: &mut VM, dst: u64, src: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            *r.add(dst as usize) = f64::to_bits(*r.add(src as usize) as i64 as f64);
        }
        vm.st.pc += 1; // fallthrough
    }
}

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

/// 制御系
impl Operations {
    /// ジャンプ
    /// pc = *dst + offset
    #[inline(always)]
    pub fn jump(vm: &mut VM, dst: u64, offset: u64) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            vm.st.pc = (*r.add(dst as usize)).wrapping_add(offset) as usize;
        }
    }

    /// 等しい場合のジャンプ
    /// if *a == *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn eq_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) == *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// 等しくない場合のジャンプ
    /// if *a != *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn neq_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) != *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より小さい場合のジャンプ (符号なし)
    /// if *a < *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn lt_u64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) < *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より小さいか等しい場合のジャンプ (符号なし)
    /// if *a <= *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn lte_u64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) <= *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より小さい場合のジャンプ (符号付き)
    /// if *a < *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn lt_i64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) < (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より小さいか等しい場合のジャンプ (符号付き)
    /// if *a <= *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn lte_i64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) <= (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より大きい場合のジャンプ (符号なし)
    /// if *a > *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn gt_u64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) > *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より大きいか等しい場合のジャンプ (符号なし)
    /// if *a >= *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn gte_u64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if *r.add(a) >= *r.add(b) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より大きい場合のジャンプ (符号付き)
    /// if *a > *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn gt_i64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) > (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// より大きいか等しい場合のジャンプ (符号付き)
    /// if *a >= *b { pc = *addr_reg + offset } else { pc += 1 }
    /// addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]
    #[inline(always)]
    pub fn gte_i64_jump(vm: &mut VM, addr_a_b: u64, offset: u64) {
        let addr_reg = ((addr_a_b >> 16) & 0xFF) as usize;
        let a = ((addr_a_b >> 8) & 0xFF) as usize;
        let b = (addr_a_b & 0xFF) as usize;
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let addr = (*r.add(addr_reg)).wrapping_add(offset) as usize;
            if (*r.add(a) as i64) >= (*r.add(b) as i64) {
                vm.st.pc = addr;
            } else {
                vm.st.pc += 1; // fallthrough
            }
        }
    }

    /// 関数呼び出し
    /// call func_index
    /// set pc ( 普通は関数先頭アドレスで0 )
    #[inline(always)]
    pub fn call(vm: &mut VM, func_index: u64, pc: u64) {
        vm.st.call_stack.push(vm.st.pc);
        vm.st.call_stack.push(vm.st.now_call_index);
        vm.st.pc = pc as usize;
        vm.st.now_call_index = func_index as usize; 
        vm.st.now_function_ptr = vm.function_table[vm.st.now_call_index];
    }

    /// 関数リターン
    /// ret
    #[inline(always)]
    pub fn ret(vm: &mut VM, _: u64, _: u64) {
        vm.st.now_call_index = vm.st.call_stack.pop().expect("Call stack underflow on return");
        vm.st.pc = vm.st.call_stack.pop().unwrap() + 1;
        vm.st.now_function_ptr = vm.function_table[vm.st.now_call_index];
    }
}

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

/// 特殊制御
impl Operations {
    /// LocalDecodedByteCodeの更新
    /// 呼び出された場合CodeManagerにデコードを依頼し、VMのFuctionTableを更新します
    /// Code Manager は未デコードのfunctionをこれに置き換えます。
    #[inline(always)]
    pub fn get_decode(vm: &mut VM, decode_id: u64, deep: u64) {
        vm.function_table = vm.cm.get_decode(decode_id, vm.st.now_call_index as u64, deep);
        vm.st.pc += 1; // fallthrough
    }

    /// 最新のデコード済みByteCodeを取得
    #[unsafe(link_section = ".text.hot")]
    #[inline(always)]
    pub fn get_decoded(vm: &mut VM, _:u64, _: u64) {
        vm.function_table = vm.cm.get_decoded();
        vm.st.pc += 1; // fallthrough
    }
}

