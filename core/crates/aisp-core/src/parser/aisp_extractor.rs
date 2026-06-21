//! AISP Code Block Extraction for Mixed Format Documents
//!
//! This module implements content extraction capabilities for ADR-023,
//! enabling extraction of AISP code blocks from Markdown and other
//! container formats while preserving context and metadata.

use crate::error::{AispError, AispResult};
use std::collections::HashMap;

/// Extracted AISP code block with metadata
#[derive(Debug, Clone)]
pub struct ExtractedAispBlock {
    /// The raw AISP content without fence markers
    pub content: String,

    /// Line number where the block starts in the source document
    pub start_line: usize,

    /// Line number where the block ends in the source document  
    pub end_line: usize,

    /// Optional label/identifier for the block
    pub label: Option<String>,

    /// Additional attributes from the code fence (e.g., ```aisp{label=test})
    pub attributes: HashMap<String, String>,
}

/// Context information about the extraction process
#[derive(Debug, Clone)]
pub struct ExtractionContext {
    /// Original document format
    pub source_format: String,

    /// Total number of AISP blocks found
    pub block_count: usize,

    /// Any warnings or issues encountered during extraction
    pub warnings: Vec<String>,

    /// Markdown content with AISP blocks removed
    pub cleaned_markdown: Option<String>,
}

/// AISP code block extractor for mixed format documents
pub struct AispCodeBlockExtractor {
    /// Whether to preserve empty lines in extracted blocks
    preserve_whitespace: bool,

    /// Whether to validate extracted AISP syntax
    validate_syntax: bool,
}

impl Default for AispCodeBlockExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl AispCodeBlockExtractor {
    /// Create a new extractor with default settings
    pub fn new() -> Self {
        Self {
            preserve_whitespace: true,
            validate_syntax: false, // Fast extraction by default
        }
    }

    /// Create an extractor with custom settings
    pub fn with_options(preserve_whitespace: bool, validate_syntax: bool) -> Self {
        Self {
            preserve_whitespace,
            validate_syntax,
        }
    }

    /// Extract all AISP code blocks from markdown content
    pub fn extract_aisp_blocks(&self, content: &str) -> AispResult<Vec<ExtractedAispBlock>> {
        let mut blocks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_line = 0;

        while current_line < lines.len() {
            if let Some(block) = self.extract_next_block(&lines, &mut current_line)? {
                if self.validate_syntax {
                    self.validate_aisp_block(&block)?;
                }
                blocks.push(block);
            }
        }

        Ok(blocks)
    }

    /// Extract AISP blocks and return both blocks and cleaned markdown
    pub fn extract_with_context(
        &self,
        content: &str,
    ) -> AispResult<(Vec<ExtractedAispBlock>, ExtractionContext)> {
        let blocks = self.extract_aisp_blocks(content)?;
        let cleaned_markdown = self.remove_aisp_blocks(content);

        let context = ExtractionContext {
            source_format: "markdown".to_string(),
            block_count: blocks.len(),
            warnings: Vec::new(), // TODO: Collect warnings during extraction
            cleaned_markdown: Some(cleaned_markdown),
        };

        Ok((blocks, context))
    }

    /// Remove AISP code blocks from content, keeping markdown
    pub fn remove_aisp_blocks(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        let mut result = Vec::new();
        let mut current_line = 0;

        while current_line < lines.len() {
            if self.is_aisp_fence_start(lines[current_line]) {
                // Skip the fence start
                current_line += 1;

                // Skip until fence end
                while current_line < lines.len() && !self.is_aisp_fence_end(lines[current_line]) {
                    current_line += 1;
                }

                // Skip the fence end
                if current_line < lines.len() {
                    current_line += 1;
                }
            } else {
                result.push(lines[current_line]);
                current_line += 1;
            }
        }

        result.join("\n")
    }

    /// Extract the next AISP block starting from current_line
    fn extract_next_block(
        &self,
        lines: &[&str],
        current_line: &mut usize,
    ) -> AispResult<Option<ExtractedAispBlock>> {
        // Find the start of the next AISP block
        while *current_line < lines.len() && !self.is_aisp_fence_start(lines[*current_line]) {
            *current_line += 1;
        }

        if *current_line >= lines.len() {
            return Ok(None); // No more blocks
        }

        let start_line = *current_line;
        let fence_line = lines[*current_line];
        let (label, attributes) = self.parse_fence_attributes(fence_line);

        // Move past the opening fence
        *current_line += 1;

        // Collect content until closing fence
        let mut content_lines = Vec::new();
        let content_start = *current_line;

        while *current_line < lines.len() && !self.is_aisp_fence_end(lines[*current_line]) {
            let line = lines[*current_line];
            if self.preserve_whitespace || !line.trim().is_empty() {
                content_lines.push(line);
            }
            *current_line += 1;
        }

        if *current_line >= lines.len() {
            return Err(AispError::ParseError {
                message: format!(
                    "Unclosed AISP code block starting at line {}",
                    start_line + 1
                ),
                line: start_line,
                column: 0,
            });
        }

        let end_line = *current_line;
        *current_line += 1; // Move past closing fence

        let content = content_lines.join("\n");

        Ok(Some(ExtractedAispBlock {
            content,
            start_line: content_start,
            end_line,
            label,
            attributes,
        }))
    }

    /// Check if a line starts an AISP code fence
    fn is_aisp_fence_start(&self, line: &str) -> bool {
        let trimmed = line.trim();
        trimmed.starts_with("```aisp")
    }

    /// Check if a line ends an AISP code fence  
    fn is_aisp_fence_end(&self, line: &str) -> bool {
        let trimmed = line.trim();
        trimmed == "```"
    }

    /// Parse attributes from fence line (e.g., ```aisp{label=test,validate=true})
    fn parse_fence_attributes(
        &self,
        fence_line: &str,
    ) -> (Option<String>, HashMap<String, String>) {
        let mut label = None;
        let mut attributes = HashMap::new();

        // Simple attribute parsing - could be enhanced with proper parser
        if let Some(attr_start) = fence_line.find('{') {
            if let Some(attr_end) = fence_line.rfind('}') {
                let attr_content = &fence_line[attr_start + 1..attr_end];

                for pair in attr_content.split(',') {
                    let pair = pair.trim();
                    if pair.contains('=') {
                        let parts: Vec<&str> = pair.splitn(2, '=').collect();
                        if parts.len() == 2 {
                            let key = parts[0].trim();
                            let value = parts[1].trim().trim_matches('"');

                            if key == "label" {
                                label = Some(value.to_string());
                            }
                            attributes.insert(key.to_string(), value.to_string());
                        }
                    }
                }
            }
        }

        (label, attributes)
    }

    /// Basic validation of extracted AISP content
    fn validate_aisp_block(&self, block: &ExtractedAispBlock) -> AispResult<()> {
        let content = block.content.trim();

        // Must not be empty
        if content.is_empty() {
            return Err(AispError::ParseError {
                message: "AISP block cannot be empty".to_string(),
                line: block.start_line,
                column: 0,
            });
        }

        // Should contain AISP elements
        let has_aisp_elements = content.contains('𝔸')
            || content.contains('⟦')
            || content.contains('≜')
            || content.contains('∀')
            || content.contains('∃');

        if !has_aisp_elements {
            return Err(AispError::ParseError {
                message: "AISP block does not contain recognizable AISP elements".to_string(),
                line: block.start_line,
                column: 0,
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_single_aisp_block() {
        let content = r#"# Documentation

Some text here.

```aisp
𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{domain≜"test"}
```

More text here."#;

        let extractor = AispCodeBlockExtractor::new();
        let blocks = extractor.extract_aisp_blocks(content).unwrap();

        assert_eq!(blocks.len(), 1);
        assert!(blocks[0].content.contains("𝔸5.1.test@2026-01-30"));
        assert!(blocks[0].content.contains("⟦Ω:Meta⟧"));
    }

    #[test]
    fn test_extract_multiple_aisp_blocks() {
        let content = r#"# Documentation

```aisp
𝔸5.1.first@2026-01-30
⟦Ω:Meta⟧{domain≜"first"}
```

Some text between blocks.

```aisp
𝔸5.1.second@2026-01-30
⟦Ω:Meta⟧{domain≜"second"}
```"#;

        let extractor = AispCodeBlockExtractor::new();
        let blocks = extractor.extract_aisp_blocks(content).unwrap();

        assert_eq!(blocks.len(), 2);
        assert!(blocks[0].content.contains("first"));
        assert!(blocks[1].content.contains("second"));
    }

    #[test]
    fn test_extract_with_attributes() {
        let content = r#"```aisp{label="test-spec", validate=true}
𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{domain≜"test"}
```"#;

        let extractor = AispCodeBlockExtractor::new();
        let blocks = extractor.extract_aisp_blocks(content).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].label, Some("test-spec".to_string()));
        assert_eq!(
            blocks[0].attributes.get("validate"),
            Some(&"true".to_string())
        );
    }

    #[test]
    fn test_remove_aisp_blocks() {
        let content = r#"# Documentation

Some text here.

```aisp
𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{domain≜"test"}
```

More text here.

```aisp
Another block
```

Final text."#;

        let extractor = AispCodeBlockExtractor::new();
        let cleaned = extractor.remove_aisp_blocks(content);

        assert!(!cleaned.contains("𝔸5.1.test"));
        assert!(!cleaned.contains("Another block"));
        assert!(cleaned.contains("# Documentation"));
        assert!(cleaned.contains("Some text here."));
        assert!(cleaned.contains("More text here."));
        assert!(cleaned.contains("Final text."));
    }

    #[test]
    fn test_extract_with_context() {
        let content = r#"# Documentation

```aisp
𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{domain≜"test"}
```

```aisp
Another block
```"#;

        let extractor = AispCodeBlockExtractor::new();
        let (blocks, context) = extractor.extract_with_context(content).unwrap();

        assert_eq!(blocks.len(), 2);
        assert_eq!(context.block_count, 2);
        assert_eq!(context.source_format, "markdown");
        assert!(context.cleaned_markdown.is_some());
    }

    #[test]
    fn test_unclosed_block_error() {
        let content = r#"```aisp
𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{domain≜"test"}"#; // Missing closing fence

        let extractor = AispCodeBlockExtractor::new();
        let result = extractor.extract_aisp_blocks(content);

        assert!(result.is_err());
        if let Err(AispError::ParseError { message, .. }) = result {
            assert!(message.contains("Unclosed AISP code block"));
        }
    }

    #[test]
    fn test_validation_enabled() {
        let content = r#"```aisp
Just plain text with no AISP elements
```"#;

        let extractor = AispCodeBlockExtractor::with_options(true, true); // Enable validation
        let result = extractor.extract_aisp_blocks(content);

        assert!(result.is_err());
        if let Err(AispError::ParseError { message, .. }) = result {
            assert!(message.contains("does not contain recognizable AISP elements"));
        }
    }

    #[test]
    fn test_empty_block_validation() {
        let content = r#"```aisp
```"#;

        let extractor = AispCodeBlockExtractor::with_options(true, true); // Enable validation
        let result = extractor.extract_aisp_blocks(content);

        assert!(result.is_err());
        if let Err(AispError::ParseError { message, .. }) = result {
            assert!(message.contains("AISP block cannot be empty"));
        }
    }

    #[test]
    fn test_preserve_whitespace_disabled() {
        let content = r#"```aisp
𝔸5.1.test@2026-01-30

⟦Ω:Meta⟧{domain≜"test"}

```"#;

        let extractor = AispCodeBlockExtractor::with_options(false, false); // Don't preserve whitespace
        let blocks = extractor.extract_aisp_blocks(content).unwrap();

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].content.lines().count(), 2); // Empty lines removed
    }
}
