use mikan_script::vm::{pre_decoder::PreDecoder, VMPool, VM};

fn main() {
    let mut pool = VMPool::new();
    let source = r#"
MAIN
CALL INIT
ADD_U64_IMMEDIATE r1 1
LT_U64_JUMP r0 r1 r2 1
PRINT_U64 r1
EXIT 0

INIT
LOAD_U64_IMMEDIATE r2 1000000000
RET
"#;
    let decoder = PreDecoder::new();
    let functions = decoder.decode(source).expect("decode succeeds");
    pool.code_manager.latest_function_table.write().unwrap().extend(functions.into_iter().map(|f| mikan_script::vm::FunctionPtr(Box::into_raw(Box::new(f)))));
    let vm = VM::new();
    pool.push_and_run_threaded(vm,false);
    pool.wait_all();
}