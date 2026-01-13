use crate::config::FormatConfig;
use crate::ir::Doc;

/// Pretty Printer
pub struct Printer<'a> {
    config: &'a FormatConfig,
    output: String,
    current_line: String,
    indent: usize,
}

impl<'a> Printer<'a> {
    pub fn new(config: &'a FormatConfig) -> Self {
        Self {
            config,
            output: String::new(),
            current_line: String::new(),
            indent: 0,
        }
    }

    pub fn print(mut self, doc: &Doc) -> String {
        self.print_doc(doc, false);
        self.flush();

        if self.config.trailing_newline {
            if !self.output.ends_with('\n') {
                self.output.push('\n');
            }
        } else {
            while self.output.ends_with('\n') {
                self.output.pop();
            }
        }

        self.output
    }

    fn print_doc(&mut self, doc: &Doc, flat: bool) {
        match doc {
            Doc::Nil => {}

            Doc::Text(s) => {
                self.current_line.push_str(s);
            }

            Doc::Hardline => {
                self.flush();
            }

            Doc::Softline => {
                if flat {
                    self.current_line.push(' ');
                } else {
                    self.flush();
                }
            }

            Doc::Concat(docs) => {
                for d in docs {
                    self.print_doc(d, flat);
                }
            }

            Doc::Indent(inner) => {
                self.indent += 1;
                self.print_doc(inner, flat);
                self.indent -= 1;
            }

            Doc::Group(inner) => {
                let fits = self.fits(inner);
                self.print_doc(inner, fits);
            }
        }
    }

    fn flush(&mut self) {
        if !self.current_line.is_empty() {
            let indent_str = self.config.indent_str(self.indent);
            self.output.push_str(&indent_str);
            self.output.push_str(&self.current_line);
            self.current_line.clear();
        }
        self.output.push('\n');
    }

    fn fits(&self, doc: &Doc) -> bool {
        let current_width = self.indent * self.config.indent_size + self.current_line.len();
        let remaining = self.config.max_width.saturating_sub(current_width);

        self.measure(doc) <= remaining
    }

    fn measure(&self, doc: &Doc) -> usize {
        match doc {
            Doc::Nil => 0,
            Doc::Text(s) => s.len(),
            Doc::Hardline => usize::MAX,
            Doc::Softline => 1,
            Doc::Concat(docs) => docs.iter().map(|d| self.measure(d)).sum(),
            Doc::Indent(inner) => self.measure(inner),
            Doc::Group(inner) => self.measure(inner),
        }
    }
}

pub fn print(doc: &Doc, config: &FormatConfig) -> String {
    Printer::new(config).print(doc)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> FormatConfig {
        FormatConfig::default()
    }

    #[test]
    fn test_simple_text() {
        let doc = Doc::text("hello");
        let result = print(&doc, &default_config());
        assert_eq!(result, "hello\n");
    }

    #[test]
    fn test_concat() {
        let doc = Doc::concat(vec![Doc::text("a"), Doc::text(" "), Doc::text("b")]);
        let result = print(&doc, &default_config());
        assert_eq!(result, "a b\n");
    }

    #[test]
    fn test_hardline() {
        let doc = Doc::concat(vec![Doc::text("a"), Doc::hardline(), Doc::text("b")]);
        let result = print(&doc, &default_config());
        assert_eq!(result, "a\nb\n");
    }

    #[test]
    fn test_indent() {
        let doc = Doc::concat(vec![
            Doc::text("outer"),
            Doc::hardline(),
            Doc::indent(Doc::text("inner")),
        ]);
        let result = print(&doc, &default_config());
        assert_eq!(result, "outer\n    inner\n");
    }

    #[test]
    fn test_group_fits() {
        let doc = Doc::group(Doc::concat(vec![
            Doc::text("a"),
            Doc::softline(),
            Doc::text("b"),
        ]));
        let result = print(&doc, &default_config());
        assert_eq!(result, "a b\n");
    }

    #[test]
    fn test_no_trailing_newline() {
        let mut config = default_config();
        config.trailing_newline = false;

        let doc = Doc::text("hello");
        let result = print(&doc, &config);
        assert_eq!(result, "hello");
    }
}
