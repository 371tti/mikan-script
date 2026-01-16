
use std::fs;
use std::io::{self, Read};
use std::sync::Arc;

use fovia_vm::vm::{VMPool, pre_decoder::PreDecoder};

fn main() {
    let pool = VMPool::new();
    let source = read_source();
    let decoder = PreDecoder::new();
    let functions = decoder.decode(&source, pool.memory.as_ref()).expect("decode succeeds");
    pool.code_manager.set_functions(functions);
    let arc_pool = Arc::new(pool);
    arc_pool.run();
    arc_pool.wait_all();
}

fn read_source() -> String {
    let mut args = std::env::args().skip(1);
    if let Some(path) = args.next() {
        if path == "-" {
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .expect("failed to read stdin");
            return buf;
        }
        return fs::read_to_string(path).expect("failed to read asm file");
    }

    if let Ok(exe) = std::env::current_exe() {
        let candidate = exe.with_extension("fvo");
        if candidate.exists() {
            return fs::read_to_string(candidate).expect("failed to read bundled fvo");
        }
    }

    fs::read_to_string("program.asm").expect("failed to read program.asm")
}