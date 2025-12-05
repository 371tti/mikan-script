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

use mikan_script::vm::io::{IoEngine, IocpReactor, IoOp, Reactor};

fn main() {
    // テスト文字列
    let msg = b"Async Hello, World! by IOCP!\n";
    let buf_ptr = msg.as_ptr() as u64;
    let len = msg.len() as u64;

    // IOCPリアクター生成
    let reactor = IocpReactor::new().unwrap();
    let mut engine = IoEngine::new(reactor);

    // 非同期標準出力リクエスト
    let fu_id = engine.submit_op(IoOp::StdoutWrite { buf_ptr, len });

    // 完了イベント待ち
    if let Some((done_fu_id, result)) = engine.wait_a_event(1000000) {
        println!("fu_id={} completed: {:?}", done_fu_id, result);
    } else {
        println!("タイムアウトまたは失敗");
    }
}