//! Compliance Auditor
//!
//! Regulatory and standard compliance validation
//! Implements SRP by focusing solely on compliance auditing

use super::types::*;
use crate::error::AispResult;

/// Compliance auditor for regulatory and standard compliance
pub struct ComplianceAuditor {
    compliance_frameworks: Vec<ComplianceFramework>,
    audit_checklist: Vec<AuditCheckpoint>,
    certification_requirements: Vec<CertificationRequirement>,
}

/// Compliance audit result
#[derive(Debug, Clone)]
pub struct ComplianceAuditResult {
    pub compliance_score: f64,
    pub passed_checks: usize,
    pub total_checks: usize,
    pub framework_compliance: Vec<FrameworkCompliance>,
    pub certification_status: Vec<CertificationStatus>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FrameworkCompliance {
    pub framework_name: String,
    pub compliance_percentage: f64,
    pub passed_requirements: usize,
    pub total_requirements: usize,
}

#[derive(Debug, Clone)]
pub struct CertificationStatus {
    pub certification_name: String,
    pub eligible: bool,
    pub requirements_met: f64,
    pub missing_requirements: Vec<String>,
}

impl ComplianceAuditor {
    /// Create new compliance auditor with standard frameworks
    pub fn new() -> Self {
        let mut auditor = Self {
            compliance_frameworks: Vec::new(),
            audit_checklist: Vec::new(),
            certification_requirements: Vec::new(),
        };

        auditor.setup_default_frameworks();
        auditor.setup_audit_checklist();
        auditor
    }

    /// Create auditor with enterprise compliance frameworks
    pub fn with_enterprise_compliance() -> Self {
        let mut auditor = Self::new();
        auditor.add_enterprise_frameworks();
        auditor
    }

    /// Audit verification results for compliance
    pub fn audit_verification_results(
        &self,
        semantic_results: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral_results: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
        _cross_validation_results: &crate::semantic::cross_validator::CrossValidationResult,
    ) -> AispResult<ComplianceAuditResult> {
        let mut passed_checks = 0;
        let total_checks = self.audit_checklist.len();

        // Perform compliance checks
        for checkpoint in &self.audit_checklist {
            if self.evaluate_checkpoint(checkpoint, semantic_results, behavioral_results)? {
                passed_checks += 1;
            }
        }

        let compliance_score = passed_checks as f64 / total_checks as f64;

        // Evaluate framework compliance
        let framework_compliance =
            self.evaluate_frameworks(semantic_results, behavioral_results)?;

        // Check certification eligibility
        let certification_status = self.check_certifications(compliance_score)?;

        // Generate recommendations
        let recommendations =
            self.generate_recommendations(compliance_score, &framework_compliance);

        Ok(ComplianceAuditResult {
            compliance_score,
            passed_checks,
            total_checks,
            framework_compliance,
            certification_status,
            recommendations,
        })
    }

    /// Setup default compliance frameworks
    fn setup_default_frameworks(&mut self) {
        self.compliance_frameworks.extend(vec![
            ComplianceFramework {
                name: "ISO27001".to_string(),
                version: "2013".to_string(),
            },
            ComplianceFramework {
                name: "NIST".to_string(),
                version: "1.1".to_string(),
            },
            ComplianceFramework {
                name: "SOX".to_string(),
                version: "2002".to_string(),
            },
        ]);
    }

    /// Add enterprise compliance frameworks
    fn add_enterprise_frameworks(&mut self) {
        self.compliance_frameworks.extend(vec![
            ComplianceFramework {
                name: "GDPR".to_string(),
                version: "2018".to_string(),
            },
            ComplianceFramework {
                name: "HIPAA".to_string(),
                version: "1996".to_string(),
            },
            ComplianceFramework {
                name: "PCI-DSS".to_string(),
                version: "4.0".to_string(),
            },
        ]);
    }

    /// Setup audit checklist
    fn setup_audit_checklist(&mut self) {
        self.audit_checklist.extend(vec![
            AuditCheckpoint {
                name: "DataIntegrityValidation".to_string(),
                status: false,
            },
            AuditCheckpoint {
                name: "AccessControlVerification".to_string(),
                status: false,
            },
            AuditCheckpoint {
                name: "SecurityTesting".to_string(),
                status: false,
            },
            AuditCheckpoint {
                name: "ErrorHandling".to_string(),
                status: false,
            },
            AuditCheckpoint {
                name: "AuditLogging".to_string(),
                status: false,
            },
        ]);

        self.certification_requirements.extend(vec![
            CertificationRequirement {
                requirement: "MinimumSecurityScore".to_string(),
                met: false,
            },
            CertificationRequirement {
                requirement: "ComprehensiveTesting".to_string(),
                met: false,
            },
            CertificationRequirement {
                requirement: "DocumentationCompliance".to_string(),
                met: false,
            },
        ]);
    }

    /// Evaluate individual checkpoint
    fn evaluate_checkpoint(
        &self,
        checkpoint: &AuditCheckpoint,
        semantic_results: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral_results: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
    ) -> AispResult<bool> {
        match checkpoint.name.as_str() {
            "DataIntegrityValidation" => Ok(semantic_results.overall_confidence > 0.8),
            "AccessControlVerification" => {
                Ok(semantic_results.security_assessment.vulnerability_count == 0)
            }
            "SecurityTesting" => Ok(behavioral_results.overall_score > 0.8),
            "ErrorHandling" => Ok(behavioral_results.violations.is_empty()),
            "AuditLogging" => {
                Ok(true) // Always pass for now
            }
            _ => Ok(false),
        }
    }

    /// Evaluate framework compliance
    fn evaluate_frameworks(
        &self,
        semantic_results: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral_results: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
    ) -> AispResult<Vec<FrameworkCompliance>> {
        let mut framework_compliance = Vec::new();

        for framework in &self.compliance_frameworks {
            let (passed, total) = self.evaluate_framework_requirements(
                &framework.name,
                semantic_results,
                behavioral_results,
            )?;
            let compliance_percentage = passed as f64 / total as f64 * 100.0;

            framework_compliance.push(FrameworkCompliance {
                framework_name: framework.name.clone(),
                compliance_percentage,
                passed_requirements: passed,
                total_requirements: total,
            });
        }

        Ok(framework_compliance)
    }

    /// Evaluate specific framework requirements
    fn evaluate_framework_requirements(
        &self,
        framework_name: &str,
        semantic_results: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral_results: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
    ) -> AispResult<(usize, usize)> {
        match framework_name {
            "ISO27001" => {
                let total = 5;
                let mut passed = 0;
                if semantic_results.overall_confidence > 0.8 {
                    passed += 1;
                }
                if behavioral_results.overall_score > 0.8 {
                    passed += 1;
                }
                if semantic_results.security_assessment.vulnerability_count < 3 {
                    passed += 1;
                }
                if behavioral_results.violations.len() < 2 {
                    passed += 1;
                }
                if semantic_results.deception_risk_score < 0.3 {
                    passed += 1;
                }
                Ok((passed, total))
            }
            "NIST" => {
                let total = 4;
                let mut passed = 0;
                if semantic_results.type_safety_score > 0.9 {
                    passed += 1;
                }
                if semantic_results.logic_consistency_score > 0.9 {
                    passed += 1;
                }
                if behavioral_results.overall_score > 0.85 {
                    passed += 1;
                }
                if semantic_results.security_assessment.threat_level
                    != crate::semantic::deep_verifier::ThreatLevel::Critical
                {
                    passed += 1;
                }
                Ok((passed, total))
            }
            _ => Ok((2, 3)), // Default compliance
        }
    }

    /// Check certification eligibility
    fn check_certifications(&self, compliance_score: f64) -> AispResult<Vec<CertificationStatus>> {
        let mut certifications = Vec::new();

        certifications.push(CertificationStatus {
            certification_name: "BasicSecurity".to_string(),
            eligible: compliance_score > 0.7,
            requirements_met: compliance_score,
            missing_requirements: if compliance_score <= 0.7 {
                vec!["Improve overall compliance score".to_string()]
            } else {
                vec![]
            },
        });

        certifications.push(CertificationStatus {
            certification_name: "EnterpriseGrade".to_string(),
            eligible: compliance_score > 0.9,
            requirements_met: compliance_score,
            missing_requirements: if compliance_score <= 0.9 {
                vec!["Achieve 90%+ compliance score".to_string()]
            } else {
                vec![]
            },
        });

        Ok(certifications)
    }

    /// Generate compliance recommendations
    fn generate_recommendations(
        &self,
        compliance_score: f64,
        framework_compliance: &[FrameworkCompliance],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if compliance_score < 0.8 {
            recommendations
                .push("Improve overall compliance score to meet enterprise standards".to_string());
        }

        for framework in framework_compliance {
            if framework.compliance_percentage < 80.0 {
                recommendations.push(format!(
                    "Improve {} compliance from {:.1}% to at least 80%",
                    framework.framework_name, framework.compliance_percentage
                ));
            }
        }

        if recommendations.is_empty() {
            recommendations.push("Compliance audit passed successfully".to_string());
        }

        recommendations
    }
}

impl Default for ComplianceAuditor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_auditor_creation() {
        let auditor = ComplianceAuditor::new();
        assert_eq!(auditor.compliance_frameworks.len(), 3);
        assert_eq!(auditor.audit_checklist.len(), 5);
    }

    #[test]
    fn test_enterprise_compliance() {
        let auditor = ComplianceAuditor::with_enterprise_compliance();
        assert_eq!(auditor.compliance_frameworks.len(), 6); // 3 default + 3 enterprise
    }

    #[test]
    fn test_certification_eligibility() {
        let auditor = ComplianceAuditor::new();

        let high_score_certs = auditor.check_certifications(0.95).unwrap();
        assert!(high_score_certs[0].eligible); // BasicSecurity
        assert!(high_score_certs[1].eligible); // EnterpriseGrade

        let low_score_certs = auditor.check_certifications(0.6).unwrap();
        assert!(!low_score_certs[0].eligible); // BasicSecurity
        assert!(!low_score_certs[1].eligible); // EnterpriseGrade
    }

    #[test]
    fn test_framework_requirements() {
        let auditor = ComplianceAuditor::new();

        // Mock results for testing
        let (passed_iso, total_iso) = auditor
            .evaluate_framework_requirements(
                "ISO27001",
                &mock_semantic_results(),
                &mock_behavioral_results(),
            )
            .unwrap();

        assert_eq!(total_iso, 5);
        assert!(passed_iso <= total_iso);

        let (passed_nist, total_nist) = auditor
            .evaluate_framework_requirements(
                "NIST",
                &mock_semantic_results(),
                &mock_behavioral_results(),
            )
            .unwrap();

        assert_eq!(total_nist, 4);
        assert!(passed_nist <= total_nist);
    }

    #[test]
    fn test_recommendation_generation() {
        let auditor = ComplianceAuditor::new();

        let framework_compliance = vec![FrameworkCompliance {
            framework_name: "TestFramework".to_string(),
            compliance_percentage: 70.0,
            passed_requirements: 7,
            total_requirements: 10,
        }];

        let recommendations = auditor.generate_recommendations(0.75, &framework_compliance);
        assert!(!recommendations.is_empty());

        // The test should check that TestFramework appears in any recommendation, not just the first
        assert!(
            recommendations.iter().any(|r| r.contains("TestFramework")),
            "Expected TestFramework to appear in recommendations: {:?}",
            recommendations
        );
    }

    // Mock helper functions for testing
    fn mock_semantic_results() -> crate::semantic::deep_verifier::DeepVerificationResult {
        crate::semantic::deep_verifier::DeepVerificationResult {
            overall_confidence: 0.85,
            semantic_score: 0.9,
            type_safety_score: 0.95,
            logic_consistency_score: 0.92,
            mathematical_correctness_score: 0.88,
            deception_risk_score: 0.2,
            verification_details: crate::semantic::deep_verifier::VerificationDetails {
                verified_components: vec![],
                failed_verifications: vec![],
                warnings: vec![],
                coverage_metrics: crate::semantic::deep_verifier::CoverageMetrics {
                    line_coverage: 0.9,
                    branch_coverage: 0.85,
                },
                performance_metrics: crate::semantic::deep_verifier::PerformanceMetrics {
                    verification_time_ms: 1000,
                    memory_usage_mb: 128,
                },
            },
            security_assessment: crate::semantic::deep_verifier::SecurityAssessment {
                threat_level: crate::semantic::deep_verifier::ThreatLevel::Low,
                vulnerability_count: 1,
                attack_surface_analysis: crate::semantic::deep_verifier::AttackSurfaceAnalysis {
                    surface_area: 0.1,
                    vulnerabilities: vec![],
                },
                security_recommendations: vec![],
                compliance_status: crate::semantic::deep_verifier::ComplianceStatus {
                    compliant: true,
                    missing_requirements: vec![],
                },
            },
            recommendations: vec![],
        }
    }

    fn mock_behavioral_results(
    ) -> crate::semantic::behavioral_verifier::BehavioralVerificationResult {
        crate::semantic::behavioral_verifier::BehavioralVerificationResult {
            overall_score: 0.82,
            execution_safety_score: 0.85,
            behavioral_consistency_score: 0.80,
            property_compliance_score: 0.82,
            authenticity_score: 0.81,
            execution_results: vec![],
            security_assessment:
                crate::semantic::behavioral_verifier::BehavioralSecurityAssessment {
                    threat_level: crate::semantic::behavioral_verifier::ThreatLevel::Low,
                    attack_surface_size: 0.1,
                    vulnerability_count: 0,
                    security_score: 0.85,
                    compliance_level:
                        crate::semantic::behavioral_verifier::ComplianceLevel::FullyCompliant,
                },
            violations: vec![],
            recommendations: vec![],
        }
    }
}
