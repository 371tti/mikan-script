pub mod windows;

use std::collections::{HashMap, VecDeque};


pub type IoId = u32;  // ソケット/ファイルなどのハンドル
pub type FuId = u64;  // Future ID

/// 非同期I/Oリアクターパターン
pub trait Reactor: Sync + Send {
    fn new() -> std::io::Result<Self>
    where
        Self: Sized;
    /// fu_id と IoOp を受け取って OS に投げる
    fn submit(&self, fu_id: FuId, op: &IoOp) -> IoResult;

    /// 完了イベントを最大 max_events 個まで取得
    /// timeout_ms が負の場合は無限に待機
    fn wait(
        &self,
        timeout_ms: i64,
        max_events: usize,
        out: &mut Vec<Event>,
    ) -> ();
}

#[derive(Debug, Clone, Copy)]
pub enum IoType {
    // ---- Timer ----
    Sleep,      // ms

    // ---- StdIO ----
    StdoutWrite,  // buf_ptr, len
    StdinRead,    // buf_ptr, len

    // ---- TCP ----
    TcpConnect,  // addr_ptr, addr_len
    TcpListen,   // addr_ptr, addr_len
    TcpAccept,   // listener_handle
    Read,        // handle, buf_ptr, len
    Write,       // handle, buf_ptr, len

    // ---- System info ----
    TimeNow,       // ()
    RandomBytes,   // buf_ptr, len
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct TcpListenFlags: u16 {
        const REUSE_ADDR = 0x0001;
        const REUSE_PORT = 0x0002;
        const IPV6_ONLY  = 0x0004;
    }
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
    StdinRead {
        buf_ptr: u64,
        len: u64,
    },
    TcpConnect {
        /// IP address pointer
        /// IPv4 -> 4 bytes, IPv6 -> 16 bytes
        /// not force align
        ip_ptr: u64,
        /// port number 0-65535
        port: u16,
        /// features flags
        flags: TcpListenFlags,
        /// IPv4 = 4, IPv6 = 6
        family: u8,
    },
    TcpListen {
        /// IP address pointer
        /// IPv4 -> 4 bytes, IPv6 -> 16 bytes
        /// not force align
        ip_ptr: u64,
        /// port number 0-65535
        port: u16,
        /// backlog
        backlog: u16,
        /// features flags
        flags: TcpListenFlags,
        /// IPv4 = 4, IPv6 = 6
        family: u8,
    },
    TcpAccept {
        listener_handle: IoId,
    },
    TimeNow,
    RandomBytes {
        buf_ptr: u64,
        len: u64,
    },
}

#[derive(Debug, Clone)]
pub enum IoOk {
    StreamIo { len: u64 },            // Read/Write
    NewHandle { handle: IoId },       // Accept/Connect
    SleepDone,
    TimeNow { low: u64, high: u64 },
    Simple,                           // FileSync/Shutdownなど
}

#[derive(Debug, Clone, Copy)]
pub enum IoErrorKind {
    Interrupted,      // EINTR
    WouldBlock,       // EWOULDBLOCK / EAGAIN
    TimedOut,         // ETIMEDOUT
    ConnectionReset,  // ECONNRESET
    ConnectionRefused,// ECONNREFUSED
    ConnectionAborted,// ECONNABORTED
    BrokenPipe,       // EPIPE
    NotFound,         // ENOENT
    PermissionDenied, // EACCES, EPERM
    AlreadyExists,    // EEXIST
    InvalidInput,     // EINVAL
    AddrInUse,        // EADDRINUSE
    AddrNotAvailable, // EADDRNOTAVAIL
    ResourceLimit,    // EMFILE, ENFILE, ENOMEM, etc.
    Other,            // どれにも分類できない
    NotImplYet,       // 未実装
    ClonedHandle,     // クローンしたハンドルを使おうとした
}

#[derive(Debug, Clone, Copy)]
pub struct IoError {
    pub kind: IoErrorKind,
    pub raw_os_error: i32,   // errno / Win32 error code そのまま
    pub retryable: bool,     // 再試行すべきか？（WouldBlock/Interruptedなど）
}

#[derive(Debug, Clone)]
pub enum IoResult {
    Pending,
    Ok(IoOk),
    Err(IoError),
}

#[derive(Debug)]
pub struct IoEngine<R: Reactor> {
    next_fu: FuId,
    futures: HashMap<FuId, IoResult>,
    completed: VecDeque<FuId>,
    reactor: R,
    unacquired_events: u64,
}

pub struct Event {
    pub fu_id: FuId,
    pub result: IoResult,
}

impl<R: Reactor> IoEngine<R> {
    pub fn new() -> Self {
        IoEngine {
            next_fu: 0,
            futures: HashMap::new(),
            completed: VecDeque::new(),
            reactor: R::new().unwrap(),
            unacquired_events: 0,
        }
    }

    fn alloc_fu(&mut self) -> FuId {
        let id = self.next_fu;
        self.next_fu += 1;
        id
    }

    /// IoOpを投げてFuture IDを取得する
    pub fn submit_op(&mut self, op: IoOp) -> FuId {
        let fu_id = self.alloc_fu();
        let option_result = self.reactor.submit(fu_id, &op);
        match option_result {
            IoResult::Pending => {
                self.futures.insert(fu_id, IoResult::Pending);
            }
            IoResult::Ok(ok) => {
                self.futures.insert(fu_id, IoResult::Ok(ok));
                self.completed.push_back(fu_id);
            }
            IoResult::Err(err) => {
                self.futures.insert(fu_id, IoResult::Err(err));
                self.completed.push_back(fu_id);
            }
        }
        self.unacquired_events += 1;
        fu_id
    }

    /// 完了したイベントを1つ取得する（ノンブロッキング）
    pub fn get_a_event(&mut self) -> Option<Event> {
        if let Some(fu_id) = self.completed.pop_front() {
            if let Some(state) = self.futures.get_mut(&fu_id) {
                self.unacquired_events -= 1;
                return Some(Event {
                    fu_id,
                    result: state.clone(),
                });
            }
        }
        None
    }

    /// 完了イベントを最大 max_events 個まで収集する（ブロッキング）
    /// timeout_ms が負の場合は無限に待機
    pub fn blocking_collect_events(&mut self, timeout_ms: i64, max_events: usize) -> usize {
        let mut events = Vec::with_capacity(max_events);
        self.reactor.wait(timeout_ms, max_events, &mut events);
        let num_events = events.len();
        for event in events {
            if let Some(state) = self.futures.get_mut(&event.fu_id) {
                *state = event.result.clone();
                self.completed.push_back(event.fu_id);
            }
        }
        num_events
    }

    /// OSが処理、もしくはキューしてるイベント数
    pub fn events_holed_os(&self) -> u64 {
        self.unacquired_events - self.completed.len() as u64
    }

    /// 未取得の完了イベント数
    pub fn unacquired_completed_events(&self) -> u64 {
        self.completed.len() as u64
    }

    /// 保持してるFutureの数
    pub fn total_futures(&self) -> u64 {
        self.unacquired_events
    }
}

impl<R> Clone for IoEngine<R>
where
    R: Reactor,
{
    fn clone(&self) -> Self {
        // リアクターは共有しない 新規作成する（IOCPハンドルを共有すると問題が起きる可能性があるため）
        // 既存の非同期操作はクローン先には引き継がれない 全部失敗状態に上書きする
        let clone_base: Vec<(FuId, IoResult)> = self.futures.iter()
                .map(|(&fu_id, _)| {
                    (
                        fu_id,
                        IoResult::Err(IoError {
                            kind: IoErrorKind::ClonedHandle,
                            raw_os_error: -1,
                            retryable: false,
                        }),
                    )
                })
                .collect();
        let completed: Vec<FuId> = clone_base.iter().map(|(fu_id, _)| *fu_id).collect();
        let futures: HashMap<FuId, IoResult> = clone_base.into_iter().collect();

            
        IoEngine {
            next_fu: self.next_fu,
            futures: futures,
            completed: completed.into(),
            reactor: R::new().unwrap(),
            unacquired_events: self.unacquired_events,
        }
    }
}