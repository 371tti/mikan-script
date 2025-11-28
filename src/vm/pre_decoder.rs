use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::OnceLock;

use crate::vm::function::Function;
use crate::vm::instruction::Instruction;

use super::{instruction::{Operations, Op}};

/// 事前デコーダ
/// バイトコードをfunction_ptr_vecに変換する
/// 
/// バイトコードをについて
/// ```
/// MAIN    ; 関数名 改行後コードが続く これはコメントアウト MAINは特別な名前でエントリーポイントになる
/// <OPECODE> <値1> <値2> ...  ; 命令コード 引数1 引数2 ... となり 空白で区切る tabなどでも可能 改行で次の命令へ
/// CALL FUNC1 ; 関数呼び出し
/// EXIT 0  ; プログラム終了 戻り値0 MAINのみRETでなくEXITで終了すること 次の行は何も書かないこと 関数の区切りを示すため
/// 
/// FUNC1   ; 関数名 改行後コードが続く これはコメントアウト
/// ...
/// RET     ; 関数終了
/// ```
pub struct PreDecoder;

#[derive(Copy, Clone)]
struct OpcodeSpec {
    handler: Op,
    operands: &'static [OperandPlan],
}
impl OpcodeSpec {
    const fn new(handler: Op, operands: &'static [OperandPlan]) -> Self {
        Self { handler, operands }
    }

    fn min_tokens(self) -> usize {
        self.operands.iter().map(|plan| plan.min_tokens()).sum()
    }

    fn max_tokens(self) -> usize {
        self.operands.iter().map(|plan| plan.max_tokens()).sum()
    }
}

#[derive(Copy, Clone)]
enum OperandPlan {
    Value,
    PackedRegisters(u8),
}

impl OperandPlan {
    const fn min_tokens(self) -> usize {
        match self {
            OperandPlan::Value => 1,
            OperandPlan::PackedRegisters(_) => 1,
        }
    }

    const fn max_tokens(self) -> usize {
        match self {
            OperandPlan::Value => 1,
            OperandPlan::PackedRegisters(count) => count as usize,
        }
    }
}

const OPERANDS_NONE: &[OperandPlan] = &[];
// const OPERANDS_VALUE: &[OperandPlan] = &[OperandPlan::Value];
const OPERANDS_TWO_VALUES: &[OperandPlan] = &[OperandPlan::Value, OperandPlan::Value];
// const OPERANDS_PACK2: &[OperandPlan] = &[OperandPlan::PackedRegisters(2)];
// const OPERANDS_PACK3: &[OperandPlan] = &[OperandPlan::PackedRegisters(3)];
// const OPERANDS_PACK4: &[OperandPlan] = &[OperandPlan::PackedRegisters(4)];
const OPERANDS_PACK2_VALUE: &[OperandPlan] = &[OperandPlan::PackedRegisters(2), OperandPlan::Value];
const OPERANDS_PACK3_VALUE: &[OperandPlan] = &[OperandPlan::PackedRegisters(3), OperandPlan::Value];
const OPERANDS_PACK4_VALUE: &[OperandPlan] = &[OperandPlan::PackedRegisters(4), OperandPlan::Value];

impl PreDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&self, source: &str) -> Result<Vec<Function>, PreDecodeError> {
        let mut functions = Vec::new();
        let mut current: Option<ParsedFunction> = None;
        let mut defined_names: HashSet<String> = HashSet::new();
        let opcode_table = opcode_table();

        for (line_idx, raw_line) in source.lines().enumerate() {
            let line_no = line_idx + 1;
            let line = raw_line
                .splitn(2, ';')
                .next()
                .map(str::trim)
                .unwrap_or_default();

            if line.is_empty() {
                continue;
            }

            let mut tokens = line.split_whitespace().collect::<Vec<_>>();
            if tokens.is_empty() {
                continue;
            }
            let first_raw = tokens.remove(0);
            let first_upper = first_raw.to_ascii_uppercase();
            let is_opcode = opcode_table.contains_key(first_upper.as_str());

            if current.is_none() {
                if is_opcode {
                    return Err(PreDecodeError::InstructionOutsideFunction {
                        opcode: first_upper,
                        line: line_no,
                    });
                }

                let func_name = first_upper;
                if !defined_names.insert(func_name.clone()) {
                    return Err(PreDecodeError::DuplicateFunction {
                        name: func_name,
                        line: line_no,
                    });
                }

                current = Some(ParsedFunction::new(func_name));
                continue;
            }

            if !is_opcode && tokens.is_empty() {
                let func_name = first_upper;
                if !defined_names.insert(func_name.clone()) {
                    return Err(PreDecodeError::DuplicateFunction {
                        name: func_name,
                        line: line_no,
                    });
                }

                functions.push(current.take().unwrap());
                current = Some(ParsedFunction::new(func_name));
                continue;
            }

            let opcode_name = first_upper;
            let spec = opcode_table
                .get(opcode_name.as_str())
                .copied()
                .ok_or_else(|| PreDecodeError::UnknownOpcode {
                    name: opcode_name.clone(),
                    line: line_no,
                })?;

            let min_tokens = spec.min_tokens();
            let max_tokens = spec.max_tokens();

            // 引数が足りない場合は0で埋める
            while tokens.len() < max_tokens {
                tokens.push("0");
            }

            if tokens.len() < min_tokens {
                return Err(PreDecodeError::NotEnoughArguments {
                    opcode: opcode_name.clone(),
                    expected: min_tokens,
                    line: line_no,
                });
            }
            if tokens.len() > max_tokens {
                return Err(PreDecodeError::TooManyArguments {
                    opcode: opcode_name.clone(),
                    provided: tokens.len(),
                    allowed: max_tokens,
                    line: line_no,
                });
            }

            let tokens_slice = tokens.as_slice();
            let mut cursor = 0usize;
            let mut parsed_args: Vec<Arg> = Vec::new();

            for operand in spec.operands {
                match operand {
                    OperandPlan::Value => {
                        let token = tokens_slice[cursor];
                        let parsed = parse_arg(token, line_no)?;
                        parsed_args.push(match parsed {
                            Arg::Label(label) => Arg::Label(label.to_ascii_uppercase()),
                            other => other,
                        });
                        cursor += 1;
                    }
                    OperandPlan::PackedRegisters(count) => {
                        let arg = parse_packed_operand(
                            opcode_name.as_str(),
                            tokens_slice,
                            &mut cursor,
                            *count,
                            line_no,
                        )?;
                        parsed_args.push(arg);
                    }
                }
            }

            if cursor < tokens_slice.len() {
                return Err(PreDecodeError::TooManyArguments {
                    opcode: opcode_name.clone(),
                    provided: tokens_slice.len(),
                    allowed: max_tokens,
                    line: line_no,
                });
            }

            let mut args = [Arg::Value(0), Arg::Value(0)];
            for (idx, arg) in parsed_args.into_iter().enumerate() {
                args[idx] = arg;
            }

            current
                .as_mut()
                .unwrap()
                .instructions
                .push(ParsedInstruction {
                    opcode: opcode_name,
                    handler: spec.handler,
                    args,
                    line: line_no,
                });
        }

        if let Some(function) = current.take() {
            functions.push(function);
        }

        if functions.is_empty() {
            return Err(PreDecodeError::MissingMain);
        }

        let main_index = functions
            .iter()
            .position(|f| f.name == "MAIN")
            .ok_or(PreDecodeError::MissingMain)?;

        let mut ordered = Vec::with_capacity(functions.len());
        let main = functions.remove(main_index);
        ordered.push(main);
        ordered.extend(functions);

        let name_to_index: HashMap<_, _> = ordered
            .iter()
            .enumerate()
            .map(|(idx, func)| (func.name.clone(), idx))
            .collect();

        ordered
            .into_iter()
            .map(|parsed| parsed.into_function(&name_to_index))
            .collect()
    }
}

fn parse_arg(token: &str, line: usize) -> Result<Arg, PreDecodeError> {
    if let Some(value) = parse_register(token) {
        return Ok(Arg::Value(value));
    }

    match parse_numeric(token) {
        Ok(value) => Ok(Arg::Value(value)),
        Err(_) => {
            if token.is_empty() {
                Err(PreDecodeError::ParseValue {
                    token: token.to_string(),
                    line,
                })
            } else {
                Ok(Arg::Label(token.to_string()))
            }
        }
    }
}

fn parse_numeric(token: &str) -> Result<u64, ()> {
    let mut cleaned = token.replace('_', "");
    if cleaned.is_empty() {
        return Err(());
    }

    let negative = cleaned.starts_with('-');
    if negative {
        cleaned.remove(0);
    }

    let (base, digits) = if let Some(rest) = cleaned.strip_prefix("0x") {
        (16, rest)
    } else if let Some(rest) = cleaned.strip_prefix("0b") {
        (2, rest)
    } else if let Some(rest) = cleaned.strip_prefix("0o") {
        (8, rest)
    } else {
        (10, cleaned.as_str())
    };

    if digits.is_empty() {
        return Err(());
    }

    if negative {
        let value = i128::from_str_radix(digits, base).map_err(|_| ())?;
        Ok((-value) as u64)
    } else {
        Ok(u64::from_str_radix(digits, base).map_err(|_| ())?)
    }
}

fn parse_packed_operand(
    opcode: &str,
    tokens: &[&str],
    cursor: &mut usize,
    count: u8,
    line: usize,
) -> Result<Arg, PreDecodeError> {
    if *cursor >= tokens.len() {
        return Err(PreDecodeError::NotEnoughPackedRegisters {
            opcode: opcode.to_string(),
            expected: count as usize,
            line,
        });
    }

    let remaining = tokens.len() - *cursor;
    if remaining >= count as usize {
        let mut regs = Vec::with_capacity(count as usize);
        for idx in 0..count as usize {
            let token = tokens[*cursor + idx];
            match parse_register_index(token, line) {
                Ok(value) => regs.push(value),
                Err(err) => {
                    if idx == 0 {
                        if let Ok(value) = parse_numeric(token) {
                            *cursor += 1;
                            return Ok(Arg::Value(value));
                        }
                    }
                    return Err(err);
                }
            }
        }

        *cursor += count as usize;
        return Ok(Arg::Value(pack_registers(&regs)));
    }

    let token = tokens[*cursor];
    if is_register_token(token) {
        return Err(PreDecodeError::NotEnoughPackedRegisters {
            opcode: opcode.to_string(),
            expected: count as usize,
            line,
        });
    }

    let value = parse_numeric(token).map_err(|_| PreDecodeError::ParseValue {
        token: token.to_string(),
        line,
    })?;
    *cursor += 1;
    Ok(Arg::Value(value))
}

fn parse_register_index(token: &str, line: usize) -> Result<u8, PreDecodeError> {
    if let Some(value) = parse_register(token) {
        if value <= u8::MAX as u64 {
            return Ok(value as u8);
        }
        return Err(PreDecodeError::RegisterOutOfRange {
            token: token.to_string(),
            line,
        });
    }

    let value = parse_numeric(token).map_err(|_| PreDecodeError::ExpectedRegister {
        token: token.to_string(),
        line,
    })?;

    if value > u8::MAX as u64 {
        return Err(PreDecodeError::RegisterOutOfRange {
            token: token.to_string(),
            line,
        });
    }

    Ok(value as u8)
}

fn pack_registers(regs: &[u8]) -> u64 {
    regs.iter().fold(0u64, |acc, &reg| (acc << 8) | reg as u64)
}

fn is_register_token(token: &str) -> bool {
    token.starts_with('r') || token.starts_with('R')
}

fn parse_register(token: &str) -> Option<u64> {
    let rest = token.strip_prefix('r').or_else(|| token.strip_prefix('R'))?;
    if rest.is_empty() {
        return None;
    }

    let digits = rest.replace('_', "");
    if digits.is_empty() || !digits.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }

    digits.parse().ok()
}

#[derive(Debug)]
pub enum PreDecodeError {
    MissingMain,
    DuplicateFunction { name: String, line: usize },
    InstructionOutsideFunction { opcode: String, line: usize },
    UnknownOpcode { name: String, line: usize },
    TooManyArguments { opcode: String, provided: usize, allowed: usize, line: usize },
    NotEnoughArguments { opcode: String, expected: usize, line: usize },
    NotEnoughPackedRegisters { opcode: String, expected: usize, line: usize },
    ParseValue { token: String, line: usize },
    UnknownFunction { name: String, line: usize },
    UnexpectedLabel { opcode: String, label: String, line: usize },
    ExpectedRegister { token: String, line: usize },
    RegisterOutOfRange { token: String, line: usize },
}

impl fmt::Display for PreDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PreDecodeError::MissingMain => write!(f, "MAIN function is required"),
            PreDecodeError::DuplicateFunction { name, line } => {
                write!(f, "function '{name}' defined multiple times (line {line})")
            }
            PreDecodeError::InstructionOutsideFunction { opcode, line } => {
                write!(
                    f,
                    "instruction '{opcode}' appears before any function definition (line {line})"
                )
            }
            PreDecodeError::UnknownOpcode { name, line } => {
                write!(f, "unknown opcode '{name}' at line {line}")
            }
            PreDecodeError::TooManyArguments {
                opcode,
                provided,
                allowed,
                line,
            } => write!(
                f,
                "opcode '{opcode}' accepts at most {allowed} operand token(s) but {provided} were provided (line {line})"
            ),
            PreDecodeError::NotEnoughArguments {
                opcode,
                expected,
                line,
            } => write!(
                f,
                "opcode '{opcode}' expects at least {expected} operand token(s) (line {line})"
            ),
            PreDecodeError::NotEnoughPackedRegisters {
                opcode,
                expected,
                line,
            } => write!(
                f,
                "opcode '{opcode}' expects {expected} register operand(s) for the packed field (line {line})"
            ),
            PreDecodeError::ParseValue { token, line } => {
                write!(
                    f,
                    "failed to parse '{token}' as numeric literal (line {line})"
                )
            }
            PreDecodeError::UnknownFunction { name, line } => {
                write!(
                    f,
                    "referenced function '{name}' is not defined (line {line})"
                )
            }
            PreDecodeError::UnexpectedLabel {
                opcode,
                label,
                line,
            } => write!(
                f,
                "opcode '{opcode}' does not accept label '{label}' (line {line})"
            ),
            PreDecodeError::ExpectedRegister { token, line } => write!(
                f,
                "'{token}' should be a register (rN) or 8-bit value when expanding packed operands (line {line})"
            ),
            PreDecodeError::RegisterOutOfRange { token, line } => write!(
                f,
                "register value '{token}' must fit in 8 bits for packed operands (line {line})"
            ),
        }
    }
}

impl std::error::Error for PreDecodeError {}

#[derive(Clone)]
struct ParsedFunction {
    name: String,
    instructions: Vec<ParsedInstruction>,
}

impl ParsedFunction {
    fn new(name: String) -> Self {
        Self {
            name,
            instructions: Vec::new(),
        }
    }

    fn into_function(
        self,
        name_to_index: &HashMap<String, usize>,
    ) -> Result<Function, PreDecodeError> {
        let instructions = self
            .instructions
            .into_iter()
            .map(|instruction| instruction.into_instruction(name_to_index))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Function::new(instructions.into_boxed_slice()))
    }
}

#[derive(Clone)]
struct ParsedInstruction {
    opcode: String,
    handler: Op,
    args: [Arg; 2],
    line: usize,
}

impl ParsedInstruction {
    fn into_instruction(
        self,
        name_to_index: &HashMap<String, usize>,
    ) -> Result<Instruction, PreDecodeError> {
        let a = resolve_arg(&self.opcode, &self.args[0], name_to_index, self.line)?;
        let b = resolve_arg(&self.opcode, &self.args[1], name_to_index, self.line)?;
        Ok(Instruction::new(self.handler, a, b))
    }
}

fn resolve_arg(
    opcode: &str,
    arg: &Arg,
    name_to_index: &HashMap<String, usize>,
    line: usize,
) -> Result<u64, PreDecodeError> {
    match arg {
        Arg::Value(value) => Ok(*value),
        Arg::Label(label) => {
            if opcode != "CALL" {
                return Err(PreDecodeError::UnexpectedLabel {
                    opcode: opcode.to_string(),
                    label: label.clone(),
                    line,
                });
            }

            name_to_index
                .get(label)
                .map(|idx| *idx as u64)
                .ok_or_else(|| PreDecodeError::UnknownFunction {
                    name: label.clone(),
                    line,
                })
        }
    }
}

#[derive(Clone)]
enum Arg {
    Value(u64),
    Label(String),
}

fn opcode_table() -> &'static HashMap<&'static str, OpcodeSpec> {
    static OPCODES: OnceLock<HashMap<&'static str, OpcodeSpec>> = OnceLock::new();
    OPCODES.get_or_init(|| {
        let mut map: HashMap<&'static str, OpcodeSpec> = HashMap::new();
        macro_rules! insert {
            ($name:literal, $func:expr, $operands:expr) => {
                map.insert($name, OpcodeSpec::new($func as Op, $operands));
            };
        }

        // 整数演算
        insert!("ADD_U64", Operations::add_u64, OPERANDS_TWO_VALUES); // *dst = *dst + *src
        insert!("ADD_U64_IMMEDIATE", Operations::add_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst + imm
        insert!("ADD_I64", Operations::add_i64, OPERANDS_TWO_VALUES); // *dst = *dst + *src
        insert!("ADD_I64_IMMEDIATE", Operations::add_i64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst + imm
        insert!("SUB_U64", Operations::sub_u64, OPERANDS_TWO_VALUES); // *dst = *dst - *src
        insert!("SUB_U64_IMMEDIATE", Operations::sub_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst - imm
        insert!("SUB_I64", Operations::sub_i64, OPERANDS_TWO_VALUES); // *dst = *dst - *src
        insert!("SUB_I64_IMMEDIATE", Operations::sub_i64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst - imm
        insert!("MUL_U64", Operations::mul_u64, OPERANDS_TWO_VALUES); // *dst = *dst * *src
        insert!("MUL_U64_IMMEDIATE", Operations::mul_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst * imm
        insert!("MUL_I64", Operations::mul_i64, OPERANDS_TWO_VALUES); // *dst = *dst * *src
        insert!("MUL_I64_IMMEDIATE", Operations::mul_i64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst * imm
        insert!("DIV_U64", Operations::div_u64, OPERANDS_TWO_VALUES); // *dst = *dst / *src
        insert!("DIV_U64_IMMEDIATE", Operations::div_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst / imm
        insert!("DIV_I64", Operations::div_i64, OPERANDS_TWO_VALUES); // *dst = *dst / *src
        insert!("DIV_I64_IMMEDIATE", Operations::div_i64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst / imm
        insert!("ABS", Operations::abs, OPERANDS_TWO_VALUES); // *dst = abs(*src)
        insert!("MOD_I64", Operations::mod_i64, OPERANDS_TWO_VALUES); // *dst = *dst % *src
        insert!("NEG_I64", Operations::neg_i64, OPERANDS_TWO_VALUES); // *dst = -(*src)
        insert!("U64_TO_F64", Operations::u64_to_f64, OPERANDS_TWO_VALUES); // *dst = (*src as f64)
        insert!("I64_TO_F64", Operations::i64_to_f64, OPERANDS_TWO_VALUES); // *dst = (*src as i64) as f64

        // 浮動小数点演算
        insert!("ADD_F64", Operations::add_f64, OPERANDS_TWO_VALUES); // *dst = *dst + *src
        insert!("ADD_F64_IMMEDIATE", Operations::add_f64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst + imm
        insert!("SUB_F64", Operations::sub_f64, OPERANDS_TWO_VALUES); // *dst = *dst - *src
        insert!("SUB_F64_IMMEDIATE", Operations::sub_f64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst - imm
        insert!("MUL_F64", Operations::mul_f64, OPERANDS_TWO_VALUES); // *dst = *dst * *src
        insert!("MUL_F64_IMMEDIATE", Operations::mul_f64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst * imm
        insert!("DIV_F64", Operations::div_f64, OPERANDS_TWO_VALUES); // *dst = *dst / *src
        insert!("DIV_F64_IMMEDIATE", Operations::div_f64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst / imm
        insert!("ABS_F64", Operations::abs_f64, OPERANDS_TWO_VALUES); // *dst = abs(*src)
        insert!("NEG_F64", Operations::neg_f64, OPERANDS_TWO_VALUES); // *dst = -(*src)
        insert!("TO_I64", Operations::to_i64, OPERANDS_TWO_VALUES); // *dst = (*src as f64) as i64 as u64

        // 論理演算
        insert!("AND_U64", Operations::and_u64, OPERANDS_TWO_VALUES); // *dst = *dst & *src
        insert!("AND_U64_IMMEDIATE", Operations::and_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst & imm
        insert!("OR_U64", Operations::or_u64, OPERANDS_TWO_VALUES); // *dst = *dst | *src
        insert!("OR_U64_IMMEDIATE", Operations::or_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst | imm
        insert!("XOR_U64", Operations::xor_u64, OPERANDS_TWO_VALUES); // *dst = *dst ^ *src
        insert!("XOR_U64_IMMEDIATE", Operations::xor_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst ^ imm
        insert!("NOT_U64", Operations::not_u64, OPERANDS_TWO_VALUES); // *dst = !*src
        insert!("SHL_U64", Operations::shl_u64, OPERANDS_TWO_VALUES); // *dst = *dst << *src
        insert!("SHL_U64_IMMEDIATE", Operations::shl_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst << imm
        insert!("SHL_I64", Operations::shl_i64, OPERANDS_TWO_VALUES); // *dst = *dst << *src
        insert!("SHL_I64_IMMEDIATE", Operations::shl_i64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst << imm
        insert!("SHR_U64", Operations::shr_u64, OPERANDS_TWO_VALUES); // *dst = *dst >> *src
        insert!("SHR_U64_IMMEDIATE", Operations::shr_u64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst >> imm
        insert!("SHR_I64", Operations::shr_i64, OPERANDS_TWO_VALUES); // *dst = *dst >> *src
        insert!("SHR_I64_IMMEDIATE", Operations::shr_i64_immediate, OPERANDS_TWO_VALUES); // *dst = *dst >> imm
        insert!("ROL_U64", Operations::rol_u64, OPERANDS_TWO_VALUES); // *dst = rol(*dst, *src)
        insert!("ROL_U64_IMMEDIATE", Operations::rol_u64_immediate, OPERANDS_TWO_VALUES); // *dst = rol(*dst, imm)
        insert!("ROL_I64", Operations::rol_i64, OPERANDS_TWO_VALUES); // *dst = rol(*dst, *src)
        insert!("ROL_I64_IMMEDIATE", Operations::rol_i64_immediate, OPERANDS_TWO_VALUES); // *dst = rol(*dst, imm)
        insert!("ROR_U64", Operations::ror_u64, OPERANDS_TWO_VALUES); // *dst = ror(*dst, *src)
        insert!("ROR_U64_IMMEDIATE", Operations::ror_u64_immediate, OPERANDS_TWO_VALUES); // *dst = ror(*dst, imm)
        insert!("ROR_I64", Operations::ror_i64, OPERANDS_TWO_VALUES); // *dst = ror(*dst, *src)
        insert!("ROR_I64_IMMEDIATE", Operations::ror_i64_immediate, OPERANDS_TWO_VALUES); // *dst = ror(*dst, imm
        insert!("COUNT_ONES_U64", Operations::count_ones_u64, OPERANDS_TWO_VALUES); // *dst = count_ones(*src)
        insert!("COUNT_ZEROS_U64", Operations::count_zeros_u64, OPERANDS_TWO_VALUES); // *dst = count_zeros(*src)
        insert!("TRAILING_ZEROS_U64", Operations::trailing_zeros_u64, OPERANDS_TWO_VALUES); // *dst = trailing_zeros(*src)

        // レジスタ操作系
        insert!("MOV", Operations::mov, OPERANDS_TWO_VALUES); // *dst = *src
        insert!("LOAD_U64_IMMEDIATE", Operations::load_u64_immediate, OPERANDS_TWO_VALUES); // *dst = imm
        insert!("SWAP", Operations::swap, OPERANDS_TWO_VALUES); // *reg_a, *reg_b = *reg_b, *reg_a

        // 制御系
        insert!("JUMP", Operations::jump, OPERANDS_TWO_VALUES); // pc = *dst + offset
        insert!("EQ_JUMP", Operations::eq_jump, OPERANDS_PACK3_VALUE); // if *a == *b { pc = *addr_reg + offset }
        insert!("NEQ_JUMP", Operations::neq_jump, OPERANDS_PACK3_VALUE); // if *a != *b { pc = *addr_reg + offset }
        insert!("LT_U64_JUMP", Operations::lt_u64_jump, OPERANDS_PACK3_VALUE); // if *a < *b { pc = *addr_reg + offset }
        insert!("LTE_U64_JUMP", Operations::lte_u64_jump, OPERANDS_PACK3_VALUE); // if *a <= *b { pc = *addr_reg + offset }
        insert!("LT_I64_JUMP", Operations::lt_i64_jump, OPERANDS_PACK3_VALUE); // if *a < *b { pc = *addr_reg + offset }
        insert!("LTE_I64_JUMP", Operations::lte_i64_jump, OPERANDS_PACK3_VALUE); // if *a <= *b { pc = *addr_reg + offset }
        insert!("GT_U64_JUMP", Operations::gt_u64_jump, OPERANDS_PACK3_VALUE); // if *a > *b { pc = *addr_reg + offset }
        insert!("GTE_U64_JUMP", Operations::gte_u64_jump, OPERANDS_PACK3_VALUE); // if *a >= *b { pc = *addr_reg + offset }
        insert!("GT_I64_JUMP", Operations::gt_i64_jump, OPERANDS_PACK3_VALUE); // if *a > *b { pc = *addr_reg + offset }
        insert!("GTE_I64_JUMP", Operations::gte_i64_jump, OPERANDS_PACK3_VALUE); // if *a >= *b { pc = *addr_reg + offset }
        insert!("CALL", Operations::call, OPERANDS_TWO_VALUES); // call func_index, pc
        insert!("RET", Operations::ret, OPERANDS_NONE); // ret

        // IO操作
        insert!("PRINT_U64", Operations::print_u64, OPERANDS_TWO_VALUES); // print_u64 *src
        insert!("ALLOC", Operations::alloc, OPERANDS_PACK2_VALUE); // allocate *size + add_size, store id in *id_res_reg
        insert!("REALLOC", Operations::realloc, OPERANDS_TWO_VALUES); // reallocate *size for *id
        insert!("DEALLOC", Operations::dealloc, OPERANDS_TWO_VALUES); // deallocate *id
        insert!("EXIT", Operations::exit, OPERANDS_TWO_VALUES); // exit with code *code_reg

        // メモリ操作
        insert!("LOAD_U64", Operations::load_u64, OPERANDS_PACK3_VALUE); // *result_reg = *(heep_ptr(*id_reg) + *addr_reg + offset)
        insert!("LOAD_U32", Operations::load_u32, OPERANDS_PACK3_VALUE);
        insert!("LOAD_U16", Operations::load_u16, OPERANDS_PACK3_VALUE);
        insert!("LOAD_U8", Operations::load_u8, OPERANDS_PACK3_VALUE);
        insert!("STORE_U64", Operations::store_u64, OPERANDS_PACK3_VALUE);
        insert!("STORE_U32", Operations::store_u32, OPERANDS_PACK3_VALUE);
        insert!("STORE_U16", Operations::store_u16, OPERANDS_PACK3_VALUE);
        insert!("STORE_U8", Operations::store_u8, OPERANDS_PACK3_VALUE);

        // atomic
        insert!("ATOMIC_LOAD_U64", Operations::atomic_load_u64, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_STORE_U64", Operations::atomic_store_u64, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_ADD_U64", Operations::atomic_add_u64, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_SUB_U64", Operations::atomic_sub_u64, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_LOAD_U32", Operations::atomic_load_u32, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_STORE_U32", Operations::atomic_store_u32, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_ADD_U32", Operations::atomic_add_u32, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_SUB_U32", Operations::atomic_sub_u32, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_LOAD_U16", Operations::atomic_load_u16, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_STORE_U16", Operations::atomic_store_u16, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_ADD_U16", Operations::atomic_add_u16, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_SUB_U16", Operations::atomic_sub_u16, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_LOAD_U8", Operations::atomic_load_u8, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_STORE_U8", Operations::atomic_store_u8, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_ADD_U8", Operations::atomic_add_u8, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_SUB_U8", Operations::atomic_sub_u8, OPERANDS_PACK4_VALUE);

        // 符号拡張ロード/ストア
        insert!("LOAD_I8", Operations::load_i8, OPERANDS_PACK3_VALUE);
        insert!("LOAD_I16", Operations::load_i16, OPERANDS_PACK3_VALUE);
        insert!("LOAD_I32", Operations::load_i32, OPERANDS_PACK3_VALUE);
        insert!("LOAD_I64", Operations::load_i64, OPERANDS_PACK3_VALUE);
        insert!("STORE_I8", Operations::store_i8, OPERANDS_PACK3_VALUE);
        insert!("STORE_I16", Operations::store_i16, OPERANDS_PACK3_VALUE);
        insert!("STORE_I32", Operations::store_i32, OPERANDS_PACK3_VALUE);
        insert!("STORE_I64", Operations::store_i64, OPERANDS_PACK3_VALUE);

        // atomic 符号拡張
        insert!("ATOMIC_LOAD_I8", Operations::atomic_load_i8, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_LOAD_I16", Operations::atomic_load_i16, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_LOAD_I32", Operations::atomic_load_i32, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_LOAD_I64", Operations::atomic_load_i64, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_STORE_I8", Operations::atomic_store_i8, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_STORE_I16", Operations::atomic_store_i16, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_STORE_I32", Operations::atomic_store_i32, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_STORE_I64", Operations::atomic_store_i64, OPERANDS_PACK3_VALUE);
        insert!("ATOMIC_ADD_I8", Operations::atomic_add_i8, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_ADD_I16", Operations::atomic_add_i16, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_ADD_I32", Operations::atomic_add_i32, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_ADD_I64", Operations::atomic_add_i64, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_SUB_I8", Operations::atomic_sub_i8, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_SUB_I16", Operations::atomic_sub_i16, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_SUB_I32", Operations::atomic_sub_i32, OPERANDS_PACK4_VALUE);
        insert!("ATOMIC_SUB_I64", Operations::atomic_sub_i64, OPERANDS_PACK4_VALUE);

        // 特殊制御
        insert!("GET_DECODE", Operations::get_decode, OPERANDS_TWO_VALUES); // get_decode(vm, fn_r, deepr)
        insert!("GET_DECODED", Operations::get_decoded, OPERANDS_TWO_VALUES); // get_decoded(vm, _, _)

        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::instruction::Operations;

    #[test]
    fn decode_matches_code_manager_set_test2() {
        let source = r#"
MAIN
ALLOC r0 r3 1
ADD_U64_IMMEDIATE r1 1
LOAD_U64_IMMEDIATE r2 1000000000
STORE_U64 r3 r0 0
ATOMIC_ADD_U64 r8 r3 r0 r1
ATOMIC_LOAD_U64 r3 r0 r4
LT_U64_JUMP r0 r4 r2 4
PRINT_U64 r4
EXIT 0
"#;

        let decoder = PreDecoder::new();
        let functions = decoder.decode(source).expect("decode succeeds");
        assert_eq!(functions.len(), 2);

        let main = &functions[0].instructions;
        assert_eq!(main.len(), 9);

        let expected = [
            (Operations::alloc as Op, 0x0003, 1), // ALLOC r0 r3 1 → pack([0,3]), 1
            (Operations::add_u64_immediate as Op, 1, 1),
            (Operations::load_u64_immediate as Op, 2, 1000000000),
            (Operations::store_u64 as Op, 0x030000, 0), // STORE_U64 r3 r0 0 → pack([3,0,0]), 0
            (Operations::atomic_add_u64 as Op, 0x08030001, 0), // ATOMIC_ADD_U64 r8 r3 r0 r1 → pack([8,3,0,1]), 0
            (Operations::atomic_load_u64 as Op, 0x030004, 0), // ATOMIC_LOAD_U64 r3 r0 r4 → pack([3,0,4]), 0
            (Operations::lt_u64_jump as Op, 0x000402, 4), // LT_U64_JUMP r0 r4 r2 4 → pack([0,4,2]), 4
            (Operations::print_u64 as Op, 4, 0), // PRINT_U64 r4 → 4, 0
            (Operations::exit as Op, 0, 0), // EXIT 0 → 0, 0
        ];

        for (idx, (op, a, b)) in expected.iter().enumerate() {
            assert_eq!(main[idx].f as usize, *op as usize, "opcode mismatch at {}", idx);
            assert_eq!(main[idx].a, *a, "arg a mismatch at {}", idx);
            assert_eq!(main[idx].b, *b, "arg b mismatch at {}", idx);
        }
    }
}
