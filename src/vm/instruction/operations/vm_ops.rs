use crate::vm::{instruction::operations::Operations, vm::{VM, state_flag}};


/// 特殊制御
impl Operations {
    /// LocalDecodedByteCodeの更新
    /// 呼び出された場合CodeManagerにデコードを依頼し、VMのFuctionTableを更新します
    /// Code Manager は未デコードのfunctionをこれに置き換えます。
    #[inline(always)]
    pub fn get_decode(vm: &mut VM) {
        let decode_id = vm.next_operand_imm();
        let deep = vm.next_operand_imm();
        vm.function_table = vm.cm.get_decode(decode_id, vm.st.now_call_index as u64, deep);
        vm.next_step();
    }

    /// 最新のデコード済みByteCodeを取得
    #[inline(always)]
    pub fn get_decoded(vm: &mut VM) {
        vm.function_table = vm.cm.get_decoded();
        vm.next_step();
    }

    
    /// プログラム終了
    /// exit with code *code_reg
    #[inline(always)]
    pub fn exit(vm: &mut VM) {
        let code = {
            let ol = vm.next_operand();
            let code_reg = ol[0] as usize;
            unsafe {
                let r = vm.st.r.as_mut_ptr();
                *r.add(code_reg)
            }
        };
        // update VM state before actually exiting so any tooling or profiling
        // that inspects VM memory sees the correct values
        vm.st.r[0] = code; // return code
        vm.st.state_flag |= state_flag::PAUSE;
        // use fully qualified path to avoid recursive call
        vm.st.pc -= 1; // stay at exit instruction
    }

    /// 関数呼び出し
    /// call func_index
    /// set pc ( 普通は関数先頭アドレスで0 )
    #[inline(always)]
    pub fn call(vm: &mut VM) {
        let ol = vm.next_operand();
        let pc_reg = ol[0] as usize;
        let func_index = vm.next_operand_imm();
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let pc = *r.add(pc_reg);
            vm.st.call_stack.push(vm.st.pc);
            vm.st.call_stack.push(vm.st.now_call_index);
            vm.st.pc = pc as usize;
            vm.st.now_call_index = func_index as usize; 
            vm.st.now_function_ptr = vm.function_table[vm.st.now_call_index];
        }
    }

    /// 関数リターン
    /// ret
    #[inline(always)]
    pub fn ret(vm: &mut VM) {
        vm.st.now_call_index = vm.st.call_stack.pop().expect("Call stack underflow on return");
        vm.st.pc = vm.st.call_stack.pop().unwrap();
        vm.next_step();
        vm.st.now_function_ptr = vm.function_table[vm.st.now_call_index];
    }

    /// スレッド呼び出し
    /// call_thread func_index
    /// set pc ( 普通は関数先頭アドレスで0 )
    /// res_reg: 呼び出し先のVM IDを格納するレジスタ 以下の返値
    /// - N: vmのID
    /// - negative: Poolが存在しない等のエラー
    pub fn call_thread(vm: &mut VM) {
        let ol = vm.next_operand();
        let pc_reg = ol[0] as usize;
        let res_reg = ol[1] as usize;
        let func_index = vm.next_operand_imm();
        if let Some(pool) = &vm.pool {
            unsafe {
                let r = vm.st.r.as_mut_ptr();
                let pc = *r.add(pc_reg);
                let reactor = pool.reactor.clone();
                let mut new_vm = VM::new(reactor);
                new_vm.st.r[pc_reg] = pc;
                new_vm.st.now_call_index = func_index as usize;
                new_vm.st.now_function_ptr = new_vm.function_table[new_vm.st.now_call_index];
                let id = pool.push_and_run_threaded(new_vm, false);
                vm.st.r[res_reg] = id;
            }
        } else {
            // エラー: Poolが存在しない
            vm.st.r[res_reg] = u64::MAX; // -1
        }
    }
}

