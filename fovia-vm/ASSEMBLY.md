# Fovia VM Assembler Specification
from [instructions_text](./src/vm/instruction/instructions)

## 1. Source Structure (PreDecoder Rules)

- The source is composed of **function blocks**.
- The function named `MAIN` is the entry point and is required.
- A function block begins with its name on a line by itself and is followed by instruction lines.
- Lines can contain comments after `;`.
- Blank lines are ignored.
- Function names and labels are **case-insensitive** (internally uppercased).

Example:
```
MAIN
    LOAD_U64_IMMEDIATE r0 123
    EXIT r0

FUNC1
    RET
```

## 2. Labels (Function-Local)

- A label is defined by appending `:` to a name.
- Labels are **function-local**.
- Labels are resolved to a **word offset** within the function.
- Labels may appear before an instruction on the same line:

```
LOOP: ADD_U64 r1 r2
```

## 3. Tokens and Comments

- Tokens are separated by whitespace (spaces or tabs).
- The `;` character starts a comment that runs to end-of-line.

## 4. Registers and Operands

- Registers are written as `rN` (case-insensitive) where `N` is a number.
- A register token is treated as a **register index**.
- Immediate values are 64-bit integers with optional prefixes:
  - `0x` for hex, `0b` for binary, `0o` for octal, otherwise decimal.
  - `_` is allowed as a visual separator.
  - Negative values are allowed.

  ### Packed Register Operands

  Many instructions use *packed registers* in the first operand word. In source, these are written as multiple `rN` tokens (e.g. `rA rB rC`), but the predecoder packs them into a single 64-bit operand (one byte per register index).

  Rules derived from the predecoder:
  - The packed register list length is fixed per opcode (e.g. 1, 2, 3, 4, or 7).
  - Register indices must fit in 8 bits.
  - If the packed operand expects N registers but the **first token is numeric (not `rN`)**, it is treated as a **single immediate value** instead of a packed register list. This allows using immediates in packed positions where supported.

## 5. Instruction Encoding (Word Length)

Instruction length is determined by the operand plan:

- **1 word**: no operands
- **2 words**: `rA...` (packed register operand)
- **3 words**: `rA... imm` or `rA... offset` or `rOffset call_frame` or `rOffset rB call_frame`

Notes (from PreDecoder):
- `imm` is a 64-bit immediate.
- `offset` is a label offset (word index).
- `call_frame` is a function address (resolved by the pre-decoder).
- If an opcode allows fewer tokens than its maximum, missing operands are **auto-filled with 0**.
- If too many tokens are provided, decoding fails.

## 6. Static Data (DATA Directive)

Static data can be defined **outside functions only**:

```
DATA NAME "text\n"
DATA BYTES 0 1 2 255
DATA BIN b64"SGVsbG8gV29ybGQh"
```

- `DATA NAME "..."` defines bytes from a string with escapes: `\n`, `\r`, `\t`, `\\`, `\"`, `\xNN`.
- `DATA NAME <byte>...` defines raw bytes (each token must be in `0..=255`).
- `DATA NAME b64"..."` or `DATA NAME base64"..."` defines bytes from Base64 (whitespace is ignored). You can also omit quotes and pass a single Base64 token.
- The symbol `NAME` can be used as an immediate anywhere a value is accepted, and resolves to the allocated static pointer (a `VPtr` as `u64`).
- `DATA` is **not allowed inside a function**.

## 7. Control Flow Rules (PreDecoder Resolution)

- `CALL` operands are function names (resolved to function indices).
- Jump instructions accept labels as the offset operand.
- Labels used outside jump opcodes are treated as **static data symbols** (if defined). Otherwise, it is an error.
- `MAIN` must end with `EXIT`, not `RET`.

## 8. Opcode List

The following list is based on the instruction reference file:

### Integer Arithmetic
- `ADD_U64 rA rB` — `rA = rA + rB`
- `ADD_U64_IMMEDIATE rA imm` — `rA = rA + imm`
- `ADD_I64 rA rB`
- `ADD_I64_IMMEDIATE rA imm`
- `SUB_U64 rA rB`
- `SUB_U64_IMMEDIATE rA imm`
- `SUB_I64 rA rB`
- `SUB_I64_IMMEDIATE rA imm`
- `MUL_U64 rA rB`
- `MUL_U64_IMMEDIATE rA imm`
- `MUL_I64 rA rB`
- `MUL_I64_IMMEDIATE rA imm`
- `DIV_U64 rA rB`
- `DIV_U64_IMMEDIATE rA imm`
- `DIV_I64 rA rB`
- `DIV_I64_IMMEDIATE rA imm`
- `ABS rA` — `rA = |rA|`
- `MOD_I64 rA rB` — `rA = rA % rB`
- `NEG_I64 rA` — `rA = -rA`

### Integer ↔ Float
- `U64_TO_F64 rA rB` — `rA = f64(rB)`
- `I64_TO_F64 rA rB` — `rA = f64(rB)`
- `TO_I64 rA rB` — `rA = i64(rB)`

### Float Arithmetic
- `ADD_F64 rA rB`
- `ADD_F64_IMMEDIATE rA imm`
- `SUB_F64 rA rB`
- `SUB_F64_IMMEDIATE rA imm`
- `MUL_F64 rA rB`
- `MUL_F64_IMMEDIATE rA imm`
- `DIV_F64 rA rB`
- `DIV_F64_IMMEDIATE rA imm`
- `ABS_F64 rA`
- `NEG_F64 rA`

### Bit Operations
- `AND_U64 rA rB`
- `AND_U64_IMMEDIATE rA imm`
- `OR_U64 rA rB`
- `OR_U64_IMMEDIATE rA imm`
- `XOR_U64 rA rB`
- `XOR_U64_IMMEDIATE rA imm`
- `NOT_U64 rA`
- `SHL_U64 rA rB`
- `SHL_U64_IMMEDIATE rA imm`
- `SHL_I64 rA rB`
- `SHL_I64_IMMEDIATE rA imm`
- `SHR_U64 rA rB`
- `SHR_U64_IMMEDIATE rA imm`
- `SHR_I64 rA rB`
- `SHR_I64_IMMEDIATE rA imm`
- `ROL_U64 rA rB`
- `ROL_U64_IMMEDIATE rA imm`
- `ROR_U64 rA rB`
- `ROR_U64_IMMEDIATE rA imm`
- `COUNT_ONES_U64 rA rB`
- `COUNT_ZEROS_U64 rA rB`
- `TRAILING_ZEROS_U64 rA rB`

### Register Ops
- `MOV rA rB` — `rA = rB`
- `LOAD_U64_IMMEDIATE rA imm` — `rA = imm`
- `SWAP rA rB` — swap registers

### Memory Load/Store
- `LOAD_U64 rA rB imm` — `u64 rB = memory[rA + imm]`
- `LOAD_U32 rA rB imm`
- `LOAD_U16 rA rB imm`
- `LOAD_U8 rA rB imm`
- `STORE_U64 rA rB imm` — `memory[rA + imm] = u64 rB`
- `STORE_U32 rA rB imm`
- `STORE_U16 rA rB imm`
- `STORE_U8 rA rB imm`
- `LOAD_I8 rA rB imm`
- `LOAD_I16 rA rB imm`
- `LOAD_I32 rA rB imm`
- `LOAD_I64 rA rB imm`
- `STORE_I8 rA rB imm`
- `STORE_I16 rA rB imm`
- `STORE_I32 rA rB imm`
- `STORE_I64 rA rB imm`

### Atomic Memory Ops
- `ATOMIC_LOAD_U64 rA rB imm`
- `ATOMIC_LOAD_U32 rA rB imm`
- `ATOMIC_LOAD_U16 rA rB imm`
- `ATOMIC_LOAD_U8 rA rB imm`
- `ATOMIC_STORE_U64 rA rB imm`
- `ATOMIC_STORE_U32 rA rB imm`
- `ATOMIC_STORE_U16 rA rB imm`
- `ATOMIC_STORE_U8 rA rB imm`
- `ATOMIC_ADD_U64 rA rB rC imm`
- `ATOMIC_ADD_U32 rA rB rC imm`
- `ATOMIC_ADD_U16 rA rB rC imm`
- `ATOMIC_ADD_U8 rA rB rC imm`
- `ATOMIC_SUB_U64 rA rB rC imm`
- `ATOMIC_SUB_U32 rA rB rC imm`
- `ATOMIC_SUB_U16 rA rB rC imm`
- `ATOMIC_SUB_U8 rA rB rC imm`
- `ATOMIC_LOAD_I8 rA rB imm`
- `ATOMIC_LOAD_I16 rA rB imm`
- `ATOMIC_LOAD_I32 rA rB imm`
- `ATOMIC_LOAD_I64 rA rB imm`
- `ATOMIC_STORE_I8 rA rB imm`
- `ATOMIC_STORE_I16 rA rB imm`
- `ATOMIC_STORE_I32 rA rB imm`
- `ATOMIC_STORE_I64 rA rB imm`
- `ATOMIC_ADD_I8 rA rB rC imm`
- `ATOMIC_ADD_I16 rA rB rC imm`
- `ATOMIC_ADD_I32 rA rB rC imm`
- `ATOMIC_ADD_I64 rA rB rC imm`
- `ATOMIC_SUB_I8 rA rB rC imm`
- `ATOMIC_SUB_I16 rA rB rC imm`
- `ATOMIC_SUB_I32 rA rB rC imm`
- `ATOMIC_SUB_I64 rA rB rC imm`

### Memory Management
- `ALLOC rA rB imm` — `rB = allocate_memory(rA + imm)`
- `REALLOC rA rB` — `rB = reallocate_memory(rA)`
- `DEALLOC rA` — `deallocate_memory(rA)`
- `MEMORY_COPY rA rB rC` — copy memory range

### Control Flow
- `JUMP rA offset` — `pc = rA + offset`
- `EQ_JUMP rA rB rC offset` — jump if `rB == rC`
- `NEQ_JUMP rA rB rC offset` — jump if `rB != rC`
- `LT_U64_JUMP rA rB rC offset`
- `LTE_U64_JUMP rA rB rC offset`
- `LT_I64_JUMP rA rB rC offset`
- `LTE_I64_JUMP rA rB rC offset`
- `GT_U64_JUMP rA rB rC offset`
- `GTE_U64_JUMP rA rB rC offset`
- `GT_I64_JUMP rA rB rC offset`
- `GTE_I64_JUMP rA rB rC offset`
- `CALL rOffset call_frame` — push frame and jump
- `CALL_THREAD rOffset rB call_frame`
- `RET` — return
- `EXIT r1` — exit program

### IO (Detailed)

#### SET_IO

Signature:
`SET_IO rA rB rC rD rE rF rG`

- `rA` = destination register for `fu_id`
- `rB` = IO type constant
- `rC..rG` = IO-type specific arguments (some are unused depending on type)

Mapping to operations (from io_ops):

1. **STDOUT_WRITE / STD_IO_WRITE**
  - `rC` = buffer pointer (VPtr), `rD` = length
2. **STDIN_READ / STD_IO_READ**
  - `rC` = buffer pointer (VPtr), `rD` = length
3. **SLEEP**
  - `rC` = milliseconds
4. **READ** (file/socket)
  - `rC` = handle, `rD` = buffer pointer (VPtr), `rE` = length
5. **WRITE** (file/socket)
  - `rC` = handle, `rD` = buffer pointer (VPtr), `rE` = length
6. **TCP_LISTEN**
  - `rC` = ip pointer (VPtr), `rD` = port (u16), `rE` = backlog (u16), `rF` = flags (u16), `rG` = family (u8)
7. **TCP_CONNECT**
  - `rC` = ip pointer (VPtr), `rD` = port (u16), `rE` = flags (u16), `rF` = family (u8)
8. **TCP_ACCEPT**
  - `rC` = listener handle
9. **SHUTDOWN**
  - `rC` = socket handle
10. **RANDOM_BYTES**
   - `rC` = buffer pointer (VPtr), `rD` = length
11. **TIME_NOW**
   - no extra args

Notes:
- `SET_IO` returns a **future id** (`fu_id`) in `rA` immediately.
- Pointer arguments (`rC`, `rD`, etc.) are **VPtr** virtual pointers that are resolved to actual host pointers by the VM.
- For TCP ops, `ip_ptr` points to raw bytes of IPv4 (4 bytes) or IPv6 (16 bytes). See io backend for platform details.

#### WAIT_IO

Signature:
`WAIT_IO rA rB rC`

- `rA` = timeout ms (`-1` for infinite)
- `rB` = max events
- `rC` = output: number of collected events

This is a blocking instruction that collects completed IO events into the internal queue.

#### GET_AN_IO

Signature:
`GET_AN_IO rA rB rC rD`

- `rA` = output `fu_id` (0 if no event)
- `rB` = output type code
- `rC`, `rD` = output payload fields (type-dependent)

Type codes:
- `1`: Stream IO completed (read/write). `rC = len`.
- `2`: New handle (accept/connect). `rC = handle`.
- `3`: Sleep completed. No payload.
- `4`: TimeNow. `rC = low`, `rD = high` (u64 split).
- `5`: Simple OK (shutdown, etc.). No payload.
- Negative value (encoded as `u64`): error kind. `rC = retryable (1/0)`. `rD` unused.

If there is no completed event, only `rA` is set to `0` and other registers are left unchanged.

#### IO Type Constants (for `SET_IO`)
- `STD_IO_WRITE` / `STDOUT_WRITE` — 1
- `STD_IO_READ` / `STDIN_READ` — 2
- `SLEEP` — 3
- `READ` — 4
- `WRITE` — 5
- `TCP_LISTEN` — 6
- `TCP_CONNECT` — 7
- `TCP_ACCEPT` — 8
- `SHUTDOWN` — 9
- `RANDOM_BYTES` — 10
- `TIME_NOW` — 11

## 9. Notes

- Atomic operations return the old value in `rA`.
- Packed register operands fit within one word when register count is 8 or fewer.
- The VM maintains independent program counters per call frame.
- The predecoder is defined in [fovia-vm/src/vm/pre_decoder.rs](fovia-vm/src/vm/pre_decoder.rs).
- IO behaviors are implemented in [fovia-vm/src/vm/instruction/operations/io_ops.rs](fovia-vm/src/vm/instruction/operations/io_ops.rs) and [fovia-vm/src/vm/io/mod.rs](fovia-vm/src/vm/io/mod.rs).
