use std::{path::PathBuf, sync::RwLock};

use rustc_hash::FxHashMap;

use crate::vm::{Function, FunctionPtr, pre_decoder::PreDecoder};

pub struct CodeManager {
    pub latest_function_table: RwLock<Vec<FunctionPtr>>,
    // 所有権保持実態 ptrで直アクセスされるためRUSTの所有権システム外になるので GC実装に注意
    pub owned_functions: RwLock<FxHashMap<FunctionId, Function>>,
    /// 遅延ロードを実現するためにbytecodeのfunction id を置き換えます。
    /// index = decode_id
    pub waiting_decode_functions: RwLock<FxHashMap<DecodeId, UnDecodedFunction>>,
    pub decoder: PreDecoder,
}

type FunctionId = u64;

type DecodeId = u64;

impl CodeManager {
    pub fn new() -> Self {
        let latest_function_table = RwLock::new(Vec::new());
        CodeManager { latest_function_table, owned_functions: RwLock::new(FxHashMap::default()), waiting_decode_functions: RwLock::new(FxHashMap::default()), decoder: PreDecoder::new() }
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
    // 差し替えfunction
    replacement_function: FunctionPtr,
    // バイトコードのソースパス
    source_path: PathBuf,
    // 参照カウント
    ref_count: u64,
}

// main読み込み 遅延ロードとして
// 読み込み済み functionが呼び出すcall のfunction_id はすべて読み込み済みでマッピングされているか 遅延ロード用のfunctionに置き換えられている必要がある
// 遅延ロード用のfunctionは 作成時、 owned_functionsにいれておき、 waiting_decode_functionsで参照カウントで管理しながら入れる
// 遅延ロードが行われたとき waiting_decode_functionsからデコードし、 owned_functionsに追加、 latest_function_tableを書き換える ref_countが0になったらowned_functionsとwaiting_decode_functionsから削除する