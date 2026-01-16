use std::collections::{HashMap, HashSet};

pub struct AsmOptimizer;

impl AsmOptimizer {
    pub fn optimize(asm: &str) -> String {
        let mut lines: Vec<String> = asm.lines().map(|l| l.to_string()).collect();

        Self::fold_simple_loop_increment(&mut lines);
        Self::fold_simple_counter_loop(&mut lines);
        Self::fold_loop_match_increment(&mut lines);
        Self::fold_mov_add_mov(&mut lines);
        Self::inline_single_use_jump_block(&mut lines);
        Self::remove_overwritten_defs(&mut lines);
        Self::remove_zero_immediates(&mut lines);
        Self::fold_load_mov(&mut lines);
        Self::collapse_mov_chain(&mut lines);
        Self::remove_self_mov(&mut lines);
        Self::remove_duplicate_lines(&mut lines);

        let mut out = String::new();
        for line in lines {
            out.push_str(&line);
            out.push('\n');
        }
        out
    }

    fn remove_self_mov(lines: &mut Vec<String>) {
        lines.retain(|line| {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("MOV ") {
                let mut parts = rest.split_whitespace();
                if let (Some(dst), Some(src)) = (parts.next(), parts.next()) {
                    return dst != src;
                }
            }
            true
        });
    }

    fn remove_duplicate_lines(lines: &mut Vec<String>) {
        let mut out = Vec::with_capacity(lines.len());
        let mut prev: Option<String> = None;
        for line in lines.iter() {
            let trimmed = line.trim();
            if prev.as_deref() == Some(trimmed) {
                continue;
            }
            prev = Some(trimmed.to_string());
            out.push(line.clone());
        }
        *lines = out;
    }

    fn fold_mov_add_mov(lines: &mut Vec<String>) {
        let mut out = Vec::with_capacity(lines.len());
        let mut i = 0usize;
        while i + 2 < lines.len() {
            let l1 = lines[i].trim();
            let l2 = lines[i + 1].trim();
            let l3 = lines[i + 2].trim();

            if let Some((dst1, src1)) = Self::parse_mov(l1) {
                if let Some((add_dst, imm, is_add)) = Self::parse_add_sub_imm(l2) {
                    if add_dst == dst1 {
                        if let Some((dst3, src3)) = Self::parse_mov(l3) {
                            if src3 == dst1 {
                                let indent2 = &lines[i + 1][..lines[i + 1].len() - l2.len()];
                                let indent1 = &lines[i][..lines[i].len() - l1.len()];
                                out.push(format!("{indent1}MOV {dst3} {src1}"));
                                let op = if is_add { "ADD_U64_IMMEDIATE" } else { "SUB_U64_IMMEDIATE" };
                                out.push(format!("{indent2}{op} {dst3} {imm}"));
                                i += 3;
                                continue;
                            }
                        }
                    }
                }
            }

            out.push(lines[i].clone());
            i += 1;
        }
        while i < lines.len() {
            out.push(lines[i].clone());
            i += 1;
        }
        *lines = out;
    }

    fn parse_mov(line: &str) -> Option<(String, String)> {
        let rest = line.strip_prefix("MOV ")?;
        let mut parts = rest.split_whitespace();
        let dst = parts.next()?.to_string();
        let src = parts.next()?.to_string();
        Some((dst, src))
    }

    fn parse_add_sub_imm(line: &str) -> Option<(String, String, bool)> {
        if let Some(rest) = line.strip_prefix("ADD_U64_IMMEDIATE ") {
            let mut parts = rest.split_whitespace();
            let dst = parts.next()?.to_string();
            let imm = parts.next()?.to_string();
            return Some((dst, imm, true));
        }
        if let Some(rest) = line.strip_prefix("SUB_U64_IMMEDIATE ") {
            let mut parts = rest.split_whitespace();
            let dst = parts.next()?.to_string();
            let imm = parts.next()?.to_string();
            return Some((dst, imm, false));
        }
        None
    }

    fn fold_loop_match_increment(lines: &mut Vec<String>) {
        let mut label_pos: HashMap<String, usize> = HashMap::new();
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                let label = trimmed.trim_end_matches(':').to_string();
                label_pos.insert(label, idx);
            }
        }

        let mut ref_counts: HashMap<String, usize> = HashMap::new();
        for line in lines.iter() {
            let trimmed = line.trim();
            if let Some(label) = Self::parse_jump_label(trimmed) {
                *ref_counts.entry(label).or_insert(0) += 1;
            }
        }

        let mut to_remove: HashSet<usize> = HashSet::new();
        let mut replacements: HashMap<usize, Vec<String>> = HashMap::new();

        let mut i = 0usize;
        while i + 3 < lines.len() {
            let l0 = lines[i].trim();
            let l1 = lines[i + 1].trim();
            let l2 = lines[i + 2].trim();
            let l3 = lines[i + 3].trim();

            if let Some((reg_a, reg_b, label_arm)) = Self::parse_gte_jump(l0) {
                if let Some((mov_dst, mov_src)) = Self::parse_mov(l1) {
                    if let Some((add_dst, imm, is_add)) = Self::parse_add_sub_imm(l2) {
                        if is_add && mov_dst == add_dst && mov_src == reg_a {
                            if let Some(label_end) = Self::parse_jump_only(l3) {
                                let Some(&pos_arm) = label_pos.get(&label_arm) else {
                                    i += 1;
                                    continue;
                                };
                                let Some(&pos_end) = label_pos.get(&label_end) else {
                                    i += 1;
                                    continue;
                                };

                                if *ref_counts.get(&label_arm).unwrap_or(&0) != 1
                                    || *ref_counts.get(&label_end).unwrap_or(&0) != 1
                                {
                                    i += 1;
                                    continue;
                                }

                                // Arm block: LABEL, LOAD_U64_IMMEDIATE rX 0, JUMP exit, LOAD_U64_IMMEDIATE mov_dst 0, JUMP label_end
                                if pos_arm + 4 >= lines.len() {
                                    i += 1;
                                    continue;
                                }
                                let arm_load = lines[pos_arm + 1].trim();
                                let arm_jump = lines[pos_arm + 2].trim();
                                let arm_load2 = lines[pos_arm + 3].trim();
                                let arm_jump2 = lines[pos_arm + 4].trim();
                                let Some(exit_label) = Self::parse_jump_only(arm_jump) else {
                                    i += 1;
                                    continue;
                                };
                                if !arm_load.starts_with("LOAD_U64_IMMEDIATE ") {
                                    i += 1;
                                    continue;
                                }
                                if !arm_load2.starts_with("LOAD_U64_IMMEDIATE ") {
                                    i += 1;
                                    continue;
                                }
                                if let Some(end_label2) = Self::parse_jump_only(arm_jump2) {
                                    if end_label2 != label_end {
                                        i += 1;
                                        continue;
                                    }
                                } else {
                                    i += 1;
                                    continue;
                                }

                                // End block: LABEL, MOV reg_a mov_dst, LOAD_U64_IMMEDIATE mov_dst 0, JUMP loop_label
                                if pos_end + 3 >= lines.len() {
                                    i += 1;
                                    continue;
                                }
                                let end_mov = lines[pos_end + 1].trim();
                                let end_load = lines[pos_end + 2].trim();
                                let end_jump = lines[pos_end + 3].trim();
                                let Some(loop_label) = Self::parse_jump_only(end_jump) else {
                                    i += 1;
                                    continue;
                                };
                                if let Some((end_dst, end_src)) = Self::parse_mov(end_mov) {
                                    if end_dst != reg_a || end_src != mov_dst {
                                        i += 1;
                                        continue;
                                    }
                                } else {
                                    i += 1;
                                    continue;
                                }
                                if !end_load.starts_with("LOAD_U64_IMMEDIATE ") {
                                    i += 1;
                                    continue;
                                }

                                // Replace with: GTE jump to exit, add imm to reg_a, jump loop
                                let indent0 = &lines[i][..lines[i].len() - l0.len()];
                                let indent1 = &lines[i + 2][..lines[i + 2].len() - l2.len()];
                                let indent2 = &lines[i + 3][..lines[i + 3].len() - l3.len()];
                                replacements.insert(
                                    i,
                                    vec![
                                        format!("{indent0}GTE_U64_JUMP r255 {reg_a} {reg_b} {exit_label}"),
                                        format!("{indent1}ADD_U64_IMMEDIATE {reg_a} {imm}"),
                                        format!("{indent2}JUMP r255 {loop_label}"),
                                    ],
                                );

                                for idx in i + 1..=i + 3 {
                                    to_remove.insert(idx);
                                }
                                // Remove arm/end blocks
                                for idx in pos_arm..=pos_arm + 4 {
                                    to_remove.insert(idx);
                                }
                                for idx in pos_end..=pos_end + 3 {
                                    to_remove.insert(idx);
                                }
                            }
                        }
                    }
                }
            }

            i += 1;
        }

        if replacements.is_empty() && to_remove.is_empty() {
            return;
        }

        let mut out = Vec::with_capacity(lines.len());
        for (idx, line) in lines.iter().enumerate() {
            if let Some(rep) = replacements.get(&idx) {
                out.extend(rep.clone());
                continue;
            }
            if to_remove.contains(&idx) {
                continue;
            }
            out.push(line.clone());
        }
        *lines = out;
    }

    fn parse_jump_label(line: &str) -> Option<String> {
        let mut parts = line.split_whitespace();
        let op = parts.next()?;
        if op.ends_with("_JUMP") || op == "JUMP" {
            return parts.last().map(|v| v.to_string());
        }
        None
    }

    fn parse_jump_only(line: &str) -> Option<String> {
        let rest = line.strip_prefix("JUMP ")?;
        let mut parts = rest.split_whitespace();
        parts.next()?;
        parts.next().map(|v| v.to_string())
    }

    fn parse_gte_jump(line: &str) -> Option<(String, String, String)> {
        let rest = line.strip_prefix("GTE_U64_JUMP ")?;
        let mut parts = rest.split_whitespace();
        let _addr = parts.next()?;
        let reg_a = parts.next()?.to_string();
        let reg_b = parts.next()?.to_string();
        let label = parts.next()?.to_string();
        Some((reg_a, reg_b, label))
    }

    fn fold_simple_counter_loop(lines: &mut Vec<String>) {
        let mut label_pos: HashMap<String, usize> = HashMap::new();
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                let label = trimmed.trim_end_matches(':').to_string();
                label_pos.insert(label, idx);
            }
        }

        let mut to_remove: HashSet<usize> = HashSet::new();
        let mut replacements: HashMap<usize, Vec<String>> = HashMap::new();

        let mut i = 0usize;
        while i + 8 < lines.len() {
            let loop_label_line = lines[i].trim();
            if !loop_label_line.ends_with(':') {
                i += 1;
                continue;
            }
            let loop_label = loop_label_line.trim_end_matches(':').to_string();

            let l1 = lines[i + 1].trim();
            let l2 = lines[i + 2].trim();
            let l3 = lines[i + 3].trim();
            let l4 = lines[i + 4].trim();

            let Some((imm_reg, imm_val)) = Self::parse_load_imm(l1) else {
                i += 1;
                continue;
            };
            let Some((counter_reg, limit_reg, arm_label)) = Self::parse_gte_jump(l2) else {
                i += 1;
                continue;
            };
            if limit_reg != imm_reg {
                i += 1;
                continue;
            }
            let Some((mov_tmp, mov_src)) = Self::parse_mov(l3) else {
                i += 1;
                continue;
            };
            let Some((add_dst, add_imm, is_add)) = Self::parse_add_sub_imm(l4) else {
                i += 1;
                continue;
            };
            if !is_add || mov_src != counter_reg || add_dst != mov_tmp || add_imm != "1" {
                i += 1;
                continue;
            }

            if i + 5 >= lines.len() {
                i += 1;
                continue;
            }
            let l5 = lines[i + 5].trim();
            let Some(end_label) = Self::parse_jump_only(l5) else {
                i += 1;
                continue;
            };

            if i + 6 >= lines.len() {
                i += 1;
                continue;
            }
            let l6 = lines[i + 6].trim();
            let Some((move_back_dst, move_back_src)) = Self::parse_mov(l6) else {
                i += 1;
                continue;
            };

            let Some(&arm_pos) = label_pos.get(&arm_label) else {
                i += 1;
                continue;
            };
            if arm_pos + 2 >= lines.len() {
                i += 1;
                continue;
            }
            let arm_load = lines[arm_pos + 1].trim();
            let arm_jump = lines[arm_pos + 2].trim();
            let Some(exit_label) = Self::parse_jump_only(arm_jump) else {
                i += 1;
                continue;
            };
            if !arm_load.starts_with("LOAD_U64_IMMEDIATE ") {
                i += 1;
                continue;
            }

            if let Some((dst, imm)) = Self::parse_load_imm(arm_load) {
                if dst != "r1" || imm != "0" {
                    i += 1;
                    continue;
                }
            } else {
                i += 1;
                continue;
            }

            let Some(&end_pos) = label_pos.get(&end_label) else {
                i += 1;
                continue;
            };
            if end_pos + 3 >= lines.len() {
                i += 1;
                continue;
            }
            let end_mov = lines[end_pos + 1].trim();
            let end_load = lines[end_pos + 2].trim();
            let end_jump = lines[end_pos + 3].trim();
            let Some(loop_jump_label) = Self::parse_jump_only(end_jump) else {
                i += 1;
                continue;
            };
            if loop_jump_label != loop_label {
                i += 1;
                continue;
            }
            let Some((end_dst, end_src)) = Self::parse_mov(end_mov) else {
                i += 1;
                continue;
            };
            if end_dst != move_back_dst || end_src != move_back_src {
                i += 1;
                continue;
            }
            if move_back_dst != counter_reg || move_back_src != mov_tmp {
                i += 1;
                continue;
            }
            if let Some((dst, imm)) = Self::parse_load_imm(end_load) {
                if dst != mov_tmp || imm != "0" {
                    i += 1;
                    continue;
                }
            } else {
                i += 1;
                continue;
            }

            let indent1 = &lines[i + 1][..lines[i + 1].len() - l1.len()];
            let indent2 = &lines[i + 2][..lines[i + 2].len() - l2.len()];
            let indent3 = &lines[i + 3][..lines[i + 3].len() - l3.len()];

            replacements.insert(
                i + 1,
                vec![
                    format!("{indent1}LOAD_U64_IMMEDIATE {imm_reg} {imm_val}"),
                    format!("{indent2}GTE_U64_JUMP r255 {counter_reg} {imm_reg} {exit_label}"),
                    format!("{indent3}ADD_U64_IMMEDIATE {counter_reg} 1"),
                    format!("{indent3}JUMP r255 {loop_label}"),
                ],
            );

            for idx in i + 2..=i + 6 {
                to_remove.insert(idx);
            }
            for idx in arm_pos..=arm_pos + 2 {
                to_remove.insert(idx);
            }
            for idx in end_pos..=end_pos + 3 {
                to_remove.insert(idx);
            }

            i += 1;
        }

        if replacements.is_empty() && to_remove.is_empty() {
            return;
        }

        let mut out = Vec::with_capacity(lines.len());
        for (idx, line) in lines.iter().enumerate() {
            if let Some(rep) = replacements.get(&idx) {
                out.extend(rep.clone());
                continue;
            }
            if to_remove.contains(&idx) {
                continue;
            }
            out.push(line.clone());
        }
        *lines = out;
    }

    fn fold_simple_loop_increment(lines: &mut Vec<String>) {
        let mut label_pos: HashMap<String, usize> = HashMap::new();
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                let label = trimmed.trim_end_matches(':').to_string();
                label_pos.insert(label, idx);
            }
        }

        let mut to_remove: HashSet<usize> = HashSet::new();
        let mut replacements: HashMap<usize, Vec<String>> = HashMap::new();

        let mut i = 0usize;
        while i + 5 < lines.len() {
            let loop_label_line = lines[i].trim();
            if !loop_label_line.ends_with(':') {
                i += 1;
                continue;
            }
            let loop_label = loop_label_line.trim_end_matches(':').to_string();

            let l1 = lines[i + 1].trim();
            let l2 = lines[i + 2].trim();
            let l3 = lines[i + 3].trim();
            let l4 = lines[i + 4].trim();
            let l5 = lines[i + 5].trim();
            let Some((imm_reg, imm_val)) = Self::parse_load_imm(l1) else {
                i += 1;
                continue;
            };
            let Some((counter_reg, limit_reg, arm_label)) = Self::parse_gte_jump(l2) else {
                i += 1;
                continue;
            };
            if limit_reg != imm_reg {
                i += 1;
                continue;
            }
            let Some((mov_tmp, mov_src)) = Self::parse_mov(l3) else {
                i += 1;
                continue;
            };
            let Some((add_dst, add_imm, is_add)) = Self::parse_add_sub_imm(l4) else {
                i += 1;
                continue;
            };
            if !is_add || mov_src != counter_reg || add_dst != mov_tmp || add_imm != "1" {
                i += 1;
                continue;
            }
            let Some(end_label) = Self::parse_jump_only(l5) else {
                i += 1;
                continue;
            };

            let mut arm_pos: Option<usize> = None;
            let mut end_pos: Option<usize> = None;
            let mut arm_extra = 0usize;
            let mut exit_label: Option<String> = None;

            if i + 14 < lines.len() {
                let l6 = lines[i + 6].trim();
                let l7 = lines[i + 7].trim();
                let l8 = lines[i + 8].trim();
                let l9 = lines[i + 9].trim();
                let l10 = lines[i + 10].trim();
                let l11 = lines[i + 11].trim();
                let l12 = lines[i + 12].trim();
                let l13 = lines[i + 13].trim();
                let l14 = lines[i + 14].trim();

                let mut local_ok = l6 == format!("{arm_label}:");
                if local_ok {
                    if let Some((dst, imm)) = Self::parse_load_imm(l7) {
                        local_ok = dst == "r1" && imm == "0";
                    } else {
                        local_ok = false;
                    }
                }
                let mut local_exit_label: Option<String> = None;
                if local_ok {
                    local_exit_label = Self::parse_jump_only(l8);
                    local_ok = local_exit_label.is_some();
                }
                if local_ok {
                    if let Some((dst, imm)) = Self::parse_load_imm(l9) {
                        local_ok = dst == mov_tmp && imm == "0";
                    } else {
                        local_ok = false;
                    }
                }
                if local_ok {
                    if let Some(jump) = Self::parse_jump_only(l10) {
                        local_ok = jump == end_label;
                    } else {
                        local_ok = false;
                    }
                }
                if local_ok {
                    local_ok = l11 == format!("{end_label}:");
                }
                if local_ok {
                    if let Some((dst, src)) = Self::parse_mov(l12) {
                        local_ok = dst == counter_reg && src == mov_tmp;
                    } else {
                        local_ok = false;
                    }
                }
                if local_ok {
                    if let Some((dst, imm)) = Self::parse_load_imm(l13) {
                        local_ok = dst == mov_tmp && imm == "0";
                    } else {
                        local_ok = false;
                    }
                }
                if local_ok {
                    if let Some(jump) = Self::parse_jump_only(l14) {
                        local_ok = jump == loop_label;
                    } else {
                        local_ok = false;
                    }
                }

                if local_ok {
                    arm_pos = Some(i + 6);
                    end_pos = Some(i + 11);
                    arm_extra = 2;
                    exit_label = local_exit_label;
                }
            }

            if arm_pos.is_none() {
                let Some(&pos) = label_pos.get(&arm_label) else {
                    i += 1;
                    continue;
                };
                arm_pos = Some(pos);
            }
            let arm_pos = arm_pos.unwrap();
            if arm_pos + 2 >= lines.len() {
                i += 1;
                continue;
            }
            let arm_load = lines[arm_pos + 1].trim();
            let arm_jump = lines[arm_pos + 2].trim();
            if exit_label.is_none() {
                exit_label = Self::parse_jump_only(arm_jump);
            }
            let Some(exit_label) = exit_label else {
                i += 1;
                continue;
            };
            let Some((arm_dst, arm_imm)) = Self::parse_load_imm(arm_load) else {
                i += 1;
                continue;
            };
            if arm_dst != "r1" || arm_imm != "0" {
                i += 1;
                continue;
            }

            if arm_extra == 0 && arm_pos + 4 < lines.len() {
                let arm_load2 = lines[arm_pos + 3].trim();
                let arm_jump2 = lines[arm_pos + 4].trim();
                if let Some((dst, imm)) = Self::parse_load_imm(arm_load2) {
                    if dst == mov_tmp && imm == "0" {
                        if let Some(jump2) = Self::parse_jump_only(arm_jump2) {
                            if jump2 == end_label {
                                arm_extra = 2;
                            }
                        }
                    }
                }
            }

            if end_pos.is_none() {
                let Some(&pos) = label_pos.get(&end_label) else {
                    i += 1;
                    continue;
                };
                end_pos = Some(pos);
            }
            let end_pos = end_pos.unwrap();
            if end_pos + 2 >= lines.len() {
                i += 1;
                continue;
            }
            let end_mov = lines[end_pos + 1].trim();
            let end_load = lines[end_pos + 2].trim();
            let end_jump = lines[end_pos + 3].trim();
            let Some(loop_jump_label) = Self::parse_jump_only(end_jump) else {
                i += 1;
                continue;
            };
            if loop_jump_label != loop_label {
                i += 1;
                continue;
            }
            let Some((end_dst, end_src)) = Self::parse_mov(end_mov) else {
                i += 1;
                continue;
            };
            if end_dst != counter_reg || end_src != mov_tmp {
                i += 1;
                continue;
            }
            let Some((end_zero_dst, end_zero_imm)) = Self::parse_load_imm(end_load) else {
                i += 1;
                continue;
            };
            if end_zero_dst != mov_tmp || end_zero_imm != "0" {
                i += 1;
                continue;
            }

            let indent1 = &lines[i + 1][..lines[i + 1].len() - l1.len()];
            let indent2 = &lines[i + 2][..lines[i + 2].len() - l2.len()];
            let indent3 = &lines[i + 3][..lines[i + 3].len() - l3.len()];

            replacements.insert(
                i + 1,
                vec![
                    format!("{indent1}LOAD_U64_IMMEDIATE {imm_reg} {imm_val}"),
                    format!("{indent2}GTE_U64_JUMP r255 {counter_reg} {imm_reg} {exit_label}"),
                    format!("{indent3}ADD_U64_IMMEDIATE {counter_reg} 1"),
                    format!("{indent3}JUMP r255 {loop_label}"),
                ],
            );

            for idx in i + 2..=i + 5 {
                to_remove.insert(idx);
            }
            for idx in arm_pos..=arm_pos + 2 + arm_extra {
                to_remove.insert(idx);
            }
            for idx in end_pos..=end_pos + 3 {
                to_remove.insert(idx);
            }

            i += 1;
        }

        if replacements.is_empty() && to_remove.is_empty() {
            return;
        }

        let mut out = Vec::with_capacity(lines.len());
        for (idx, line) in lines.iter().enumerate() {
            if let Some(rep) = replacements.get(&idx) {
                out.extend(rep.clone());
                continue;
            }
            if to_remove.contains(&idx) {
                continue;
            }
            out.push(line.clone());
        }
        *lines = out;
    }

    fn parse_load_imm(line: &str) -> Option<(String, String)> {
        let rest = line.strip_prefix("LOAD_U64_IMMEDIATE ")?;
        let mut parts = rest.split_whitespace();
        let dst = parts.next()?.to_string();
        let imm = parts.next()?.to_string();
        Some((dst, imm))
    }

    fn remove_overwritten_defs(lines: &mut Vec<String>) {
        let mut out = Vec::with_capacity(lines.len());
        let mut i = 0usize;
        while i < lines.len() {
            let line = &lines[i];
            let trimmed = line.trim();
            if i + 1 < lines.len() {
                let next = &lines[i + 1];
                let next_trim = next.trim();
                if let Some((def, is_def)) = Self::parse_def_reg(trimmed) {
                    if is_def {
                        if let Some((next_def, next_is_def)) = Self::parse_def_reg(next_trim) {
                            if next_is_def && def == next_def {
                                i += 1;
                                continue;
                            }
                        }
                    }
                }
            }
            out.push(line.clone());
            i += 1;
        }
        *lines = out;
    }

    fn remove_zero_immediates(lines: &mut Vec<String>) {
        lines.retain(|line| {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("ADD_U64_IMMEDIATE ") {
                let mut parts = rest.split_whitespace();
                let _dst = parts.next();
                if let Some(imm) = parts.next() {
                    return imm != "0";
                }
            }
            if let Some(rest) = trimmed.strip_prefix("SUB_U64_IMMEDIATE ") {
                let mut parts = rest.split_whitespace();
                let _dst = parts.next();
                if let Some(imm) = parts.next() {
                    return imm != "0";
                }
            }
            true
        });
    }

    fn parse_def_reg(line: &str) -> Option<(String, bool)> {
        if let Some(rest) = line.strip_prefix("MOV ") {
            let mut parts = rest.split_whitespace();
            if let (Some(dst), Some(_src)) = (parts.next(), parts.next()) {
                return Some((dst.to_string(), true));
            }
        }
        if let Some(rest) = line.strip_prefix("LOAD_U64_IMMEDIATE ") {
            let mut parts = rest.split_whitespace();
            if let (Some(dst), Some(_imm)) = (parts.next(), parts.next()) {
                return Some((dst.to_string(), true));
            }
        }
        None
    }

    fn inline_single_use_jump_block(lines: &mut Vec<String>) {
        let mut label_pos: HashMap<String, usize> = HashMap::new();
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.ends_with(':') && !trimmed.contains(' ') {
                let label = trimmed.trim_end_matches(':').to_string();
                label_pos.insert(label, idx);
            }
        }

        let mut ref_counts: HashMap<String, usize> = HashMap::new();
        let mut ref_lines: HashMap<String, Vec<usize>> = HashMap::new();
        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let mut parts = trimmed.split_whitespace();
            let op = parts.next();
            if let Some(op) = op {
                if op.ends_with("_JUMP") || op == "JUMP" {
                    let label = parts.last();
                    if let Some(label) = label {
                        *ref_counts.entry(label.to_string()).or_insert(0) += 1;
                        ref_lines.entry(label.to_string()).or_default().push(idx);
                    }
                }
            }
        }

        let mut to_remove: HashSet<usize> = HashSet::new();
        let mut replacements: HashMap<usize, Vec<String>> = HashMap::new();

        for (label, &pos) in label_pos.iter() {
            let count = *ref_counts.get(label).unwrap_or(&0);
            if count != 1 {
                continue;
            }

            let ref_idx = *ref_lines.get(label).and_then(|v| v.first()).unwrap_or(&usize::MAX);
            if ref_idx == usize::MAX {
                continue;
            }

            let ref_line = lines[ref_idx].trim();
            if !ref_line.starts_with("JUMP ") {
                continue;
            }

            let mut block = Vec::new();
            let mut i = pos + 1;
            while i < lines.len() {
                let trimmed = lines[i].trim();
                if trimmed.ends_with(':') && !trimmed.contains(' ') {
                    break;
                }
                block.push(lines[i].clone());
                i += 1;
            }

            if block.is_empty() || block.len() > 3 {
                continue;
            }

            let mut ok = true;
            for (idx, line) in block.iter().enumerate() {
                let t = line.trim();
                if idx == block.len() - 1 {
                    if !t.starts_with("JUMP ") {
                        ok = false;
                    }
                } else if !(t.starts_with("MOV ") || t.starts_with("LOAD_U64_IMMEDIATE ")) {
                    ok = false;
                }
            }
            if !ok {
                continue;
            }

            replacements.insert(ref_idx, block.clone());
            to_remove.insert(pos);
            for j in pos + 1..i {
                to_remove.insert(j);
            }
        }

        if replacements.is_empty() && to_remove.is_empty() {
            return;
        }

        let mut out = Vec::with_capacity(lines.len());
        for (idx, line) in lines.iter().enumerate() {
            if let Some(rep) = replacements.get(&idx) {
                out.extend(rep.clone());
                continue;
            }
            if to_remove.contains(&idx) {
                continue;
            }
            out.push(line.clone());
        }
        *lines = out;
    }

    fn fold_load_mov(lines: &mut Vec<String>) {
        let mut out = Vec::with_capacity(lines.len());
        let mut i = 0usize;
        while i < lines.len() {
            let line = &lines[i];
            let trimmed = line.trim();
            if i + 1 < lines.len() {
                let next = &lines[i + 1];
                let next_trim = next.trim();
                if let Some(rest) = trimmed.strip_prefix("LOAD_U64_IMMEDIATE ") {
                    let mut parts = rest.split_whitespace();
                    if let (Some(load_dst), Some(imm)) = (parts.next(), parts.next()) {
                        if let Some(mov_rest) = next_trim.strip_prefix("MOV ") {
                            let mut mov_parts = mov_rest.split_whitespace();
                            if let (Some(mov_dst), Some(mov_src)) = (mov_parts.next(), mov_parts.next()) {
                                if mov_src == load_dst {
                                    let indent = &line[..line.len() - trimmed.len()];
                                    out.push(format!("{indent}LOAD_U64_IMMEDIATE {mov_dst} {imm}"));
                                    i += 2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }

            out.push(line.clone());
            i += 1;
        }
        *lines = out;
    }

    fn collapse_mov_chain(lines: &mut Vec<String>) {
        let mut out = Vec::with_capacity(lines.len());
        let mut i = 0usize;
        while i < lines.len() {
            let line = &lines[i];
            let trimmed = line.trim();
            if i + 1 < lines.len() {
                let next = &lines[i + 1];
                let next_trim = next.trim();
                if let Some(rest) = trimmed.strip_prefix("MOV ") {
                    let mut parts = rest.split_whitespace();
                    if let (Some(dst), Some(src)) = (parts.next(), parts.next()) {
                        if let Some(next_rest) = next_trim.strip_prefix("MOV ") {
                            let mut next_parts = next_rest.split_whitespace();
                            if let (Some(next_dst), Some(next_src)) = (next_parts.next(), next_parts.next()) {
                                if next_src == dst {
                                    out.push(line.clone());
                                    let indent = &next[..next.len() - next_trim.len()];
                                    out.push(format!("{indent}MOV {next_dst} {src}"));
                                    i += 2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
            out.push(line.clone());
            i += 1;
        }
        *lines = out;
    }
}