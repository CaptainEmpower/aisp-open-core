//! Compliance Auditor Module
//!
//! Implements regulatory compliance auditing, certification requirements verification,
//! and audit trail management for enterprise security standards.

use super::core_types::*;
use crate::ast::canonical::CanonicalAispDocument as AispDocument;
use crate::error::AispResult;

/// Compliance auditor for regulatory and standard compliance
pub struct ComplianceAuditor {
    pub compliance_frameworks: Vec<ComplianceFramework>,
    pub audit_checklist: Vec<AuditCheckpoint>,
    pub certification_requirements: Vec<CertificationRequirement>,
    pub audit_trail: AuditTrail,
    pub reporting_engine: ReportingEngine,
    audit_sessions: std::collections::HashMap<String, AuditSession>,
}

/// Active audit session tracking
#[derive(Debug, Clone)]
struct AuditSession {
    session_id: String,
    start_time: std::time::SystemTime,
    frameworks_evaluated: Vec<String>,
    checkpoints_completed: Vec<String>,
    violations_found: Vec<String>,
    compliance_score: f64,
}

impl ComplianceAuditor {
    /// Create new compliance auditor with comprehensive framework support
    pub fn new() -> Self {
        let compliance_frameworks = vec![
            ComplianceFramework {
                framework_name: "AISP-5.1".to_string(),
                version: "5.1.0".to_string(),
            },
            ComplianceFramework {
                framework_name: "ISO27001".to_string(),
                version: "2022".to_string(),
            },
            ComplianceFramework {
                framework_name: "SOC2-Type2".to_string(),
                version: "2023".to_string(),
            },
            ComplianceFramework {
                framework_name: "NIST-CSF".to_string(),
                version: "2.0".to_string(),
            },
        ];

        let audit_checklist = vec![
            AuditCheckpoint {
                checkpoint_id: "PARSE_SECURITY".to_string(),
                requirement: "Robust parsing with security validation".to_string(),
            },
            AuditCheckpoint {
                checkpoint_id: "TYPE_SAFETY".to_string(),
                requirement: "Comprehensive type safety verification".to_string(),
            },
            AuditCheckpoint {
                checkpoint_id: "DECEPTION_DETECTION".to_string(),
                requirement: "Advanced deception pattern detection".to_string(),
            },
            AuditCheckpoint {
                checkpoint_id: "ADVERSARIAL_RESISTANCE".to_string(),
                requirement: "Resistance to adversarial attacks".to_string(),
            },
            AuditCheckpoint {
                checkpoint_id: "CROSS_VALIDATION".to_string(),
                requirement: "Multi-layer cross-validation consistency".to_string(),
            },
        ];

        let certification_requirements = vec![
            CertificationRequirement {
                requirement_id: "SEC_CONFIDENCE".to_string(),
                standard: "Security confidence >= 90%".to_string(),
            },
            CertificationRequirement {
                requirement_id: "COMP_COVERAGE".to_string(),
                standard: "Compliance coverage >= 95%".to_string(),
            },
            CertificationRequirement {
                requirement_id: "ATTACK_RESIST".to_string(),
                standard: "Attack resistance >= 85%".to_string(),
            },
        ];

        Self {
            compliance_frameworks,
            audit_checklist,
            certification_requirements,
            audit_trail: AuditTrail {
                entries: Vec::new(),
            },
            reporting_engine: ReportingEngine {
                report_formats: vec![
                    "JSON".to_string(),
                    "PDF".to_string(),
                    "HTML".to_string(),
                    "XML".to_string(),
                ],
            },
            audit_sessions: std::collections::HashMap::new(),
        }
    }

    /// Create streamlined auditor for performance-focused environments
    pub fn new_streamlined() -> Self {
        let mut auditor = Self::new();

        // Reduce frameworks to essential ones
        auditor.compliance_frameworks = vec![
            ComplianceFramework {
                framework_name: "AISP-5.1-Basic".to_string(),
                version: "5.1.0".to_string(),
            },
            ComplianceFramework {
                framework_name: "Essential-Security".to_string(),
                version: "1.0".to_string(),
            },
        ];

        // Streamline audit checklist
        auditor.audit_checklist = auditor
            .audit_checklist
            .into_iter()
            .filter(|checkpoint| {
                matches!(
                    checkpoint.checkpoint_id.as_str(),
                    "PARSE_SECURITY" | "TYPE_SAFETY" | "ADVERSARIAL_RESISTANCE"
                )
            })
            .collect();

        auditor
    }

    /// Perform comprehensive compliance audit
    pub fn perform_compliance_audit(
        &mut self,
        document: &AispDocument,
        semantic: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
        security: &EnterpriseSecurityAssessment,
    ) -> AispResult<ComplianceStatus> {
        let session_id = self.start_audit_session()?;

        let mut compliant_frameworks = Vec::new();
        let mut violations = Vec::new();

        // Audit each framework
        for framework in &self.compliance_frameworks.clone() {
            let framework_result = self.audit_framework(
                &framework.framework_name,
                document,
                semantic,
                behavioral,
                security,
            )?;

            if framework_result.compliant {
                compliant_frameworks.push(framework.framework_name.clone());
                self.log_audit_success(&session_id, &framework.framework_name);
            } else {
                self.log_audit_violations(
                    &session_id,
                    &framework.framework_name,
                    &framework_result.violations,
                );
                violations.extend(framework_result.violations);
            }
        }

        // Update audit session
        if let Some(session) = self.audit_sessions.get_mut(&session_id) {
            session.frameworks_evaluated = self
                .compliance_frameworks
                .iter()
                .map(|f| f.framework_name.clone())
                .collect();
            session.compliance_score =
                compliant_frameworks.len() as f64 / self.compliance_frameworks.len() as f64;
            session.violations_found = violations.clone();
        }

        self.finalize_audit(&session_id)?;

        Ok(ComplianceStatus {
            compliant_frameworks,
            violations,
        })
    }

    /// Audit specific compliance framework
    fn audit_framework(
        &self,
        framework_name: &str,
        document: &AispDocument,
        semantic: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
        security: &EnterpriseSecurityAssessment,
    ) -> AispResult<FrameworkAuditResult> {
        match framework_name {
            "AISP-5.1" | "AISP-5.1-Basic" => self.audit_aisp_framework(semantic, behavioral),
            "ISO27001" => self.audit_iso27001_framework(security, semantic),
            "SOC2-Type2" => self.audit_soc2_framework(semantic, behavioral, security),
            "NIST-CSF" => self.audit_nist_framework(security, semantic),
            "Essential-Security" => self.audit_essential_security(semantic, behavioral),
            _ => Ok(FrameworkAuditResult {
                compliant: false,
                violations: vec![format!("Unknown framework: {}", framework_name)],
            }),
        }
    }

    /// Audit AISP 5.1 compliance requirements
    fn audit_aisp_framework(
        &self,
        semantic: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
    ) -> AispResult<FrameworkAuditResult> {
        let mut violations = Vec::new();

        // AISP-5.1 core requirements
        if semantic.overall_confidence < 0.90 {
            violations.push("AISP-5.1: Semantic confidence below 90%".to_string());
        }

        if semantic.type_safety_score < 0.95 {
            violations.push("AISP-5.1: Type safety score below 95%".to_string());
        }

        if behavioral.execution_safety_score < 0.85 {
            violations.push("AISP-5.1: Execution safety below 85%".to_string());
        }

        if semantic.deception_risk_score > 0.15 {
            violations.push("AISP-5.1: Deception risk exceeds 15%".to_string());
        }

        Ok(FrameworkAuditResult {
            compliant: violations.is_empty(),
            violations,
        })
    }

    /// Audit ISO 27001 compliance requirements
    fn audit_iso27001_framework(
        &self,
        security: &EnterpriseSecurityAssessment,
        semantic: &crate::semantic::deep_verifier::DeepVerificationResult,
    ) -> AispResult<FrameworkAuditResult> {
        let mut violations = Vec::new();

        // ISO 27001 security requirements
        if security.security_posture != "Strong" {
            violations.push("ISO27001: Security posture not at required level".to_string());
        }

        if semantic.security_assessment.vulnerability_count > 0 {
            violations.push("ISO27001: Security vulnerabilities detected".to_string());
        }

        if !security.threat_landscape.is_empty()
            && security
                .threat_landscape
                .iter()
                .any(|t| t.contains("Critical"))
        {
            violations.push("ISO27001: Critical threats in landscape".to_string());
        }

        Ok(FrameworkAuditResult {
            compliant: violations.is_empty(),
            violations,
        })
    }

    /// Audit SOC 2 Type 2 compliance
    fn audit_soc2_framework(
        &self,
        semantic: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
        security: &EnterpriseSecurityAssessment,
    ) -> AispResult<FrameworkAuditResult> {
        let mut violations = Vec::new();

        // SOC 2 Trust Service Criteria

        // Security
        if security.security_posture == "Weak" {
            violations.push("SOC2: Security controls insufficient".to_string());
        }

        // Availability
        if behavioral.execution_safety_score < 0.90 {
            violations.push("SOC2: Availability requirements not met".to_string());
        }

        // Processing Integrity
        if semantic.logic_consistency_score < 0.88 {
            violations.push("SOC2: Processing integrity below threshold".to_string());
        }

        // Confidentiality (implied through deception detection)
        if semantic.deception_risk_score > 0.10 {
            violations.push("SOC2: Confidentiality risks detected".to_string());
        }

        Ok(FrameworkAuditResult {
            compliant: violations.is_empty(),
            violations,
        })
    }

    /// Audit NIST Cybersecurity Framework compliance
    fn audit_nist_framework(
        &self,
        security: &EnterpriseSecurityAssessment,
        semantic: &crate::semantic::deep_verifier::DeepVerificationResult,
    ) -> AispResult<FrameworkAuditResult> {
        let mut violations = Vec::new();

        // NIST CSF Core Functions: Identify, Protect, Detect, Respond, Recover

        // Identify
        if security.threat_landscape.len() > 5 {
            violations.push("NIST-CSF: Threat landscape not adequately identified".to_string());
        }

        // Protect
        if semantic.overall_confidence < 0.85 {
            violations.push("NIST-CSF: Protective measures insufficient".to_string());
        }

        // Detect
        if semantic.deception_risk_score > 0.20 {
            violations.push("NIST-CSF: Detection capabilities inadequate".to_string());
        }

        Ok(FrameworkAuditResult {
            compliant: violations.is_empty(),
            violations,
        })
    }

    /// Audit essential security requirements (streamlined)
    fn audit_essential_security(
        &self,
        semantic: &crate::semantic::deep_verifier::DeepVerificationResult,
        behavioral: &crate::semantic::behavioral_verifier::BehavioralVerificationResult,
    ) -> AispResult<FrameworkAuditResult> {
        let mut violations = Vec::new();

        // Essential security baseline
        if semantic.overall_confidence < 0.80 {
            violations.push("Essential: Basic confidence threshold not met".to_string());
        }

        if behavioral.execution_safety_score < 0.75 {
            violations.push("Essential: Execution safety below minimum".to_string());
        }

        Ok(FrameworkAuditResult {
            compliant: violations.is_empty(),
            violations,
        })
    }

    /// Start new audit session
    fn start_audit_session(&mut self) -> AispResult<String> {
        let session_id = format!(
            "audit_session_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis()
        );

        let audit_session = AuditSession {
            session_id: session_id.clone(),
            start_time: std::time::SystemTime::now(),
            frameworks_evaluated: Vec::new(),
            checkpoints_completed: Vec::new(),
            violations_found: Vec::new(),
            compliance_score: 0.0,
        };

        self.audit_sessions
            .insert(session_id.clone(), audit_session);
        self.add_audit_entry(&format!("Audit session started: {}", session_id));

        Ok(session_id)
    }

    /// Log audit success for framework
    fn log_audit_success(&mut self, session_id: &str, framework: &str) {
        self.add_audit_entry(&format!(
            "Session {}: {} - COMPLIANT",
            session_id, framework
        ));
    }

    /// Log audit violations for framework  
    fn log_audit_violations(&mut self, session_id: &str, framework: &str, violations: &[String]) {
        self.add_audit_entry(&format!(
            "Session {}: {} - VIOLATIONS: {}",
            session_id,
            framework,
            violations.join("; ")
        ));
    }

    /// Add entry to audit trail
    fn add_audit_entry(&mut self, entry: &str) {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let audit_entry = format!("[{}] {}", timestamp, entry);
        self.audit_trail.entries.push(audit_entry);
    }

    /// Finalize audit session
    pub fn finalize_audit(&mut self, session_id: &str) -> AispResult<()> {
        if let Some(session) = self.audit_sessions.remove(session_id) {
            let duration = session
                .start_time
                .elapsed()
                .unwrap_or(std::time::Duration::from_secs(0));

            self.add_audit_entry(&format!(
                "Session {} completed: {}ms, compliance: {:.2}, violations: {}",
                session_id,
                duration.as_millis(),
                session.compliance_score,
                session.violations_found.len()
            ));
        }

        Ok(())
    }

    /// Generate compliance report in specified format
    pub fn generate_compliance_report(&self, format: &str) -> AispResult<String> {
        match format {
            "JSON" => self.generate_json_report(),
            "HTML" => self.generate_html_report(),
            "PDF" => Ok("PDF report generation not implemented".to_string()),
            _ => Err(crate::error::AispError::internal_error(&format!(
                "Unsupported report format: {}",
                format
            ))),
        }
    }

    /// Generate JSON compliance report
    fn generate_json_report(&self) -> AispResult<String> {
        let report = format!(
            r#"{{
  "compliance_frameworks": {},
  "audit_trail": {{"entries": {}}},
  "active_sessions": {},
  "total_checkpoints": {},
  "certification_requirements": {}
}}"#,
            self.compliance_frameworks.len(),
            self.audit_trail.entries.len(),
            self.audit_sessions.len(),
            self.audit_checklist.len(),
            self.certification_requirements.len()
        );

        Ok(report)
    }

    /// Generate HTML compliance report
    fn generate_html_report(&self) -> AispResult<String> {
        let html = format!(
            r#"
<!DOCTYPE html>
<html>
<head><title>Compliance Audit Report</title></head>
<body>
<h1>Compliance Audit Report</h1>
<h2>Frameworks Evaluated</h2>
<ul>
{}
</ul>
<h2>Audit Trail ({} entries)</h2>
<pre>{}</pre>
</body>
</html>
"#,
            self.compliance_frameworks
                .iter()
                .map(|f| format!("<li>{} v{}</li>", f.framework_name, f.version))
                .collect::<Vec<_>>()
                .join("\n"),
            self.audit_trail.entries.len(),
            self.audit_trail.entries.join("\n")
        );

        Ok(html)
    }
}

/// Framework audit result
#[derive(Debug)]
struct FrameworkAuditResult {
    compliant: bool,
    violations: Vec<String>,
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
        assert!(!auditor.compliance_frameworks.is_empty());
        assert!(!auditor.audit_checklist.is_empty());
        assert!(!auditor.certification_requirements.is_empty());
    }

    #[test]
    fn test_streamlined_auditor() {
        let auditor = ComplianceAuditor::new_streamlined();

        // Should have fewer frameworks than full version
        let full_auditor = ComplianceAuditor::new();
        assert!(auditor.compliance_frameworks.len() < full_auditor.compliance_frameworks.len());
        assert!(auditor.audit_checklist.len() <= full_auditor.audit_checklist.len());
    }

    #[test]
    fn test_audit_session_management() {
        let mut auditor = ComplianceAuditor::new();

        let session_id = auditor.start_audit_session().unwrap();
        assert!(auditor.audit_sessions.contains_key(&session_id));
        assert!(!auditor.audit_trail.entries.is_empty());

        auditor.finalize_audit(&session_id).unwrap();
        assert!(!auditor.audit_sessions.contains_key(&session_id));
    }

    #[test]
    fn test_audit_trail() {
        let mut auditor = ComplianceAuditor::new();
        let initial_entries = auditor.audit_trail.entries.len();

        auditor.add_audit_entry("Test audit entry");
        assert_eq!(auditor.audit_trail.entries.len(), initial_entries + 1);
        assert!(auditor
            .audit_trail
            .entries
            .last()
            .unwrap()
            .contains("Test audit entry"));
    }

    #[test]
    fn test_report_generation() {
        let auditor = ComplianceAuditor::new();

        let json_report = auditor.generate_json_report();
        assert!(json_report.is_ok());
        assert!(json_report.unwrap().contains("compliance_frameworks"));

        let html_report = auditor.generate_html_report();
        assert!(html_report.is_ok());
        assert!(html_report.unwrap().contains("<html>"));
    }
}
