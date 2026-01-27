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
use crate::consts::*;
use crate::lsp::Backend;
use log::info;

impl Backend {
    pub fn is_in_string_literal(line: &str, position: u32) -> bool {
        let byte_pos = line
            .char_indices()
            .nth(position as usize)
            .map(|(i, _)| i)
            .unwrap_or(line.len());
        let prefix = match line.get(..byte_pos) {
            Some(p) => p,
            None => return false,
        };

        let mut in_double_quotes = false;
        let mut in_single_quotes = false;
        let mut escape_next = false;

        for ch in prefix.chars() {
            if escape_next {
                escape_next = false;
                continue;
            }

            match ch {
                '\\' => escape_next = true,
                '"' if !in_single_quotes => in_double_quotes = !in_double_quotes,
                '\'' if !in_double_quotes => in_single_quotes = !in_single_quotes,
                _ => {}
            }
        }

        in_double_quotes || in_single_quotes
    }

    pub fn line_contains_cql_type(&self, line: &str) -> bool {
        let split: Vec<&str> = line.split_whitespace().collect();

        info!("{:?} Split", split);
        let mut is_type = false;

        for w in split {
            let wlw = w.to_lowercase();
            if CQL_TYPES_LWC.contains(&wlw.replace(",", "").trim().to_string())
                || wlw.starts_with("set")
                || wlw.starts_with("map")
                || wlw.starts_with("list")
                || wlw.starts_with("frozen")
                || wlw.starts_with("\'")
            {
                info!("{} ^^", w.to_lowercase().to_string());
                is_type = true;
                break;
            }
        }

        return is_type;
    }

    pub fn line_contains_cql_kw(&self, line: &str) -> bool {
        let lw = line.to_lowercase();
        let split: Vec<&str> = lw.split(' ').collect();

        for kw in split {
            if CQL_KEYWORDS_LWC.contains(&kw.to_string()) {
                return false;
            }
        }

        false
    }

    pub fn is_line_inside_selectors(&self, line: &str, index: usize, lines: &Vec<String>) -> bool {
        if self.line_contains_cql_kw(line) || line.contains(";") || line.len() == 0 {
            return false;
        }

        if index == 0 || index == lines.len() - 1 {
            return false;
        }

        let lw = line.to_lowercase();

        if lw.contains("values") || lw.contains("from") {
            return false;
        }

        let mut index_up = index - 1;
        let mut index_down = index + 1;

        let mut top_bracket = false;
        let mut bottom_bracket = false;

        while index_up > 0 {
            let up_line = &lines[index_up].to_lowercase();
            if !top_bracket && up_line.contains("select") {
                top_bracket = true;
            }
            if !top_bracket {
                index_up -= 1;
            } else {
                break;
            }
        }

        let up_line = &lines[index_up].to_lowercase();
        if !top_bracket && up_line.contains("select") {
            top_bracket = true;
        }

        while index_down < lines.len() {
            let down_line = &lines[index_down].to_lowercase();
            if !bottom_bracket && down_line.contains("from") {
                bottom_bracket = true;
            }

            if !bottom_bracket && down_line.contains(";") {
                return false;
            }
            if !bottom_bracket {
                index_down += 1;
            } else {
                break;
            }
        }

        if top_bracket && bottom_bracket {
            return true;
        }

        false
    }

    pub fn is_multi_line_comment_clause(&self, line: &str) -> bool {
        if line.contains("/*") || line.contains("*/") {
            return true;
        }
        false
    }

    pub fn is_line_in_multiline_comment_ref(
        &self,
        line: &str,
        index: usize,
        lines: &Vec<&str>,
    ) -> bool {
        if index == 0 || index == lines.len() - 1 || line.contains("/*") || line.contains("*/") {
            return false;
        }

        let mut up_index = index - 1;
        let mut down_index = index + 1;

        let mut top_line = false;
        let mut bottom_line = false;

        while up_index > 0 && up_index < lines.len() {
            if !top_line && lines[up_index].contains("/*") {
                top_line = true;
            }

            if !top_line && lines[up_index].contains("*/") {
                return false;
            }

            if top_line {
                break;
            }
            up_index -= 1;
        }

        if up_index < line.len() && !top_line && lines[up_index].contains("/*") {
            top_line = true;
        }

        if up_index < line.len() && !top_line && lines[up_index].contains("*/") {
            return false;
        }

        while down_index < lines.len() {
            if !bottom_line && lines[down_index].contains("*/") {
                bottom_line = true;
            }

            if !bottom_line && lines[down_index].contains("/*") {
                return false;
            }

            if bottom_line {
                break;
            }

            down_index += 1;
        }

        if top_line && bottom_line {
            return true;
        }

        false
    }

    pub fn is_line_in_multiline_comment(
        &self,
        line: &str,
        index: usize,
        lines: &Vec<String>,
    ) -> bool {
        if index == 0 || index == lines.len() - 1 || line.contains("/*") || line.contains("*/") {
            return false;
        }

        let mut up_index = index - 1;
        let mut down_index = index + 1;

        let mut top_line = false;
        let mut bottom_line = false;

        while up_index > 0 {
            if !top_line && lines[up_index].contains("/*") {
                top_line = true;
            }

            if !top_line && lines[up_index].contains("*/") {
                return false;
            }

            if top_line {
                break;
            }
            up_index -= 1;
        }

        if up_index < line.len() && !top_line && lines[up_index].contains("/*") {
            top_line = true;
        }

        if up_index < line.len() && !top_line && lines[up_index].contains("*/") {
            return false;
        }

        while down_index < lines.len() {
            if !bottom_line && lines[down_index].contains("*/") {
                bottom_line = true;
            }

            if !bottom_line && lines[down_index].contains("/*") {
                return false;
            }

            if bottom_line {
                break;
            }

            down_index += 1;
        }

        if top_line && bottom_line {
            return true;
        }

        false
    }

    pub fn is_line_inside_init_args(&self, line: &str, index: usize, lines: &Vec<String>) -> bool {
        if self.line_contains_cql_kw(line)
            || line.contains(";")
            || line.contains("{")
            || line.contains("}")
            || line.contains("(")
            || line.contains(")")
        {
            return false;
        }

        if index == 0 || index >= lines.len() - 1 {
            return false;
        }

        let lw = line.to_lowercase();

        if lw.contains("values") || lw.contains("from") {
            return false;
        }

        let mut index_up = index - 1;
        let mut index_down = index + 1;

        let mut top_bracket = false;
        let mut bottom_bracket = false;

        loop {
            let up_line = &lines[index_up];
            if up_line.contains("{") || up_line.contains("(") {
                top_bracket = true;
                break;
            }

            if up_line.contains("}") || up_line.contains(")") {
                return false;
            }

            if self.line_contains_cql_kw(up_line) {
                return false;
            }

            if index_up == 0 {
                break;
            }
            index_up -= 1;
        }

        while index_down < lines.len() {
            let down_line = &lines[index_down];

            if down_line.contains("}") || down_line.contains(")") {
                bottom_bracket = true;
                break;
            }

            if down_line.contains("{") || down_line.contains("(") {
                return false;
            }

            if down_line.contains(";") {
                return false;
            }

            if self.line_contains_cql_kw(down_line) {
                return false;
            }

            index_down += 1;
        }

        top_bracket && bottom_bracket
    }
}
