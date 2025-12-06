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
        while let Some(ev) = engine.get_a_event() {
            if ev.fu_id == target {
                return ev.result;
            }
        }
        engine.blocking_collect_events(-1, 128);
    }
}

fn main() {
    println!("== Async HTTP client via TcpConnect (IoEngine / IocpReactor) ==");

    let mut engine = IoEngine::<IocpReactor>::new();

    // 127.0.0.1:8080 に接続
    let ip = u32::from_ne_bytes([127, 0, 0, 1]);
    let ip_ptr = &ip as *const u32 as u64;
    let port: u16 = 8080u16;

    let fu_conn = engine.submit_op(IoOp::TcpConnect {
        ip_ptr,
        port,
        flags: TcpListenFlags::empty(),
        family: 4,
    });

    let conn_res = wait_for_fu(&mut engine, fu_conn);
    let sock: IoId = match conn_res {
        IoResult::Ok(IoOk::NewHandle { handle }) => {
            println!("Connected! socket handle = {}", handle);
            handle
        }
        e => {
            println!("connect failed: {:?}", e);
            return;
        }
    };

    // ---- HTTP リクエスト送信 ----
    let req = b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    let mut send_buf = req.to_vec();
    let send_ptr = send_buf.as_ptr() as u64;
    let send_len = send_buf.len() as u64;

    let write_start = Instant::now();
    let fu_write = engine.submit_op(IoOp::Write {
        handle: sock,
        buf_ptr: send_ptr,
        len: send_len,
    });

    let write_res = wait_for_fu(&mut engine, fu_write);
    match write_res {
        IoResult::Ok(IoOk::StreamIo { len }) => {
            println!(
                "HTTP request sent ({} bytes) in {:?}",
                len,
                write_start.elapsed()
            );
        }
        other => {
            println!("write failed: {:?}", other);
            return;
        }
    }

    // ---- レスポンス受信 ----
    let mut recv_buf = vec![0u8; 4096];
    let recv_ptr = recv_buf.as_mut_ptr() as u64;
    let recv_len = recv_buf.len() as u64;

    let read_start = Instant::now();
    let fu_read = engine.submit_op(IoOp::Read {
        handle: sock,
        buf_ptr: recv_ptr,
        len: recv_len,
    });

    let read_res = wait_for_fu(&mut engine, fu_read);
    let n = match read_res {
        IoResult::Ok(IoOk::StreamIo { len }) => len as usize,
        other => {
            println!("read failed: {:?}", other);
            return;
        }
    };

   println!("Read {} bytes in {:?}", n, read_start.elapsed());
print!("Raw bytes =");
for b in &recv_buf[..n] {
    print!(" {:02X}", b);
}
println!();

let resp = String::from_utf8_lossy(&recv_buf[..n]);
println!("=== HTTP response (lossy) ===\n{}\n=====================", resp);

}
