use windows::Win32::{Foundation::{CloseHandle, GetLastError, HANDLE, INVALID_HANDLE_VALUE}, Networking::WinSock::{AF_INET, IPPROTO_TCP, SO_REUSEADDR, SOCK_STREAM, SOCKADDR_IN, SOCKET_ERROR, SOL_SOCKET, bind, setsockopt}, Storage::FileSystem::{ReadFile, WriteFile}, System::{Console::{self, GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE}, IO::{CreateIoCompletionPort, GetQueuedCompletionStatus, OVERLAPPED, PostQueuedCompletionStatus}}};

use crate::vm::io::{Event, FuId, IoError, IoErrorKind, IoOk, IoOp, IoResult, IoType, Reactor};

/// For Windows IOCP
#[derive(Debug)]
pub struct IocpReactor {
    iocp: HANDLE,
}

// OSのハンドルだから移動可能なはず。。
unsafe impl Send for IocpReactor {}
unsafe impl Sync for IocpReactor {}

impl Drop for IocpReactor {
    fn drop(&mut self) {
        unsafe {
            if !self.iocp.is_invalid() {
                let _ = CloseHandle(self.iocp);
            }
        }
    }
}

#[repr(C)]
struct IocpCtx {
    overlapped: OVERLAPPED,
    fu_id: FuId,
    io_type: IoType,
    iocp: HANDLE,
}

impl Reactor for IocpReactor {
    fn new() -> std::io::Result<IocpReactor> {
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

            Ok(IocpReactor { iocp })
        }
    }

    fn submit(&self, fu_id: FuId, op: &IoOp) -> IoResult {
        match op {
            IoOp::StdoutWrite { buf_ptr, len } => {
                unsafe {
                    // 標準出力ハンドル取得
                    let stdout = match GetStdHandle(STD_OUTPUT_HANDLE) {
                        Ok(handle) => handle,
                        Err(e) => {
                            return IoResult::Err(
                                IoError {
                                    kind: IoErrorKind::Other,
                                    raw_os_error: e.code().0 as i32,
                                    retryable: false,
                                }
                            );
                        }
                    };

                    // バッファの参照を取得
                    let buf = std::slice::from_raw_parts(*buf_ptr as *const u8, *len as usize);

                    let mut ctx = Box::new(IocpCtx {
                        overlapped: std::mem::zeroed(),
                        fu_id,
                        io_type: IoType::StdoutWrite,
                        iocp: self.iocp,
                    });

                    // IOCP に関連付け（必要なら）
                    let _ = CreateIoCompletionPort(stdout, Some(self.iocp), 0, 0);

                    let mut written = 0u32;
                    let res = WriteFile(
                        stdout,
                        Some(buf),
                        Some(&mut written),
                        Some(&mut ctx.overlapped as *mut OVERLAPPED),
                    );

                    // WriteFile は完了前に false を返すことがあるが、ERROR_IO_PENDING なら正常
                    if res.is_ok() {
                        // 即時終了
                        IoResult::Ok(
                            IoOk::StreamIo { len: written as u64 }
                        )
                    } else if windows::Win32::Foundation::GetLastError().0 == 997 {
                        // 非同期進行中
                        // 非同期にのったので回収するまでメモリを解放しないようにする
                        std::mem::forget(ctx);
                        IoResult::Pending
                    } else {
                        IoResult::Err(
                            IoError {
                                kind: IoErrorKind::Other,
                                raw_os_error: windows::Win32::Foundation::GetLastError().0 as i32,
                                retryable: false,
                            }
                        )
                    }
                }
            },
            IoOp::StdinRead { buf_ptr, len } => unsafe {
                let stdin = match windows::Win32::System::Console::GetStdHandle(STD_INPUT_HANDLE) {
                    Ok(h) => h,
                    Err(e) => {
                        return IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: e.code().0 as i32,
                            retryable: false,
                        });
                    }
                };

                // 以下パイプ/ファイルとしての stdin

                let buf = std::slice::from_raw_parts_mut(*buf_ptr as *mut u8, *len as usize);

                let mut ctx = Box::new(IocpCtx {
                    overlapped: std::mem::zeroed(),
                    fu_id,
                    io_type: IoType::StdinRead,
                    iocp: self.iocp,
                });

                // IOCP に関連付け
                let _ = CreateIoCompletionPort(stdin, Some(self.iocp), 0, 0);

                let mut read = 0u32;
                let res = ReadFile(
                    stdin,
                    Some(buf),
                    Some(&mut read),
                    Some(&mut ctx.overlapped as *mut OVERLAPPED),
                );

                if res.is_ok() {
                    // 即時完了
                    IoResult::Ok(IoOk::StreamIo { len: read as u64 })
                } else if GetLastError().0 == 997 {
                    // ERROR_IO_PENDING = 997
                    std::mem::forget(ctx); // 完了まで ctx をリークさせる
                    IoResult::Pending
                } else {
                    IoResult::Err(IoError {
                        kind: IoErrorKind::Other,
                        raw_os_error: GetLastError().0 as i32,
                        retryable: false,
                    })
                }
            }
            IoOp::Sleep { ms } => {
                unsafe {
                    let ctx = Box::new(IocpCtx {
                        overlapped: std::mem::zeroed(),
                        fu_id,
                        io_type: IoType::Sleep,
                        iocp: self.iocp,
                    });

                    let ctx_ptr = Box::into_raw(ctx);

                    let mut timer_handle = HANDLE::default();

                    // タイマーコールバック
                    unsafe extern "system" fn sleep_cb(param: *mut core::ffi::c_void, _fired: bool) {
                        let ctx = unsafe { &mut *(param as *mut IocpCtx) };

                        // Sleep 完了を IOCP にポスト
                        let _ = unsafe { PostQueuedCompletionStatus(
                            ctx.iocp,
                            0,
                            0,                                // completion_key はここでは0でもよい（fu_idは overlapped側に入ってる）
                            Some(&mut ctx.overlapped as *mut _),
                        ) };

                        // ここでは ctx を drop しない！ → 完了イベントで wait() 側が Box::from_raw する
                    }

                    let ok = windows::Win32::System::Threading::CreateTimerQueueTimer(
                        &mut timer_handle,
                        None,
                        Some(sleep_cb),
                        Some(ctx_ptr as *mut _),
                        *ms as u32,
                        0,
                        windows::Win32::System::Threading::WT_EXECUTEDEFAULT,
                    );

                    if ok.is_err() {
                        // タイマー作成失敗 → ctx を回収
                        let _ = Box::from_raw(ctx_ptr);
                        return IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                            retryable: false,
                        });
                    }

                    IoResult::Pending
                }
            }
            IoOp::TcpListen { ip_ptr, port, backlog, flags, family } => {
                unsafe {
                    let ctx = Box::new(IocpCtx {
                        overlapped: std::mem::zeroed(),
                        fu_id,
                        io_type: IoType::TcpListen,
                        iocp: self.iocp,
                    });

                    let ctx_ptr = Box::into_raw(ctx);

                    match family {
                        4 => {
                            let ok = match windows::Win32::Networking::WinSock::WSASocketW(
                                AF_INET.0 as i32, 
                                SOCK_STREAM.0 as i32, 
                                IPPROTO_TCP.0 as i32, 
                                None, 
                                0, 
                                0
                            ) {
                                Ok(s) => {s}
                                Err(e) => {
                                    // ソケット作成失敗 → ctx を回収
                                    let _ = Box::from_raw(ctx_ptr);
                                    return IoResult::Err(IoError {
                                        kind: IoErrorKind::Other,
                                        raw_os_error: e.code().0 as i32,
                                        retryable: false,
                                    });
                                }
                            }

                            let yes = 1i32;
                            let yes_bytes = yes.to_ne_bytes();
                            let ret = setsockopt(ok, SOL_SOCKET, SO_REUSEADDR, Some(&yes_bytes));

                            if ret == SOCKET_ERROR {
                                // ソケットオプション設定失敗 → ctx を回収
                                let _ = Box::from_raw(ctx_ptr);
                                return IoResult::Err(IoError {
                                    kind: IoErrorKind::Other,
                                    raw_os_error: windows::Win32::Foundation::GetLastError().0 as i32,
                                    retryable: false,
                                });
                            }

                            let mut addr = SOCKADDR_IN::default();
                            addr.sin_family = windows::Win32::Networking::WinSock::ADDRESS_FAMILY(AF_INET.0 as u16);
                            addr.sin_port = port.to_be(); // ネットワークバイトオーダー
                            addr.sin_addr.S_un.S_addr = (*ip_ptr as u32).to_be();

                            let ret = bind(
                                ok,
                                &addr as *const SOCKADDR_IN as *const windows::Win32::Networking::WinSock::SOCKADDR,
                                std::mem::size_of::<SOCKADDR_IN>() as i32
                            );

                            if ret == SOCKET_ERROR {
                                // bind失敗 → ctx を回収
                                let _ = Box::from_raw(ctx_ptr);
                                return IoResult::Err(IoError {
                                    kind: IoErrorKind::Other,
                                    raw_os_error: windows::Win32::Foundation::GetLastError().0 as i32,
                                    retryable: false,
                                });
                            }

                            let ret = windows::Win32::Networking::WinSock::listen(
                                ok,
                                *backlog as i32
                            );

                            if ret == SOCKET_ERROR {
                                // listen失敗 → ctx を回収
                                let _ = Box::from_raw(ctx_ptr);
                                return IoResult::Err(IoError {
                                    kind: IoErrorKind::Other,
                                    raw_os_error: windows::Win32::Foundation::GetLastError().0 as i32,
                                    retryable: false,
                                });
                            }

                            IoResult::Ok(IoOk::TcpListen { socket: ok.0 as u64  })
                        }
                    }



                    if ok.is_err() {
                        // 失敗 → ctx を回収
                        let _ = Box::from_raw(ctx_ptr);
                        return IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: std::io::Error::last_os_error().raw_os_error().unwrap_or(-1),
                            retryable: false,
                        });
                    }
                }
                _ => IoResult::Err(
                    IoError {
                        kind: IoErrorKind::NotImplYet,
                        raw_os_error: -1,
                        retryable: false,
                    }
                ),
            }
        }
    }

    fn wait(&self, timeout_ms: i64, max_events: usize, out: &mut Vec<Event>) {
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
                    if events == 0 { timeout } else { 0 },
                );

                if overlapped_ptr.is_null() {
                    // タイムアウト or ポートクローズ
                    if ok.is_err() {
                        // 必要なら last_error 見て IoError にする
                    }
                    break;
                }

                // ここに来たら overlapped_ptr は有効
                let ctx = Box::from_raw(overlapped_ptr as *mut IocpCtx);
                let fu_id = ctx.fu_id;
                let io_type = ctx.io_type;

                if ok.is_err() {
                    let err = std::io::Error::last_os_error();
                    out.push(Event {
                        fu_id,
                        result: IoResult::Err(IoError {
                            kind: IoErrorKind::Other, // map_os_errorにしてもOK
                            raw_os_error: err.raw_os_error().unwrap_or(-1),
                            retryable: false,
                        }),
                    });
                    events += 1;
                    continue;
                }

                match io_type {
                    IoType::StdoutWrite => {
                        out.push(Event {
                            fu_id,
                            result: IoResult::Ok(IoOk::StreamIo {
                                len: bytes_transferred as u64,
                            }),
                        });
                    }
                    IoType::Sleep => {
                        out.push(Event {
                            fu_id,
                            result: IoResult::Ok(IoOk::SleepDone),
                        });
                    },
                    IoType::StdinRead => {
                        out.push(Event {
                            fu_id,
                            result: IoResult::Ok(IoOk::StreamIo {
                                len: bytes_transferred as u64,
                            }),
                        });
                    }
                    _ => {
                        out.push(Event {
                            fu_id,
                            result: IoResult::Err(IoError {
                                kind: IoErrorKind::NotImplYet,
                                raw_os_error: -1,
                                retryable: false,
                            }),
                        });
                    }
                }

                events += 1;
            }
        }
    }

}