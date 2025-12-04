use mikan_script::vm::{VMPool, pre_decoder::PreDecoder};

fn main() {
    let mut pool = VMPool::new();
    let source = r#"
MAIN
CALL INIT
TO: ATOMIC_ADD_U64 r8 r3 r0 r1
ATOMIC_LOAD_U64 r3 r0 r4
LT_U64_JUMP r0 r4 r2 TO
PRINT_U64 r4
EXIT 0

INIT
ALLOC r0 r3 1
ADD_U64_IMMEDIATE r1 1
LOAD_U64_IMMEDIATE r2 1000000000
STORE_U64 r3 r0 0
RET
"#;
    let decoder = PreDecoder::new();
    let start = std::time::Instant::now();
    let functions = decoder.decode(source).expect("decode succeeds");
    pool.code_manager.set_functions(functions);
    let duration = start.elapsed();
    println!("Decoding took: {:?}", duration);
    pool.run();
    pool.wait_all();
}

