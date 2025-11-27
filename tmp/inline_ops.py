from __future__ import annotations
from pathlib import Path
import re

PATH = Path(r"i:\RustBuilds\mikan-script\src\vm\operations\mod.rs")
text = PATH.read_text(encoding="utf-8")

func_pattern = re.compile(r"pub fn\s+(\w+)\s*\(vm: &mut VM,\s*([^)]*?)\s*\)\s*\{", re.MULTILINE)


def find_matching_brace(src: str, open_idx: int) -> int:
    assert src[open_idx] == "{"
    depth = 0
    in_string = False
    escape = False
    for i in range(open_idx, len(src)):
        ch = src[i]
        if in_string:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == '"':
                in_string = False
        else:
            if ch == '"':
                in_string = True
            elif ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    return i
    raise ValueError("Unmatched brace")


def dedent_lines(body: str) -> list[str]:
    lines = body.splitlines()
    while lines and lines[0].strip() == "":
        lines.pop(0)
    while lines and lines[-1].strip() == "":
        lines.pop()
    if not lines:
        return []
    min_indent = min((len(line) - len(line.lstrip(" ")) for line in lines if line.strip()), default=0)
    dedented: list[str] = []
    for line in lines:
        if line.strip():
            dedented.append(line[min_indent:])
        else:
            dedented.append("")
    return dedented


func_map: dict[str, tuple[str, str, list[str]]] = {}
search_pos = 0
while True:
    match = func_pattern.search(text, search_pos)
    if not match:
        break
    name = match.group(1)
    params_raw = match.group(2)
    params = [p.strip() for p in params_raw.split(',') if p.strip()]
    if len(params) != 2:
        search_pos = match.end()
        continue
    param_names: list[str] = []
    for idx, param in enumerate(params):
        name_part = param.split(':', 1)[0].strip()
        if not name_part:
            name_part = f"arg{idx}"
        if name_part == '_':
            name_part = '_'
        param_names.append(name_part)
    brace_idx = match.end() - 1
    end_idx = find_matching_brace(text, brace_idx)
    body = text[brace_idx + 1:end_idx]
    func_map[name] = (param_names[0], param_names[1], dedent_lines(body))
    search_pos = end_idx + 1


arm_pattern = re.compile(
    r"(?P<indent>\s*)Instruction::(?P<variant>[A-Za-z0-9_]+)\(a, b\) => Operations::(?P<func>[A-Za-z0-9_]+)\(vm, \*a, \*b\),"
)


count_used = set()

def replace_arm(match: re.Match[str]) -> str:
    indent = match.group('indent')
    variant = match.group('variant')
    func = match.group('func')
    info = func_map.get(func)
    if info is None:
        raise KeyError(f"Function body for {func} not found")
    dst_name, src_name, body_lines = info
    inner_indent = indent + "    "
    lines: list[str] = []
    lines.append(f"{indent}Instruction::{variant}(a, b) => {{")
    def binding(name: str, expr: str) -> str:
        if name == '_':
            return f"{inner_indent}let _ = {expr};"
        return f"{inner_indent}let {name} = {expr};"
    lines.append(binding(dst_name, "*a"))
    lines.append(binding(src_name, "*b"))
    if body_lines:
        for line in body_lines:
            if line:
                lines.append(f"{inner_indent}{line}")
            else:
                lines.append("")
    lines.append(f"{indent}}},")
    count_used.add(func)
    return "\n".join(lines)

new_text, replacements = arm_pattern.subn(replace_arm, text)
if replacements == 0:
    raise RuntimeError("No match arms were replaced")
PATH.write_text(new_text, encoding="utf-8")
print(f"Replaced {replacements} match arms")
