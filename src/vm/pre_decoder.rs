use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::OnceLock;

use crate::vm::function::Function;
use crate::vm::instruction::Instruction;

use super::{instruction::{Operations, Op}};

#[derive(Clone)]
enum Arg {
    Value(u64),
    Label(String),
}

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
const OPERANDS_VALUE: &[OperandPlan] = &[OperandPlan::Value];
const OPERANDS_TWO_VALUES: &[OperandPlan] = &[OperandPlan::Value, OperandPlan::Value];
const OPERANDS_PACK1: &[OperandPlan] = &[OperandPlan::PackedRegisters(1)];
const OPERANDS_PACK2: &[OperandPlan] = &[OperandPlan::PackedRegisters(2)];
// const OPERANDS_PACK3: &[OperandPlan] = &[OperandPlan::PackedRegisters(3)];
// const OPERANDS_PACK4: &[OperandPlan] = &[OperandPlan::PackedRegisters(4)];
const OPERANDS_PACK1_VALUE: &[OperandPlan] =
    &[OperandPlan::PackedRegisters(1), OperandPlan::Value];
const OPERANDS_PACK2_VALUE: &[OperandPlan] =
    &[OperandPlan::PackedRegisters(2), OperandPlan::Value];
const OPERANDS_PACK3_VALUE: &[OperandPlan] =
    &[OperandPlan::PackedRegisters(3), OperandPlan::Value];
const OPERANDS_PACK4_VALUE: &[OperandPlan] =
    &[OperandPlan::PackedRegisters(4), OperandPlan::Value];

impl PreDecoder {
    pub fn new() -> Self {
        Self
    }

    pub fn decode(&self, source: &str) -> Result<Vec<Function>, PreDecodeError> {
        let mut functions = Vec::new();
        let mut current: Option<ParsedFunction> = None;
        let mut defined_names: HashSet<String> = HashSet::new();
        let opcode_table = opcode_table();

        'line_loop: for (line_idx, raw_line) in source.lines().enumerate() {
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
            let mut first_raw = tokens.remove(0);

            loop {
                if let Some(label_raw) = first_raw.strip_suffix(':') {
                    let label = label_raw.to_ascii_uppercase();
                    let func = current
                        .as_mut()
                        .ok_or_else(|| PreDecodeError::LabelOutsideFunction {
                            name: label.clone(),
                            line: line_no,
                        })?;
                    func.add_label(label, line_no)?;
                    if tokens.is_empty() {
                        continue 'line_loop;
                    }
                    first_raw = tokens.remove(0);
                    continue;
                }
                break;
            }

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

            for operand in spec.operands.iter().copied() {
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
                            count,
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
            let operand_count = spec.operands.len();
            current
                .as_mut()
                .unwrap()
                .push_instruction(ParsedInstruction {
                    opcode: opcode_name,
                    handler: spec.handler,
                    args,
                    operand_count,
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
    regs.iter()
        .enumerate()
        .fold(0u64, |acc, (i, &reg)| acc | ((reg as u64) << (i * 8)))
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
    DuplicateLabel { name: String, line: usize },
    LabelOutsideFunction { name: String, line: usize },
    UnknownLabel { name: String, line: usize },
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
            PreDecodeError::DuplicateLabel { name, line } => {
                write!(f, "label '{name}' defined multiple times (line {line})")
            }
            PreDecodeError::LabelOutsideFunction { name, line } => {
                write!(f, "label '{name}' appears outside any function (line {line})")
            }
            PreDecodeError::UnknownLabel { name, line } => {
                write!(f, "label '{name}' is not defined in this function (line {line})")
            }
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
    labels: HashMap<String, usize>,
    word_offset: usize,
}

impl ParsedFunction {
    fn new(name: String) -> Self {
        Self {
            name,
            instructions: Vec::new(),
            labels: HashMap::new(),
            word_offset: 0,
        }
    }

    fn add_label(&mut self, name: String, line: usize) -> Result<(), PreDecodeError> {
        if self.labels.insert(name.clone(), self.word_offset).is_some() {
            return Err(PreDecodeError::DuplicateLabel { name, line });
        }
        Ok(())
    }

    fn push_instruction(&mut self, instruction: ParsedInstruction) {
        self.word_offset += instruction.word_len();
        self.instructions.push(instruction);
    }

    fn into_function(
        self,
        name_to_index: &HashMap<String, usize>,
    ) -> Result<Function, PreDecodeError> {
        let ParsedFunction {
            instructions,
            labels,
            ..
        } = self;
        let mut words = Vec::new();
        for instruction in instructions {
            words.extend(instruction.into_instructions(name_to_index, &labels)?);
        }
        Ok(Function::new(words.into_boxed_slice()))
    }
}

#[derive(Clone)]
struct ParsedInstruction {
    opcode: String,
    handler: Op,
    args: [Arg; 2],
    operand_count: usize,
    line: usize,
}

impl ParsedInstruction {
    fn word_len(&self) -> usize {
        1 + self.operand_count
    }

    fn into_instructions(
        self,
        name_to_index: &HashMap<String, usize>,
        labels: &HashMap<String, usize>,
    ) -> Result<Vec<Instruction>, PreDecodeError> {
        let mut resolved = [0u64; 2];
        for idx in 0..self.operand_count {
            resolved[idx] =
                resolve_arg(&self.opcode, &self.args[idx], name_to_index, labels, self.line)?;
        }
        let words = match self.operand_count {
            0 => vec![Instruction::new_1w_op(self.handler)],
            1 => Vec::from(Instruction::new_2w_op(self.handler, resolved[0].to_le_bytes())),
            2 => Vec::from(Instruction::new_3w_op(
                self.handler,
                resolved[0].to_le_bytes(),
                resolved[1].to_le_bytes(),
            )),
            _ => unreachable!(),
        };
        Ok(words)
    }
}

fn resolve_arg(
    opcode: &str,
    arg: &Arg,
    name_to_index: &HashMap<String, usize>,
    labels: &HashMap<String, usize>,
    line: usize,
) -> Result<u64, PreDecodeError> {
    match arg {
        Arg::Value(value) => Ok(*value),
        Arg::Label(label) => {
            if opcode == "CALL" {
                name_to_index
                    .get(label)
                    .map(|idx| *idx as u64)
                    .ok_or_else(|| PreDecodeError::UnknownFunction {
                        name: label.clone(),
                        line,
                    })
            } else if opcode_accepts_label(opcode) {
                labels
                    .get(label)
                    .copied()
                    .map(|offset| offset as u64)
                    .ok_or_else(|| PreDecodeError::UnknownLabel {
                        name: label.clone(),
                        line,
                    })
            } else {
                Err(PreDecodeError::UnexpectedLabel {
                    opcode: opcode.to_string(),
                    label: label.clone(),
                    line,
                })
            }
        }
    }
}

fn opcode_accepts_label(opcode: &str) -> bool {
    matches!(
        opcode,
        "JUMP"
            | "EQ_JUMP"
            | "NEQ_JUMP"
            | "LT_U64_JUMP"
            | "LTE_U64_JUMP"
            | "LT_I64_JUMP"
            | "LTE_I64_JUMP"
            | "GT_U64_JUMP"
            | "GTE_U64_JUMP"
            | "GT_I64_JUMP"
            | "GTE_I64_JUMP"
    )
}

fn opcode_table() -> &'static HashMap<&'static str, OpcodeSpec> {
    static OPCODE_TABLE: OnceLock<HashMap<&'static str, OpcodeSpec>> = OnceLock::new();
    OPCODE_TABLE.get_or_init(|| {
        let mut m: HashMap<&'static str, OpcodeSpec> = HashMap::new();

        // Control
        m.insert("RET", OpcodeSpec::new(Operations::ret as Op, OPERANDS_NONE));
        m.insert("CALL", OpcodeSpec::new(Operations::call as Op, OPERANDS_TWO_VALUES));
        m.insert("JUMP", OpcodeSpec::new(Operations::jump as Op, OPERANDS_PACK1_VALUE));
        m.insert("EQ_JUMP", OpcodeSpec::new(Operations::eq_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("NEQ_JUMP", OpcodeSpec::new(Operations::neq_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("LT_U64_JUMP", OpcodeSpec::new(Operations::lt_u64_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("LTE_U64_JUMP", OpcodeSpec::new(Operations::lte_u64_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("LT_I64_JUMP", OpcodeSpec::new(Operations::lt_i64_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("LTE_I64_JUMP", OpcodeSpec::new(Operations::lte_i64_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("GT_U64_JUMP", OpcodeSpec::new(Operations::gt_u64_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("GTE_U64_JUMP", OpcodeSpec::new(Operations::gte_u64_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("GT_I64_JUMP", OpcodeSpec::new(Operations::gt_i64_jump as Op, OPERANDS_PACK3_VALUE));
        m.insert("GTE_I64_JUMP", OpcodeSpec::new(Operations::gte_i64_jump as Op, OPERANDS_PACK3_VALUE));

        // VM ops
        m.insert("GET_DECODE", OpcodeSpec::new(Operations::get_decode as Op, OPERANDS_TWO_VALUES));
        m.insert("GET_DECODED", OpcodeSpec::new(Operations::get_decoded as Op, OPERANDS_NONE));
        m.insert("EXIT", OpcodeSpec::new(Operations::exit as Op, OPERANDS_VALUE));

        // IO / Memory
        m.insert("PRINT_U64", OpcodeSpec::new(Operations::print_u64 as Op, OPERANDS_PACK1));
        m.insert("ALLOC", OpcodeSpec::new(Operations::alloc as Op, OPERANDS_PACK2_VALUE));
        m.insert("REALLOC", OpcodeSpec::new(Operations::realloc as Op, OPERANDS_PACK2));
        m.insert("DEALLOC", OpcodeSpec::new(Operations::dealloc as Op, OPERANDS_PACK1));

        // Register ops
        m.insert("MOV", OpcodeSpec::new(Operations::mov as Op, OPERANDS_PACK2));
        m.insert("LOAD_U64_IMMEDIATE", OpcodeSpec::new(Operations::load_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("SWAP", OpcodeSpec::new(Operations::swap as Op, OPERANDS_PACK2));

        // Int calcs
        m.insert("ADD_U64", OpcodeSpec::new(Operations::add_u64 as Op, OPERANDS_PACK2));
        m.insert("ADD_U64_IMMEDIATE", OpcodeSpec::new(Operations::add_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("ADD_I64", OpcodeSpec::new(Operations::add_i64 as Op, OPERANDS_PACK2));
        m.insert("ADD_I64_IMMEDIATE", OpcodeSpec::new(Operations::add_i64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("SUB_U64", OpcodeSpec::new(Operations::sub_u64 as Op, OPERANDS_PACK2));
        m.insert("SUB_U64_IMMEDIATE", OpcodeSpec::new(Operations::sub_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("SUB_I64", OpcodeSpec::new(Operations::sub_i64 as Op, OPERANDS_PACK2));
        m.insert("SUB_I64_IMMEDIATE", OpcodeSpec::new(Operations::sub_i64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("MUL_U64", OpcodeSpec::new(Operations::mul_u64 as Op, OPERANDS_PACK2));
        m.insert("MUL_U64_IMMEDIATE", OpcodeSpec::new(Operations::mul_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("MUL_I64", OpcodeSpec::new(Operations::mul_i64 as Op, OPERANDS_PACK2));
        m.insert("MUL_I64_IMMEDIATE", OpcodeSpec::new(Operations::mul_i64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("DIV_U64", OpcodeSpec::new(Operations::div_u64 as Op, OPERANDS_PACK2));
        m.insert("DIV_U64_IMMEDIATE", OpcodeSpec::new(Operations::div_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("DIV_I64", OpcodeSpec::new(Operations::div_i64 as Op, OPERANDS_PACK2));
        m.insert("DIV_I64_IMMEDIATE", OpcodeSpec::new(Operations::div_i64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("ABS", OpcodeSpec::new(Operations::abs as Op, OPERANDS_PACK2));
        m.insert("MOD_I64", OpcodeSpec::new(Operations::mod_i64 as Op, OPERANDS_PACK2));
        m.insert("NEG_I64", OpcodeSpec::new(Operations::neg_i64 as Op, OPERANDS_PACK2));
        m.insert("U64_TO_F64", OpcodeSpec::new(Operations::u64_to_f64 as Op, OPERANDS_PACK2));
        m.insert("I64_TO_F64", OpcodeSpec::new(Operations::i64_to_f64 as Op, OPERANDS_PACK2));

        // Float ops
        m.insert("ADD_F64", OpcodeSpec::new(Operations::add_f64 as Op, OPERANDS_PACK2));
        m.insert("ADD_F64_IMMEDIATE", OpcodeSpec::new(Operations::add_f64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("SUB_F64", OpcodeSpec::new(Operations::sub_f64 as Op, OPERANDS_PACK2));
        m.insert("SUB_F64_IMMEDIATE", OpcodeSpec::new(Operations::sub_f64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("MUL_F64", OpcodeSpec::new(Operations::mul_f64 as Op, OPERANDS_PACK2));
        m.insert("MUL_F64_IMMEDIATE", OpcodeSpec::new(Operations::mul_f64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("DIV_F64", OpcodeSpec::new(Operations::div_f64 as Op, OPERANDS_PACK2));
        m.insert("DIV_F64_IMMEDIATE", OpcodeSpec::new(Operations::div_f64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("ABS_F64", OpcodeSpec::new(Operations::abs_f64 as Op, OPERANDS_PACK2));
        m.insert("NEG_F64", OpcodeSpec::new(Operations::neg_f64 as Op, OPERANDS_PACK2));
        m.insert("TO_I64", OpcodeSpec::new(Operations::to_i64 as Op, OPERANDS_PACK2));

        // Bit ops
        m.insert("AND_U64", OpcodeSpec::new(Operations::and_u64 as Op, OPERANDS_PACK2));
        m.insert("AND_U64_IMMEDIATE", OpcodeSpec::new(Operations::and_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("OR_U64", OpcodeSpec::new(Operations::or_u64 as Op, OPERANDS_PACK2));
        m.insert("OR_U64_IMMEDIATE", OpcodeSpec::new(Operations::or_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("XOR_U64", OpcodeSpec::new(Operations::xor_u64 as Op, OPERANDS_PACK2));
        m.insert("XOR_U64_IMMEDIATE", OpcodeSpec::new(Operations::xor_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("NOT_U64", OpcodeSpec::new(Operations::not_u64 as Op, OPERANDS_PACK2));
        m.insert("SHL_U64", OpcodeSpec::new(Operations::shl_u64 as Op, OPERANDS_PACK2));
        m.insert("SHL_U64_IMMEDIATE", OpcodeSpec::new(Operations::shl_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("SHL_I64", OpcodeSpec::new(Operations::shl_i64 as Op, OPERANDS_PACK2));
        m.insert("SHL_I64_IMMEDIATE", OpcodeSpec::new(Operations::shl_i64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("SHR_U64", OpcodeSpec::new(Operations::shr_u64 as Op, OPERANDS_PACK2));
        m.insert("SHR_U64_IMMEDIATE", OpcodeSpec::new(Operations::shr_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("SHR_I64", OpcodeSpec::new(Operations::shr_i64 as Op, OPERANDS_PACK2));
        m.insert("SHR_I64_IMMEDIATE", OpcodeSpec::new(Operations::shr_i64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("ROL_U64", OpcodeSpec::new(Operations::rol_u64 as Op, OPERANDS_PACK2));
        m.insert("ROL_U64_IMMEDIATE", OpcodeSpec::new(Operations::rol_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("ROR_U64", OpcodeSpec::new(Operations::ror_u64 as Op, OPERANDS_PACK2));
        m.insert("ROR_U64_IMMEDIATE", OpcodeSpec::new(Operations::ror_u64_immediate as Op, OPERANDS_PACK1_VALUE));
        m.insert("COUNT_ONES_U64", OpcodeSpec::new(Operations::count_ones_u64 as Op, OPERANDS_PACK2));
        m.insert("COUNT_ZEROS_U64", OpcodeSpec::new(Operations::count_zeros_u64 as Op, OPERANDS_PACK2));
        m.insert("TRAILING_ZEROS_U64", OpcodeSpec::new(Operations::trailing_zeros_u64 as Op, OPERANDS_PACK2));

        // Memory ops
        m.insert("LOAD_U64", OpcodeSpec::new(Operations::load_u64 as Op, OPERANDS_PACK3_VALUE));
        m.insert("LOAD_U32", OpcodeSpec::new(Operations::load_u32 as Op, OPERANDS_PACK3_VALUE));
        m.insert("LOAD_U16", OpcodeSpec::new(Operations::load_u16 as Op, OPERANDS_PACK3_VALUE));
        m.insert("LOAD_U8", OpcodeSpec::new(Operations::load_u8 as Op, OPERANDS_PACK3_VALUE));
        m.insert("STORE_U64", OpcodeSpec::new(Operations::store_u64 as Op, OPERANDS_PACK3_VALUE));
        m.insert("STORE_U32", OpcodeSpec::new(Operations::store_u32 as Op, OPERANDS_PACK3_VALUE));
        m.insert("STORE_U16", OpcodeSpec::new(Operations::store_u16 as Op, OPERANDS_PACK3_VALUE));
        m.insert("STORE_U8", OpcodeSpec::new(Operations::store_u8 as Op, OPERANDS_PACK3_VALUE));

        // atomic ops (u64/u32/u16/u8 variants for load/store/add/sub) - common pattern: PACK3_VALUE for load/store, PACK4_VALUE for add/sub
        m.insert("ATOMIC_LOAD_U64", OpcodeSpec::new(Operations::atomic_load_u64 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_STORE_U64", OpcodeSpec::new(Operations::atomic_store_u64 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_ADD_U64", OpcodeSpec::new(Operations::atomic_add_u64 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_SUB_U64", OpcodeSpec::new(Operations::atomic_sub_u64 as Op, OPERANDS_PACK4_VALUE));

        m.insert("ATOMIC_LOAD_U32", OpcodeSpec::new(Operations::atomic_load_u32 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_STORE_U32", OpcodeSpec::new(Operations::atomic_store_u32 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_ADD_U32", OpcodeSpec::new(Operations::atomic_add_u32 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_SUB_U32", OpcodeSpec::new(Operations::atomic_sub_u32 as Op, OPERANDS_PACK4_VALUE));

        m.insert("ATOMIC_LOAD_U16", OpcodeSpec::new(Operations::atomic_load_u16 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_STORE_U16", OpcodeSpec::new(Operations::atomic_store_u16 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_ADD_U16", OpcodeSpec::new(Operations::atomic_add_u16 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_SUB_U16", OpcodeSpec::new(Operations::atomic_sub_u16 as Op, OPERANDS_PACK4_VALUE));

        m.insert("ATOMIC_LOAD_U8", OpcodeSpec::new(Operations::atomic_load_u8 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_STORE_U8", OpcodeSpec::new(Operations::atomic_store_u8 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_ADD_U8", OpcodeSpec::new(Operations::atomic_add_u8 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_SUB_U8", OpcodeSpec::new(Operations::atomic_sub_u8 as Op, OPERANDS_PACK4_VALUE));

        // Atomic signed variants
        m.insert("ATOMIC_LOAD_I8", OpcodeSpec::new(Operations::atomic_load_i8 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_LOAD_I16", OpcodeSpec::new(Operations::atomic_load_i16 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_LOAD_I32", OpcodeSpec::new(Operations::atomic_load_i32 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_LOAD_I64", OpcodeSpec::new(Operations::atomic_load_i64 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_STORE_I8", OpcodeSpec::new(Operations::atomic_store_i8 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_STORE_I16", OpcodeSpec::new(Operations::atomic_store_i16 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_STORE_I32", OpcodeSpec::new(Operations::atomic_store_i32 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_STORE_I64", OpcodeSpec::new(Operations::atomic_store_i64 as Op, OPERANDS_PACK3_VALUE));
        m.insert("ATOMIC_ADD_I8", OpcodeSpec::new(Operations::atomic_add_i8 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_ADD_I16", OpcodeSpec::new(Operations::atomic_add_i16 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_ADD_I32", OpcodeSpec::new(Operations::atomic_add_i32 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_ADD_I64", OpcodeSpec::new(Operations::atomic_add_i64 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_SUB_I8", OpcodeSpec::new(Operations::atomic_sub_i8 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_SUB_I16", OpcodeSpec::new(Operations::atomic_sub_i16 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_SUB_I32", OpcodeSpec::new(Operations::atomic_sub_i32 as Op, OPERANDS_PACK4_VALUE));
        m.insert("ATOMIC_SUB_I64", OpcodeSpec::new(Operations::atomic_sub_i64 as Op, OPERANDS_PACK4_VALUE));

        m
    })
}