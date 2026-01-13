//! Comment extraction

use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommentStyle {
    Line,
    Block,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub style: CommentStyle,
    pub content: String,
    pub line: usize,
    pub column: usize,
}

pub type CommentMap = BTreeMap<usize, Vec<Comment>>;

pub fn extract_comments(source: &str) -> CommentMap {
    let mut comments = CommentMap::new();
    let mut chars = source.char_indices().peekable();
    let mut line = 0;
    let mut line_start = 0;
    let mut in_string = false;

    while let Some((i, c)) = chars.next() {
        if c == '\n' {
            line += 1;
            line_start = i + 1;
            continue;
        }

        if c == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }

        if c == '/' {
            if let Some(&(_, next)) = chars.peek() {
                if next == '/' {
                    chars.next();
                    let start = i;
                    let mut end = i + 2;

                    while let Some(&(j, ch)) = chars.peek() {
                        if ch == '\n' {
                            break;
                        }
                        end = j + ch.len_utf8();
                        chars.next();
                    }

                    comments.entry(line).or_default().push(Comment {
                        style: CommentStyle::Line,
                        content: source[start..end].to_string(),
                        line,
                        column: i - line_start,
                    });
                } else if next == '*' {
                    chars.next();
                    let start = i;
                    let start_line = line;
                    let mut end = i + 2;

                    while let Some((j, ch)) = chars.next() {
                        if ch == '\n' {
                            line += 1;
                            line_start = j + 1;
                        }
                        if ch == '*' {
                            if let Some(&(_, '/')) = chars.peek() {
                                chars.next();
                                end = j + 2;
                                break;
                            }
                        }
                        end = j + ch.len_utf8();
                    }

                    comments.entry(start_line).or_default().push(Comment {
                        style: CommentStyle::Block,
                        content: source[start..end].to_string(),
                        line: start_line,
                        column: i - (if start_line == 0 { 0 } else { line_start }),
                    });
                }
            }
        }
    }

    comments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_comment() {
        let source = "qubit q; // comment\n";
        let comments = extract_comments(source);

        assert_eq!(comments.len(), 1);
        let line_comments = &comments[&0];
        assert_eq!(line_comments.len(), 1);
        assert_eq!(line_comments[0].style, CommentStyle::Line);
        assert_eq!(line_comments[0].content, "// comment");
    }

    #[test]
    fn test_block_comment() {
        let source = "/* block */\nqubit q;";
        let comments = extract_comments(source);

        assert_eq!(comments.len(), 1);
        let line_comments = &comments[&0];
        assert_eq!(line_comments[0].style, CommentStyle::Block);
    }

    #[test]
    fn test_comment_in_string() {
        let source = r#"include "// not comment";"#;
        let comments = extract_comments(source);
        assert!(comments.is_empty());
    }
}
