use std::{collections::HashSet, fs};

use crate::{
    builder::Builder,
    error::CompilerError,
    model::{RawRelation, RawToken},
};

/// Raw semantic data produced by the parser.
#[derive(Debug)]
pub struct ParseResult {
    pub tokens: Vec<RawToken>,
    pub relations: Vec<RawRelation>,
}

/// Parses an external source into raw semantic data.
#[derive(Debug, Default)]
pub struct Parser;

impl Parser {
    pub fn parse(&self, builder: &Builder) -> Result<ParseResult, CompilerError> {
        let input_path = builder.input_path().ok_or(CompilerError::MissingInput)?;

        let content = fs::read_to_string(input_path)?;

        let mut known_tokens = HashSet::new();
        let mut tokens = Vec::new();
        let mut relations = Vec::new();

        for (index, original_line) in content.lines().enumerate() {
            let line_number = index + 1;
            let line = original_line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let (source, relation_data) =
                line.split_once("->").ok_or_else(|| CompilerError::InvalidSyntax {
                    line: line_number,
                    content: original_line.to_owned(),
                })?;

            let (target, weight) =
                relation_data.rsplit_once(':').ok_or_else(|| CompilerError::InvalidSyntax {
                    line: line_number,
                    content: original_line.to_owned(),
                })?;

            let source = source.trim();
            let target = target.trim();
            let weight = weight.trim();

            if source.is_empty() || target.is_empty() || weight.is_empty() {
                return Err(CompilerError::InvalidSyntax {
                    line: line_number,
                    content: original_line.to_owned(),
                });
            }

            let weight = weight.parse::<f32>().map_err(|_| CompilerError::InvalidSyntax {
                line: line_number,
                content: original_line.to_owned(),
            })?;

            insert_token(source, &mut known_tokens, &mut tokens);

            insert_token(target, &mut known_tokens, &mut tokens);

            relations.push(RawRelation {
                source: source.to_owned(),
                target: target.to_owned(),
                weight,
            });
        }

        Ok(ParseResult { tokens, relations })
    }
}

fn insert_token(text: &str, known_tokens: &mut HashSet<String>, tokens: &mut Vec<RawToken>) {
    if known_tokens.insert(text.to_owned()) {
        tokens.push(RawToken { text: text.to_owned() });
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::{Path, PathBuf},
        time::{SystemTime, UNIX_EPOCH},
    };

    use super::Parser;
    use crate::{builder::Builder, error::CompilerError};

    #[test]
    fn parses_plain_text_dataset() {
        let path = create_test_file(
            "\
# Rust semantic dataset

rust -> systems : 0.95
rust -> memory : 0.87
memory -> ownership : 1.0
",
        );

        let builder = Builder::new().input(path.to_string_lossy().into_owned());

        let result = Parser.parse(&builder).expect("dataset parsing should succeed");

        assert_eq!(result.tokens.len(), 4);
        assert_eq!(result.relations.len(), 3);

        assert_eq!(result.tokens[0].text, "rust");
        assert_eq!(result.tokens[1].text, "systems");
        assert_eq!(result.tokens[2].text, "memory");
        assert_eq!(result.tokens[3].text, "ownership");

        assert_eq!(result.relations[0].source, "rust");
        assert_eq!(result.relations[0].target, "systems");
        assert_eq!(result.relations[0].weight, 0.95);

        remove_test_file(&path);
    }

    #[test]
    fn ignores_comments_and_empty_lines() {
        let path = create_test_file(
            "\
# Comment


rust -> ownership : 1.0

# Another comment
ownership -> memory : 0.8
",
        );

        let builder = Builder::new().input(path.to_string_lossy().into_owned());

        let result = Parser.parse(&builder).expect("comments and empty lines should be ignored");

        assert_eq!(result.tokens.len(), 3);
        assert_eq!(result.relations.len(), 2);

        remove_test_file(&path);
    }

    #[test]
    fn rejects_invalid_syntax() {
        let path = create_test_file(
            "\
rust ownership 1.0
",
        );

        let builder = Builder::new().input(path.to_string_lossy().into_owned());

        let result = Parser.parse(&builder);

        assert!(matches!(result, Err(CompilerError::InvalidSyntax { line: 1, .. })));

        remove_test_file(&path);
    }

    #[test]
    fn reports_missing_input() {
        let builder = Builder::new();

        let result = Parser.parse(&builder);

        assert!(matches!(result, Err(CompilerError::MissingInput)));
    }

    fn create_test_file(content: &str) -> PathBuf {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();

        let path = std::env::temp_dir()
            .join(format!("lxdb-parser-test-{}-{timestamp}.txt", std::process::id(),));

        fs::write(&path, content).expect("test input file should be created");

        path
    }

    fn remove_test_file(path: &Path) {
        let _ = fs::remove_file(path);
    }
}
