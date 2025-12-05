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

    pub fn submit_op(&mut self, op: IoOp) -> FuId {
        let fu_id = self.alloc_fu();
        self.reactor.submit(fu_id, &op).unwrap(); // エラーハンドリングは後で
        self.futures.insert(fu_id, IoFutureState::Pending(op));
        fu_id
    }

    pub fn wait_a_event(&mut self, timeout_ms: i64) -> Option<(FuId, IoResult)> {
        let mut events = Vec::new();
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
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE}, Storage::FileSystem::WriteFile, System::{Console::{GetStdHandle, STD_OUTPUT_HANDLE}, IO::{CreateIoCompletionPort, GetQueuedCompletionStatus, OVERLAPPED}}
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


impl Reactor for IocpReactor {
    fn submit(&self, fu_id: FuId, op: &IoOp) -> std::io::Result<()> {
        match op {
            IoOp::StdoutWrite { buf_ptr, len } => {
                unsafe {
                    // 標準出力ハンドル取得
                    let stdout = GetStdHandle(STD_OUTPUT_HANDLE)
                        .map_err(|e| std::io::Error::from_raw_os_error(e.code().0 as i32))?;

                    // バッファの参照を取得
                    let buf = std::slice::from_raw_parts(*buf_ptr as *const u8, *len as usize);

                    // OVERLAPPED 構造体を確保
                    let mut overlapped: Box<OVERLAPPED> = Box::new(std::mem::zeroed());

                    // IOCP に関連付け（必要なら）
                    let _ = CreateIoCompletionPort(stdout, Some(self.iocp), fu_id as usize, 0);

                    let mut written = 0u32;
                    let res = WriteFile(
                        stdout,
                        Some(buf),
                        Some(&mut written),
                        Some(&mut *overlapped),
                    );

                    // WriteFile は完了前に false を返すことがあるが、ERROR_IO_PENDING なら正常
                    if res.is_ok() || windows::Win32::Foundation::GetLastError().0 == 997 {
                        // 成功または非同期進行中
                        Ok(())
                    } else {
                        Err(std::io::Error::last_os_error())
                    }
                }
            }
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "未対応のIoOp")),
        }
    }

    fn wait(
        &self,
        timeout_ms: i64,
        max_events: usize,
        out: &mut Vec<(FuId, IoResult)>,
    ) -> std::io::Result<()> {
        unsafe {
            let timeout = if timeout_ms < 0 { u32::MAX } else { timeout_ms as u32 };
            let mut events = 0;

            while events < max_events {
                let mut bytes_transferred = 0u32;
                let mut completion_key = 0usize;
                let mut overlapped_ptr: *mut OVERLAPPED = std::ptr::null_mut();

                let ok = GetQueuedCompletionStatus(
                    self.iocp,
                    &mut bytes_transferred,
                    &mut completion_key,
                    &mut overlapped_ptr,
                    if events == 0 { timeout } else { 0 }, // 最初だけタイムアウト適用
                );

                if ok.is_ok() && !overlapped_ptr.is_null() {
                    out.push((
                        completion_key as FuId,
                        IoResult::StreamIo {
                            len: bytes_transferred as u64,
                            status: 0,
                        },
                    ));
                    events += 1;
                } else {
                    // タイムアウトまたはエラー
                    break;
                }
            }
            Ok(())
        }
    }

    fn cancel(&self, _fu_id: FuId) -> std::io::Result<()> {
        // 標準出力にはキャンセル不要
        Ok(())
    }
}
