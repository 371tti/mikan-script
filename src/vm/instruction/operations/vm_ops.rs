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
        let code = vm.next_operand_imm() as u64;
        // update VM state before actually exiting so any tooling or profiling
        // that inspects VM memory sees the correct values
        vm.st.r[0] = code; // return code
        vm.st.state_flag |= state_flag::PAUSE;
        // use fully qualified path to avoid recursive call
        std::process::exit(code as i32);
    }
}

