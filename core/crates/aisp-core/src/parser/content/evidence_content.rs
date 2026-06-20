//! Evidence Content Parser
//!
//! Focused parser for evidence block content following SRP.
//! Handles parsing of δ, φ, τ values and custom metrics.

use crate::error::{AispError, AispResult};
use std::collections::HashMap;

/// SRP-focused parser for evidence block content
pub struct EvidenceContentParser;

impl EvidenceContentParser {
    /// Parse evidence entry from "symbol≜value" format
    pub fn parse_evidence_entry(entry_text: &str) -> Option<EvidenceEntry> {
        if let Some(pos) = entry_text.find('≜') {
            let key = entry_text[..pos].trim();
            let value_text = entry_text[pos + '≜'.len_utf8()..].trim();

            match key {
                "δ" => {
                    if let Ok(delta) = value_text.parse::<f64>() {
                        Some(EvidenceEntry::Delta(delta))
                    } else {
                        None
                    }
                }
                "φ" => {
                    if let Ok(phi) = value_text.parse::<u64>() {
                        Some(EvidenceEntry::Phi(phi))
                    } else {
                        None
                    }
                }
                "τ" => {
                    let tau_value = value_text.trim_matches('"').to_string();
                    Some(EvidenceEntry::Tau(tau_value))
                }
                _ => {
                    // Custom metric
                    if let Ok(metric_value) = value_text.parse::<f64>() {
                        Some(EvidenceEntry::Metric(key.to_string(), metric_value))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    /// Parse multiple evidence entries and return structured data
    pub fn parse_evidence_block(content: &str) -> EvidenceData {
        let mut evidence_data = EvidenceData::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with(";;") {
                continue;
            }

            if let Some(entry) = Self::parse_evidence_entry(line) {
                evidence_data.add_entry(entry);
            }
        }

        evidence_data
    }

    /// Validate delta value (accuracy threshold)
    pub fn validate_delta(delta: f64) -> AispResult<()> {
        if delta < 0.0 {
            return Err(AispError::validation_error(
                "Delta (δ) must be non-negative",
            ));
        }

        if delta > 1.0 {
            return Err(AispError::validation_error(
                "Delta (δ) should typically be ≤ 1.0 for accuracy thresholds",
            ));
        }

        Ok(())
    }

    /// Validate phi value (iteration count or score)
    pub fn validate_phi(phi: u64) -> AispResult<()> {
        if phi == 0 {
            return Err(AispError::validation_error(
                "Phi (φ) should be positive for meaningful metrics",
            ));
        }

        Ok(())
    }

    /// Validate tau value (time or category string)
    pub fn validate_tau(tau: &str) -> AispResult<()> {
        if tau.is_empty() {
            return Err(AispError::validation_error("Tau (τ) cannot be empty"));
        }

        // Check for common time formats or categories
        if Self::is_time_format(tau) || Self::is_category_format(tau) {
            Ok(())
        } else {
            Err(AispError::validation_error(format!(
                "Invalid tau (τ) format: '{}'. Expected time format or category",
                tau
            )))
        }
    }

    /// Check if string represents time format
    fn is_time_format(s: &str) -> bool {
        // Simple time format validation
        s.contains(':')
            || s.contains("ms")
            || s.contains("s")
            || s.contains("min")
            || s.contains("hour")
            || s.contains("day")
    }

    /// Check if string represents valid category
    fn is_category_format(s: &str) -> bool {
        // Categories like "formal", "semantic", "syntax", etc.
        matches!(
            s.to_lowercase().as_str(),
            "formal"
                | "semantic"
                | "syntax"
                | "temporal"
                | "logical"
                | "mathematical"
                | "structural"
                | "behavioral"
                | "security"
                | "performance"
                | "correctness"
                | "completeness"
        )
    }

    /// Extract confidence score from evidence data
    pub fn calculate_confidence(evidence: &EvidenceData) -> f64 {
        let mut confidence = 1.0;

        // Factor in delta (accuracy threshold)
        if let Some(delta) = evidence.delta {
            confidence *= (1.0 - delta).max(0.0);
        }

        // Factor in phi (iteration/score)
        if let Some(phi) = evidence.phi {
            let phi_factor = (phi as f64).min(100.0) / 100.0;
            confidence *= phi_factor;
        }

        // Factor in tau category
        if let Some(tau) = &evidence.tau {
            let category_weight = match tau.to_lowercase().as_str() {
                "formal" => 1.0,
                "semantic" => 0.9,
                "syntax" => 0.7,
                "temporal" => 0.8,
                _ => 0.6,
            };
            confidence *= category_weight;
        }

        confidence.clamp(0.0, 1.0)
    }
}

/// Evidence entry types
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceEntry {
    Delta(f64),
    Phi(u64),
    Tau(String),
    Metric(String, f64),
}

/// Structured evidence data
#[derive(Debug, Clone, Default)]
pub struct EvidenceData {
    pub delta: Option<f64>,
    pub phi: Option<u64>,
    pub tau: Option<String>,
    pub metrics: HashMap<String, f64>,
    pub raw_entries: Vec<String>,
}

impl EvidenceData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_entry(&mut self, entry: EvidenceEntry) {
        match entry {
            EvidenceEntry::Delta(value) => self.delta = Some(value),
            EvidenceEntry::Phi(value) => self.phi = Some(value),
            EvidenceEntry::Tau(value) => self.tau = Some(value),
            EvidenceEntry::Metric(name, value) => {
                self.metrics.insert(name, value);
            }
        }
    }

    pub fn is_valid(&self) -> bool {
        // Evidence block should have at least one meaningful value
        self.delta.is_some() || self.phi.is_some() || self.tau.is_some() || !self.metrics.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_delta_entry() {
        let entry = EvidenceContentParser::parse_evidence_entry("δ≜0.001");
        assert_eq!(entry, Some(EvidenceEntry::Delta(0.001)));
    }

    #[test]
    fn test_parse_phi_entry() {
        let entry = EvidenceContentParser::parse_evidence_entry("φ≜42");
        assert_eq!(entry, Some(EvidenceEntry::Phi(42)));
    }

    #[test]
    fn test_parse_tau_entry() {
        let entry = EvidenceContentParser::parse_evidence_entry("τ≜formal");
        assert_eq!(entry, Some(EvidenceEntry::Tau("formal".to_string())));
    }

    #[test]
    fn test_parse_metric_entry() {
        let entry = EvidenceContentParser::parse_evidence_entry("accuracy≜0.95");
        assert_eq!(
            entry,
            Some(EvidenceEntry::Metric("accuracy".to_string(), 0.95))
        );
    }

    #[test]
    fn test_parse_evidence_block() {
        let content = r#"
        δ≜0.001
        φ≜98
        τ≜formal
        accuracy≜0.95
        // Comment
        precision≜0.92
        "#;

        let evidence = EvidenceContentParser::parse_evidence_block(content);
        assert_eq!(evidence.delta, Some(0.001));
        assert_eq!(evidence.phi, Some(98));
        assert_eq!(evidence.tau, Some("formal".to_string()));
        assert_eq!(evidence.metrics.len(), 2);
        assert!(evidence.metrics.contains_key("accuracy"));
        assert!(evidence.metrics.contains_key("precision"));
    }

    #[test]
    fn test_validate_delta() {
        assert!(EvidenceContentParser::validate_delta(0.5).is_ok());
        assert!(EvidenceContentParser::validate_delta(0.0).is_ok());
        assert!(EvidenceContentParser::validate_delta(1.0).is_ok());
        assert!(EvidenceContentParser::validate_delta(-0.1).is_err());
    }

    #[test]
    fn test_validate_phi() {
        assert!(EvidenceContentParser::validate_phi(42).is_ok());
        assert!(EvidenceContentParser::validate_phi(1).is_ok());
        assert!(EvidenceContentParser::validate_phi(0).is_err());
    }

    #[test]
    fn test_validate_tau() {
        assert!(EvidenceContentParser::validate_tau("formal").is_ok());
        assert!(EvidenceContentParser::validate_tau("semantic").is_ok());
        assert!(EvidenceContentParser::validate_tau("10:30").is_ok());
        assert!(EvidenceContentParser::validate_tau("100ms").is_ok());
        assert!(EvidenceContentParser::validate_tau("").is_err());
    }

    #[test]
    fn test_calculate_confidence() {
        let mut evidence = EvidenceData::new();
        evidence.delta = Some(0.001); // High accuracy
        evidence.phi = Some(95); // High score
        evidence.tau = Some("formal".to_string()); // Highest category

        let confidence = EvidenceContentParser::calculate_confidence(&evidence);
        assert!(confidence > 0.9); // Should be high confidence

        let mut low_evidence = EvidenceData::new();
        low_evidence.delta = Some(0.1); // Lower accuracy
        low_evidence.tau = Some("syntax".to_string()); // Lower category

        let low_confidence = EvidenceContentParser::calculate_confidence(&low_evidence);
        assert!(low_confidence < confidence); // Should be lower
    }

    #[test]
    fn test_evidence_data_validity() {
        let mut evidence = EvidenceData::new();
        assert!(!evidence.is_valid()); // Empty evidence is invalid

        evidence.delta = Some(0.001);
        assert!(evidence.is_valid()); // Has delta, now valid

        let mut metric_evidence = EvidenceData::new();
        metric_evidence.metrics.insert("accuracy".to_string(), 0.95);
        assert!(metric_evidence.is_valid()); // Has custom metric, valid
    }
}
