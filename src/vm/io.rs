use std::{collections::{HashMap, VecDeque}, sync::Arc};

pub type IoId = u32;  // ソケット/ファイルなどのハンドル
pub type FuId = u64;  // Future ID

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
}

#[derive(Debug)]
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

pub struct IoEngine {
    next_fu: FuId,
    futures: HashMap<FuId, IoFutureState>,
    completed: VecDeque<FuId>,
    // 本当はここに epoll/io_uring/IOCP などの Reactor をぶら下げる
}

impl IoEngine {
    pub fn new() -> Self {
        Self {
            next_fu: 1,
            futures: HashMap::new(),
            completed: VecDeque::new(),
        }
    }

    fn alloc_fu(&mut self) -> FuId {
        let id = self.next_fu;
        self.next_fu += 1;
        id
    }
}

pub trait Reactor {
    /// fu_id と IoOp を受け取って OS に投げる
    fn submit(&self, fu_id: FuId, op: &IoOp) -> std::io::Result<()>;

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

use windows::Win32::{
    Foundation::{HANDLE, CloseHandle, INVALID_HANDLE_VALUE},
    System::IO::CreateIoCompletionPort,
};

/// For Windows IOCP
pub struct IocpReactor {
    iocp: HANDLE,
}

impl IocpReactor {
    pub fn new() -> std::io::Result<Arc<IocpReactor>> {
        unsafe {
            let iocp = CreateIoCompletionPort(
                // 新規作成
                INVALID_HANDLE_VALUE,
                // 既存ポートなし = NULL
                None,
                // CompletionKey
                0,
                // スレッド数 0 = OS 任せ
                0,
            )
            .map_err(|e| {
                // windows_core::Error → std::io::Error 変換
                // e.code() は HRESULT / NTSTATUS 系なので適宜処理
                std::io::Error::from_raw_os_error(e.code().0 as i32)
            })?;

            Ok(Arc::new(IocpReactor { iocp }))
        }
    }
}

impl Drop for IocpReactor {
    fn drop(&mut self) {
        unsafe {
            if !self.iocp.is_invalid() {
                let _ = CloseHandle(self.iocp);
            }
        }
    }
}