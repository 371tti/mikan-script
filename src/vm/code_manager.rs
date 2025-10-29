use std::{path::PathBuf, sync::RwLock};

use rustc_hash::FxHashMap;

use crate::vm::{Function, FunctionPtr, pre_decoder::PreDecoder};

pub struct CodeManager {
    pub latest_function_table: RwLock<Vec<FunctionPtr>>,
    // 所有権保持実態 削除禁止
    pub owned_functions: RwLock<Vec<Function>>, 
    /// 遅延ロードを実現するためにbytecodeのfunction id を置き換えます。
    /// index = decode_id
    pub waiting_decode_functions: RwLock<Vec<UnDecodedFunction>>,
    pub decoder: PreDecoder,
}

type ByteCodeID = u64;

impl CodeManager {
    pub fn new() -> Self {
        let latest_function_table = RwLock::new(Vec::new());
        CodeManager { latest_function_table, owned_functions: RwLock::new(Vec::new()), waiting_decode_functions: RwLock::new(Vec::new()), decoder: PreDecoder::new() }
    }

    pub fn decode_request(&mut self, func_id: u64) {
        
    }

    pub fn get_decoded(&self) -> Box<[FunctionPtr]> {
        self.latest_function_table.read().unwrap().to_vec().into_boxed_slice()
    }

    pub fn get_decode(&self, _func_id: u64, _decode_id: u64, _deep: u64, ) -> Box<[FunctionPtr]> {
        unimplemented!()
    }
}

pub struct UnDecodedFunction {
    is_decoded: bool,
    // 差し替えfunctionの所有
    replacement_function: FunctionPtr,
    // バイトコードのソースパス
    source_path: PathBuf,
}