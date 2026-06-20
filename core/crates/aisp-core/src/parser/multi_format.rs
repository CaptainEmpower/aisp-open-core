//! Multi-Format Parser Orchestrator
//!
//! This module implements the core orchestration logic for ADR-023,
//! coordinating format detection, content extraction, and parser
//! selection to handle mixed-format documents while maintaining
//! SRP compliance.

use super::aisp_extractor::{AispCodeBlockExtractor, ExtractedAispBlock, ExtractionContext};
use super::format_detection::{DocumentFormat, FormatAnalysis, FormatDetector};
use super::robust_parser::RobustAispParser;
use crate::ast::canonical::CanonicalAispDocument as AispDocument;
use crate::error::{AispError, AispResult};

/// Parsed document with format-specific structure
#[derive(Debug)]
pub enum ParsedDocument {
    /// Pure AISP document - single parsed document
    Pure(AispDocument),

    /// Mixed format document with separated content
    Mixed {
        /// Original markdown/prose content with AISP blocks removed
        prose_content: String,

        /// Parsed AISP documents extracted from code blocks
        aisp_documents: Vec<ParsedAispDocument>,

        /// Format analysis and extraction metadata
        metadata: MixedFormatMetadata,
    },
}

/// Individual parsed AISP document with extraction context
#[derive(Debug)]
pub struct ParsedAispDocument {
    /// The parsed AISP document
    pub document: AispDocument,

    /// Original extraction information
    pub extraction_info: ExtractedAispBlock,

    /// Parse warnings specific to this document
    pub warnings: Vec<String>,
}

/// Metadata about mixed format parsing
#[derive(Debug)]
pub struct MixedFormatMetadata {
    /// Original document format detected
    pub source_format: DocumentFormat,

    /// Format analysis results
    pub analysis: FormatAnalysis,

    /// Extraction context
    pub extraction_context: ExtractionContext,

    /// Overall parsing warnings
    pub warnings: Vec<String>,

    /// Performance metrics
    pub metrics: ParsingMetrics,
}

/// Performance metrics for parsing operations
#[derive(Debug)]
pub struct ParsingMetrics {
    /// Time spent on format detection (microseconds)
    pub detection_time_us: u64,

    /// Time spent on content extraction (microseconds)
    pub extraction_time_us: u64,

    /// Time spent on AISP parsing (microseconds)
    pub parsing_time_us: u64,

    /// Total document size processed
    pub document_size_bytes: usize,

    /// Number of AISP blocks processed
    pub blocks_processed: usize,
}

/// Multi-format parser orchestrator
// TODO(#14): reserved for not-yet-implemented logic; see ROADMAP.
#[allow(dead_code)]
pub struct MultiFormatParser {
    /// Content extractor for mixed formats
    extractor: AispCodeBlockExtractor,

    /// Whether to collect detailed metrics
    collect_metrics: bool,

    /// Whether to validate extracted AISP blocks
    validate_extracted: bool,
}

impl Default for MultiFormatParser {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiFormatParser {
    /// Create a new multi-format parser with default settings
    pub fn new() -> Self {
        Self {
            extractor: AispCodeBlockExtractor::new(),
            collect_metrics: false,
            validate_extracted: true,
        }
    }

    /// Create a parser with custom settings
    pub fn with_options(collect_metrics: bool, validate_extracted: bool) -> Self {
        Self {
            extractor: AispCodeBlockExtractor::with_options(true, validate_extracted),
            collect_metrics,
            validate_extracted,
        }
    }

    /// Parse a document with automatic format detection
    pub fn parse(&self, content: &str) -> AispResult<ParsedDocument> {
        let _start_time = std::time::Instant::now();

        // Phase 1: Format Detection
        let detection_start = std::time::Instant::now();
        let analysis = FormatDetector::analyze(content);
        let detection_time_us = detection_start.elapsed().as_micros() as u64;

        if !FormatDetector::is_parseable(&analysis.format) {
            return Err(AispError::UnsupportedFormat {
                format: FormatDetector::format_description(&analysis.format).to_string(),
            });
        }

        // Phase 2: Format-specific parsing

        match analysis.format {
            DocumentFormat::PureAisp => self.parse_pure_aisp(content, detection_time_us),
            DocumentFormat::MarkdownWithAisp => {
                self.parse_markdown_with_aisp(content, analysis, detection_time_us)
            }
            DocumentFormat::MixedFormat => {
                self.parse_mixed_format(content, analysis, detection_time_us)
            }
            DocumentFormat::Unknown => Err(AispError::UnsupportedFormat {
                format: "Unknown document format".to_string(),
            }),
        }
    }

    /// Parse pure AISP document using existing parser
    fn parse_pure_aisp(
        &self,
        content: &str,
        _detection_time_us: u64,
    ) -> AispResult<ParsedDocument> {
        let parsing_start = std::time::Instant::now();

        let parser = RobustAispParser::new();
        let parse_result = parser.parse(content);
        let document = parse_result.document.ok_or_else(|| AispError::ParseError {
            message: "Failed to parse document".to_string(),
            line: 0,
            column: 0,
        })?;

        let _parsing_time_us = parsing_start.elapsed().as_micros() as u64;

        // For pure AISP, we just return the document directly
        Ok(ParsedDocument::Pure(document))
    }

    /// Parse markdown document with embedded AISP blocks
    fn parse_markdown_with_aisp(
        &self,
        content: &str,
        analysis: FormatAnalysis,
        detection_time_us: u64,
    ) -> AispResult<ParsedDocument> {
        let extraction_start = std::time::Instant::now();

        // Extract AISP blocks from markdown
        let (extracted_blocks, extraction_context) =
            self.extractor.extract_with_context(content)?;

        let extraction_time_us = extraction_start.elapsed().as_micros() as u64;

        if extracted_blocks.is_empty() {
            return Err(AispError::ParseError {
                message: "No AISP blocks found in markdown document".to_string(),
                line: 0,
                column: 0,
            });
        }

        // Parse each extracted AISP block
        let parsing_start = std::time::Instant::now();
        let mut aisp_documents = Vec::new();
        let mut parsing_warnings = Vec::new();

        for extracted_block in extracted_blocks {
            match self.parse_extracted_block(&extracted_block) {
                Ok(mut parsed_doc) => {
                    // Add extraction context to warnings if needed
                    if extracted_block.content.trim().is_empty() {
                        parsed_doc
                            .warnings
                            .push("Empty AISP block detected".to_string());
                    }
                    aisp_documents.push(parsed_doc);
                }
                Err(e) => {
                    parsing_warnings.push(format!(
                        "Failed to parse AISP block at line {}: {}",
                        extracted_block.start_line + 1,
                        e
                    ));
                }
            }
        }

        let parsing_time_us = parsing_start.elapsed().as_micros() as u64;

        if aisp_documents.is_empty() {
            return Err(AispError::ParseError {
                message: "No valid AISP documents could be parsed from extracted blocks"
                    .to_string(),
                line: 0,
                column: 0,
            });
        }

        // Build metadata
        let metrics = ParsingMetrics {
            detection_time_us,
            extraction_time_us,
            parsing_time_us,
            document_size_bytes: content.len(),
            blocks_processed: aisp_documents.len(),
        };

        let metadata = MixedFormatMetadata {
            source_format: analysis.format.clone(),
            analysis,
            extraction_context,
            warnings: parsing_warnings,
            metrics,
        };

        Ok(ParsedDocument::Mixed {
            prose_content: metadata
                .extraction_context
                .cleaned_markdown
                .as_ref()
                .unwrap_or(&String::new())
                .clone(),
            aisp_documents,
            metadata,
        })
    }

    /// Parse mixed format document (basic implementation)
    fn parse_mixed_format(
        &self,
        content: &str,
        analysis: FormatAnalysis,
        detection_time_us: u64,
    ) -> AispResult<ParsedDocument> {
        // For now, treat mixed format like markdown with AISP (tracked in #14)
        // This could be enhanced to handle more complex mixed formats
        self.parse_markdown_with_aisp(content, analysis, detection_time_us)
    }

    /// Parse an individual extracted AISP block
    fn parse_extracted_block(
        &self,
        extracted_block: &ExtractedAispBlock,
    ) -> AispResult<ParsedAispDocument> {
        let parser = RobustAispParser::new();
        let parse_result = parser.parse(&extracted_block.content);
        let document = parse_result.document.ok_or_else(|| AispError::ParseError {
            message: "Failed to parse extracted block".to_string(),
            line: 0,
            column: 0,
        })?;

        // Collect any warnings from the parser
        let warnings = parse_result
            .warnings
            .iter()
            .map(|w| w.message.clone())
            .collect();

        Ok(ParsedAispDocument {
            document,
            extraction_info: extracted_block.clone(),
            warnings,
        })
    }
}

impl ParsedDocument {
    /// Get all AISP documents from this parsed result
    pub fn get_aisp_documents(&self) -> Vec<&AispDocument> {
        match self {
            ParsedDocument::Pure(doc) => vec![doc],
            ParsedDocument::Mixed { aisp_documents, .. } => {
                aisp_documents.iter().map(|pd| &pd.document).collect()
            }
        }
    }

    /// Get the primary AISP document (first one for mixed format)
    pub fn get_primary_document(&self) -> Option<&AispDocument> {
        match self {
            ParsedDocument::Pure(doc) => Some(doc),
            ParsedDocument::Mixed { aisp_documents, .. } => {
                aisp_documents.first().map(|pd| &pd.document)
            }
        }
    }

    /// Check if this is a pure AISP document
    pub fn is_pure_aisp(&self) -> bool {
        matches!(self, ParsedDocument::Pure(_))
    }

    /// Get all parsing warnings from the result
    pub fn get_all_warnings(&self) -> Vec<&str> {
        match self {
            ParsedDocument::Pure(_) => vec![], // Pure parser warnings handled separately
            ParsedDocument::Mixed {
                aisp_documents,
                metadata,
                ..
            } => {
                let mut warnings = Vec::new();

                // Add document-level warnings
                for doc in aisp_documents {
                    warnings.extend(doc.warnings.iter().map(|s| s.as_str()));
                }

                // Add metadata warnings
                warnings.extend(metadata.warnings.iter().map(|s| s.as_str()));

                warnings
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pure_aisp() {
        let content = r#"𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{
  domain≜"test"
}
⟦Σ:Types⟧{
  Natural≜ℕ
}
⟦Γ:Rules⟧{
}
⟦Λ:Funcs⟧{
}
⟦Ε⟧⟨δ≜0.75⟩"#;

        let parser = MultiFormatParser::new();
        let result = parser.parse(content).unwrap();

        assert!(result.is_pure_aisp());
        if let ParsedDocument::Pure(doc) = result {
            assert_eq!(doc.header.name, "test");
        }
    }

    #[test]
    fn test_parse_markdown_with_aisp() {
        let content = r#"# AISP Documentation

This is some markdown content.

```aisp
𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{
  domain≜"test"
}
⟦Σ:Types⟧{
  Natural≜ℕ
}
⟦Γ:Rules⟧{
}
⟦Λ:Funcs⟧{
}
⟦Ε⟧⟨δ≜0.75⟩
```

More markdown content here."#;

        let parser = MultiFormatParser::new();
        let result = parser.parse(content).unwrap();

        assert!(!result.is_pure_aisp());
        if let ParsedDocument::Mixed {
            aisp_documents,
            prose_content,
            metadata,
            ..
        } = result
        {
            assert_eq!(aisp_documents.len(), 1);
            assert_eq!(aisp_documents[0].document.header.name, "test");
            assert!(prose_content.contains("# AISP Documentation"));
            assert!(!prose_content.contains("𝔸5.1.test"));
            assert_eq!(metadata.analysis.aisp_block_count, 1);
        }
    }

    #[test]
    fn test_parse_multiple_aisp_blocks() {
        let content = r#"# Documentation

```aisp
𝔸5.1.first@2026-01-30
⟦Ω:Meta⟧{domain≜"first"}
⟦Σ:Types⟧{Natural≜ℕ}
⟦Γ:Rules⟧{}
⟦Λ:Funcs⟧{}
⟦Ε⟧⟨δ≜0.75⟩
```

## Second Example

```aisp
𝔸5.1.second@2026-01-30
⟦Ω:Meta⟧{domain≜"second"}
⟦Σ:Types⟧{Natural≜ℕ}
⟦Γ:Rules⟧{}
⟦Λ:Funcs⟧{}
⟦Ε⟧⟨δ≜0.75⟩
```"#;

        let parser = MultiFormatParser::new();
        let result = parser.parse(content).unwrap();

        if let ParsedDocument::Mixed { aisp_documents, .. } = result {
            assert_eq!(aisp_documents.len(), 2);
            assert_eq!(aisp_documents[0].document.header.name, "first");
            assert_eq!(aisp_documents[1].document.header.name, "second");
        }
    }

    #[test]
    fn test_parse_unknown_format() {
        let content = "This is just plain text with no AISP elements";

        let parser = MultiFormatParser::new();
        let result = parser.parse(content);

        assert!(result.is_err());
        if let Err(AispError::UnsupportedFormat { format }) = result {
            assert!(format.contains("Unknown"));
        }
    }

    #[test]
    fn test_parse_markdown_no_aisp_blocks() {
        let content = r#"# Documentation

This is markdown but has no AISP blocks."#;

        let parser = MultiFormatParser::new();
        let result = parser.parse(content);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_aisp_block() {
        let content = r#"# Documentation

```aisp
This is not valid AISP content
```"#;

        let parser = MultiFormatParser::with_options(false, true); // Enable validation
        let result = parser.parse(content);

        assert!(result.is_err());
    }

    #[test]
    fn test_get_aisp_documents() {
        let content = r#"𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{domain≜"test"}
⟦Σ:Types⟧{Natural≜ℕ}
⟦Γ:Rules⟧{}
⟦Λ:Funcs⟧{}
⟦Ε⟧⟨δ≜0.75⟩"#;

        let parser = MultiFormatParser::new();
        let result = parser.parse(content).unwrap();
        let documents = result.get_aisp_documents();

        assert_eq!(documents.len(), 1);
        assert_eq!(documents[0].header.name, "test");
    }

    #[test]
    fn test_get_primary_document() {
        let content = r#"# Documentation

```aisp
𝔸5.1.test@2026-01-30
⟦Ω:Meta⟧{domain≜"test"}
⟦Σ:Types⟧{Natural≜ℕ}
⟦Γ:Rules⟧{}
⟦Λ:Funcs⟧{}
⟦Ε⟧⟨δ≜0.75⟩
```"#;

        let parser = MultiFormatParser::new();
        let result = parser.parse(content).unwrap();
        let primary = result.get_primary_document();

        assert!(primary.is_some());
        assert_eq!(primary.unwrap().header.name, "test");
    }
}
