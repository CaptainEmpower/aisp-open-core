//! Service Level Agreement Compliance
//!
//! Focused module for SLA compliance tracking and analysis.

/// Service Level Agreement compliance analysis
#[derive(Debug, Clone)]
pub struct SLACompliance {
    /// Overall SLA compliance status
    pub overall_compliance: ComplianceStatus,
    /// Individual SLA objectives
    pub objectives: Vec<SLAObjective>,
    /// Compliance history
    pub compliance_history: ComplianceHistory,
    /// Breach analysis
    pub breach_analysis: BreachAnalysis,
    /// Financial impact analysis
    pub financial_impact: FinancialImpact,
}

/// SLA compliance status
#[derive(Debug, Clone, PartialEq)]
pub enum ComplianceStatus {
    /// Fully compliant with all SLAs
    FullCompliance,
    /// Mostly compliant with minor issues
    SubstantialCompliance,
    /// Partially compliant with some breaches
    PartialCompliance,
    /// Non-compliant with significant breaches
    NonCompliant,
    /// Compliance status unknown
    Unknown,
}

/// Individual SLA objective
#[derive(Debug, Clone)]
pub struct SLAObjective {
    /// Objective identifier
    pub id: String,
    /// Objective description
    pub description: String,
    /// Objective type
    pub objective_type: ObjectiveType,
    /// Target value
    pub target_value: f64,
    /// Current achievement
    pub current_value: f64,
    /// Measurement period
    pub measurement_period: std::time::Duration,
    /// Compliance percentage
    pub compliance_percentage: f64,
    /// Status
    pub status: ObjectiveStatus,
}

/// Types of SLA objectives
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectiveType {
    /// Availability objective (uptime percentage)
    Availability,
    /// Response time objective
    ResponseTime,
    /// Throughput objective
    Throughput,
    /// Error rate objective
    ErrorRate,
    /// Recovery time objective
    RecoveryTime,
    /// Custom objective
    Custom(String),
}

/// SLA objective status
#[derive(Debug, Clone, PartialEq)]
pub enum ObjectiveStatus {
    /// Objective is being met
    Met,
    /// Objective is at risk
    AtRisk,
    /// Objective is breached
    Breached,
    /// Objective is severely breached
    SeverelyBreached,
}

/// SLA compliance history
#[derive(Debug, Clone)]
pub struct ComplianceHistory {
    /// Historical compliance data
    pub history_data: Vec<ComplianceDataPoint>,
    /// Compliance trend
    pub trend: ComplianceTrend,
    /// Average compliance over period
    pub average_compliance: f64,
    /// Best compliance period
    pub best_period: ComplianceDataPoint,
    /// Worst compliance period
    pub worst_period: ComplianceDataPoint,
}

/// Compliance data point
#[derive(Debug, Clone)]
pub struct ComplianceDataPoint {
    /// Time period
    pub period: TimePeriod,
    /// Overall compliance percentage
    pub compliance_percentage: f64,
    /// Number of breaches
    pub breach_count: usize,
    /// Severity distribution
    pub severity_distribution: SeverityDistribution,
}

/// Time period for compliance measurement
#[derive(Debug, Clone)]
pub struct TimePeriod {
    /// Start time
    pub start: std::time::SystemTime,
    /// End time
    pub end: std::time::SystemTime,
    /// Period duration
    pub duration: std::time::Duration,
}

/// Compliance trend analysis
#[derive(Debug, Clone)]
pub struct ComplianceTrend {
    /// Trend direction
    pub direction: TrendDirection,
    /// Rate of change per month
    pub monthly_change_rate: f64,
    /// Trend confidence
    pub confidence: f64,
    /// Projected compliance
    pub projected_compliance: f64,
}

/// Trend direction for compliance
#[derive(Debug, Clone, PartialEq)]
pub enum TrendDirection {
    /// Compliance improving
    Improving,
    /// Compliance stable
    Stable,
    /// Compliance declining
    Declining,
    /// Compliance volatile
    Volatile,
    /// Trend unclear
    Unknown,
}

/// Severity distribution of breaches
#[derive(Debug, Clone)]
pub struct SeverityDistribution {
    /// Minor breaches count
    pub minor_breaches: usize,
    /// Major breaches count  
    pub major_breaches: usize,
    /// Critical breaches count
    pub critical_breaches: usize,
    /// Catastrophic breaches count
    pub catastrophic_breaches: usize,
}

/// SLA breach analysis
#[derive(Debug, Clone)]
pub struct BreachAnalysis {
    /// Total breaches in period
    pub total_breaches: usize,
    /// Breach frequency (per month)
    pub breach_frequency: f64,
    /// Average breach duration
    pub average_breach_duration: std::time::Duration,
    /// Most common breach types
    pub common_breach_types: Vec<BreachType>,
    /// Breach root causes
    pub root_causes: Vec<RootCause>,
}

/// Types of SLA breaches
#[derive(Debug, Clone, PartialEq)]
pub enum BreachType {
    /// Availability breach
    AvailabilityBreach,
    /// Performance breach
    PerformanceBreach,
    /// Reliability breach
    ReliabilityBreach,
    /// Security breach
    SecurityBreach,
    /// Data integrity breach
    DataIntegrityBreach,
}

/// Root cause of SLA breach
#[derive(Debug, Clone)]
pub struct RootCause {
    /// Cause identifier
    pub id: String,
    /// Cause category
    pub category: RootCauseCategory,
    /// Frequency of this cause
    pub frequency: usize,
    /// Impact severity
    pub impact_severity: ImpactSeverity,
    /// Mitigation actions
    pub mitigation_actions: Vec<String>,
}

/// Categories of root causes
#[derive(Debug, Clone, PartialEq)]
pub enum RootCauseCategory {
    /// Infrastructure related
    Infrastructure,
    /// Application software
    Application,
    /// Network issues
    Network,
    /// External dependencies
    External,
    /// Human error
    Human,
    /// Process failure
    Process,
    /// Unknown cause
    Unknown,
}

/// Impact severity levels
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ImpactSeverity {
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Critical impact
    Critical,
}

/// Financial impact of SLA breaches
#[derive(Debug, Clone)]
pub struct FinancialImpact {
    /// Total penalty amount
    pub total_penalties: f64,
    /// Revenue impact
    pub revenue_impact: f64,
    /// Customer credit amount
    pub customer_credits: f64,
    /// Operational costs
    pub operational_costs: f64,
    /// Currency
    pub currency: String,
}

impl Default for SLACompliance {
    fn default() -> Self {
        Self {
            overall_compliance: ComplianceStatus::FullCompliance,
            objectives: Vec::new(),
            compliance_history: ComplianceHistory::default(),
            breach_analysis: BreachAnalysis::default(),
            financial_impact: FinancialImpact::default(),
        }
    }
}

impl Default for ComplianceHistory {
    fn default() -> Self {
        Self {
            history_data: Vec::new(),
            trend: ComplianceTrend::default(),
            average_compliance: 100.0,
            best_period: ComplianceDataPoint::default(),
            worst_period: ComplianceDataPoint::default(),
        }
    }
}

impl Default for ComplianceTrend {
    fn default() -> Self {
        Self {
            direction: TrendDirection::Stable,
            monthly_change_rate: 0.0,
            confidence: 1.0,
            projected_compliance: 100.0,
        }
    }
}

impl Default for ComplianceDataPoint {
    fn default() -> Self {
        Self {
            period: TimePeriod::default(),
            compliance_percentage: 100.0,
            breach_count: 0,
            severity_distribution: SeverityDistribution::default(),
        }
    }
}

impl Default for TimePeriod {
    fn default() -> Self {
        let now = std::time::SystemTime::now();
        Self {
            start: now,
            end: now,
            duration: std::time::Duration::ZERO,
        }
    }
}

impl Default for SeverityDistribution {
    fn default() -> Self {
        Self {
            minor_breaches: 0,
            major_breaches: 0,
            critical_breaches: 0,
            catastrophic_breaches: 0,
        }
    }
}

impl Default for BreachAnalysis {
    fn default() -> Self {
        Self {
            total_breaches: 0,
            breach_frequency: 0.0,
            average_breach_duration: std::time::Duration::ZERO,
            common_breach_types: Vec::new(),
            root_causes: Vec::new(),
        }
    }
}

impl Default for FinancialImpact {
    fn default() -> Self {
        Self {
            total_penalties: 0.0,
            revenue_impact: 0.0,
            customer_credits: 0.0,
            operational_costs: 0.0,
            currency: "USD".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compliance_status() {
        assert_eq!(
            ComplianceStatus::FullCompliance,
            ComplianceStatus::FullCompliance
        );
        assert_ne!(
            ComplianceStatus::FullCompliance,
            ComplianceStatus::NonCompliant
        );
    }

    #[test]
    fn test_objective_type() {
        assert_eq!(ObjectiveType::Availability, ObjectiveType::Availability);
        assert_ne!(ObjectiveType::Availability, ObjectiveType::ResponseTime);
    }

    #[test]
    fn test_objective_status() {
        assert_eq!(ObjectiveStatus::Met, ObjectiveStatus::Met);
        assert_ne!(ObjectiveStatus::Met, ObjectiveStatus::Breached);
    }

    #[test]
    fn test_impact_severity_ordering() {
        use ImpactSeverity::*;
        assert!(Low < Medium);
        assert!(Medium < High);
        assert!(High < Critical);
    }

    #[test]
    fn test_sla_objective_creation() {
        let objective = SLAObjective {
            id: "availability-99.9".to_string(),
            description: "99.9% service availability".to_string(),
            objective_type: ObjectiveType::Availability,
            target_value: 99.9,
            current_value: 99.95,
            measurement_period: std::time::Duration::from_secs(2592000), // 30 days
            compliance_percentage: 100.0,
            status: ObjectiveStatus::Met,
        };

        assert_eq!(objective.objective_type, ObjectiveType::Availability);
        assert_eq!(objective.status, ObjectiveStatus::Met);
        assert!(objective.current_value > objective.target_value);
    }
}
