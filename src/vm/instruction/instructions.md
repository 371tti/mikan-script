# 命令の一覧

## 整数演算命令

### 加算
- `add_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号なし整数加算。`*dst = *dst + *src`
- `add_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit符号なし整数加算（即値）。`*dst = *dst + imm`
- `add_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号付き整数加算。`*dst = *dst + *src`
- `add_i64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit符号付き整数加算（即値）。`*dst = *dst + imm`

### 減算
- `sub_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号なし整数減算。`*dst = *dst - *src`
- `sub_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit符号なし整数減算（即値）。`*dst = *dst - imm`
- `sub_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号付き整数減算。`*dst = *dst - *src`
- `sub_i64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit符号付き整数減算（即値）。`*dst = *dst - imm`

### 乗算
- `mul_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号なし整数乗算。`*dst = *dst * *src`
- `mul_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit符号なし整数乗算（即値）。`*dst = *dst * imm`
- `mul_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号付き整数乗算。`*dst = *dst * *src`
- `mul_i64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit符号付き整数乗算（即値）。`*dst = *dst * imm`

### 除算
- `div_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号なし整数除算。`*dst = *dst / *src`
- `div_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit符号なし整数除算（即値）。`*dst = *dst / imm`
- `div_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号付き整数除算。`*dst = *dst / *src`
- `div_i64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit符号付き整数除算（即値）。`*dst = *dst / imm`

### その他
- `abs(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号付き整数絶対値。`*dst = abs(*src)`
- `mod_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号付き整数剰余。`*dst = *dst % *src`
- `neg_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号付き整数符号反転。`*dst = -(*src)`
- `u64_to_f64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号なし整数→浮動小数点数変換。`*dst = (*src as f64)`
- `i64_to_f64(vm: &mut VM, dst: u64, src: u64)`  
  64bit符号付き整数→浮動小数点数変換。`*dst = (*src as i64) as f64`

## 浮動小数点演算命令

### 加算
- `add_f64(vm: &mut VM, dst: u64, src: u64)`  
  64bit浮動小数点加算。`*dst = *dst + *src`
- `add_f64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit浮動小数点加算（即値）。`*dst = *dst + imm`

### 減算
- `sub_f64(vm: &mut VM, dst: u64, src: u64)`  
  64bit浮動小数点減算。`*dst = *dst - *src`
- `sub_f64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit浮動小数点減算（即値）。`*dst = *dst - imm`

### 乗算
- `mul_f64(vm: &mut VM, dst: u64, src: u64)`  
  64bit浮動小数点乗算。`*dst = *dst * *src`
- `mul_f64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit浮動小数点乗算（即値）。`*dst = *dst * imm`

### 除算
- `div_f64(vm: &mut VM, dst: u64, src: u64)`  
  64bit浮動小数点除算。`*dst = *dst / *src`
- `div_f64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit浮動小数点除算（即値）。`*dst = *dst / imm`

### その他
- `abs_f64(vm: &mut VM, dst: u64, src: u64)`  
  64bit浮動小数点絶対値。`*dst = abs(*src)`
- `neg_f64(vm: &mut VM, dst: u64, src: u64)`  
  64bit浮動小数点符号反転。`*dst = -(*src)`
- `to_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit浮動小数点→整数変換。`*dst = (*src as f64) as i64`

## ビット・論理演算命令

### 論理演算
- `and_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit論理積。`*dst = *dst & *src`
- `and_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit論理積（即値）。`*dst = *dst & imm`
- `or_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit論理和。`*dst = *dst | *src`
- `or_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit論理和（即値）。`*dst = *dst | imm`
- `xor_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit排他的論理和。`*dst = *dst ^ *src`
- `xor_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit排他的論理和（即値）。`*dst = *dst ^ imm`
- `not_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit論理否定。`*dst = !*src`

### シフト・ローテート
- `shl_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit論理左シフト。`*dst = *dst << *src`
- `shl_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit論理左シフト（即値）。`*dst = *dst << imm`
- `shl_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit算術左シフト。`*dst = *dst << *src`
- `shl_i64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit算術左シフト（即値）。`*dst = *dst << imm`
- `shr_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit論理右シフト。`*dst = *dst >> *src`
- `shr_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit論理右シフト（即値）。`*dst = *dst >> imm`
- `shr_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit算術右シフト。`*dst = *dst >> *src`
- `shr_i64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit算術右シフト（即値）。`*dst = *dst >> imm`

### ローテート
- `rol_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit論理左ローテート。`*dst = rol(*dst, *src)`
- `rol_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit論理左ローテート（即値）。`*dst = rol(*dst, imm)`
- `rol_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit算術左ローテート。`*dst = rol(*dst, *src)`
- `rol_i64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit算術左ローテート（即値）。`*dst = rol(*dst, imm)`
- `ror_u64(vm: &mut VM, dst: u64, src: u64)`  
  64bit論理右ローテート。`*dst = ror(*dst, *src)`
- `ror_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit論理右ローテート（即値）。`*dst = ror(*dst, imm)`
- `ror_i64(vm: &mut VM, dst: u64, src: u64)`  
  64bit算術右ローテート。`*dst = ror(*dst, *src)`
- `ror_i64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  64bit算術右ローテート（即値）。`*dst = ror(*dst, imm)`

### ビットカウント
- `count_ones_u64(vm: &mut VM, dst: u64, src: u64)`  
  1のビット数をカウント。`*dst = count_ones(*src)`
- `count_zeros_u64(vm: &mut VM, dst: u64, src: u64)`  
  0のビット数をカウント。`*dst = count_zeros(*src)`
- `trailing_zeros_u64(vm: &mut VM, dst: u64, src: u64)`  
  下位の連続する0の数。`*dst = trailing_zeros(*src)`

## レジスタ操作命令

- `mov(vm: &mut VM, dst: u64, src: u64)`  
  レジスタ間値コピー。`*dst = *src`
- `load_u64_immediate(vm: &mut VM, dst: u64, imm: u64)`  
  即値ロード。`*dst = imm`
- `swap(vm: &mut VM, reg_a: u64, reg_b: u64)`  
  交換。`*reg_a, *reg_b = *reg_b, *reg_a`

## メモリ操作命令

### ロード（通常）
- `load_u64(vm, idr_ptr_res, offset)`  
  u64ロード。`*result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)`
- `load_u32(vm, idr_ptr_res, offset)`  
  u32ロード。`*result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)`
- `load_u16(vm, idr_ptr_res, offset)`  
  u16ロード。`*result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)`
- `load_u8(vm, idr_ptr_res, offset)`  
  u8ロード。`*result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)`

### ストア（通常）
- `store_u64(vm, idr_ptr_src, offset)`  
  u64ストア。`*(heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg`
- `store_u32(vm, idr_ptr_src, offset)`  
  u32ストア。`*(heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg`
- `store_u16(vm, idr_ptr_src, offset)`  
  u16ストア。`*(heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg`
- `store_u8(vm, idr_ptr_src, offset)`  
  u8ストア。`*(heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg`

### ロード（符号拡張）
- `load_i8(vm, idr_ptr_res, offset)`  
  i8ロード（符号拡張）。`*result_reg = (*(heep_ptr(*id_reg) + *addr_reg + offset) as i8) as i64`
- `load_i16(vm, idr_ptr_res, offset)`  
  i16ロード（符号拡張）。`*result_reg = (*(heep_ptr(*id_reg) + *addr_reg + offset) as i16) as i64`
- `load_i32(vm, idr_ptr_res, offset)`  
  i32ロード（符号拡張）。`*result_reg = (*(heep_ptr(*id_reg) + *addr_reg + offset) as i32) as i64`
- `load_i64(vm, idr_ptr_res, offset)`  
  i64ロード（符号拡張）。`*result_reg = (*(heep_ptr(*id_reg) + *addr_reg + offset) as i64) as u64`

### ストア（符号拡張）
- `store_i8(vm, idr_ptr_src, offset)`  
  i8ストア（符号拡張）。`*(heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg as i8`
- `store_i16(vm, idr_ptr_src, offset)`  
  i16ストア（符号拡張）。`*(heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg as i16`
- `store_i32(vm, idr_ptr_src, offset)`  
  i32ストア（符号拡張）。`*(heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg as i32`
- `store_i64(vm, idr_ptr_src, offset)`  
  i64ストア（符号拡張）。`*(heep_ptr(*id_reg) + *addr_reg + offset) = *src_reg as i64`

### アトミック操作（ロード・ストア・加算・減算）
- `atomic_load_u64|u32|u16|u8(vm, idr_ptr_res, offset)`  
  atomicロード。`*result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset)`
- `atomic_store_u64|u32|u16|u8(vm, idr_ptr_src, offset)`  
  atomicストア。`atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)`
- `atomic_add_u64|u32|u16|u8(vm, res_idr_ptr_src, offset)`  
  atomic加算。`*result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)`
- `atomic_sub_u64|u32|u16|u8(vm, res_idr_ptr_src, offset)`  
  atomic減算。`*result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg)`

### アトミック操作（符号拡張）
- `atomic_load_i8|i16|i32|i64(vm, idr_ptr_res, offset)`  
  atomicロード（符号拡張）。`*result_reg = atomic_load(heep_ptr(*id_reg) + *addr_reg + offset) as 型 as i64`
- `atomic_store_i8|i16|i32|i64(vm, idr_ptr_src, offset)`  
  atomicストア（符号拡張）。`atomic_store(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg as 型)`
- `atomic_add_i8|i16|i32|i64(vm, res_idr_ptr_src, offset)`  
  atomic加算（符号拡張）。`*result_reg = atomic_fetch_add(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg as 型) as 型 as i64`
- `atomic_sub_i8|i16|i32|i64(vm, res_idr_ptr_src, offset)`  
  atomic減算（符号拡張）。`*result_reg = atomic_fetch_sub(heep_ptr(*id_reg) + *addr_reg + offset, *src_reg as 型) as 型 as i64`

## IO・メモリ管理命令

- `print_u64(vm, src, _)`  
  整数の出力。`print_u64 *src`
- `alloc(vm, size_idr, add_size)`  
  メモリ確保。`allocate *size + add_size, store id in *id_res_reg`  
  `size_idr: [ size_reg(8bit) | id_res_reg(8bit) ]`
- `realloc(vm, size, id)`  
  メモリ再確保。`reallocate *size for *id`
- `dealloc(vm, id, _)`  
  メモリ解放。`deallocate *id`

## 制御命令

### ジャンプ
- `jump(vm, dst, offset)`  
  ジャンプ。`pc = *dst + offset`

### 条件ジャンプ
- `eq_jump(vm, addr_a_b, offset)`  
  等しい場合ジャンプ。`if *a == *b { pc = *addr_reg + offset } else { pc += 1 }`  
  `addr_a_b: [ addr_reg(8bit) | a(8bit) | b(8bit) ]`
- `neq_jump(vm, addr_a_b, offset)`  
  等しくない場合ジャンプ。`if *a != *b { pc = *addr_reg + offset } else { pc += 1 }`
- `lt_u64_jump(vm, addr_a_b, offset)`  
  より小さい場合ジャンプ（符号なし）。`if *a < *b { pc = *addr_reg + offset } else { pc += 1 }`
- `lte_u64_jump(vm, addr_a_b, offset)`  
  より小さいか等しい場合ジャンプ（符号なし）。`if *a <= *b { pc = *addr_reg + offset } else { pc += 1 }`
- `lt_i64_jump(vm, addr_a_b, offset)`  
  より小さい場合ジャンプ（符号付き）。`if *a < *b { pc = *addr_reg + offset } else { pc += 1 }`
- `lte_i64_jump(vm, addr_a_b, offset)`  
  より小さいか等しい場合ジャンプ（符号付き）。`if *a <= *b { pc = *addr_reg + offset } else { pc += 1 }`
- `gt_u64_jump(vm, addr_a_b, offset)`  
  より大きい場合ジャンプ（符号なし）。`if *a > *b { pc = *addr_reg + offset } else { pc += 1 }`
- `gte_u64_jump(vm, addr_a_b, offset)`  
  より大きいか等しい場合ジャンプ（符号なし）。`if *a >= *b { pc = *addr_reg + offset } else { pc += 1 }`
- `gt_i64_jump(vm, addr_a_b, offset)`  
  より大きい場合ジャンプ（符号付き）。`if *a > *b { pc = *addr_reg + offset } else { pc += 1 }`
- `gte_i64_jump(vm, addr_a_b, offset)`  
  より大きいか等しい場合ジャンプ（符号付き）。`if *a >= *b { pc = *addr_reg + offset } else { pc += 1 }`

### 関数呼び出し・リターン
- `call(vm, func_index, pc)`  
  関数呼び出し。`call func_index`（pcは関数先頭アドレス）
- `ret(vm, _, _)`  
  関数リターン。`ret`

## 特殊制御命令

- `get_decode(vm, decode_id, deep)`  
  LocalDecodedByteCodeの更新。CodeManagerにデコードを依頼し、VMのFunctionTableを更新。
- `get_decoded(vm, _, _)`  
  最新のデコード済みByteCodeを取得。FunctionTableを更新。
- `exit(vm, code_reg, _)`  
  プログラム終了。`exit with code *code_reg`（終了コードはレジスタから取得し、VMを停止）


# 追加予定の命令
## Async IO命令群
- SET_IO op_type r0a r1a r2a r3a r4a r5a r6a r0b r1b r2b r3b r4b r5b r6b r7b 
- IO_POLL fu_id -> status
  任意のfu_idの状態を確認
- WAIT_IO timeout -> fu_id
  新しい通知がくるまでスレッドを停止 きたらfu_idを1つ返却
- IO_TAKE fu_id -> new_buf_ptr status
  fu_idを渡してデータなどを受け取る
- IO_CANCEL fu_id -> status
  fu_idを渡してIOをキャンセル


