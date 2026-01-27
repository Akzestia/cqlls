/*
MIT License

Copyright (c) 2025-2026 アクゼスティア

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/
use log::info;
use tower_lsp::lsp_types::*;

use crate::{consts::*, lsp::Backend};

#[derive(Debug, Clone)]
pub struct InsertStatement {
    pub table_name: String,
    pub fields: Vec<String>,
    pub values: Vec<Vec<String>>,
}

impl InsertStatement {
    pub fn new() -> Self {
        Self {
            table_name: "".to_string(),
            fields: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl Backend {
    pub fn remove_leading_spaces_wildcards(&self, line: &mut String) {
        let mut result = String::with_capacity(line.len());
        let chars: Vec<char> = line.chars().collect();
        let mut iter = chars.iter().peekable();

        while let Some(&current) = iter.next() {
            if current == ' ' {
                if let Some(&&next) = iter.peek() {
                    if matches!(next, ' ' | ';' | ',' | ')' | '>') {
                        continue;
                    }
                }
            }
            result.push(current);
        }

        *line = result;
    }

    pub fn remove_tailing_spaces_wildcards(&self, line: &mut String) {
        let mut chars: Vec<char> = line.chars().collect();
        let mut index = 0;
        let mut met_wild_card = false;

        while index < chars.len() {
            let current_char = chars[index];

            if !met_wild_card && (current_char == '(' || current_char == '<') {
                met_wild_card = true;
            }

            if met_wild_card && current_char != '(' && current_char != '<' {
                met_wild_card = false;
            }

            if met_wild_card && index + 1 < chars.len() && chars[index + 1] == ' ' {
                chars.remove(index + 1);
                met_wild_card = false;
                if index >= 2 {
                    index -= 2;
                } else if index > 0 {
                    index -= 1;
                }
            }

            index += 1;
        }

        *line = chars.into_iter().collect();
    }

    pub fn is_insert_statement_line(&self, line: &str) -> bool {
        if line.trim().to_lowercase().starts_with("insert into") {
            return true;
        } else {
            return false;
        };
    }

    pub async fn format_insert(&self, lines: &mut Vec<String>, _document_url: &Url) {
        let mut indices: Vec<usize> = Vec::new();
        let mut ending_indices: Vec<usize> = Vec::new();
        let mut met_insert_start = false;

        for (pos, line) in lines.iter().enumerate() {
            let is_insert_line = line.to_lowercase().trim().starts_with("insert into");
            let mut is_end_st = false;

            if pos + 1 != lines.len() {
                is_end_st = (line.trim().ends_with(";") || lines[pos + 1].trim().is_empty());
            } else {
                is_end_st = true
            }

            if is_insert_line && met_insert_start {
                is_end_st = true;
            }

            if is_insert_line && !met_insert_start {
                indices.push(pos);
                met_insert_start = true;
            }
            if met_insert_start && is_end_st {
                ending_indices.push(pos);
                met_insert_start = false;
            }
        }

        let mut total_new_lines: i32 = 0;
        let mut statements_info: Vec<(usize, usize, Vec<String>)> = Vec::new();

        for (start, end) in indices.iter().zip(ending_indices.iter()) {
            if *start <= *end && *end < lines.len() {
                let statement: String = lines[*start..=*end].join(" ");
                let normalized: String = statement
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" ");

                let formatted = self.format_insert_statement(&normalized);
                let formatted_lines: Vec<String> =
                    formatted.lines().map(|s| s.to_string()).collect();

                let original_count = (*end - *start + 1) as i32;
                let new_count = formatted_lines.len() as i32;
                total_new_lines += new_count - original_count;

                statements_info.push((*start, *end, formatted_lines));
            }
        }

        if total_new_lines > 0 {
            let new_size = lines.len() + total_new_lines as usize;
            lines.resize(new_size, String::new());
        }

        for (start, end, formatted_lines) in statements_info.into_iter().rev() {
            let original_line_count = end - start + 1;
            let new_line_count = formatted_lines.len();

            if new_line_count > original_line_count {
                let shift = new_line_count - original_line_count;
                for i in (end + 1..lines.len() - shift).rev() {
                    lines[i + shift] = lines[i].clone();
                }
            } else if new_line_count < original_line_count {
                let shift = original_line_count - new_line_count;
                for i in (end + 1)..lines.len() {
                    if i + shift < lines.len() {
                        lines[i] = lines[i + shift].clone();
                    }
                }
                lines.truncate(lines.len() - shift);
            }

            for (i, formatted_line) in formatted_lines.into_iter().enumerate() {
                lines[start + i] = formatted_line;
            }
        }
    }

    pub fn format_insert_statement(&self, input: &str) -> String {
        let mut result = String::new();
        let mut indent_level = 0;
        let mut chars = input.chars().peekable();
        let mut in_string = false;
        let mut string_char = ' ';
        let mut paren_depth = 0;
        let mut in_function = false;
        let mut after_into_table = false;

        while let Some(c) = chars.next() {
            if (c == '\'' || c == '"') && !in_string {
                in_string = true;
                string_char = c;
                result.push(c);
                continue;
            }
            if in_string && c == string_char {
                in_string = false;
                result.push(c);
                continue;
            }
            if in_string {
                result.push(c);
                continue;
            }
            match c {
                '(' => {
                    let is_function_call = self.is_function_call(&result);
                    if is_function_call && !after_into_table {
                        if !in_function {
                            in_function = true;
                            paren_depth = 1;
                        } else {
                            paren_depth += 1;
                        }
                        result.push(c);
                    } else {
                        after_into_table = false;
                        result.push(c);
                        indent_level += 1;
                        result.push('\n');
                        result.push_str(&"    ".repeat(indent_level));
                    }
                }
                ')' => {
                    if in_function {
                        paren_depth -= 1;
                        result.push(c);
                        if paren_depth == 0 {
                            in_function = false;
                        }
                    } else {
                        indent_level = indent_level.saturating_sub(1);
                        result.push('\n');
                        result.push_str(&"    ".repeat(indent_level));
                        result.push(c);
                    }
                }
                '{' => {
                    result.push(c);
                    indent_level += 1;
                    result.push('\n');
                    result.push_str(&"    ".repeat(indent_level));
                }
                '}' => {
                    indent_level = indent_level.saturating_sub(1);
                    result.push('\n');
                    result.push_str(&"    ".repeat(indent_level));
                    result.push(c);
                }
                ',' => {
                    result.push(c);
                    if !in_function {
                        result.push('\n');
                        result.push_str(&"    ".repeat(indent_level));
                    }
                    while chars.peek() == Some(&' ') {
                        chars.next();
                    }
                }
                ' ' => {
                    if !result.ends_with(' ')
                        && !result.ends_with('\n')
                        && !result.ends_with("    ")
                    {
                        result.push(c);
                    }
                    if self.is_after_into_tablename(&result) {
                        after_into_table = true;
                    }
                }
                _ => {
                    result.push(c);
                }
            }
        }

        result = result.replace("VALUES", "\nVALUES");

        result
            .lines()
            .map(|line| line.trim_end())
            .collect::<Vec<&str>>()
            .join("\n")
    }

    fn is_function_call(&self, s: &str) -> bool {
        let known_functions = [
            "UUID",
            "NOW",
            "TOTIMESTAMP",
            "TOUNIXTIMESTAMP",
            "TODATE",
            "TOKEN",
            "TTL",
            "WRITETIME",
            "CAST",
            "COUNT",
            "MIN",
            "MAX",
            "SUM",
            "AVG",
            "BLOB",
            "ASCII",
            "BIGINT",
            "BOOLEAN",
            "COUNTER",
            "DATE",
            "DECIMAL",
            "DOUBLE",
            "FLOAT",
            "INET",
            "INT",
            "SMALLINT",
            "TEXT",
            "TIME",
            "TIMESTAMP",
            "TIMEUUID",
            "TINYINT",
            "VARCHAR",
            "VARINT",
            "MINTIMEUUID",
            "MAXTIMEUUID",
            "BLOBASINT",
            "BLOBASTEXT",
            "INTASBLOB",
            "TEXTASBLOB",
        ];

        let last_word: String = s
            .chars()
            .rev()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>()
            .chars()
            .rev()
            .collect();

        if last_word.is_empty() {
            return false;
        }

        let upper = last_word.to_uppercase();

        known_functions.contains(&upper.as_str())
    }

    fn is_after_into_tablename(&self, s: &str) -> bool {
        let trimmed = s.trim_end();
        let words: Vec<&str> = trimmed.split_whitespace().collect();

        if words.len() >= 3 {
            let len = words.len();
            if words[len - 3].to_uppercase() == "INSERT" && words[len - 2].to_uppercase() == "INTO"
            {
                return true;
            }
        }

        false
    }

    pub async fn add_tabs_to_cql_types(&self, lines: &mut Vec<String>, document_url: &Url) {
        let mut index = 0;
        let mut in_function_or_aggregate = false;

        for line in lines.iter_mut() {
            if line.trim().is_empty() {
                index += 1;
                continue;
            }

            let trimmed_lower = line.trim().to_lowercase();

            if trimmed_lower.starts_with("create function")
                || trimmed_lower.starts_with("create or replace function")
                || trimmed_lower.starts_with("create aggregate")
                || trimmed_lower.starts_with("create or replace aggregate")
            {
                in_function_or_aggregate = true;
            }

            if in_function_or_aggregate {
                if line.contains(";") {
                    in_function_or_aggregate = false;
                }
                index += 1;
                continue;
            }

            let mut found_type = None;
            for word in line.split_whitespace() {
                if (CQL_TYPES_LWC
                    .contains(&word.to_lowercase().replace(",", "").trim().to_string())
                    || word.starts_with("set")
                    || word.starts_with("map")
                    || word.starts_with("list")
                    || word.starts_with("frozen"))
                    && !(line.trim().starts_with("--")
                        || line.trim().starts_with("//")
                        || self
                            .is_inside_multiline_comment_no_position(index, &document_url)
                            .await)
                {
                    found_type = Some(word);
                    break;
                }
            }

            if let Some(typ) = found_type {
                if let Some(offset) = line.find(typ) {
                    if offset > 0 {
                        if !line[..offset]
                            .ends_with(&" ".repeat(self.formatting_config.type_alignment_offset))
                        {
                            line.insert_str(
                                offset,
                                &" ".repeat(self.formatting_config.type_alignment_offset),
                            );
                        }
                    }
                }
            }

            index += 1;
        }
    }

    pub async fn add_tabs_to_function_keywords(&self, lines: &mut Vec<String>, document_url: &Url) {
        let function_keywords = ["called", "returns", "language"];

        let aggregate_keywords = ["sfunc", "stype", "finalfunc"];

        let indent = " ".repeat(4);

        let mut inside_function = false;
        let mut inside_aggregate = false;
        let mut index = 0;

        for line in lines.iter_mut() {
            if line.trim().is_empty() {
                index += 1;
                continue;
            }

            if line.trim().starts_with("--")
                || line.trim().starts_with("//")
                || self
                    .is_inside_multiline_comment_no_position(index, &document_url)
                    .await
            {
                index += 1;
                continue;
            }

            let line_lower = line.to_lowercase();

            if line_lower.contains("create function")
                || line_lower.contains("create or replace function")
            {
                inside_function = true;
                inside_aggregate = false;
            }

            if line_lower.contains("create aggregate")
                || line_lower.contains("create or replace aggregate")
            {
                inside_aggregate = true;
                inside_function = false;
            }

            if (inside_function || inside_aggregate) && line.trim().ends_with(";") {
                let trimmed = line.trim_start();
                let first_word = trimmed
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_lowercase();

                let keywords = if inside_function {
                    &function_keywords[..]
                } else {
                    &aggregate_keywords[..]
                };

                if keywords.contains(&first_word.as_str()) {
                    *line = format!("{}{}", indent, trimmed);
                }

                inside_function = false;
                inside_aggregate = false;
                index += 1;
                continue;
            }

            if inside_function {
                let trimmed = line.trim_start();
                let first_word = trimmed
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_lowercase();

                if function_keywords.contains(&first_word.as_str()) {
                    *line = format!("{}{}", indent, trimmed);
                }
            }

            if inside_aggregate {
                let trimmed = line.trim_start();
                let first_word = trimmed
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .to_lowercase();

                if aggregate_keywords.contains(&first_word.as_str()) {
                    *line = format!("{}{}", indent, trimmed);
                }
            }

            index += 1;
        }
    }

    pub async fn add_tabs_to_args(&self, lines: &mut Vec<String>, document_url: &Url) {
        let mut indices: Vec<usize> = Vec::new();

        for (idx, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            let trimmed_lower = trimmed.to_lowercase();

            let is_inside_multiline_comment =
                self.is_line_in_multiline_comment(trimmed, idx, lines);
            let is_arg = self.is_line_inside_init_args(trimmed, idx, lines);
            let is_selector = self.is_line_inside_selectors(trimmed, idx, lines);
            let is_ml_comment_clause = self.is_multi_line_comment_clause(trimmed);
            let is_pk_line = trimmed_lower.starts_with("primary");

            let is_in_create_table = self
                .is_inside_create_table_no_position(idx, document_url)
                .await;
            let is_in_create_type = self
                .is_inside_create_type_no_position(idx, document_url)
                .await;
            let is_inside_curly_bracers =
                self.is_inside_curly_braces_block(idx, document_url).await;

            let banned_starts = [
                "create table",
                "create type",
                "alter table",
                "alter",
                "select",
                "as",
                "on",
                "where",
                "with",
                "--",
                "//",
                "/*",
                "*/",
            ];

            let starts_with_banned = banned_starts.iter().any(|p| trimmed_lower.starts_with(p));

            if is_inside_multiline_comment && !is_ml_comment_clause && !starts_with_banned {
                indices.push(idx);
                continue;
            }

            if is_inside_curly_bracers && !starts_with_banned {
                indices.push(idx);
                continue;
            }

            let is_arg_context = (is_arg || is_selector || (is_pk_line && is_in_create_table))
                && !is_inside_multiline_comment
                && !is_ml_comment_clause;

            if is_arg_context && (is_in_create_table || is_in_create_type) && !starts_with_banned {
                indices.push(idx);
            }
        }

        for i in indices {
            lines[i].insert_str(0, "    ");
        }
    }

    pub fn fix_string_literals(&self, lines: &mut Vec<String>) {
        for line in lines.iter_mut() {
            let mut position = 0;
            while position < line.len() {
                if let Some(start) = line[position..].find('"').map(|p| p + position) {
                    if let Some(end) = line[start + 1..].find('"').map(|p| p + start + 1) {
                        let str = String::from(&line[start + 1..end]);
                        let trimmed = str.trim();
                        line.replace_range(start + 1..end, trimmed);
                        position = end + 1;
                    } else {
                        position = start + 1;
                    }
                } else if let Some(start) = line[position..].find('\'').map(|p| p + position) {
                    if let Some(end) = line[start + 1..].find('\'').map(|p| p + start + 1) {
                        let str = String::from(&line[start + 1..end]);
                        let trimmed = str.trim();
                        line.replace_range(start + 1..end, trimmed);
                        position = end + 1;
                    } else {
                        position = start + 1;
                    }
                } else {
                    break;
                }
            }
        }
    }

    /*
    Removes spaces before ;
    */
    pub fn fix_semi_colon(&self, lines: &mut Vec<String>) {
        let mut index = 0;

        while index < lines.len() {
            self.remove_leading_spaces_wildcards(&mut lines[index]);
            self.remove_tailing_spaces_wildcards(&mut lines[index]);
            index += 1;
        }
    }

    /*
    Removes duplicates of ;
    */
    pub fn fix_duplicate_semi_colon(&self, line: &mut String) {
        let mut chars: Vec<char> = line.chars().collect();
        let mut last_colon = false;
        let mut index = 0;

        while index < chars.len() {
            let current_char = chars[index];

            if !last_colon && current_char == ';' {
                last_colon = true;
            } else if last_colon && current_char == ';' {
                chars.remove(index);
                last_colon = false;
                if index >= 2 {
                    index -= 2;
                } else if index > 0 {
                    index -= 1;
                }
                continue;
            } else if current_char != ';' {
                last_colon = false;
            }
            index += 1;
        }

        *line = chars.into_iter().collect();
    }

    pub fn fix_spacing(&self, line: &mut String) {
        let mut chars: Vec<char> = line.chars().collect();
        let mut last_space = false;
        let mut index = 0;

        while index < chars.len() {
            let current_char = chars[index];

            if !last_space && current_char == ' ' {
                last_space = true;
            } else if last_space && current_char == ' ' {
                chars.remove(index);
                last_space = false;
                if index >= 2 {
                    index -= 2;
                } else if index > 0 {
                    index -= 1;
                }
                continue;
            } else if current_char != ' ' {
                last_space = false;
            }
            index += 1;
        }

        *line = chars.into_iter().collect();
    }

    pub fn fix_new_lines(&self, lines: &mut Vec<String>) {
        let mut index = 0;
        let mut last_new_line = false;
        let mut last_bracket = false;

        while index < lines.len() {
            if last_new_line && lines[index].len() == 0 {
                lines.remove(index);
                if index >= 2 {
                    index -= 2;
                } else if index > 0 {
                    index -= 1;
                }
            }

            if last_bracket && lines[index].len() == 0 {
                lines.remove(index);
                if index >= 2 {
                    index -= 2;
                } else if index > 0 {
                    index -= 1;
                }
            }

            if lines[index].len() == 0
                && !self.is_line_in_multiline_comment(&lines[index], index, lines)
            {
                last_new_line = true;
            } else {
                last_new_line = false;
            }

            if lines[index].contains("(") {
                last_bracket = true;
            } else {
                last_bracket = false
            }

            index += 1;
        }
    }

    /*
        Removes all '\n' inside code_blocks
    */
    pub fn remove_new_lines_from_code_block(&self, lines: &mut Vec<String>) {
        let mut index = 0;
        let mut inside_code_block = false;

        while index < lines.len() {
            let line = lines[index].to_lowercase();

            if !inside_code_block && line.len() > 0 && !line.contains(";") {
                inside_code_block = true;
            }

            if inside_code_block && line.contains(";") {
                inside_code_block = false;
            }

            if inside_code_block
                && line.len() == 0
                && !self.is_line_in_multiline_comment(&line, index, lines)
            {
                lines.remove(index);
                if index >= 2 {
                    index -= 2;
                } else if index > 0 {
                    index -= 1;
                }
            }

            index += 1;
        }
    }

    /*
        Adds missing semi colon to the and of CQL command

        The list of Keywords that start CQL commands is strored inside
        CQL_KEYWORDS_LWC | LWC - lower_case
    */
    pub fn apply_semi_colon(&self, lines: &mut Vec<String>) {
        let mut index = 0;

        while index < lines.len() {
            let line = lines[index].to_lowercase();

            if index + 1 != lines.len()
                && line.len() > 0
                && !line.contains(";")
                && !line.contains("begin")
                && !line.contains("//")
                && !line.contains("--")
                && !line.contains("/*")
                && !line.contains("*/")
                && !line.ends_with("as")
                && !line.ends_with("with")
                && !self.is_line_in_multiline_comment(&line, index, lines)
            {
                let lw = lines[index + 1].to_lowercase();
                let split: Vec<&str> = lw.split(' ').collect();
                if lines[index + 1].to_lowercase().len() == 0
                    || CQL_KEYWORDS_LWC.contains(&split[0].to_string())
                {
                    lines[index].push(';');
                }
            }

            if index == lines.len() - 1
                && line.len() > 0
                && !line.contains(";")
                && !line.contains("begin")
                && !line.contains("//")
                && !line.contains("--")
                && !line.contains("/*")
                && !line.contains("*/")
                && !line.ends_with("as")
                && !line.ends_with("with")
                && !self.is_line_in_multiline_comment(&line, index, lines)
            {
                lines[index].push(';');
            }

            index += 1;
        }
    }

    pub fn add_spacing_new_lines(&self, lines: &mut Vec<String>) {
        let mut index = 0;

        while index < lines.len() {
            if index + 1 != lines.len()
                && (lines[index].contains(";") || lines[index].to_lowercase().contains("begin"))
                && lines[index + 1].len() > 0
            {
                lines.insert(index + 1, "".to_string());
            }

            index += 1;
        }
    }

    pub fn add_new_line_before_pk(&self, lines: &mut Vec<String>) {
        let mut index = lines.len() - 1;

        let mut indicies: Vec<usize> = Vec::new();

        while index > 0 {
            if index - 1 != 0
                && (lines[index - 1]
                    .trim_start()
                    .to_lowercase()
                    .starts_with("primary"))
            {
                indicies.push(index - 1);
            }

            index -= 1;
        }

        for index in indicies.iter() {
            lines.insert(*index, "".to_string());
        }
    }

    pub fn add_spacing_after_comma(&self, lines: &mut Vec<String>) {
        for line in lines.iter_mut() {
            let mut chars: Vec<char> = line.chars().collect();
            let mut idx = 0;

            while idx < chars.len() {
                if idx + 1 < chars.len() && chars[idx] == ',' && chars[idx + 1] != ' ' {
                    chars.insert(idx + 1, ' ');
                }
                idx += 1;
            }

            *line = chars.into_iter().collect();
        }
    }

    /*
        Hate this shit だよ xD
        Formats select statements in the following manner

        SELECT
        selector1,
        selector2,
        selector3,
        ...,
        selectorN,
        FROM [keyspace_name].table_name;
    */
    pub fn format_selectors(&self, lines: &mut Vec<String>) {
        let mut index = 0;
        let mut working_buf = Vec::<String>::new();

        while index < lines.len() {
            let lw = lines[index].to_lowercase();
            if lw.contains("select") && self.should_edit_select_statement(&lines[index], lines) {
                working_buf.clear();

                let mut idx = index;

                while idx < lines.len() {
                    if !lines[idx].to_lowercase().contains("from") && lines[idx].contains(";") {
                        return;
                    }

                    if lines[idx].to_lowercase().contains("from") {
                        let split: Vec<&str> = lines[idx].split(' ').collect();
                        for sp in split.into_iter() {
                            if sp.to_lowercase() == "from" {
                                break;
                            }
                            if sp.to_lowercase() != "select" {
                                let mut retained = sp.to_string();
                                retained.retain(|c| c != '\n' && c != '\r');
                                retained.push('\n');
                                working_buf.push(retained);
                            }
                        }

                        let from_pos = lines[idx].to_lowercase().rfind("from").unwrap();
                        working_buf.push(lines[idx][from_pos..].to_string());
                        break;
                    }

                    let split: Vec<&str> = lines[idx].split(' ').collect();

                    for sp in split.into_iter() {
                        if sp.to_lowercase() != "select" {
                            working_buf.push(sp.to_string());
                        }
                    }

                    idx += 1;
                }

                let mut start_idx = index + 1;
                for kw in working_buf.iter_mut() {
                    kw.retain(|c| c != '\n' && c != '\r');
                    kw.push('\n');

                    if start_idx < lines.len() {
                        lines.insert(start_idx, kw.clone());
                    } else {
                        lines.push(kw.clone());
                    }
                    start_idx += 1;
                }

                if lines[index].chars().next().unwrap_or('s') == 'S' {
                    lines[index] = "SELECT".to_string();
                } else {
                    lines[index] = "select".to_string();
                }

                index += working_buf.len();
            } else {
                index += 1;
            }
        }
    }

    pub async fn align_types_inside_create_statement(
        &self,
        lines: &mut Vec<String>,
        document_url: &Url,
    ) {
        let mut working_blocks: Vec<Vec<usize>> = Vec::new();
        let mut current_block: Vec<usize> = Vec::new();
        let mut in_table = false;
        let mut in_function_or_aggregate = false;
        let mut parenthesis_depth = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed_lower = line.trim().to_lowercase();

            if trimmed_lower.starts_with("create function")
                || trimmed_lower.starts_with("create or replace function")
                || trimmed_lower.starts_with("create aggregate")
                || trimmed_lower.starts_with("create or replace aggregate")
            {
                in_function_or_aggregate = true;
            }

            if in_function_or_aggregate {
                if line.contains(";") {
                    in_function_or_aggregate = false;
                }
                continue;
            }

            let is_create_st_start = trimmed_lower.starts_with("create table")
                || trimmed_lower.starts_with("create type");
            let contains_cql_type = self.line_contains_cql_type(line);
            let has_open_brace = line.contains('(');
            let has_close_brace = line.contains(')');

            if has_open_brace {
                parenthesis_depth += line.matches('(').count();
            }
            if has_close_brace {
                parenthesis_depth = parenthesis_depth.saturating_sub(line.matches(')').count());
            }

            if is_create_st_start {
                if in_table && !current_block.is_empty() {
                    working_blocks.push(current_block.clone());
                    current_block.clear();
                }
                in_table = true;
                parenthesis_depth = 0;
            }

            if in_table && contains_cql_type {
                current_block.push(i);
            }

            if in_table && parenthesis_depth == 0 && has_close_brace {
                if !current_block.is_empty() {
                    working_blocks.push(current_block.clone());
                    current_block.clear();
                }
                in_table = false;
                parenthesis_depth = 0;
            }
        }

        if !current_block.is_empty() {
            working_blocks.push(current_block);
        }

        info!("Size of working blocks: {}", working_blocks.len());

        for vec in working_blocks.iter() {
            let mut max_offset_x = 0;
            let mut offsets: Vec<(usize, usize)> = Vec::new();

            for index in vec {
                let line = &lines[*index].to_lowercase();
                let split: Vec<&str> = line.split_whitespace().collect();
                let mut line_type = String::from("");

                for w in split {
                    if CQL_TYPES_LWC.contains(&w.to_lowercase().replace(",", "").trim().to_string())
                        || w.starts_with("set")
                        || w.starts_with("map")
                        || w.starts_with("list")
                        || w.starts_with("frozen")
                    {
                        line_type = w.to_string();
                        break;
                    }
                }

                if let Some(offset_x) = line.find(&line_type) {
                    info!("\n\nOFFSET: {}, LINE_TYPE: {}\n\n", offset_x, line_type);
                    offsets.push((*index, offset_x));
                    if max_offset_x < offset_x {
                        max_offset_x = offset_x;
                    }
                }
            }

            info!("Size of offsets: {}", offsets.len());

            for offset in offsets {
                if offset.1 < max_offset_x && !lines[offset.0].trim().starts_with("--") {
                    let diff = max_offset_x - offset.1;
                    info!("\n\nediting line: {}, {}\n\n", lines[offset.0], offset.1);
                    lines[offset.0].insert_str(offset.1 - 1, &" ".repeat(diff));
                }
            }
        }
    }

    pub fn format_table_fields(&self, lines: &mut Vec<String>) {}

    pub async fn format_file(&self, lines: &Vec<&str>, document_url: &Url) -> Vec<TextEdit> {
        let mut working_vec: Vec<String> = lines.into_iter().map(|s| s.to_string()).collect();

        for index in 0..working_vec.len() {
            working_vec[index] = working_vec[index].trim().to_string();
            self.fix_spacing(&mut working_vec[index]);
            self.fix_duplicate_semi_colon(&mut working_vec[index]);
        }

        self.fix_semi_colon(&mut working_vec);
        self.fix_string_literals(&mut working_vec);
        self.fix_new_lines(&mut working_vec);
        self.remove_new_lines_from_code_block(&mut working_vec);
        self.apply_semi_colon(&mut working_vec);
        self.add_spacing_new_lines(&mut working_vec);
        self.add_spacing_after_comma(&mut working_vec);
        self.add_tabs_to_args(&mut working_vec, document_url).await;
        self.add_new_line_before_pk(&mut working_vec);
        self.add_tabs_to_cql_types(&mut working_vec, document_url)
            .await;
        self.align_types_inside_create_statement(&mut working_vec, document_url)
            .await;
        self.format_insert(&mut working_vec, document_url).await;
        self.add_tabs_to_function_keywords(&mut working_vec, document_url)
            .await;

        info!("Working Vec Size: {}", working_vec.len());
        info!("Original lines: {}", lines.len());

        let formatted_content = working_vec.join("\n");

        let last_line_idx = if lines.is_empty() { 0 } else { lines.len() - 1 };
        let last_line_len = if lines.is_empty() {
            0
        } else {
            lines[last_line_idx].len()
        };

        let full_edit = TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: last_line_idx as u32,
                    character: last_line_len as u32,
                },
            },
            new_text: formatted_content,
        };

        vec![full_edit]
    }
}
