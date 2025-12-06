// use mikan_script::vm::{VMPool, pre_decoder::PreDecoder};

// fn main() {
//     let mut pool = VMPool::new();
//     let source = r#"
// MAIN
// CALL INIT
// TO: ADD_U64_IMMEDIATE r1 1
// LT_U64_JUMP r0 r1 r2 TO
// PRINT_U64 r1
// EXIT 0

// INIT
// LOAD_U64_IMMEDIATE r2 1000000000
// RET
// "#;
//     let decoder = PreDecoder::new();
//     let start = std::time::Instant::now();
//     let functions = decoder.decode(source).expect("decode succeeds");
//     pool.code_manager.set_functions(functions);
//     let duration = start.elapsed();
//     println!("Decoding took: {:?}", duration);
//     pool.run();
//     pool.wait_all();
// }

use std::time::Instant;

use mikan_script::vm::io::{
    FuId, IoEngine, IoId, IoOk, IoOp, IoResult, TcpListenFlags,
    windows::IocpReactor,
};

/// 特定の FuId の完了を待って、その結果を返すユーティリティ
fn wait_for_fu(engine: &mut IoEngine<IocpReactor>, target: FuId) -> IoResult {
    loop {
        // すでに完了済みのイベントを全部見る
        while let Some(ev) = engine.get_a_event() {
            if ev.fu_id == target {
                return ev.result;
            }
        }

        // まだ来てなければブロッキングで待つ
        engine.blocking_collect_events(-1, 128);
    }
}

fn main() {
    println!("== Async HTTP dump server (IoEngine / IocpReactor) ==");

    let mut engine = IoEngine::<IocpReactor>::new();

    // 0.0.0.0:8080 で listen
    let ip = u32::from_ne_bytes([0, 0, 0, 0]);
    let ip_ptr = &ip as *const u32 as u64;
    let port: u16 = 8080;
    let backlog: u16 = 128;

    // --- listen 開始 ---
    let fu_listen = engine.submit_op(IoOp::TcpListen {
        ip_ptr,
        port,
        backlog,
        flags: TcpListenFlags::REUSE_ADDR,
        family: 4, // IPv4
    });

    let listen_res = wait_for_fu(&mut engine, fu_listen);
    let listener_handle: IoId = match listen_res {
        IoResult::Ok(IoOk::NewHandle { handle }) => {
            println!("Listening on 127.0.0.1:{} (handle={})", port, handle);
            handle
        }
        IoResult::Err(e) => {
            eprintln!("TcpListen failed: {:?} (os={})", e.kind, e.raw_os_error);
            return;
        }
        _ => {
            eprintln!("TcpListen returned unexpected result: {:?}", listen_res);
            return;
        }
    };

    // --- accept 1回だけ ---
    let fu_accept = engine.submit_op(IoOp::TcpAccept {
        listener_handle,
    });

    println!("Waiting for one TCP connection...");

    let accept_res = wait_for_fu(&mut engine, fu_accept);
    let client_handle: IoId = match accept_res {
        IoResult::Ok(IoOk::NewHandle { handle }) => {
            println!("Accepted connection! client handle = {}", handle);
            handle
        }
        IoResult::Err(e) => {
            eprintln!("TcpAccept failed: {:?} (os={})", e.kind, e.raw_os_error);
            return;
        }
        _ => {
            eprintln!("TcpAccept returned unexpected result: {:?}", accept_res);
            return;
        }
    };

    // --- HTTP リクエストを 1 回だけ読む ---
    let mut buf = vec![0u8; 4096];
    let buf_ptr = buf.as_mut_ptr() as u64;
    let len = buf.len() as u64;

    let async_start = Instant::now();

    let fu_read = engine.submit_op(IoOp::Read {
        handle: client_handle,
        buf_ptr,
        len,
    });

    let read_res = wait_for_fu(&mut engine, fu_read);
    let read_len = match read_res {
        IoResult::Ok(IoOk::StreamIo { len }) => len as usize,
        IoResult::Err(e) => {
            eprintln!("Read failed: {:?} (os={})", e.kind, e.raw_os_error);
            return;
        }
        _ => {
            eprintln!("Read returned unexpected result: {:?}", read_res);
            return;
        }
    };

    let async_duration = async_start.elapsed();

    // 読み取ったバイト列をそのまま HTTP リクエスト文字列として表示
    let req_str = String::from_utf8_lossy(&buf[..read_len]);
    println!("=== HTTP request start ===");
    print!("{}", req_str);
    println!("=== HTTP request end ===");
    println!("Asynchronous socket read took: {:?}", async_duration);

    // --- HTTP レスポンスを返す（Write を submit）---

    // シンプルなテキストレスポンス
    let body = b"Hello from mikan-script async HTTP dump server!\r\n";
    let header = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Length: {}\r\n\
         Content-Type: text/plain; charset=utf-8\r\n\
         Connection: close\r\n\
         \r\n",
        body.len()
    );

    // ヘッダ + ボディ を 1つの Vec<u8> にまとめる
    let mut resp_buf = Vec::with_capacity(header.len() + body.len());
    resp_buf.extend_from_slice(header.as_bytes());
    resp_buf.extend_from_slice(body);

    // バッファへの生ポインタ
    let resp_ptr = resp_buf.as_ptr() as u64;
    let resp_len = resp_buf.len() as u64;

    let write_start = Instant::now();

    let fu_write = engine.submit_op(IoOp::Write {
        handle: client_handle,
        buf_ptr: resp_ptr,
        len: resp_len,
    });

    let write_res = wait_for_fu(&mut engine, fu_write);
    match write_res {
        IoResult::Ok(IoOk::StreamIo { len }) => {
            println!(
                "HTTP response sent ({} bytes) in {:?}",
                len,
                write_start.elapsed()
            );
        }
        IoResult::Err(e) => {
            eprintln!("Write failed: {:?} (os={})", e.kind, e.raw_os_error);
        }
        _ => {
            eprintln!("Write returned unexpected result: {:?}", write_res);
        }
    }

    println!("Done. Closing server.");
    // client_handle / listener_handle の Close は、今の実装なら Drop側 or OS に任せてOK
}
