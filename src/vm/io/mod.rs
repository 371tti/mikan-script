

use std::{collections::{HashMap, VecDeque}, sync::Arc};

pub type IoId = u32;  // ソケット/ファイルなどのハンドル
pub type FuId = u64;  // Future ID

/// 非同期I/Oリアクターパターン
pub trait Reactor {
    /// fu_id と IoOp を受け取って OS に投げる
    fn submit(&self, fu_id: FuId, op: &IoOp) -> std::io::Result<Option<IoResult>>;

    /// 完了イベントを最大 max_events 個まで取得
    /// timeout_ms が負の場合は無限に待機
    fn wait(
        &self,
        timeout_ms: i64,
        max_events: usize,
        out: &mut Vec<(FuId, IoResult)>,
    ) -> std::io::Result<()>;

    /// キャンセル要求 サポートできない場合はBestEffort
    fn cancel(&self, fu_id: FuId) -> std::io::Result<()>;
}

#[derive(Debug, Clone, Copy)]
pub enum IoType {
    // 汎用ストリーム I/O
    Read,       // handle, buf_ptr, len
    Write,      // handle, buf_ptr, len

    // タイマー
    Sleep,      // ms

    // TCP
    TcpConnect, // addr_ptr, addr_len
    TcpListen,  // port or addr_ptr/len
    TcpAccept,  // listener_handle

    // UDP
    UdpSendTo,  // handle, buf_ptr, len, addr_ptr, addr_len
    UdpRecvFrom,// handle, buf_ptr, len

    // ファイル
    FileRead,   // handle, buf_ptr, len
    FileWrite,  // handle, buf_ptr, len
    FileReadAt, // handle, offset, buf_ptr, len
    FileWriteAt,// handle, offset, buf_ptr, len
    FileSync,        // handle
    FileStat,        // handle, dst_ptr
    DirRead,         // handle, buf_ptr, len

    // 標準入出力
    StdinRead,      // buf_ptr, len
    StdoutWrite,    // buf_ptr, len
    StderrWrite,    // buf_ptr, len

    // ストリーム操作系
    Shutdown,   // handle + how

    // パイプ・匿名チャネル
    PipeRead,   // handle, buf_ptr, len
    PipeWrite,  // handle, buf_ptr, len

    // プロセス
    ProcessSpawn,  // spec_ptr
    ProcessWait,   // handle

    // OS 情報系
    TimeNow,       // ()
    RandomBytes,   // buf_ptr, len
}

#[derive(Debug)]
pub enum IoOp {
    Read {
        handle: IoId,
        buf_ptr: u64,
        len: u64,
    },
    Write {
        handle: IoId,
        buf_ptr: u64,
        len: u64,
    },
    Sleep {
        ms: u64,
    },
    StdoutWrite {
        buf_ptr: u64,
        len: u64,
    },
    StderrWrite {
        buf_ptr: u64,
        len: u64,
    },
    StdinRead {
        buf_ptr: u64,
        len: u64,
    },
    FileRead {
        handle: IoId,
        buf_ptr: u64,
        len: u64,
    },
}

#[derive(Debug, Clone)]
pub enum IoResult {
    StreamIo { len: u64, status: i32 }, // Read/Writeなど
    NewHandle { handle: IoId, status: i32 }, // Accept/Connectなど
    SleepDone { status: i32 },
    TimeNow { low: u64, high: u64, status: i32 },
    Simple { status: i32 }, // FileSync/Shutdown等
}

pub enum IoFutureState {
    Pending(IoOp),
    Completed(IoResult),
    Canceled,
    Failed(i32), // errnoなど
}

pub struct IoEngine<R: Reactor> {
    next_fu: FuId,
    futures: HashMap<FuId, IoFutureState>,
    completed: VecDeque<FuId>,
    reactor: Arc<R>,
}

impl<R: Reactor> IoEngine<R> {
    pub fn new(reactor: Arc<R>) -> Self {
        Self {
            next_fu: 1,
            futures: HashMap::new(),
            completed: VecDeque::new(),
            reactor,
        }
    }

    fn alloc_fu(&mut self) -> FuId {
        let id = self.next_fu;
        self.next_fu += 1;
        id
    }

    pub fn submit_op(&mut self, op: IoOp) -> std::io::Result<FuId> {
        let fu_id = self.alloc_fu();
        let option_result = self.reactor.submit(fu_id, &op)?;
        if let Some(result) = option_result {
            self.futures.insert(fu_id, IoFutureState::Completed(result));
            self.completed.push_back(fu_id);
        } else {
            self.futures.insert(fu_id, IoFutureState::Pending(op));
        }
        Ok(fu_id)
    }

    pub fn wait_a_event(&mut self, timeout_ms: i64) -> Option<(FuId, IoResult)> {
        if let Some(fu_id) = self.completed.pop_front() {
            if let Some(state) = self.futures.get_mut(&fu_id) {
                if let IoFutureState::Completed(result) = state {
                    return Some((fu_id, result.clone()));
                }
            }
        }

        let mut events = Vec::new();
        // ここでblocking
        self.reactor.wait(timeout_ms, 1, &mut events).unwrap(); // エラーハンドリングは後で

        for (fu_id, result) in events {
            if let Some(state) = self.futures.get_mut(&fu_id) {
                *state = IoFutureState::Completed(result.clone());
                return Some((fu_id, result));
            }
        }
        None
    }

    
}

