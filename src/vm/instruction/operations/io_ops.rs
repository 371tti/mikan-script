use crate::vm::{instruction::operations::Operations, io::{FuId, IoOk, IoOp, IoResult, TcpListenFlags}, memory::VPtr, vm::VM};


/// IO操作
impl Operations {
    // /// 整数の出力
    // /// 2 word instruction
    // /// ol[0]: src register index
    // #[inline(always)]
    // pub fn print_u64(vm: &mut VM) {
    //     unsafe {
    //         let ol = vm.next_operand();
    //         let src = ol[0] as usize;
    //         let r = vm.st.r.as_mut_ptr();
    //         println!("{}", *r.add(src));
    //     }
    //     vm.next_step();
    // }

    /// allocate memory
    /// allocate *size + add_size, store id in *id_res_reg
    /// ol[0]: size reg
    /// ol[1]: id res reg
    /// oh: immediate add_size
    #[inline(always)]
    pub fn alloc(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let size_reg = ol[0] as usize;
            let ptr_reg = ol[1] as usize;
            let add_size = vm.next_operand_imm();
            let r = vm.st.r.as_mut_ptr();
            let size = (*r.add(size_reg)).wrapping_add(add_size) as usize;
            let v_ptr = vm.st.mem.alloc_heep(size);
            *r.add(ptr_reg) = v_ptr.0;
        }
        vm.next_step();
    }

    /// reallocate memory
    /// 2 word instruction
    /// ol[0]: size reg
    /// ol[1]: id reg
    #[inline(always)]
    pub fn realloc(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let size_reg = ol[0] as usize;
            let ptr_reg = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let size = *r.add(size_reg) as usize;
            let v_ptr = VPtr(*r.add(ptr_reg));
            vm.st.mem.realloc_heep(v_ptr, size);
        }
        vm.next_step();
    }

    /// deallocate memory
    /// 2 word instruction
    /// ol[0]: id reg
    #[inline(always)]
    pub fn dealloc(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let ptr_reg = ol[0] as usize;
            let r = vm.st.r.as_mut_ptr();
            let v_ptr = VPtr(*r.add(ptr_reg));
            vm.st.mem.dealloc_heep(v_ptr);
        }
        vm.next_step();
    }
}

/// Async IO
impl Operations {
    /// set io operation
    /// ol[0]: fu_id reg
    /// ol[1]: order_type
    /// ol[2..]: order specific operands
    #[inline(always)]
    pub fn set_io(vm: &mut VM) {
        unsafe {
            let r = vm.st.r.as_mut_ptr();
            let ol = vm.next_operand();
            let fu_id_reg = ol[0];
            let order_type = ol[1];


            let fu_id: FuId = match order_type {
                1 => { // std io write: ol[2]: buf_ptr reg, ol[3]: len reg
                    let v_ptr = VPtr(*r.add(ol[2] as usize));
                    let buf_ptr = vm.st.mem.as_ptr(v_ptr) as u64;
                    let len = *r.add(ol[3] as usize);
                    vm.io.submit_op(
                        IoOp::StdoutWrite { buf_ptr, len }
                    )
                },
                2 => { // std io read: ol[2]: buf_ptr reg, ol[3]: len reg
                    let v_ptr = VPtr(*r.add(ol[2] as usize));
                    let buf_ptr = vm.st.mem.as_ptr(v_ptr) as u64;
                    let len = *r.add(ol[3] as usize);
                    vm.io.submit_op(
                        IoOp::StdinRead { buf_ptr, len }
                    )
                },
                3 => { // sleep: ol[2]: duration reg ms
                    let ms = *r.add(ol[2] as usize);
                    vm.io.submit_op(
                        IoOp::Sleep { ms }
                    )
                },
                4 => { // read: ol[2]: handle reg, ol[3]: buf_ptr reg, ol[4]: len reg
                    let handle = *r.add(ol[2] as usize);
                    let v_ptr = VPtr(*r.add(ol[3] as usize));
                    let buf_ptr = vm.st.mem.as_ptr(v_ptr) as u64;
                    let len = *r.add(ol[4] as usize);
                    vm.io.submit_op(
                        IoOp::Read { handle, buf_ptr, len }
                    )
                },
                5 => { // write: ol[2]: handle reg, ol[3]: buf_ptr reg, ol[4]: len reg
                    let handle = *r.add(ol[2] as usize);
                    let v_ptr = VPtr(*r.add(ol[3] as usize));
                    let buf_ptr = vm.st.mem.as_ptr(v_ptr) as u64;
                    let len = *r.add(ol[4] as usize);
                    vm.io.submit_op(
                        IoOp::Write { handle, buf_ptr, len }
                    )
                },
                6 => { // tcp listen: ol[2]: ip_ptr reg, ol[3]: port reg, ol[4]: backlog reg, ol[5]: flags reg, ol[6]: family reg
                    let ip_v_ptr = VPtr(*r.add(ol[2] as usize));
                    let ip_ptr = vm.st.mem.as_ptr(ip_v_ptr) as u64;
                    let port = *r.add(ol[3] as usize) as u16;
                    let backlog = *r.add(ol[4] as usize) as u16;
                    let flags = *r.add(ol[5] as usize) as u16;
                    let family = *r.add(ol[6] as usize) as u8;
                    vm.io.submit_op(
                        IoOp::TcpListen {
                            ip_ptr,
                            port,
                            backlog,
                            flags: TcpListenFlags::from_bits_truncate(flags),
                            family,
                        }
                    )
                },
                7 => { // tcp connect: ol[2]: ip_ptr reg, ol[3]: port reg, ol[4]: flags reg, ol[5]: family reg
                    let ip_v_ptr = VPtr(*r.add(ol[2] as usize));
                    let ip_ptr = vm.st.mem.as_ptr(ip_v_ptr) as u64;
                    let port = *r.add(ol[3] as usize) as u16;
                    let flags = *r.add(ol[4] as usize) as u16;
                    let family = *r.add(ol[5] as usize) as u8;
                    vm.io.submit_op(
                        IoOp::TcpConnect {
                            ip_ptr,
                            port,
                            flags: TcpListenFlags::from_bits_truncate(flags),
                            family,
                        }
                    )
                },
                8 => { // tcp accept: ol[2]: listener_handle reg
                    let listener_handle = *r.add(ol[2] as usize);
                    vm.io.submit_op(
                        IoOp::TcpAccept { listener_handle }
                    )
                },
                9 => { // socket shutdown: ol[2]: socket_handle reg
                    let handle = *r.add(ol[2] as usize);
                    vm.io.submit_op(
                        IoOp::Shutdown { handle }
                    )
                },
                10 => { // get random bytes: ol[2]: buf_ptr reg, ol[3]: len reg
                    let v_ptr = VPtr(*r.add(ol[2] as usize));
                    let buf_ptr = vm.st.mem.as_ptr(v_ptr) as u64;
                    let len = *r.add(ol[3] as usize);
                    vm.io.submit_op(
                        IoOp::RandomBytes { buf_ptr, len }
                    )
                },
                11 => { // get time now
                    vm.io.submit_op(
                        IoOp::TimeNow
                    )
                },
                _ => {
                    panic!("Invalid IO order type: {}", order_type);
                }
            };

            *r.add(fu_id_reg as usize) = fu_id;
        }
        vm.next_step();
    }

    /// wait for IO
    /// 
    /// blocking instruction
    /// collect events
    /// ol[0]: timeout ms reg (-1 = infinite)
    /// ol[1]: max events reg
    /// ol[3]: result num reg
    #[inline(always)]
    pub fn wait_io(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let timeout_ms_reg = ol[0] as usize;
            let max_events_reg = ol[1] as usize;
            let r = vm.st.r.as_mut_ptr();
            let timeout_ms = *r.add(timeout_ms_reg) as i64;
            let max_events = *r.add(max_events_reg) as usize;
            let num = vm.io.blocking_collect_events(timeout_ms, max_events);
            *r.add(ol[2] as usize) = num as u64;
        }
        vm.next_step();
    }

    /// get an IO completed event
    /// 
    /// ol[0]: result fu_id reg
    /// - 0: if no event
    /// - n: fu_id of completed event
    /// ol[1]: result type reg
    /// - 0: pending, impossible
    /// - 1: stream io ok
    /// - 2: new handkle ok
    /// - 3: sleep ok
    /// - 4: time now ok
    /// - 5: simple ok
    /// - negative value: error code
    /// ol[2]: choice by result type
    /// 1. len for any read/write
    /// 2. handle id for socket operations
    /// 4. low for time now (u64 bits)
    /// -n. os raw error code
    /// ol[3]: choice by result type
    /// 4. high for time now (u64 bits)
    /// -n. err retryble flag (1 = retryable, 0 = not retryable)
    /// 
    /// if not have change register, it instructions won't change its values.
    #[inline(always)]
    pub fn get_an_io(vm: &mut VM) {
        unsafe {
            let ol = vm.next_operand();
            let fu_id_res_reg = ol[0] as usize;
            let type_res_reg = ol[1] as usize;
            let choice_res_reg_1 = ol[2] as usize;
            let choice_res_reg_2 = ol[3] as usize;
            let r = vm.st.r.as_mut_ptr();

            if let Some(event) = vm.io.get_a_event() {
                *r.add(fu_id_res_reg) = event.fu_id;
                match event.result {
                    IoResult::Pending => {
                        // should not happen
                        *r.add(type_res_reg) = 0;
                    },
                    IoResult::Ok(ok) => {
                        match ok {
                            IoOk::Simple => {
                                *r.add(type_res_reg) = 5;
                            }
                            IoOk::SleepDone => {
                                *r.add(type_res_reg) = 3;
                            }
                            IoOk::StreamIo { len } => {
                                *r.add(type_res_reg) = 1;
                                *r.add(choice_res_reg_1) = len;
                            }
                            IoOk::NewHandle { handle } => {
                                *r.add(type_res_reg) = 2;
                                *r.add(choice_res_reg_1) = handle;
                            }
                            IoOk::TimeNow { low, high } => {
                                *r.add(type_res_reg) = 4;
                                *r.add(choice_res_reg_1) = low;
                                *r.add(choice_res_reg_2) = high;
                            }
                        }
                    },
                    IoResult::Err(err) => {
                        *r.add(type_res_reg) = -(err.kind as i64) as u64;
                        *r.add(choice_res_reg_1) = if err.retryable { 1 } else { 0 };
                    },
                }
            } else {
                // no event
                *r.add(fu_id_res_reg) = 0;
            }
        }
        
        vm.next_step();
    }
}

