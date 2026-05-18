//! Format Detection for Multi-Format Parser Architecture
//!
//! This module implements ADR-023: Multi-Format Parser Architecture
//! by providing fast document format identification to enable
//! format-aware parser selection.

// use crate::error::AispResult; // Currently unused

/// Supported document formats for parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentFormat {
    /// Pure AISP document starting with 𝔸5.1.name@date
    PureAisp,

    /// Markdown document containing AISP code blocks (```aisp)
    MarkdownWithAisp,

    /// Complex mixed format with multiple embedded formats
    MixedFormat,

    /// Unrecognized format that cannot be parsed
    Unknown,
}

/// Fast document format detector using O(1) scanning
pub struct FormatDetector;

impl FormatDetector {
    /// Detect document format by analyzing content structure
    ///
    /// This function uses efficient heuristics to identify document types:
    /// - Pure AISP: Starts with 𝔸 (Unicode MATHEMATICAL DOUBLE-STRUCK A)
    /// - Markdown with AISP: Has # headers and ```aisp code blocks
    /// - Mixed format: Contains AISP elements but doesn't fit pure patterns
    /// - Unknown: No recognizable AISP elements
    pub fn detect(content: &str) -> DocumentFormat {
        let trimmed = content.trim();

        // Fast path: empty content
        if trimmed.is_empty() {
            return DocumentFormat::Unknown;
        }

        // Pure AISP detection: starts with 𝔸
        if trimmed.starts_with('𝔸') {
            return DocumentFormat::PureAisp;
        }

        // Check for AISP elements in content
        let has_aisp_blocks = content.contains("```aisp");
        let has_aisp_delimiters = content.contains("⟦") && content.contains("⟧");
        let has_markdown_headers = trimmed.starts_with('#') || content.contains("\n#");
        let has_aisp_header = content.contains('𝔸');

        // Markdown with embedded AISP
        if has_markdown_headers && has_aisp_blocks {
            return DocumentFormat::MarkdownWithAisp;
        }

        // Mixed format: contains AISP elements but not pure format
        if has_aisp_blocks || has_aisp_delimiters || has_aisp_header {
            return DocumentFormat::MixedFormat;
        }

        // No AISP elements found
        DocumentFormat::Unknown
    }

    /// Validate that a format can be parsed by our system
    pub fn is_parseable(format: &DocumentFormat) -> bool {
        match format {
            DocumentFormat::PureAisp => true,
            DocumentFormat::MarkdownWithAisp => true,
            DocumentFormat::MixedFormat => true, // Basic support
            DocumentFormat::Unknown => false,
        }
    }

    /// Get a human-readable description of the format
    pub fn format_description(format: &DocumentFormat) -> &'static str {
        match format {
            DocumentFormat::PureAisp => "Pure AISP document",
            DocumentFormat::MarkdownWithAisp => "Markdown document with embedded AISP blocks",
            DocumentFormat::MixedFormat => "Mixed format document with AISP elements",
            DocumentFormat::Unknown => "Unknown or unsupported format",
        }
    }
}

/// Format detection result with additional metadata
#[derive(Debug, Clone)]
pub struct FormatAnalysis {
    pub format: DocumentFormat,
    pub confidence: f32,
    pub aisp_block_count: usize,
    pub has_unicode_math: bool,
    pub estimated_complexity: FormatComplexity,
}

/// Complexity estimation for parsing strategy selection
#[derive(Debug, Clone, PartialEq)]
pub enum FormatComplexity {
    Simple,   // Pure AISP or simple markdown
    Moderate, // Markdown with few AISP blocks
    Complex,  // Mixed format or many embedded blocks
}

impl FormatDetector {
    /// Perform detailed format analysis for parsing strategy
    pub fn analyze(content: &str) -> FormatAnalysis {
        let format = Self::detect(content);
        let aisp_block_count = content.matches("```aisp").count();
        let has_unicode_math = content.chars().any(|c| {
            matches!(
                c,
                '⟦' | '⟧' | '≜' | '∀' | '∃' | 'λ' | '→' | 'ℕ' | 'ℝ' | 'ℤ' | '𝔸' | '◊'
            )
        });

        let confidence = match format {
            DocumentFormat::PureAisp if content.trim().starts_with('𝔸') => 1.0,
            DocumentFormat::MarkdownWithAisp if aisp_block_count > 0 => 0.9,
            DocumentFormat::MixedFormat if has_unicode_math => 0.7,
            DocumentFormat::Unknown => 0.0,
            _ => 0.5,
        };

        let estimated_complexity = match (&format, aisp_block_count) {
            (DocumentFormat::PureAisp, _) => FormatComplexity::Simple,
            (DocumentFormat::MarkdownWithAisp, 0..=3) => FormatComplexity::Moderate,
            (DocumentFormat::MarkdownWithAisp, _) => FormatComplexity::Complex,
            (DocumentFormat::MixedFormat, _) => FormatComplexity::Complex,
            (DocumentFormat::Unknown, _) => FormatComplexity::Simple,
        };

        FormatAnalysis {
            format,
            confidence,
            aisp_block_count,
            has_unicode_math,
            estimated_complexity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pure_aisp_detection() {
        let content = "𝔸5.1.test@2026-01-30\n⟦Ω:Meta⟧{domain≜\"test\"}";
        assert_eq!(FormatDetector::detect(content), DocumentFormat::PureAisp);
    }

    #[test]
    fn test_markdown_with_aisp_detection() {
        let content = "# AISP Documentation\n\n```aisp\n𝔸5.1.test@2026-01-30\n⟦Ω:Meta⟧{}\n```";
        assert_eq!(
            FormatDetector::detect(content),
            DocumentFormat::MarkdownWithAisp
        );
    }

    #[test]
    fn test_mixed_format_detection() {
        let content = "Some text ⟦Ω:Meta⟧{} more text";
        assert_eq!(FormatDetector::detect(content), DocumentFormat::MixedFormat);
    }

    #[test]
    fn test_unknown_format_detection() {
        let content = "Just plain text with no AISP elements";
        assert_eq!(FormatDetector::detect(content), DocumentFormat::Unknown);
    }

    #[test]
    fn test_empty_content() {
        assert_eq!(FormatDetector::detect(""), DocumentFormat::Unknown);
        assert_eq!(FormatDetector::detect("   "), DocumentFormat::Unknown);
    }

    #[test]
    fn test_format_parseable() {
        assert!(FormatDetector::is_parseable(&DocumentFormat::PureAisp));
        assert!(FormatDetector::is_parseable(
            &DocumentFormat::MarkdownWithAisp
        ));
        assert!(FormatDetector::is_parseable(&DocumentFormat::MixedFormat));
        assert!(!FormatDetector::is_parseable(&DocumentFormat::Unknown));
    }

    #[test]
    fn test_format_analysis() {
        let content = "# Header\n```aisp\n𝔸5.1.test@2026-01-30\n⟦Ω:Meta⟧{}\n```";
        let analysis = FormatDetector::analyze(content);

        assert_eq!(analysis.format, DocumentFormat::MarkdownWithAisp);
        assert_eq!(analysis.aisp_block_count, 1);
        assert!(analysis.has_unicode_math);
        assert_eq!(analysis.estimated_complexity, FormatComplexity::Moderate);
        assert!(analysis.confidence > 0.8);
    }

    #[test]
    fn test_complex_markdown_analysis() {
        let content = "# Header\n```aisp\nblock1\n```\n## Section\n```aisp\nblock2\n```\n```aisp\nblock3\n```\n```aisp\nblock4\n```";
        let analysis = FormatDetector::analyze(content);

        assert_eq!(analysis.format, DocumentFormat::MarkdownWithAisp);
        assert_eq!(analysis.aisp_block_count, 4);
        assert_eq!(analysis.estimated_complexity, FormatComplexity::Complex);
    }

    #[test]
    fn test_format_descriptions() {
        assert_eq!(
            FormatDetector::format_description(&DocumentFormat::PureAisp),
            "Pure AISP document"
        );
        assert_eq!(
            FormatDetector::format_description(&DocumentFormat::MarkdownWithAisp),
            "Markdown document with embedded AISP blocks"
        );
        assert_eq!(
            FormatDetector::format_description(&DocumentFormat::MixedFormat),
            "Mixed format document with AISP elements"
        );
        assert_eq!(
            FormatDetector::format_description(&DocumentFormat::Unknown),
            "Unknown or unsupported format"
        );
    }
}
