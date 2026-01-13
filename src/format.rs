//! Formatting logic

use oq3_syntax::SourceFile;
use oq3_syntax::ast::{self, AstNode, HasArgList, HasName};

use crate::config::FormatConfig;
use crate::error::FormatError;
use crate::ir::{Doc, join};

pub struct FormatContext<'a> {
    pub config: &'a FormatConfig,
    pub source: &'a str,
}

impl<'a> FormatContext<'a> {
    pub fn new(config: &'a FormatConfig, source: &'a str) -> Self {
        Self { config, source }
    }
}

pub fn format_source(source: &str, config: &FormatConfig) -> Result<String, FormatError> {
    if source.trim().is_empty() {
        return Err(FormatError::EmptyInput);
    }

    let parse_result = SourceFile::parse(source);
    let source_file = parse_result.tree();

    let errors = parse_result.errors();
    if !errors.is_empty() {
        let first = &errors[0];
        return Err(FormatError::Syntax(format!("{:?}", first)));
    }

    let ctx = FormatContext::new(config, source);
    let doc = format_file(&ctx, &source_file);
    let result = crate::printer::print(&doc, config);

    Ok(result)
}

fn format_file(ctx: &FormatContext, file: &ast::SourceFile) -> Doc {
    let mut docs = Vec::new();

    for stmt in file.statements() {
        docs.push(format_stmt(ctx, stmt));
        docs.push(Doc::hardline());
    }

    Doc::concat(docs)
}

fn format_stmt(ctx: &FormatContext, stmt: ast::Stmt) -> Doc {
    match stmt {
        ast::Stmt::VersionString(v) => format_version(v),
        ast::Stmt::Include(i) => format_include(i),
        ast::Stmt::QuantumDeclarationStatement(q) => format_quantum_decl(q),
        ast::Stmt::ClassicalDeclarationStatement(c) => format_classical_decl(c),
        ast::Stmt::Gate(g) => format_gate_def(ctx, g),
        ast::Stmt::ExprStmt(e) => format_expr_stmt(e),
        ast::Stmt::Measure(m) => format_measure_stmt(m),
        ast::Stmt::AssignmentStmt(a) => format_assignment(a),
        ast::Stmt::Reset(r) => format_reset(r),
        ast::Stmt::Barrier(b) => format_barrier(b),
        _ => Doc::text(stmt.syntax().text().to_string().trim()),
    }
}

fn format_version(v: ast::VersionString) -> Doc {
    // TODO: oq3_syntax's VersionString::version() always returns None
    // because lexer treats "OPENQASM 3.0" as a single token.
    // See: https://github.com/Qiskit/openqasm3_parser/issues/118
    let version_str = extract_version_number(&v);
    Doc::concat(vec![
        Doc::text("OPENQASM "),
        Doc::text(version_str),
        Doc::text(";"),
    ])
}

/// Workaround: extract version from VERSION_STRING token since version() doesn't work.
fn extract_version_number(v: &ast::VersionString) -> String {
    use oq3_syntax::SyntaxKind::VERSION_STRING;

    for token in v.syntax().children_with_tokens() {
        if let Some(tok) = token.as_token()
            && tok.kind() == VERSION_STRING {
                return tok
                    .text()
                    .strip_prefix("OPENQASM")
                    .map(|s| s.trim().to_string())
                    .unwrap_or_default();
            }
    }
    String::new()
}

fn format_include(i: ast::Include) -> Doc {
    let file_path = i
        .file()
        .map(|f| f.syntax().text().to_string())
        .unwrap_or_default();
    Doc::concat(vec![
        Doc::text("include "),
        Doc::text(file_path),
        Doc::text(";"),
    ])
}

fn format_quantum_decl(q: ast::QuantumDeclarationStatement) -> Doc {
    let mut parts = vec![Doc::text("qubit")];

    if let Some(qubit_type) = q.qubit_type()
        && let Some(designator) = qubit_type.designator()
            && let Some(expr) = designator.expr() {
                parts.push(Doc::text("["));
                parts.push(format_expr(expr));
                parts.push(Doc::text("]"));
            }

    parts.push(Doc::space());
    if let Some(name) = q.name() {
        parts.push(Doc::text(name.to_string()));
    }
    parts.push(Doc::text(";"));

    Doc::concat(parts)
}

fn format_expr_stmt(e: ast::ExprStmt) -> Doc {
    let mut parts = vec![];
    if let Some(expr) = e.expr() {
        parts.push(format_expr(expr));
    }
    parts.push(Doc::text(";"));
    Doc::concat(parts)
}

fn format_expr(expr: ast::Expr) -> Doc {
    match expr {
        ast::Expr::Identifier(id) => {
            Doc::text(id.ident_token().map(|t| t.to_string()).unwrap_or_default())
        }
        ast::Expr::Literal(lit) => Doc::text(lit.syntax().text().to_string()),
        ast::Expr::IndexedIdentifier(idx) => format_indexed_identifier(&idx),
        ast::Expr::GateCallExpr(g) => format_gate_call(g),
        ast::Expr::MeasureExpression(m) => format_measure_expr(m),
        ast::Expr::ParenExpr(paren) => {
            let mut parts = vec![Doc::text("(")];
            if let Some(inner) = paren.expr() {
                parts.push(format_expr(inner));
            }
            parts.push(Doc::text(")"));
            Doc::concat(parts)
        }
        _ => Doc::text(expr.syntax().text().to_string()),
    }
}

fn format_indexed_identifier(idx: &ast::IndexedIdentifier) -> Doc {
    let mut parts = vec![];

    // TODO: oq3_syntax's IndexedIdentifier::name() always returns None
    // because parser creates IDENTIFIER node instead of NAME node.
    let name = extract_indexed_identifier_name(idx);
    if !name.is_empty() {
        parts.push(Doc::text(name));
    }

    for index in idx.index_operators() {
        parts.push(Doc::text("["));
        let inner_text = index.syntax().text().to_string();
        let inner = inner_text
            .trim_start_matches('[')
            .trim_end_matches(']')
            .trim();
        parts.push(Doc::text(inner));
        parts.push(Doc::text("]"));
    }
    Doc::concat(parts)
}

/// Workaround: extract name from syntax children since name() doesn't work.
fn extract_indexed_identifier_name(idx: &ast::IndexedIdentifier) -> String {
    use oq3_syntax::SyntaxKind::IDENTIFIER;

    for child in idx.syntax().children() {
        if child.kind() == IDENTIFIER {
            return child.text().to_string();
        }
    }
    String::new()
}

fn format_gate_call(g: ast::GateCallExpr) -> Doc {
    let mut parts = vec![];

    // TODO: oq3_syntax's GateCallExpr::name() always returns None
    // because parser creates IDENTIFIER node instead of NAME node.
    let gate_name = extract_gate_call_name(&g);
    if !gate_name.is_empty() {
        parts.push(Doc::text(gate_name));
    }

    if let Some(params) = g.arg_list() {
        let params_text = params.syntax().text().to_string();
        if !params_text.trim().is_empty() {
            parts.push(Doc::text(params_text));
        }
    }

    if let Some(qubits) = g.qubit_list() {
        let qubit_operands: Vec<_> = qubits.gate_operands().collect();
        if !qubit_operands.is_empty() {
            parts.push(Doc::space());
            let qubit_docs: Vec<Doc> = qubit_operands
                .into_iter()
                .map(format_gate_operand)
                .collect();
            parts.push(join(qubit_docs, Doc::text(", ")));
        }
    }

    Doc::concat(parts)
}

/// Workaround: extract gate name from syntax children since name() doesn't work.
fn extract_gate_call_name(g: &ast::GateCallExpr) -> String {
    use oq3_syntax::SyntaxKind::IDENTIFIER;

    for child in g.syntax().children() {
        if child.kind() == IDENTIFIER {
            return child.text().to_string();
        }
    }
    String::new()
}

fn format_gate_operand(op: ast::GateOperand) -> Doc {
    match op {
        ast::GateOperand::Identifier(id) => {
            Doc::text(id.ident_token().map(|t| t.to_string()).unwrap_or_default())
        }
        ast::GateOperand::IndexedIdentifier(idx) => format_indexed_identifier(&idx),
        ast::GateOperand::HardwareQubit(hw) => Doc::text(hw.syntax().text().to_string()),
    }
}

fn format_gate_def(ctx: &FormatContext, g: ast::Gate) -> Doc {
    let mut parts = vec![];

    parts.push(Doc::text("gate "));

    if let Some(name) = g.name() {
        parts.push(Doc::text(name.to_string()));
    }

    // TODO: oq3_syntax's Param::name() always returns None.
    // Workaround: use syntax text directly.
    if let Some(params) = g.angle_params() {
        let param_text = params.syntax().text().to_string();
        if !param_text.trim().is_empty() && param_text != "()" {
            parts.push(Doc::text(param_text));
        }
    }

    if let Some(qubits) = g.qubit_params() {
        let qubit_text = qubits.syntax().text().to_string();
        if !qubit_text.trim().is_empty() {
            parts.push(Doc::space());
            parts.push(Doc::text(qubit_text));
        }
    }

    parts.push(Doc::text(" {"));
    if let Some(body) = g.body() {
        parts.push(Doc::indent(Doc::concat(vec![
            Doc::hardline(),
            format_gate_body(ctx, body),
        ])));
        parts.push(Doc::hardline());
    }
    parts.push(Doc::text("}"));

    Doc::concat(parts)
}

fn format_gate_body(ctx: &FormatContext, body: ast::BlockExpr) -> Doc {
    let stmts: Vec<Doc> = body
        .statements()
        .map(|stmt| format_stmt(ctx, stmt))
        .collect();
    join(stmts, Doc::hardline())
}

fn format_measure_expr(m: ast::MeasureExpression) -> Doc {
    let mut parts = vec![Doc::text("measure ")];

    if let Some(qubit) = m.gate_operand() {
        parts.push(format_gate_operand(qubit));
    }

    Doc::concat(parts)
}

fn format_measure_stmt(m: ast::Measure) -> Doc {
    Doc::text(m.syntax().text().to_string().trim())
}

fn format_reset(r: ast::Reset) -> Doc {
    let mut parts = vec![Doc::text("reset ")];

    if let Some(qubit) = r.gate_operand() {
        parts.push(format_gate_operand(qubit));
    }

    parts.push(Doc::text(";"));
    Doc::concat(parts)
}

fn format_barrier(b: ast::Barrier) -> Doc {
    let mut parts = vec![Doc::text("barrier")];

    if let Some(qubit_list) = b.qubit_list() {
        let qubits: Vec<_> = qubit_list.gate_operands().collect();
        if !qubits.is_empty() {
            parts.push(Doc::space());
            let qubit_docs: Vec<Doc> = qubits.into_iter().map(format_gate_operand).collect();
            parts.push(join(qubit_docs, Doc::text(", ")));
        }
    }

    parts.push(Doc::text(";"));
    Doc::concat(parts)
}

fn format_assignment(a: ast::AssignmentStmt) -> Doc {
    Doc::text(a.syntax().text().to_string().trim())
}

fn format_classical_decl(c: ast::ClassicalDeclarationStatement) -> Doc {
    let mut parts = vec![];

    if let Some(scalar_type) = c.scalar_type() {
        parts.push(Doc::text(scalar_type.syntax().text().to_string()));
    } else if let Some(array_type) = c.array_type() {
        parts.push(Doc::text(array_type.syntax().text().to_string()));
    }

    parts.push(Doc::space());

    if let Some(name) = c.name() {
        parts.push(Doc::text(name.to_string()));
    }

    if let Some(init) = c.expr() {
        parts.push(Doc::text(" = "));
        parts.push(format_expr(init));
    }

    parts.push(Doc::text(";"));
    Doc::concat(parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(source: &str) -> String {
        let config = FormatConfig::default();
        format_source(source, &config).unwrap()
    }

    #[test]
    fn test_version() {
        let result = fmt("OPENQASM  3.0;");
        assert_eq!(result.trim(), "OPENQASM 3.0;");
    }

    #[test]
    fn test_qubit_decl() {
        let result = fmt("qubit[2] q;");
        assert_eq!(result.trim(), "qubit[2] q;");
    }

    #[test]
    fn test_gate_call() {
        let result = fmt("h q[0];");
        assert_eq!(result.trim(), "h q[0];");
    }

    #[test]
    fn test_cx_gate() {
        let result = fmt("cx q[0], q[1];");
        assert_eq!(result.trim(), "cx q[0], q[1];");
    }

    #[test]
    fn test_gate_def() {
        let input = "gate mygate(theta) q { rz(theta) q; }";
        let result = fmt(input);
        assert!(result.contains("gate mygate(theta) q {"));
        assert!(result.contains("rz(theta) q;"));
    }
}
