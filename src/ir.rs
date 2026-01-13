#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Doc {
    Nil,
    Text(String),
    Hardline,
    Softline,
    Concat(Vec<Doc>),
    Indent(Box<Doc>),
    Group(Box<Doc>),
}

impl Doc {
    pub fn nil() -> Self {
        Doc::Nil
    }

    pub fn text(s: impl Into<String>) -> Self {
        Doc::Text(s.into())
    }

    pub fn hardline() -> Self {
        Doc::Hardline
    }

    pub fn softline() -> Self {
        Doc::Softline
    }

    pub fn space() -> Self {
        Doc::Text(" ".into())
    }

    pub fn concat(docs: Vec<Doc>) -> Self {
        let docs: Vec<_> = docs
            .into_iter()
            .filter(|d| !matches!(d, Doc::Nil))
            .collect();
        match docs.len() {
            0 => Doc::Nil,
            1 => docs.into_iter().next().unwrap(),
            _ => Doc::Concat(docs),
        }
    }
    pub fn indent(doc: Doc) -> Self {
        Doc::Indent(Box::new(doc))
    }
    pub fn group(doc: Doc) -> Self {
        Doc::Group(Box::new(doc))
    }
}

pub fn join(docs: Vec<Doc>, sep: Doc) -> Doc {
    if docs.is_empty() {
        return Doc::Nil;
    }

    let mut result = Vec::new();
    let mut first = true;

    for doc in docs {
        if !first {
            result.push(sep.clone());
        }
        first = false;
        result.push(doc);
    }
    Doc::concat(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let doc = Doc::text("hello");
        assert_eq!(doc, Doc::Text("hello".into()));
    }

    #[test]
    fn test_concat_optimization() {
        let doc = Doc::concat(vec![Doc::Nil, Doc::text("a"), Doc::Nil, Doc::text("b")]);
        match doc {
            Doc::Concat(docs) => assert_eq!(docs.len(), 2),
            _ => panic!("Expected Concat"),
        }
    }

    #[test]
    fn test_join() {
        let docs = vec![Doc::text("a"), Doc::text("b"), Doc::text("c")];
        let result = join(docs, Doc::text(" "));

        match result {
            Doc::Concat(parts) => assert_eq!(parts.len(), 5),
            _ => panic!("Expected Concat"),
        }
    }
}
