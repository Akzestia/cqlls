/*
    MIT License

    Copyright (c) 2026 アクゼスティア
*/

use crate::tree_sitter::TS_CQL;
use tower_lsp::lsp_types::*;
use tree_sitter::Node;

use crate::lsp::Backend;

// TODO: Add better docs
impl Backend {
    pub async fn compute_diagnostics(&self, text: &str) -> Vec<Diagnostic> {
        if !self.diagnostics {
            return vec![];
        }

        let mut parser = TS_CQL.lock().await;
        let tree = match parser.parse(text, None) {
            Some(t) => t,
            None => return vec![],
        };

        let mut diags = Vec::new();
        let mut cursor = tree.walk();
        let root = tree.root_node();
        Self::collect_error_nodes(root, &mut cursor, &mut diags);
        diags
    }

    fn collect_error_nodes(
        node: Node,
        cursor: &mut tree_sitter::TreeCursor,
        out: &mut Vec<Diagnostic>,
    ) {
        if node.is_error() || node.is_missing() {
            let range = Self::node_to_range(node);
            out.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("cql".to_string()),
                message: "Syntax error".to_string(),
                ..Default::default()
            });
        }

        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                Self::collect_error_nodes(child, cursor, out);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            cursor.goto_parent();
        }
    }

    fn node_to_range(node: Node) -> Range {
        let start = node.start_position();
        let end = node.end_position();
        Range {
            start: Position::new(start.row as u32, start.column as u32),
            end: Position::new(end.row as u32, end.column as u32),
        }
    }
}
