
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
    fn submit(&self, fu_id: FuId, op: &IoOp) -> std::io::Result<Option<IoResult>> {
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
                    if res.is_ok() {
                        // 即時終了
                        Ok(Some(
                            IoResult::StreamIo {
                                len: written as u64,
                                status: 0,
                            }
                        ))
                    } else if windows::Win32::Foundation::GetLastError().0 == 997 {
                        // 非同期進行中
                        Ok(None)
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

            println!("waiting IOCP events...");

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

                println!("IOCP event received: ok={:?}, bytes_transferred={}, completion_key={}, overlapped_ptr={:?}", ok, bytes_transferred, completion_key, overlapped_ptr);

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
