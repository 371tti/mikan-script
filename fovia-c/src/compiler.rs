use std::collections::{HashMap, HashSet};

use crate::optimizer::AsmOptimizer;

use crate::ast::{BinOp, Block, Expr, Function, LoopExpr, MatchExpr, MatchPattern, Program, Stmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParamKind {
    StaticStr,
    Value,
}

#[derive(Debug, Clone)]
struct ParamLayout {
    name: String,
    kind: ParamKind,
    reg: u8,
}

struct DataItem {
    name: String,
    bytes: Vec<u8>,
}

struct LoopContext {
    label: Option<String>,
    end_label: String,
    result_reg: u8,
}

struct FnContext {
    zero_reg: u8,
    saved_reg: u8,
    next_reg: u8,
    free_list: Vec<u8>,
    temp_regs: HashSet<u8>,
    vars: HashMap<String, u8>,
    param_meta: HashMap<String, ParamLayout>,
    loop_stack: Vec<LoopContext>,
    label_counter: usize,
}

impl FnContext {
    fn new(layouts: Vec<ParamLayout>) -> Self {
        let zero_reg = 255u8;
        let saved_reg = 254u8;
        let mut vars = HashMap::new();
        let mut param_meta = HashMap::new();
        let mut max_reg = 0u8;
        for layout in layouts {
            vars.insert(layout.name.clone(), layout.reg);
            param_meta.insert(layout.name.clone(), layout.clone());
            max_reg = max_reg.max(layout.reg + match layout.kind {
                ParamKind::StaticStr => 1,
                ParamKind::Value => 0,
            });
        }
        let mut next_reg = max_reg.saturating_add(1);
        while next_reg == zero_reg || next_reg == saved_reg {
            next_reg = next_reg.saturating_add(1);
        }
        Self {
            zero_reg,
            saved_reg,
            next_reg,
            free_list: Vec::new(),
            temp_regs: HashSet::new(),
            vars,
            param_meta,
            loop_stack: Vec::new(),
            label_counter: 0,
        }
    }

    fn alloc_reg(&mut self) -> u8 {
        if let Some(reg) = self.free_list.pop() {
            return reg;
        }
        let mut reg = self.next_reg;
        while reg == self.zero_reg || reg == self.saved_reg {
            reg = reg.saturating_add(1);
        }
        self.next_reg = reg.saturating_add(1);
        reg
    }

    fn alloc_temp(&mut self) -> u8 {
        let reg = self.alloc_reg();
        self.temp_regs.insert(reg);
        reg
    }

    fn alloc_var(&mut self) -> u8 {
        let reg = self.alloc_reg();
        self.temp_regs.remove(&reg);
        reg
    }

    fn is_temp(&self, reg: u8) -> bool {
        self.temp_regs.contains(&reg)
    }

    fn free_temp(&mut self, reg: u8) {
        if reg == self.zero_reg || reg == self.saved_reg {
            return;
        }
        if self.temp_regs.remove(&reg) {
            self.free_list.push(reg);
        }
    }

    fn fresh_label(&mut self, prefix: &str) -> String {
        let label = format!("{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }
}

pub struct Compiler {
    program: Program,
    data: Vec<DataItem>,
    data_map: HashMap<Vec<u8>, String>,
}

impl Compiler {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            data: Vec::new(),
            data_map: HashMap::new(),
        }
    }

    pub fn compile(&mut self) -> Result<String, String> {
        let signatures = self.collect_signatures();
        let mut out = String::new();
        let functions = self.program.functions.clone();
        let impl_functions = self.collect_impl_functions();

        for func in &functions {
            if func.name == "main" {
                continue;
            }
            self.emit_function(func, &signatures, &mut out)?;
            out.push('\n');
        }

        for func in &impl_functions {
            self.emit_function(func, &signatures, &mut out)?;
            out.push('\n');
        }

        if let Some(main_fn) = functions.iter().find(|f| f.name == "main") {
            self.emit_function(main_fn, &signatures, &mut out)?;
        }

        let mut data_section = String::new();
        for item in &self.data {
            data_section.push_str(&format!("DATA {} {}\n", item.name, format_bytes(&item.bytes)));
        }

        let asm = format!("{data_section}\n{out}");
        Ok(AsmOptimizer::optimize(&asm))
    }

    fn collect_signatures(&self) -> HashMap<String, Vec<ParamLayout>> {
        let mut map = HashMap::new();
        for func in &self.program.functions {
            let mut layouts = Vec::new();
            let mut reg = 0u8;
            for param in &func.params {
                let kind = if param.ty.trim().eq_ignore_ascii_case("static str") {
                    ParamKind::StaticStr
                } else {
                    ParamKind::Value
                };
                layouts.push(ParamLayout {
                    name: param.name.clone(),
                    kind,
                    reg,
                });
                reg = match kind {
                    ParamKind::StaticStr => reg.saturating_add(2),
                    ParamKind::Value => reg.saturating_add(1),
                };
            }
            map.insert(func.name.clone(), layouts);
        }

        for imp in &self.program.impls {
            for method in &imp.methods {
                let mut layouts = Vec::new();
                let mut reg = 0u8;
                for param in &method.params {
                    let kind = if param.ty.trim().eq_ignore_ascii_case("static str") {
                        ParamKind::StaticStr
                    } else {
                        ParamKind::Value
                    };
                    layouts.push(ParamLayout {
                        name: param.name.clone(),
                        kind,
                        reg,
                    });
                    reg = match kind {
                        ParamKind::StaticStr => reg.saturating_add(2),
                        ParamKind::Value => reg.saturating_add(1),
                    };
                }
                map.insert(mangle_impl_method(&imp.name, &method.name), layouts);
            }
        }
        map
    }

    fn collect_impl_functions(&self) -> Vec<Function> {
        let mut out = Vec::new();
        for imp in &self.program.impls {
            for method in &imp.methods {
                let mut func = method.clone();
                func.name = mangle_impl_method(&imp.name, &method.name);
                out.push(func);
            }
        }
        out
    }

    fn emit_function(
        &mut self,
        func: &Function,
        signatures: &HashMap<String, Vec<ParamLayout>>,
        out: &mut String,
    ) -> Result<(), String> {
        let name = func.name.to_ascii_uppercase();
        out.push_str(&format!("{name}\n"));

        let layouts = signatures.get(&func.name).cloned().unwrap_or_default();
        let mut ctx = FnContext::new(layouts);

        out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", ctx.zero_reg));

        for stmt in &func.body {
            self.emit_stmt(stmt, signatures, &mut ctx, out)?;
        }

        if name == "MAIN" {
            out.push_str("    LOAD_U64_IMMEDIATE r0 0\n");
            out.push_str("    EXIT r0\n");
        } else {
            out.push_str("    RET\n");
        }
        Ok(())
    }

    fn emit_stmt(
        &mut self,
        stmt: &Stmt,
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<(), String> {
        match stmt {
            Stmt::Call { name, args } => self.emit_call(name, args, signatures, ctx, out),
            Stmt::ReactorStdout { arg } => self.emit_stdout(arg, ctx, out),
            Stmt::Let { name, ty: _, expr } => {
                let reg = self.emit_expr(expr, signatures, ctx, out)?;
                let dst = ctx.alloc_var();
                ctx.vars.insert(name.clone(), dst);
                if dst != reg {
                    out.push_str(&format!("    MOV r{} r{}\n", dst, reg));
                }
                if ctx.is_temp(reg) {
                    ctx.free_temp(reg);
                }
                Ok(())
            }
            Stmt::Assign { name, expr } => {
                let reg = self.emit_expr(expr, signatures, ctx, out)?;
                let dst = ctx
                    .vars
                    .get(name)
                    .copied()
                    .ok_or_else(|| format!("unknown variable: {name}"))?;
                if dst != reg {
                    out.push_str(&format!("    MOV r{} r{}\n", dst, reg));
                }
                if ctx.is_temp(reg) {
                    ctx.free_temp(reg);
                }
                Ok(())
            }
            Stmt::Break { label, expr } => self.emit_break(label.as_deref(), expr, signatures, ctx, out),
            Stmt::ExprStmt(expr) => {
                let reg = self.emit_expr(expr, signatures, ctx, out)?;
                if ctx.is_temp(reg) {
                    ctx.free_temp(reg);
                }
                Ok(())
            }
        }
    }

    fn emit_call(
        &mut self,
        name: &str,
        args: &[Expr],
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<(), String> {
        if let Some(_) = self.emit_intrinsic(name, args, signatures, ctx, out)? {
            return Ok(());
        }

        let mut saved_ptr = None;
        let mut saved_vars: Vec<(String, u8)> = ctx
            .vars
            .iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        saved_vars.sort_by(|a, b| a.0.cmp(&b.0));

        if !saved_vars.is_empty() {
            let size_reg = ctx.alloc_temp();
            let ptr_reg = ctx.saved_reg;
            let byte_len = (saved_vars.len() * 8) as u64;
            out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", size_reg, byte_len));
            out.push_str(&format!("    ALLOC r{} r{} 0\n", size_reg, ptr_reg));
            for (idx, (_, reg)) in saved_vars.iter().enumerate() {
                let offset = (idx * 8) as u64;
                out.push_str(&format!("    STORE_U64 r{} r{} {}\n", ptr_reg, reg, offset));
            }
            if ctx.is_temp(size_reg) {
                ctx.free_temp(size_reg);
            }
            saved_ptr = Some((ptr_reg, byte_len));
        }

        let layouts = signatures
            .get(name)
            .ok_or_else(|| format!("unknown function: {name}"))?;

        if args.len() != layouts.len() {
            return Err(format!("argument count mismatch for {name}"));
        }

        for (arg, layout) in args.iter().zip(layouts.iter()) {
            match layout.kind {
                ParamKind::StaticStr => {
                    let (sym, len) = match arg {
                        Expr::StrLiteral(bytes) => {
                            let sym = self.intern_data(bytes.clone());
                            (sym, bytes.len())
                        }
                        Expr::Ident(name) => {
                            let reg = ctx
                                .vars
                                .get(name)
                                .copied()
                                .ok_or_else(|| "static str argument must be a string literal or param".to_string())?;
                            let meta = ctx
                                .param_meta
                                .get(name)
                                .ok_or_else(|| "static str argument must be a string literal or param".to_string())?;
                            if meta.kind == ParamKind::StaticStr {
                                if layout.reg != reg {
                                    out.push_str(&format!("    MOV r{} r{}\n", layout.reg, reg));
                                }
                                if layout.reg + 1 != reg + 1 {
                                    out.push_str(&format!("    MOV r{} r{}\n", layout.reg + 1, reg + 1));
                                }
                                continue;
                            }
                            return Err("static str argument must be a string literal or param".to_string());
                        }
                        _ => return Err("static str argument must be a string literal".to_string()),
                    };
                    out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", layout.reg, sym));
                    out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", layout.reg + 1, len));
                }
                ParamKind::Value => {
                    let reg = self.emit_expr(arg, signatures, ctx, out)?;
                    if layout.reg != reg {
                        out.push_str(&format!("    MOV r{} r{}\n", layout.reg, reg));
                    }
                    if ctx.is_temp(reg) {
                        ctx.free_temp(reg);
                    }
                }
            }
        }

        out.push_str(&format!("    CALL r{} {}\n", ctx.zero_reg, name.to_ascii_uppercase()));

        if let Some((ptr_reg, byte_len)) = saved_ptr {
            for (idx, (_, reg)) in saved_vars.iter().enumerate() {
                let offset = (idx * 8) as u64;
                out.push_str(&format!("    LOAD_U64 r{} r{} {}\n", ptr_reg, reg, offset));
            }
            let _ = byte_len;
        }
        Ok(())
    }

    fn emit_stdout(&mut self, arg: &Expr, ctx: &mut FnContext, out: &mut String) -> Result<(), String> {
        let (ptr_reg, len_reg) = match arg {
            Expr::Ident(name) => {
                let reg = ctx
                    .vars
                    .get(name)
                    .ok_or_else(|| format!("unknown variable: {name}"))?;
                let meta = ctx
                    .param_meta
                    .get(name)
                    .ok_or_else(|| "stdout expects static str param".to_string())?;
                if meta.kind != ParamKind::StaticStr {
                    return Err("stdout expects static str param".to_string());
                }
                (*reg, reg + 1)
            }
            Expr::StrLiteral(bytes) => {
                let sym = self.intern_data(bytes.clone());
                let len = bytes.len();
                let ptr_reg = ctx.alloc_temp();
                let len_reg = ctx.alloc_temp();
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", ptr_reg, sym));
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", len_reg, len));
                (ptr_reg, len_reg)
            }
            _ => return Err("stdout expects static str argument".to_string()),
        };

        let fu_reg = ctx.alloc_temp();
        let timeout_reg = ctx.alloc_temp();
        let max_events_reg = ctx.alloc_temp();
        let result_reg = ctx.alloc_temp();
        let type_reg = ctx.alloc_temp();
        let choice_reg1 = ctx.alloc_temp();
        let choice_reg2 = ctx.alloc_temp();
        out.push_str(&format!(
            "    SET_IO r{} STDOUT_WRITE r{} r{}\n",
            fu_reg, ptr_reg, len_reg
        ));
        out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} -1\n", timeout_reg));
        out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 1\n", max_events_reg));
        out.push_str(&format!("    WAIT_IO r{} r{} r{}\n", timeout_reg, max_events_reg, result_reg));
        out.push_str(&format!("    GET_AN_IO r{} r{} r{} r{}\n", fu_reg, type_reg, choice_reg1, choice_reg2));
        for reg in [
            fu_reg,
            timeout_reg,
            max_events_reg,
            result_reg,
            type_reg,
            choice_reg1,
            choice_reg2,
            ptr_reg,
            len_reg,
        ] {
            if ctx.is_temp(reg) {
                ctx.free_temp(reg);
            }
        }
        Ok(())
    }

    fn emit_expr(
        &mut self,
        expr: &Expr,
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<u8, String> {
        match expr {
            Expr::Ident(name) => ctx
                .vars
                .get(name)
                .copied()
                .ok_or_else(|| format!("unknown variable: {name}")),
            Expr::StrLiteral(_) => Err("string literal is only supported in calls/stdout".to_string()),
            Expr::IntLiteral(value) => {
                let reg = ctx.alloc_temp();
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", reg, value));
                Ok(reg)
            }
            Expr::BoolLiteral(value) => {
                let reg = ctx.alloc_temp();
                let imm = if *value { 1 } else { 0 };
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", reg, imm));
                Ok(reg)
            }
            Expr::Call { name, args } => self.emit_call_expr(name, args, signatures, ctx, out),
            Expr::Binary { left, op, right } => {
                self.emit_binary_expr(left, *op, right, signatures, ctx, out)
            }
            Expr::Loop(loop_expr) => self.emit_loop_expr(loop_expr, signatures, ctx, out),
            Expr::Match(match_expr) => self.emit_match_expr(match_expr, signatures, ctx, out),
            Expr::Block(block) => self.emit_block(block, signatures, ctx, out),
        }
    }

    fn emit_call_expr(
        &mut self,
        name: &str,
        args: &[Expr],
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<u8, String> {
        if let Some(reg) = self.emit_intrinsic(name, args, signatures, ctx, out)? {
            return Ok(reg);
        }

        self.emit_call(name, args, signatures, ctx, out)?;
        let reg = ctx.alloc_temp();
        out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", reg));
        Ok(reg)
    }

    fn emit_intrinsic(
        &mut self,
        name: &str,
        args: &[Expr],
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<Option<u8>, String> {
        match name {
            "alloc" => {
                if args.len() != 1 {
                    return Err("alloc expects 1 argument".to_string());
                }
                let size_reg = self.emit_expr(&args[0], signatures, ctx, out)?;
                let ptr_reg = ctx.alloc_temp();
                out.push_str(&format!("    ALLOC r{} r{} 0\n", size_reg, ptr_reg));
                if ctx.is_temp(size_reg) {
                    ctx.free_temp(size_reg);
                }
                Ok(Some(ptr_reg))
            }
            "dealloc" => {
                if args.len() != 1 {
                    return Err("dealloc expects 1 argument".to_string());
                }
                let ptr_reg = self.emit_expr(&args[0], signatures, ctx, out)?;
                out.push_str(&format!("    DEALLOC r{}\n", ptr_reg));
                if ctx.is_temp(ptr_reg) {
                    ctx.free_temp(ptr_reg);
                }
                Ok(Some(ctx.zero_reg))
            }
            "store_u8" => {
                if args.len() != 3 {
                    return Err("store_u8 expects 3 arguments".to_string());
                }
                let ptr = self.emit_expr(&args[0], signatures, ctx, out)?;
                let offset = self.emit_expr(&args[1], signatures, ctx, out)?;
                let value = self.emit_expr(&args[2], signatures, ctx, out)?;
                let tmp = ctx.alloc_temp();
                out.push_str(&format!("    MOV r{} r{}\n", tmp, ptr));
                out.push_str(&format!("    ADD_U64 r{} r{}\n", tmp, offset));
                out.push_str(&format!("    STORE_U8 r{} r{} 0\n", tmp, value));
                for reg in [tmp, ptr, offset, value] {
                    if ctx.is_temp(reg) {
                        ctx.free_temp(reg);
                    }
                }
                Ok(Some(ctx.zero_reg))
            }
            "load_u8" => {
                if args.len() != 2 {
                    return Err("load_u8 expects 2 arguments".to_string());
                }
                let ptr = self.emit_expr(&args[0], signatures, ctx, out)?;
                let offset = self.emit_expr(&args[1], signatures, ctx, out)?;
                let tmp = ctx.alloc_temp();
                let result = ctx.alloc_temp();
                out.push_str(&format!("    MOV r{} r{}\n", tmp, ptr));
                out.push_str(&format!("    ADD_U64 r{} r{}\n", tmp, offset));
                out.push_str(&format!("    LOAD_U8 r{} r{} 0\n", tmp, result));
                for reg in [tmp, ptr, offset] {
                    if ctx.is_temp(reg) {
                        ctx.free_temp(reg);
                    }
                }
                Ok(Some(result))
            }
            "store_u64" => {
                if args.len() != 3 {
                    return Err("store_u64 expects 3 arguments".to_string());
                }
                let ptr = self.emit_expr(&args[0], signatures, ctx, out)?;
                let offset = self.emit_expr(&args[1], signatures, ctx, out)?;
                let value = self.emit_expr(&args[2], signatures, ctx, out)?;
                let tmp = ctx.alloc_temp();
                out.push_str(&format!("    MOV r{} r{}\n", tmp, ptr));
                out.push_str(&format!("    ADD_U64 r{} r{}\n", tmp, offset));
                out.push_str(&format!("    STORE_U64 r{} r{} 0\n", tmp, value));
                for reg in [tmp, ptr, offset, value] {
                    if ctx.is_temp(reg) {
                        ctx.free_temp(reg);
                    }
                }
                Ok(Some(ctx.zero_reg))
            }
            "load_u64" => {
                if args.len() != 2 {
                    return Err("load_u64 expects 2 arguments".to_string());
                }
                let ptr = self.emit_expr(&args[0], signatures, ctx, out)?;
                let offset = self.emit_expr(&args[1], signatures, ctx, out)?;
                let tmp = ctx.alloc_temp();
                let result = ctx.alloc_temp();
                out.push_str(&format!("    MOV r{} r{}\n", tmp, ptr));
                out.push_str(&format!("    ADD_U64 r{} r{}\n", tmp, offset));
                out.push_str(&format!("    LOAD_U64 r{} r{} 0\n", tmp, result));
                for reg in [tmp, ptr, offset] {
                    if ctx.is_temp(reg) {
                        ctx.free_temp(reg);
                    }
                }
                Ok(Some(result))
            }
            "stdout_buf" => {
                if args.len() != 2 {
                    return Err("stdout_buf expects 2 arguments".to_string());
                }
                let ptr = self.emit_expr(&args[0], signatures, ctx, out)?;
                let len = self.emit_expr(&args[1], signatures, ctx, out)?;
                self.emit_stdout_buf(ptr, len, ctx, out);
                for reg in [ptr, len] {
                    if ctx.is_temp(reg) {
                        ctx.free_temp(reg);
                    }
                }
                Ok(Some(ctx.zero_reg))
            }
            _ => Ok(None),
        }
    }

    fn emit_stdout_buf(&mut self, ptr_reg: u8, len_reg: u8, ctx: &mut FnContext, out: &mut String) {
        let fu_reg = ctx.alloc_temp();
        let timeout_reg = ctx.alloc_temp();
        let max_events_reg = ctx.alloc_temp();
        let result_reg = ctx.alloc_temp();
        let type_reg = ctx.alloc_temp();
        let choice_reg1 = ctx.alloc_temp();
        let choice_reg2 = ctx.alloc_temp();
        out.push_str(&format!(
            "    SET_IO r{} STDOUT_WRITE r{} r{}\n",
            fu_reg, ptr_reg, len_reg
        ));
        out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} -1\n", timeout_reg));
        out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 1\n", max_events_reg));
        out.push_str(&format!("    WAIT_IO r{} r{} r{}\n", timeout_reg, max_events_reg, result_reg));
        out.push_str(&format!(
            "    GET_AN_IO r{} r{} r{} r{}\n",
            fu_reg, type_reg, choice_reg1, choice_reg2
        ));
        for reg in [
            fu_reg,
            timeout_reg,
            max_events_reg,
            result_reg,
            type_reg,
            choice_reg1,
            choice_reg2,
        ] {
            if ctx.is_temp(reg) {
                ctx.free_temp(reg);
            }
        }
    }

    fn emit_binary_expr(
        &mut self,
        left: &Expr,
        op: BinOp,
        right: &Expr,
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<u8, String> {
        let left_reg = self.emit_expr(left, signatures, ctx, out)?;
        let dst = ctx.alloc_temp();
        out.push_str(&format!("    MOV r{} r{}\n", dst, left_reg));

        match op {
            BinOp::Add => {
                if let Expr::IntLiteral(value) = right {
                    out.push_str(&format!("    ADD_U64_IMMEDIATE r{} {}\n", dst, value));
                } else {
                    let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                    out.push_str(&format!("    ADD_U64 r{} r{}\n", dst, right_reg));
                    if ctx.is_temp(right_reg) {
                        ctx.free_temp(right_reg);
                    }
                }
            }
            BinOp::Sub => {
                if let Expr::IntLiteral(value) = right {
                    out.push_str(&format!("    SUB_U64_IMMEDIATE r{} {}\n", dst, value));
                } else {
                    let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                    out.push_str(&format!("    SUB_U64 r{} r{}\n", dst, right_reg));
                    if ctx.is_temp(right_reg) {
                        ctx.free_temp(right_reg);
                    }
                }
            }
            BinOp::Mul => {
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!("    MUL_U64 r{} r{}\n", dst, right_reg));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
            BinOp::Div => {
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!("    DIV_U64 r{} r{}\n", dst, right_reg));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
            BinOp::Mod => {
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!("    MOD_I64 r{} r{}\n", dst, right_reg));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
            BinOp::Eq => {
                let true_label = ctx.fresh_label("EQ_TRUE");
                let end_label = ctx.fresh_label("EQ_END");
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", dst));
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!(
                    "    EQ_JUMP r{} r{} r{} {}\n",
                    ctx.zero_reg, left_reg, right_reg, true_label
                ));
                out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
                out.push_str(&format!("{}:\n", true_label));
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 1\n", dst));
                out.push_str(&format!("{}:\n", end_label));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
            BinOp::Ne => {
                let true_label = ctx.fresh_label("NE_TRUE");
                let end_label = ctx.fresh_label("NE_END");
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", dst));
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!(
                    "    NEQ_JUMP r{} r{} r{} {}\n",
                    ctx.zero_reg, left_reg, right_reg, true_label
                ));
                out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
                out.push_str(&format!("{}:\n", true_label));
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 1\n", dst));
                out.push_str(&format!("{}:\n", end_label));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
            BinOp::Lt => {
                let true_label = ctx.fresh_label("LT_TRUE");
                let end_label = ctx.fresh_label("LT_END");
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", dst));
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!(
                    "    LT_U64_JUMP r{} r{} r{} {}\n",
                    ctx.zero_reg, left_reg, right_reg, true_label
                ));
                out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
                out.push_str(&format!("{}:\n", true_label));
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 1\n", dst));
                out.push_str(&format!("{}:\n", end_label));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
            BinOp::Gt => {
                let true_label = ctx.fresh_label("GT_TRUE");
                let end_label = ctx.fresh_label("GT_END");
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", dst));
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!(
                    "    GT_U64_JUMP r{} r{} r{} {}\n",
                    ctx.zero_reg, left_reg, right_reg, true_label
                ));
                out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
                out.push_str(&format!("{}:\n", true_label));
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 1\n", dst));
                out.push_str(&format!("{}:\n", end_label));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
            BinOp::Le => {
                let true_label = ctx.fresh_label("LE_TRUE");
                let end_label = ctx.fresh_label("LE_END");
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", dst));
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!(
                    "    LTE_U64_JUMP r{} r{} r{} {}\n",
                    ctx.zero_reg, left_reg, right_reg, true_label
                ));
                out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
                out.push_str(&format!("{}:\n", true_label));
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 1\n", dst));
                out.push_str(&format!("{}:\n", end_label));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
            BinOp::Ge => {
                let true_label = ctx.fresh_label("GE_TRUE");
                let end_label = ctx.fresh_label("GE_END");
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", dst));
                let right_reg = self.emit_expr(right, signatures, ctx, out)?;
                out.push_str(&format!(
                    "    GTE_U64_JUMP r{} r{} r{} {}\n",
                    ctx.zero_reg, left_reg, right_reg, true_label
                ));
                out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
                out.push_str(&format!("{}:\n", true_label));
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 1\n", dst));
                out.push_str(&format!("{}:\n", end_label));
                if ctx.is_temp(right_reg) {
                    ctx.free_temp(right_reg);
                }
            }
        }
        if ctx.is_temp(left_reg) {
            ctx.free_temp(left_reg);
        }
        Ok(dst)
    }

    fn emit_block(
        &mut self,
        block: &Block,
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<u8, String> {
        for stmt in &block.stmts {
            self.emit_stmt(stmt, signatures, ctx, out)?;
        }
        if let Some(expr) = &block.tail {
            self.emit_expr(expr, signatures, ctx, out)
        } else {
            let reg = ctx.alloc_temp();
            out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", reg));
            Ok(reg)
        }
    }

    fn emit_loop_expr(
        &mut self,
        loop_expr: &LoopExpr,
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<u8, String> {
        let result_reg = ctx.alloc_var();
        out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} 0\n", result_reg));
        let start_label = ctx.fresh_label("LOOP");
        let end_label = ctx.fresh_label("END");
        out.push_str(&format!("{}:\n", start_label));
        ctx.loop_stack.push(LoopContext {
            label: loop_expr.label.clone(),
            end_label: end_label.clone(),
            result_reg,
        });
        let _ = self.emit_block(&loop_expr.body, signatures, ctx, out)?;
        ctx.loop_stack.pop();
        out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, start_label));
        out.push_str(&format!("{}:\n", end_label));
        Ok(result_reg)
    }

    fn emit_break(
        &mut self,
        label: Option<&str>,
        expr: &Expr,
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<(), String> {
        let target = if let Some(label) = label {
            ctx.loop_stack
                .iter()
                .rev()
                .find(|l| l.label.as_deref() == Some(label))
        } else {
            ctx.loop_stack.last()
        };

        let Some(target) = target else {
            return Err("break used outside of loop".to_string());
        };

        let end_label = target.end_label.clone();
        let result_reg = target.result_reg;
        let value_reg = self.emit_expr(expr, signatures, ctx, out)?;
        if result_reg != value_reg {
            out.push_str(&format!("    MOV r{} r{}\n", result_reg, value_reg));
        }
        if ctx.is_temp(value_reg) {
            ctx.free_temp(value_reg);
        }
        out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
        Ok(())
    }

    fn emit_match_expr(
        &mut self,
        match_expr: &MatchExpr,
        signatures: &HashMap<String, Vec<ParamLayout>>,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<u8, String> {
        let scrut_reg = self.emit_expr(&match_expr.scrutinee, signatures, ctx, out)?;
        let result_reg = ctx.alloc_temp();
        let end_label = ctx.fresh_label("MATCH_END");

        let mut arm_labels = Vec::new();
        let mut default_arm = None;
        for (idx, arm) in match_expr.arms.iter().enumerate() {
            if matches!(arm.pattern, MatchPattern::Wildcard) {
                default_arm = Some(idx);
            } else {
                arm_labels.push((idx, ctx.fresh_label(&format!("MATCH_ARM_{idx}"))));
            }
        }

        if let (Some(default_idx), 1) = (default_arm, arm_labels.len()) {
            let (arm_idx, arm_label) = arm_labels[0].clone();
            let arm = &match_expr.arms[arm_idx];
            self.emit_match_jump(scrut_reg, &arm.pattern, &arm_label, ctx, out)?;

            let default_arm = &match_expr.arms[default_idx];
            let value_reg = self.emit_block(&default_arm.body, signatures, ctx, out)?;
            if value_reg != result_reg {
                out.push_str(&format!("    MOV r{} r{}\n", result_reg, value_reg));
            }
            if ctx.is_temp(value_reg) {
                ctx.free_temp(value_reg);
            }
            out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));

            out.push_str(&format!("{}:\n", arm_label));
            let value_reg = self.emit_block(&arm.body, signatures, ctx, out)?;
            if value_reg != result_reg {
                out.push_str(&format!("    MOV r{} r{}\n", result_reg, value_reg));
            }
            if ctx.is_temp(value_reg) {
                ctx.free_temp(value_reg);
            }
            out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));

            out.push_str(&format!("{}:\n", end_label));
            if ctx.is_temp(scrut_reg) {
                ctx.free_temp(scrut_reg);
            }
            return Ok(result_reg);
        }

        for (idx, label) in arm_labels.iter() {
            let arm = &match_expr.arms[*idx];
            self.emit_match_jump(scrut_reg, &arm.pattern, label, ctx, out)?;
        }

        if let Some(default_idx) = default_arm {
            let arm = &match_expr.arms[default_idx];
            let value_reg = self.emit_block(&arm.body, signatures, ctx, out)?;
            if value_reg != result_reg {
                out.push_str(&format!("    MOV r{} r{}\n", result_reg, value_reg));
            }
            if ctx.is_temp(value_reg) {
                ctx.free_temp(value_reg);
            }
            out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
        } else {
            return Err("match requires a '_' default arm".to_string());
        }

        for (idx, label) in arm_labels.iter() {
            let arm = &match_expr.arms[*idx];
            out.push_str(&format!("{}:\n", label));
            let value_reg = self.emit_block(&arm.body, signatures, ctx, out)?;
            if value_reg != result_reg {
                out.push_str(&format!("    MOV r{} r{}\n", result_reg, value_reg));
            }
            if ctx.is_temp(value_reg) {
                ctx.free_temp(value_reg);
            }
            out.push_str(&format!("    JUMP r{} {}\n", ctx.zero_reg, end_label));
        }

        out.push_str(&format!("{}:\n", end_label));
        if ctx.is_temp(scrut_reg) {
            ctx.free_temp(scrut_reg);
        }
        Ok(result_reg)
    }

    fn emit_match_jump(
        &mut self,
        scrut_reg: u8,
        pattern: &MatchPattern,
        label: &str,
        ctx: &mut FnContext,
        out: &mut String,
    ) -> Result<(), String> {
        match pattern {
            MatchPattern::LessThan(value) => {
                let lit_reg = ctx.alloc_temp();
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", lit_reg, value));
                out.push_str(&format!("    LT_U64_JUMP r{} r{} r{} {}\n", ctx.zero_reg, scrut_reg, lit_reg, label));
                ctx.free_temp(lit_reg);
            }
            MatchPattern::LessEqual(value) => {
                let lit_reg = ctx.alloc_temp();
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", lit_reg, value));
                out.push_str(&format!("    LTE_U64_JUMP r{} r{} r{} {}\n", ctx.zero_reg, scrut_reg, lit_reg, label));
                ctx.free_temp(lit_reg);
            }
            MatchPattern::Equal(value) => {
                let lit_reg = ctx.alloc_temp();
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", lit_reg, value));
                out.push_str(&format!("    EQ_JUMP r{} r{} r{} {}\n", ctx.zero_reg, scrut_reg, lit_reg, label));
                ctx.free_temp(lit_reg);
            }
            MatchPattern::NotEqual(value) => {
                let lit_reg = ctx.alloc_temp();
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", lit_reg, value));
                out.push_str(&format!("    NEQ_JUMP r{} r{} r{} {}\n", ctx.zero_reg, scrut_reg, lit_reg, label));
                ctx.free_temp(lit_reg);
            }
            MatchPattern::GreaterThan(value) => {
                let lit_reg = ctx.alloc_temp();
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", lit_reg, value));
                out.push_str(&format!("    GT_U64_JUMP r{} r{} r{} {}\n", ctx.zero_reg, scrut_reg, lit_reg, label));
                ctx.free_temp(lit_reg);
            }
            MatchPattern::GreaterEqual(value) => {
                let lit_reg = ctx.alloc_temp();
                out.push_str(&format!("    LOAD_U64_IMMEDIATE r{} {}\n", lit_reg, value));
                out.push_str(&format!("    GTE_U64_JUMP r{} r{} r{} {}\n", ctx.zero_reg, scrut_reg, lit_reg, label));
                ctx.free_temp(lit_reg);
            }
            MatchPattern::Wildcard => {}
        }
        Ok(())
    }

    fn intern_data(&mut self, bytes: Vec<u8>) -> String {
        if let Some(name) = self.data_map.get(&bytes) {
            return name.clone();
        }
        let name = format!("STR{}", self.data.len());
        self.data.push(DataItem {
            name: name.clone(),
            bytes: bytes.clone(),
        });
        self.data_map.insert(bytes, name.clone());
        name
    }
}

fn mangle_impl_method(impl_name: &str, method_name: &str) -> String {
    format!("{impl_name}__{method_name}")
}

fn format_bytes(bytes: &[u8]) -> String {
    let mut out = String::new();
    out.push('"');
    for &b in bytes {
        match b {
            b'\\' => out.push_str("\\\\"),
            b'\"' => out.push_str("\\\""),
            b'\n' => out.push_str("\\n"),
            b'\r' => out.push_str("\\r"),
            b'\t' => out.push_str("\\t"),
            0x20..=0x7E => out.push(b as char),
            _ => out.push_str(&format!("\\x{:02X}", b)),
        }
    }
    out.push('"');
    out
}

fn optimize_asm(asm: &str) -> String {
    let lines: Vec<&str> = asm.lines().collect();
    let mut out = String::new();
    let mut prev_line: Option<String> = None;

    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        if let Some(rest) = trimmed.strip_prefix("MOV ") {
            let mut parts = rest.split_whitespace();
            if let (Some(dst), Some(src)) = (parts.next(), parts.next()) {
                if dst == src {
                    i += 1;
                    continue;
                }
            }
        }

        if i + 1 < lines.len() {
            let next_line = lines[i + 1];
            let next_trim = next_line.trim();


            if let Some(rest) = trimmed.strip_prefix("LOAD_U64_IMMEDIATE ") {
                let mut parts = rest.split_whitespace();
                if let (Some(load_dst), Some(imm)) = (parts.next(), parts.next()) {
                    if let Some(mov_rest) = next_trim.strip_prefix("MOV ") {
                        let mut mov_parts = mov_rest.split_whitespace();
                        if let (Some(mov_dst), Some(mov_src)) = (mov_parts.next(), mov_parts.next()) {
                            if mov_src == load_dst {
                                let indent = &line[..line.len() - trimmed.len()];
                                let new_line = format!("{indent}LOAD_U64_IMMEDIATE {mov_dst} {imm}");
                                if prev_line.as_deref() != Some(new_line.trim()) {
                                    out.push_str(&new_line);
                                    out.push('\n');
                                    prev_line = Some(new_line.trim().to_string());
                                }
                                i += 2;
                                continue;
                            }
                        }
                    }
                }
            }

            if let Some(rest) = trimmed.strip_prefix("MOV ") {
                let mut parts = rest.split_whitespace();
                if let (Some(dst), Some(src)) = (parts.next(), parts.next()) {
                    if let Some(next_rest) = next_trim.strip_prefix("MOV ") {
                        let mut next_parts = next_rest.split_whitespace();
                        if let (Some(next_dst), Some(next_src)) = (next_parts.next(), next_parts.next()) {
                            if next_src == dst {
                                let indent = &next_line[..next_line.len() - next_trim.len()];
                                let new_line = format!("{indent}MOV {next_dst} {src}");
                                if prev_line.as_deref() != Some(new_line.trim()) {
                                    out.push_str(&new_line);
                                    out.push('\n');
                                    prev_line = Some(new_line.trim().to_string());
                                }
                                i += 2;
                                continue;
                            }
                            if next_dst == dst {
                                let new_trim = next_trim;
                                if prev_line.as_deref() != Some(new_trim) {
                                    out.push_str(next_line);
                                    out.push('\n');
                                    prev_line = Some(new_trim.to_string());
                                }
                                i += 2;
                                continue;
                            }
                        }
                    }
                }
            }
        }

        if let Some(prev) = &prev_line {
            if prev == trimmed {
                i += 1;
                continue;
            }
        }

        prev_line = Some(trimmed.to_string());
        out.push_str(line);
        out.push('\n');
        i += 1;
    }

    out
}
