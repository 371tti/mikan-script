use std::{ops::Deref, path::PathBuf, sync::{Arc, RwLock}};

use rustc_hash::FxHashMap;

use crate::vm::{function::{Function, FunctionPtr}, pre_decoder::PreDecoder};

pub struct CodeManager {
    inner: Arc<CodeManagerInner>,
}

impl CodeManager {
    pub fn new(root_dir: PathBuf) -> Self {
        CodeManager {
            inner: Arc::new(CodeManagerInner::new(root_dir)),
        }
    }

    pub fn clone_shared(&self) -> CodeManager {
        CodeManager {
            inner: self.inner.clone(),
        }
    }
}

impl Deref for CodeManager {
    type Target = CodeManagerInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct CodeManagerInner {
    /// 最新の関数テーブル
    /// 初期でMainとその差し替え関数のみが入ってるとしておく
    pub latest_function_table: RwLock<Vec<FunctionPtr>>,
    /// 所有権保持実態 RUST安全外
    /// 削除禁止
    pub owned_functions: RwLock<Vec<Function>>,
    /// 遅延ロードを実現するためにbytecodeのfunction id を置き換えます。
    /// index = decode_id
    /// 関数ぜんぶここになげこんで、MAINから再帰的にパースしていく感じ？
    /// 再帰のためのバッファは処理系に投げとくか
    pub functions: RwLock<FxHashMap<FunctionPath, UnDecodedFunction>>,
    pub decoder: PreDecoder,
    /// MAINあるやつ
    pub root_dir: PathBuf,
}

type FunctionPath = String;

impl CodeManagerInner {
    pub fn new(root_dir: PathBuf) -> Self {
        let latest_function_table = RwLock::new(Vec::new());
        CodeManagerInner { 
            latest_function_table, 
            owned_functions: RwLock::new(Vec::new()), 
            functions: RwLock::new(FxHashMap::default()), 
            decoder: PreDecoder::new(),
            root_dir,
        }
    }

    pub fn set_functions(&self, functions: Vec<Function>) {
        let mut owned_functions = self.owned_functions.write().unwrap();
        for func in functions {
            owned_functions.push(func);
        }
        drop(owned_functions);
        let owned_functions = self.owned_functions.read().unwrap();
        let mut latest_function_table = self.latest_function_table.write().unwrap();
        owned_functions.iter().for_each(|f| {
            latest_function_table.push(FunctionPtr(Box::into_raw(Box::new(f.clone()))));
        });
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
    /// 未割当ならMAXで
    table_index: usize,
    /// 差し替えfunction
    replacement_function: FunctionPtr,
    /// バイトコードのソースパス
    source_path: PathBuf,
}