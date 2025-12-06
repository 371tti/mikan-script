

use core::ffi::c_void;
use std::{slice, sync::Once};
use windows::{
    Win32::{
        Foundation::{
            CloseHandle, ERROR_IO_PENDING, GetLastError, HANDLE, INVALID_HANDLE_VALUE
        },
        Networking::WinSock::{
            AF_INET, IPPROTO_TCP, LPFN_ACCEPTEX, SIO_GET_EXTENSION_FUNCTION_POINTER, SO_REUSEADDR, SOCK_STREAM, SOCKADDR_IN, SOCKET, SOCKET_ERROR, SOL_SOCKET, WSA_FLAG_OVERLAPPED, WSA_IO_PENDING, WSADATA, WSAGetLastError, WSAID_ACCEPTEX, WSAIoctl, WSASocketW, WSAStartup, bind, closesocket, listen, setsockopt
        },
        Security::Cryptography::{BCRYPT_ALG_HANDLE, BCRYPT_RNG_ALGORITHM, BCRYPT_USE_SYSTEM_PREFERRED_RNG, BCRYPTGENRANDOM_FLAGS, BCryptGenRandom, BCryptOpenAlgorithmProvider},
        Storage::FileSystem::{ReadFile, WriteFile},
        System::{
            Console::{GetStdHandle, STD_INPUT_HANDLE, STD_OUTPUT_HANDLE},
            IO::{CreateIoCompletionPort, GetQueuedCompletionStatus, OVERLAPPED, PostQueuedCompletionStatus},
            SystemInformation::GetSystemTimePreciseAsFileTime,
            Threading::{CreateTimerQueueTimer, WT_EXECUTEDEFAULT},
        },
    }, core::GUID
};

use crate::vm::io::{
    Event, FuId, IoError, IoErrorKind, IoOk, IoOp, IoResult, IoType, Reactor,
};

static WSA_INIT: Once = Once::new();
static mut WSA_INIT_ERR: i32 = 0;

fn ensure_wsa_initialized() -> std::io::Result<()> {
    WSA_INIT.call_once(|| {
        unsafe {
            let mut data = WSADATA::default();
            // Winsock 2.2 を要求
            let ret = WSAStartup(0x0202, &mut data);
            if ret != 0 {
                WSA_INIT_ERR = ret;
            }
        }
    });

    unsafe {
        if WSA_INIT_ERR != 0 {
            Err(std::io::Error::from_raw_os_error(WSA_INIT_ERR))
        } else {
            Ok(())
        }
    }
}

/// For Windows IOCP
/// 雑テスト済みメソッド
/// - Read
/// - Write
/// - StdoutWrite
/// - StdinRead
/// - Sleep
/// - TcpListen (IPv4 only)
/// - TcpAccept (IPv4 only)
/// - TimeNow
/// - RandomBytes
/// 
/// 未チェック
/// - TcpConnect
/// - Shutdown
#[derive(Debug)]
pub struct IocpReactor {
    iocp: HANDLE,
}

// OS ハンドルなので移動可能で OK
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

/// IOCP のコンテキスト
///
/// `OVERLAPPED` が先頭にあることだけが重要で、
/// それ以外は Rust 側専用の情報を詰め込んでいる。
#[repr(C)]
struct IocpCtx {
    overlapped: OVERLAPPED,
    fu_id: FuId,
    io_type: IoType,
    iocp: HANDLE,

    /// TcpAccept で使う accepted socket
    socket: Option<SOCKET>,

    /// AcceptEx 用のアドレスバッファ
    addr_buf: Option<Box<[u8]>>,
}

impl IocpCtx {
    fn new_basic(fu_id: FuId, io_type: IoType, iocp: HANDLE) -> Box<IocpCtx> {
        Box::new(IocpCtx {
            overlapped: unsafe { std::mem::zeroed() },
            fu_id,
            io_type,
            iocp,
            socket: None,
            addr_buf: None,
        })
    }
}

impl Reactor for IocpReactor {
    fn new() -> std::io::Result<IocpReactor> {
        // ここで Winsock を初期化
        ensure_wsa_initialized()?;
        unsafe {
            let iocp = CreateIoCompletionPort(
                INVALID_HANDLE_VALUE,
                None,
                0,
                0,
            )
            .map_err(|e| std::io::Error::from_raw_os_error(e.code().0 as i32))?;

            Ok(IocpReactor { iocp })
        }
    }

    fn submit(&self, fu_id: FuId, op: &IoOp) -> IoResult {
        match op {
            // -----------------------------
            // 標準出力
            // -----------------------------
            IoOp::StdoutWrite { buf_ptr, len } => {
                unsafe {
                    let stdout = match GetStdHandle(STD_OUTPUT_HANDLE) {
                        Ok(handle) => handle,
                        Err(e) => {
                            return IoResult::Err(IoError {
                                kind: IoErrorKind::Other,
                                raw_os_error: e.code().0 as i32,
                                retryable: false,
                            });
                        }
                    };

                    let buf = slice::from_raw_parts(*buf_ptr as *const u8, *len as usize);

                    let mut ctx = IocpCtx::new_basic(fu_id, IoType::StdoutWrite, self.iocp);

                    // IOCP 関連付け
                    let _ = CreateIoCompletionPort(stdout, Some(self.iocp), 0, 0);

                    let mut written = 0u32;
                    let res = WriteFile(
                        stdout,
                        Some(buf),
                        Some(&mut written),
                        Some(&mut ctx.overlapped as *mut OVERLAPPED),
                    );

                    if res.is_ok() {
                        // 即時完了
                        IoResult::Ok(IoOk::StreamIo { len: written as u64 })
                    } else {
                        let err = GetLastError();
                        if err == ERROR_IO_PENDING {
                            // 非同期進行中 → ctx をリーク（完了時に回収）
                            std::mem::forget(ctx);
                            IoResult::Pending
                        } else {
                            IoResult::Err(IoError {
                                kind: IoErrorKind::Other,
                                raw_os_error: err.0 as i32,
                                retryable: false,
                            })
                        }
                    }
                }
            }

            // -----------------------------
            // 標準入力
            // -----------------------------
            IoOp::StdinRead { buf_ptr, len } => unsafe {
                let stdin = match GetStdHandle(STD_INPUT_HANDLE) {
                    Ok(h) => h,
                    Err(e) => {
                        return IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: e.code().0 as i32,
                            retryable: false,
                        });
                    }
                };

                let buf = slice::from_raw_parts_mut(*buf_ptr as *mut u8, *len as usize);

                let mut ctx = IocpCtx::new_basic(fu_id, IoType::StdinRead, self.iocp);

                let _ = CreateIoCompletionPort(stdin, Some(self.iocp), 0, 0);

                let mut read = 0u32;
                let res = ReadFile(
                    stdin,
                    Some(buf),
                    Some(&mut read),
                    Some(&mut ctx.overlapped as *mut OVERLAPPED),
                );

                if res.is_ok() {
                    IoResult::Ok(IoOk::StreamIo { len: read as u64 })
                } else {
                    let err = GetLastError();
                    if err == ERROR_IO_PENDING {
                        std::mem::forget(ctx);
                        IoResult::Pending
                    } else {
                        IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: err.0 as i32,
                            retryable: false,
                        })
                    }
                }
            }

            // -----------------------------
            // 汎用 Read (ファイルハンドル想定)
            // -----------------------------
            IoOp::Read { handle, buf_ptr, len } => unsafe {
                let h = HANDLE(*handle as *mut c_void);
                let buf = slice::from_raw_parts_mut(*buf_ptr as *mut u8, *len as usize);

                let mut ctx = IocpCtx::new_basic(fu_id, IoType::Read, self.iocp);
                let _ = CreateIoCompletionPort(h, Some(self.iocp), 0, 0);

                let mut read = 0u32;
                let res = ReadFile(
                    h,
                    Some(buf),
                    Some(&mut read),
                    Some(&mut ctx.overlapped as *mut OVERLAPPED),
                );

                if res.is_ok() {
                    IoResult::Ok(IoOk::StreamIo { len: read as u64 })
                } else {
                    let err = GetLastError();
                    if err == ERROR_IO_PENDING {
                        std::mem::forget(ctx);
                        IoResult::Pending
                    } else {
                        IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: err.0 as i32,
                            retryable: false,
                        })
                    }
                }
            }

            // -----------------------------
            // 汎用 Write (ファイルハンドル想定)
            // -----------------------------
            IoOp::Write { handle, buf_ptr, len } => unsafe {
                let h = HANDLE(*handle as *mut c_void);
                let buf = slice::from_raw_parts(*buf_ptr as *const u8, *len as usize);

                let mut ctx = IocpCtx::new_basic(fu_id, IoType::Write, self.iocp);
                let _ = CreateIoCompletionPort(h, Some(self.iocp), 0, 0);

                let mut written = 0u32;
                let res = WriteFile(
                    h,
                    Some(buf),
                    Some(&mut written),
                    Some(&mut ctx.overlapped as *mut OVERLAPPED),
                );

                if res.is_ok() {
                    IoResult::Ok(IoOk::StreamIo { len: written as u64 })
                } else {
                    let err = GetLastError();
                    if err == ERROR_IO_PENDING {
                        std::mem::forget(ctx);
                        IoResult::Pending
                    } else {
                        IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: err.0 as i32,
                            retryable: false,
                        })
                    }
                }
            }

            // -----------------------------
            // Sleep (タイマキュー)
            // -----------------------------
            IoOp::Sleep { ms } => unsafe {
                unsafe extern "system" fn sleep_cb(param: *mut c_void, _fired: bool) {
                    let ctx = unsafe { &mut *(param as *mut IocpCtx) };
                    let _ = unsafe { PostQueuedCompletionStatus(
                        ctx.iocp,
                        0,
                        0,
                        Some(&mut ctx.overlapped as *mut OVERLAPPED),
                    ) };
                    // ctx の解放は wait() 側の Box::from_raw に任せる
                }

                let ctx = IocpCtx::new_basic(fu_id, IoType::Sleep, self.iocp);
                let ctx_ptr = Box::into_raw(ctx);

                let mut timer_handle: HANDLE = HANDLE::default();

                let ok = CreateTimerQueueTimer(
                    &mut timer_handle,
                    None,
                    Some(sleep_cb),
                    Some(ctx_ptr as *mut c_void),
                    *ms as u32,
                    0,
                    WT_EXECUTEDEFAULT,
                );

                if ok.is_ok() {
                    IoResult::Pending
                } else {
                    let _ = Box::from_raw(ctx_ptr);
                    IoResult::Err(IoError {
                        kind: IoErrorKind::Other,
                        raw_os_error: std::io::Error::last_os_error()
                            .raw_os_error()
                            .unwrap_or(-1),
                        retryable: false,
                    })
                }
            }

            // -----------------------------
            // TcpListen (IPv4 のみ)
            // -----------------------------
            IoOp::TcpListen { ip_ptr, port, backlog, flags: _flags, family } => unsafe {
                match family {
                    4 => {
                        use windows::Win32::Networking::WinSock::WSA_FLAG_OVERLAPPED;

                        let sock = match WSASocketW(
                            AF_INET.0 as i32,
                            SOCK_STREAM.0 as i32,
                            IPPROTO_TCP.0 as i32,
                            None,
                            0,
                            WSA_FLAG_OVERLAPPED, // ★ 必須
                        ) {
                            Ok(s) => s,
                            Err(e) => {
                                return IoResult::Err(IoError {
                                    kind: IoErrorKind::Other,
                                    raw_os_error: e.code().0 as i32,
                                    retryable: false,
                                });
                            }
                        };

                        // SO_REUSEADDRとかは今のままでOK

                        // INADDR_ANY テスト版（まずこれで動作確認すると良い）
                        let mut addr = SOCKADDR_IN::default();
                        addr.sin_family = AF_INET;
                        addr.sin_port = port.to_be();
                        addr.sin_addr.S_un.S_addr = 0;

                        if bind(
                            sock,
                            &addr as *const SOCKADDR_IN
                                as *const windows::Win32::Networking::WinSock::SOCKADDR,
                            std::mem::size_of::<SOCKADDR_IN>() as i32,
                        ) == SOCKET_ERROR
                        {
                            let err = WSAGetLastError();
                            closesocket(sock);
                            return IoResult::Err(IoError {
                                kind: IoErrorKind::Other,
                                raw_os_error: err.0,
                                retryable: false,
                            });
                        }

                        if listen(sock, *backlog as i32) == SOCKET_ERROR {
                            let err = WSAGetLastError();
                            closesocket(sock);
                            return IoResult::Err(IoError {
                                kind: IoErrorKind::Other,
                                raw_os_error: err.0,
                                retryable: false,
                            });
                        }

                        // ★ listen ソケットを IOCP に紐付け
                        let _ = CreateIoCompletionPort(
                            HANDLE(sock.0 as *mut c_void),
                            Some(self.iocp),
                            0,
                            0,
                        );

                        IoResult::Ok(IoOk::NewHandle { handle: sock.0 as u64 })
                    }
                    _ => {
                        IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: -1,
                            retryable: false,
                        })
                    }
                }
            }


            // -----------------------------
            // TcpAccept (AcceptEx + IOCP)
            // -----------------------------
            IoOp::TcpAccept { listener_handle } => unsafe {
                let listener = SOCKET(*listener_handle as usize);

                use windows::Win32::Networking::WinSock::WSA_FLAG_OVERLAPPED;

                // 受け側ソケットも overlapped で作る
                let accept_socket = match WSASocketW(
                    AF_INET.0 as i32,
                    SOCK_STREAM.0 as i32,
                    IPPROTO_TCP.0 as i32,
                    None,
                    0,
                    WSA_FLAG_OVERLAPPED, // ★
                ) {
                    Ok(s) => s,
                    Err(e) => {
                        return IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: e.code().0 as i32,
                            retryable: false,
                        });
                    }
                };

                const ADDR_SINGLE: usize = std::mem::size_of::<SOCKADDR_IN>() + 16;
                const ADDR_BUF_LEN: usize = ADDR_SINGLE * 2;

                let mut ctx = IocpCtx {
                    overlapped: std::mem::zeroed(),
                    fu_id,
                    io_type: IoType::TcpAccept,
                    iocp: self.iocp,
                    socket: Some(accept_socket),
                    addr_buf: Some(vec![0u8; ADDR_BUF_LEN].into_boxed_slice()),
                };
                let ctx_ptr: *mut IocpCtx = Box::into_raw(Box::new(ctx));

                // ★ AcceptEx 関数ポインタは listener に対して取得する
                let mut guid: GUID = WSAID_ACCEPTEX;
                let mut func: LPFN_ACCEPTEX = None;
                let mut bytes: u32 = 0;

                let r = WSAIoctl(
                    listener,
                    SIO_GET_EXTENSION_FUNCTION_POINTER,
                    Some(&mut guid as *mut _ as *mut _),
                    std::mem::size_of::<GUID>() as u32,
                    Some(&mut func as *mut _ as *mut _),
                    std::mem::size_of::<LPFN_ACCEPTEX>() as u32,
                    &mut bytes as *mut u32,
                    None,
                    None,
                );

                if r == SOCKET_ERROR {
                    let err = WSAGetLastError();
                    let _ = Box::from_raw(ctx_ptr);
                    closesocket(accept_socket);
                    return IoResult::Err(IoError {
                        kind: IoErrorKind::Other,
                        raw_os_error: err.0,
                        retryable: false,
                    });
                }

                let accept_ex = match func {
                    Some(f) => f,
                    None => {
                        let _ = Box::from_raw(ctx_ptr);
                        closesocket(accept_socket);
                        return IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: -1,
                            retryable: false,
                        });
                    }
                };

                let addr_buf_ptr = (*ctx_ptr)
                    .addr_buf
                    .as_mut()
                    .unwrap()
                    .as_mut_ptr() as *mut core::ffi::c_void;

                let mut bytes = 0u32;

                let res = accept_ex(
                    listener,
                    accept_socket,
                    addr_buf_ptr,
                    0,
                    ADDR_SINGLE as u32,
                    ADDR_SINGLE as u32,
                    &mut bytes,
                    &mut (*ctx_ptr).overlapped,
                );

                if res.as_bool() {
                    // 同期完了しても IOCP にもキューされるので Pending 扱いでいい
                    IoResult::Pending
                } else {
                    let err = WSAGetLastError();
                    if err != windows::Win32::Networking::WinSock::WSA_ERROR(WSA_IO_PENDING.0 as i32) {
                        let _ = Box::from_raw(ctx_ptr);
                        closesocket(accept_socket);
                        return IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: err.0,
                            retryable: false,
                        });
                    }
                    IoResult::Pending
                }
            }

            IoOp::Shutdown { handle } => unsafe {
                let socket = SOCKET(*handle as usize);
                let res = windows::Win32::Networking::WinSock::shutdown(socket, windows::Win32::Networking::WinSock::SD_BOTH);
                if res == SOCKET_ERROR {
                    let err = WSAGetLastError();
                    IoResult::Err(IoError {
                        kind: IoErrorKind::Other,
                        raw_os_error: err.0,
                        retryable: false,
                    })
                } else {
                    IoResult::Ok(IoOk::StreamIo { len: 0 })
                }
            }


            // -----------------------------
            // 現在時刻（FILETIME そのまま返す）
            // -----------------------------
            IoOp::TimeNow => unsafe {
                let ft = GetSystemTimePreciseAsFileTime();
                IoResult::Ok(IoOk::TimeNow {
                    low: ft.dwLowDateTime as u64,
                    high: ft.dwHighDateTime as u64,
                })
            }

            // -----------------------------
            // 暗号学的乱数
            // -----------------------------
            IoOp::RandomBytes { buf_ptr, len } => unsafe {
                let buf = slice::from_raw_parts_mut(*buf_ptr as *mut u8, *len as usize);

                let status = BCryptGenRandom(
                    None,
                    buf,
                    BCRYPT_USE_SYSTEM_PREFERRED_RNG,
                );

                if status.is_ok() {
                    IoResult::Ok(IoOk::StreamIo { len: *len })
                } else {
                    IoResult::Err(IoError {
                        kind: IoErrorKind::Other,
                        raw_os_error: status.0 as i32,
                        retryable: false,
                    })
                }
            }

            // -----------------------------
            // TcpConnect はまだ
            // -----------------------------
            IoOp::TcpConnect { .. } => IoResult::Err(IoError {
                kind: IoErrorKind::NotImplYet,
                raw_os_error: -1,
                retryable: false,
            }),
        }
    }

    fn wait(&self, timeout_ms: i64, max_events: usize, out: &mut Vec<Event>) {
        unsafe {
            let timeout = if timeout_ms < 0 {
                u32::MAX
            } else {
                timeout_ms as u32
            };

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
                        // 必要ならここで IoError を飛ばす選択もある
                    }
                    break;
                }

                // ここに来たら overlapped_ptr は IocpCtx に対応している
                let ctx: Box<IocpCtx> = Box::from_raw(overlapped_ptr as *mut IocpCtx);
                let fu_id = ctx.fu_id;
                let io_type = ctx.io_type;
                let socket = ctx.socket;

                if ok.is_err() {
                    let err = std::io::Error::last_os_error();
                    out.push(Event {
                        fu_id,
                        result: IoResult::Err(IoError {
                            kind: IoErrorKind::Other,
                            raw_os_error: err.raw_os_error().unwrap_or(-1),
                            retryable: false,
                        }),
                    });
                    events += 1;
                    continue;
                }

                match io_type {
                    IoType::StdoutWrite
                    | IoType::StdinRead
                    | IoType::Read
                    | IoType::Write => {
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
                    }
                    IoType::TcpAccept => {
                        if let Some(sock) = socket {
                            // 受け付けたソケットを IOCP に登録
                            let _ = CreateIoCompletionPort(
                                HANDLE(sock.0 as *mut c_void),
                                Some(self.iocp),
                                0,
                                0,
                            );

                            out.push(Event {
                                fu_id,
                                result: IoResult::Ok(IoOk::NewHandle {
                                    handle: sock.0 as u64,
                                }),
                            });
                        } else {
                            out.push(Event {
                                fu_id,
                                result: IoResult::Err(IoError {
                                    kind: IoErrorKind::Other,
                                    raw_os_error: -1,
                                    retryable: false,
                                }),
                            });
                        }
                    }
                    // ここに来るのは想定外（TimeNow/RandomBytes は IOCP 経由しない）
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
