use mikan_script::vm::{VM, VMPool};

fn main() {
    let mut pool = VMPool::new();
    let vm = VM::new();
    pool.code_manager.set_test2();
    pool.push_and_run_threaded(vm, 0, false);
    pool.wait_all();
}