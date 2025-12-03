use crate::vm::{instruction::operations::Operations, vm::{VM, state_flag}};


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
    #[inline(always)]
    pub fn get_decoded(vm: &mut VM, _:u64, _: u64) {
        vm.function_table = vm.cm.get_decoded();
        vm.st.pc += 1; // fallthrough
    }

    
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

