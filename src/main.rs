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

use mikan_script::vm::io::{IoEngine, IoOp, windows::IocpReactor};

fn main() {

    println!("== Async stdin test (IoEngine / StdinRead) ==");

    // IOエンジンとリアクターの初期化
    let mut engine = IoEngine::<IocpReactor>::new();

    // 読み取り用バッファを用意（コンソールなら実質同期なので stack / Vec でも一応動く）
    let mut buf = vec![0u8; 1024];
    let buf_ptr = buf.as_mut_ptr() as u64;
    let len = buf.len() as u64;

    let async_start = Instant::now();

    // 非同期標準入力リクエストを1回投げる
    engine.submit_op(IoOp::StdinRead { buf_ptr, len });
    println!("{}", engine.events_holed_os());
    // 完了イベント待ち
    while 0 != engine.events_holed_os() {
            println!("eee");
        // timeout_ms や max_events は stdout と同じノリで
        engine.blocking_collect_events(100, 1024);
    }

    let async_duration = async_start.elapsed();

    // ここまで来た時点で buf に読み取ったデータが入っている想定
    // ヌル文字は入らないので、実際の長さだけを見つけてから String に変換する
    let read_len = buf.iter()
        .position(|&b| b == b'\n' || b == 0)
        .unwrap_or(buf.len());

    let line = String::from_utf8_lossy(&buf[..read_len]);

    println!("非同期ランタイム経由で読み取った内容: {:?}", line.trim_end());
    println!("Asynchronous stdin read took: {:?}", async_duration);
}