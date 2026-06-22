// Semantic analysis module for AISP documents
// Includes deep verification architecture for enterprise security

pub mod behavioral_verifier;
pub mod cross_validator;
pub mod deep_verifier;
pub mod pipeline;
pub mod verification_pipeline;

pub use deep_verifier::{
    CoverageMetrics, DeceptionDetector, DeepSemanticVerifier, DeepVerificationResult,
    DependencyGraphAnalyzer, LogicConsistencyChecker, MathematicalCorrectnessEngine,
    PerformanceMetrics, SecurityAssessment, ThreatLevel, TypeSystemAnalyzer, VerificationDetails,
};

pub use behavioral_verifier::{
    BehavioralVerificationResult, BehavioralVerifier, SafeExecutionSandbox,
};

pub use cross_validator::{
    ConflictResolver, ConsistencyAnalyzer, CrossValidationChecker, CrossValidationResult,
    FinalSecurityAssessment, VerificationOrchestrator,
};

pub use verification_pipeline::{
    AttackResistanceRating, ComplianceAuditor, ComprehensiveVerificationResult,
    MultiLayerVerificationPipeline, PerformanceMonitor, PipelineOrchestrator, SecurityEnforcer,
};

// Compatibility types for legacy code
pub use deep_verifier::DeepVerificationResult as SemanticAnalysisResult;
pub type SemanticAnalysis = deep_verifier::DeepVerificationResult;

// Quality tier enum for compatibility
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum QualityTier {
    Reject,
    Bronze,
    Silver,
    Gold,
    Platinum,
}

// Semantic analyzer compatibility wrapper
pub struct SemanticAnalyzer {
    verifier: DeepSemanticVerifier,
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            verifier: DeepSemanticVerifier::new(),
        }
    }

    pub fn analyze(
        &mut self,
        document: &crate::ast::canonical::CanonicalAispDocument,
    ) -> crate::error::AispResult<DeepVerificationResult> {
        self.verifier.verify_document(document)
    }
}

// Legacy compatibility adapter for old ValidationResult usage
impl DeepVerificationResult {
    /// Overall verification-confidence gate. NOTE: this is a *different* signal
    /// from [`Self::tier`]. `valid()` reflects verification confidence
    /// (`overall_confidence`), whereas the quality tier is a function of δ
    /// (`semantic_score`) per the AISP spec, so they can legitimately diverge
    /// (a high-confidence result with low δ is `valid() == true` but a low
    /// tier). The authoritative document gate, `ValidationResult::is_valid`,
    /// additionally requires `tier != Reject`.
    pub fn valid(&self) -> bool {
        self.overall_confidence > 0.8
    }

    /// Quality tier (◊) per the AISP 5.1 spec — a function of δ (delta).
    pub fn tier(&self) -> QualityTier {
        QualityTier::from_delta(self.delta())
    }

    pub fn delta(&self) -> f64 {
        self.semantic_score
    }

    pub fn pure_density(&self) -> f64 {
        self.type_safety_score
    }

    pub fn ambiguity(&self) -> f64 {
        1.0 - self.logic_consistency_score
    }

    pub fn completeness(&self) -> f64 {
        self.mathematical_correctness_score
    }

    pub fn quality_score(&self) -> f64 {
        self.overall_confidence
    }

    pub fn warnings(&self) -> Vec<String> {
        self.recommendations
            .iter()
            .map(|r| r.recommendation.clone())
            .collect()
    }

    pub fn errors(&self) -> Vec<String> {
        self.verification_details
            .failed_verifications
            .iter()
            .map(|f| format!("{}: {}", f.component, f.reason))
            .collect()
    }

    pub fn to_result(&self) -> Self {
        self.clone()
    }

    // Additional compatibility fields
    pub fn type_analysis(&self) -> MockTypeAnalysis {
        MockTypeAnalysis {
            undefined_types: Vec::new(),
        }
    }

    pub fn relational_analysis(&self) -> Option<MockRelationalAnalysis> {
        Some(MockRelationalAnalysis {
            consistency_score: self.logic_consistency_score,
            constraint_analysis: MockConstraintAnalysis {
                constraints: vec![
                    "type_consistency".to_string(),
                    "logical_constraints".to_string(),
                ],
                satisfied: vec!["type_consistency".to_string()],
            },
            conflict_analysis: MockConflictAnalysis { conflicts: vec![] },
        })
    }

    pub fn temporal_analysis(&self) -> Option<MockTemporalAnalysis> {
        Some(MockTemporalAnalysis {
            consistency_score: self.logic_consistency_score,
            formula_analysis: MockFormulaAnalysis {
                formulas: vec![
                    "temporal_formula_1".to_string(),
                    "temporal_formula_2".to_string(),
                ],
            },
            pattern_analysis: MockPatternAnalysis {
                patterns: vec!["pattern_1".to_string()],
            },
        })
    }

    pub fn symbol_stats(&self) -> MockSymbolStats {
        MockSymbolStats {
            category_counts: std::collections::HashMap::new(),
        }
    }

    pub fn block_score(&self) -> f64 {
        (self.semantic_score
            + self.type_safety_score
            + self.logic_consistency_score
            + self.mathematical_correctness_score)
            / 4.0
    }
}

// Mock types for compatibility
#[derive(Debug, Clone)]
pub struct MockTypeAnalysis {
    pub undefined_types: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockRelationalAnalysis {
    pub consistency_score: f64,
    pub constraint_analysis: MockConstraintAnalysis,
    pub conflict_analysis: MockConflictAnalysis,
}

#[derive(Debug, Clone)]
pub struct MockTemporalAnalysis {
    pub consistency_score: f64,
    pub formula_analysis: MockFormulaAnalysis,
    pub pattern_analysis: MockPatternAnalysis,
}

#[derive(Debug, Clone)]
pub struct MockConstraintAnalysis {
    pub constraints: Vec<String>,
    pub satisfied: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockConflictAnalysis {
    pub conflicts: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockFormulaAnalysis {
    pub formulas: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockPatternAnalysis {
    pub patterns: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MockSymbolStats {
    pub category_counts: std::collections::HashMap<String, usize>,
}

impl QualityTier {
    /// Map a δ (delta) score to its AISP 5.1 spec quality tier (◊), per
    /// AI_GUIDE.md §Tiers — `⌈⌉≜λd.[≥¾↦◊⁺⁺,≥⅗↦◊⁺,≥⅖↦◊,≥⅕↦◊⁻,_↦⊘]`:
    /// ◊⁺⁺ Platinum δ≥0.75; ◊⁺ Gold δ≥0.60; ◊ Silver δ≥0.40; ◊⁻ Bronze δ≥0.20;
    /// ⊘ Reject δ<0.20.
    pub fn from_delta(delta: f64) -> QualityTier {
        match delta {
            d if d >= 0.75 => QualityTier::Platinum,
            d if d >= 0.60 => QualityTier::Gold,
            d if d >= 0.40 => QualityTier::Silver,
            d if d >= 0.20 => QualityTier::Bronze,
            _ => QualityTier::Reject,
        }
    }

    pub fn symbol(&self) -> &str {
        match self {
            QualityTier::Reject => "⊘",
            QualityTier::Bronze => "⚫",
            QualityTier::Silver => "⚪",
            QualityTier::Gold => "🟡",
            QualityTier::Platinum => "⭐",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            QualityTier::Reject => "Reject",
            QualityTier::Bronze => "Bronze",
            QualityTier::Silver => "Silver",
            QualityTier::Gold => "Gold",
            QualityTier::Platinum => "Platinum",
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            QualityTier::Reject => 0,
            QualityTier::Bronze => 1,
            QualityTier::Silver => 2,
            QualityTier::Gold => 3,
            QualityTier::Platinum => 4,
        }
    }
}

#[cfg(test)]
mod tier_tests {
    use super::QualityTier;

    /// Lock in the AISP 5.1 spec δ-ladder (◊) directly on the mapping function,
    /// so the tier semantics stay covered even while end-to-end δ computation is
    /// still being made quality-graded (#18).
    #[test]
    fn from_delta_matches_spec_ladder() {
        // Band interiors.
        assert_eq!(QualityTier::from_delta(0.98), QualityTier::Platinum);
        assert_eq!(QualityTier::from_delta(0.70), QualityTier::Gold);
        assert_eq!(QualityTier::from_delta(0.50), QualityTier::Silver);
        assert_eq!(QualityTier::from_delta(0.30), QualityTier::Bronze);
        assert_eq!(QualityTier::from_delta(0.10), QualityTier::Reject);

        // Inclusive lower boundaries.
        assert_eq!(QualityTier::from_delta(0.75), QualityTier::Platinum);
        assert_eq!(QualityTier::from_delta(0.60), QualityTier::Gold);
        assert_eq!(QualityTier::from_delta(0.40), QualityTier::Silver);
        assert_eq!(QualityTier::from_delta(0.20), QualityTier::Bronze);

        // Just below each boundary drops one tier.
        assert_eq!(QualityTier::from_delta(0.7499), QualityTier::Gold);
        assert_eq!(QualityTier::from_delta(0.5999), QualityTier::Silver);
        assert_eq!(QualityTier::from_delta(0.3999), QualityTier::Bronze);
        assert_eq!(QualityTier::from_delta(0.1999), QualityTier::Reject);

        // Edge values.
        assert_eq!(QualityTier::from_delta(0.0), QualityTier::Reject);
        assert_eq!(QualityTier::from_delta(1.0), QualityTier::Platinum);
    }
}
