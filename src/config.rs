use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IndentStyle {
    #[default]
    Spaces,
    Tabs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct FormatConfig {
    pub indent_style: IndentStyle,
    pub indent_size: usize,
    pub max_width: usize,
    pub trailing_newline: bool,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Spaces,
            indent_size: 4,
            max_width: 100,
            trailing_newline: true,
        }
    }
}

impl FormatConfig {
    pub fn from_toml(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }
    pub fn indent_str(&self, level: usize) -> String {
        match self.indent_style {
            IndentStyle::Spaces => " ".repeat(self.indent_size * level),
            IndentStyle::Tabs => "\t".repeat(level),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = FormatConfig::default();
        assert_eq!(config.indent_size, 4);
        assert_eq!(config.max_width, 100);
    }

    #[test]
    fn test_from_toml() {
        let toml = r#"
            indent_size = 2
            max_width = 80
        "#;
        let config = FormatConfig::from_toml(toml).unwrap();
        assert_eq!(config.indent_size, 2);
        assert_eq!(config.max_width, 80);
    }

    #[test]
    fn test_indent_str() {
        let config = FormatConfig::default();
        assert_eq!(config.indent_str(0), "");
        assert_eq!(config.indent_str(1), "    ");
        assert_eq!(config.indent_str(2), "        ");
    }
}
