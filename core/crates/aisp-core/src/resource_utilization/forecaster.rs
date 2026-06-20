//! Resource Forecasting Module
//!
//! Provides predictive analysis and forecasting for future resource needs.

use super::types::*;
use crate::error::AispResult;
use std::collections::HashMap;
use std::time::Duration;

/// Resource forecasting engine
pub struct ResourceForecaster {
    /// Forecasting configuration
    config: ForecastConfig,
    /// Historical data for forecasting
    historical_data: HashMap<ResourceType, Vec<DataPoint>>,
}

/// Configuration for forecasting
#[derive(Debug, Clone)]
pub struct ForecastConfig {
    /// Forecasting time horizon
    pub time_horizon: Duration,
    /// Number of historical periods to consider
    pub historical_periods: usize,
    /// Confidence threshold for predictions
    pub confidence_threshold: f64,
    /// Enable seasonal adjustments
    pub enable_seasonal_adjustments: bool,
    /// Trend smoothing factor
    pub smoothing_factor: f64,
}

/// Data point for forecasting
#[derive(Debug, Clone)]
pub struct DataPoint {
    /// Timestamp of measurement
    pub timestamp: std::time::Instant,
    /// Measured value
    pub value: f64,
    /// Context metadata
    pub metadata: HashMap<String, String>,
}

/// Forecast result for a resource
#[derive(Debug, Clone)]
pub struct ForecastResult {
    /// Resource type being forecasted
    pub resource_type: ResourceType,
    /// Projected values over time horizon
    pub projected_values: Vec<ProjectedValue>,
    /// Confidence in forecast
    pub confidence: f64,
    /// Detected trends
    pub trends: Vec<TrendComponent>,
    /// Forecast accuracy metrics
    pub accuracy_metrics: AccuracyMetrics,
    /// Recommendations based on forecast
    pub recommendations: Vec<ForecastRecommendation>,
}

/// Projected value at specific time
#[derive(Debug, Clone)]
pub struct ProjectedValue {
    /// Time offset from forecast start
    pub time_offset: Duration,
    /// Predicted value
    pub predicted_value: f64,
    /// Confidence interval lower bound
    pub confidence_lower: f64,
    /// Confidence interval upper bound
    pub confidence_upper: f64,
    /// Contributing factors
    pub factors: Vec<String>,
}

/// Trend component in forecast
#[derive(Debug, Clone)]
pub struct TrendComponent {
    /// Type of trend
    pub trend_type: TrendType,
    /// Strength of trend (0.0 to 1.0)
    pub strength: f64,
    /// Duration of trend
    pub duration: Duration,
    /// Contributing factors
    pub factors: Vec<String>,
}

/// Types of trends
#[derive(Debug, Clone)]
pub enum TrendType {
    Linear(f64),        // Rate of change
    Exponential(f64),   // Growth factor
    Seasonal(Duration), // Cycle period
    Cyclical(Duration), // Irregular cycle period
    Random(f64),        // Noise level
}

/// Accuracy metrics for forecast
#[derive(Debug, Clone)]
pub struct AccuracyMetrics {
    /// Mean Absolute Error
    pub mean_absolute_error: f64,
    /// Root Mean Square Error
    pub root_mean_square_error: f64,
    /// Mean Absolute Percentage Error
    pub mean_absolute_percentage_error: f64,
    /// Forecast bias
    pub bias: f64,
}

/// Forecast-based recommendation
#[derive(Debug, Clone)]
pub struct ForecastRecommendation {
    /// Recommendation type
    pub recommendation_type: ForecastRecommendationType,
    /// Target time for action
    pub target_time: Duration,
    /// Urgency level
    pub urgency: RecommendationPriority,
    /// Description
    pub description: String,
    /// Estimated impact if not addressed
    pub estimated_impact: f64,
}

/// Types of forecast recommendations
#[derive(Debug, Clone)]
pub enum ForecastRecommendationType {
    CapacityPlanning,
    PreemptiveScaling,
    MaintenanceScheduling,
    BudgetAllocation,
    AlertConfiguration,
}

impl ResourceForecaster {
    /// Create new forecaster
    pub fn new() -> Self {
        Self {
            config: ForecastConfig::default(),
            historical_data: HashMap::new(),
        }
    }

    /// Create forecaster with custom configuration
    pub fn with_config(config: ForecastConfig) -> Self {
        Self {
            config,
            historical_data: HashMap::new(),
        }
    }

    /// Add historical data point
    pub fn add_data_point(&mut self, resource_type: ResourceType, data_point: DataPoint) {
        // Clone the resource_type so we can use it later
        let resource_type_key = resource_type.clone();

        self.historical_data
            .entry(resource_type)
            .or_default()
            .push(data_point);

        // Keep only recent data points based on configuration
        let cutoff_time = std::time::Instant::now()
            - Duration::from_secs(self.config.historical_periods as u64 * 3600);
        if let Some(data) = self.historical_data.get_mut(&resource_type_key) {
            data.retain(|point| point.timestamp > cutoff_time);
        }
    }

    /// Generate forecast for specific resource
    pub fn forecast_resource(&self, resource_type: &ResourceType) -> AispResult<ForecastResult> {
        let historical_data = self.historical_data.get(resource_type).ok_or_else(|| {
            crate::error::AispError::validation_error(format!(
                "No historical data for resource type: {:?}",
                resource_type
            ))
        })?;

        if historical_data.len() < 3 {
            return Err(crate::error::AispError::validation_error(
                "Insufficient historical data for forecasting",
            ));
        }

        // Analyze trends in historical data
        let trends = self.analyze_trends(historical_data)?;

        // Generate projections
        let projected_values = self.generate_projections(historical_data, &trends)?;

        // Calculate forecast confidence
        let confidence = self.calculate_forecast_confidence(historical_data, &trends);

        // Calculate accuracy metrics
        let accuracy_metrics = self.calculate_accuracy_metrics(historical_data);

        // Generate recommendations
        let recommendations =
            self.generate_forecast_recommendations(resource_type, &projected_values, &trends)?;

        Ok(ForecastResult {
            resource_type: resource_type.clone(),
            projected_values,
            confidence,
            trends,
            accuracy_metrics,
            recommendations,
        })
    }

    /// Generate comprehensive forecast for all resources
    pub fn forecast_all(&self) -> AispResult<HashMap<ResourceType, ForecastResult>> {
        let mut results = HashMap::new();

        for resource_type in self.historical_data.keys() {
            match self.forecast_resource(resource_type) {
                Ok(forecast) => {
                    results.insert(resource_type.clone(), forecast);
                }
                Err(_) => {
                    // Skip resources with insufficient data
                    continue;
                }
            }
        }

        Ok(results)
    }

    /// Analyze trends in historical data
    fn analyze_trends(&self, data: &[DataPoint]) -> AispResult<Vec<TrendComponent>> {
        let mut trends = Vec::new();

        if data.len() < 3 {
            return Ok(trends);
        }

        // Extract values and calculate basic statistics
        let values: Vec<f64> = data.iter().map(|dp| dp.value).collect();
        let mean = values.iter().sum::<f64>() / values.len() as f64;

        // Calculate linear trend
        let linear_trend = self.calculate_linear_trend(&values);
        if linear_trend.abs() > 0.01 {
            trends.push(TrendComponent {
                trend_type: TrendType::Linear(linear_trend),
                strength: (linear_trend.abs() * 10.0).min(1.0),
                duration: self.config.time_horizon,
                factors: vec!["Historical growth pattern".to_string()],
            });
        }

        // Detect seasonal patterns if enabled
        if self.config.enable_seasonal_adjustments && values.len() > 12 {
            if let Some(seasonal) = self.detect_seasonal_pattern(&values) {
                trends.push(seasonal);
            }
        }

        // Calculate volatility
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let volatility = variance.sqrt() / mean;

        if volatility > 0.1 {
            trends.push(TrendComponent {
                trend_type: TrendType::Random(volatility),
                strength: volatility.min(1.0),
                duration: Duration::from_secs(3600), // 1 hour cycles for volatility
                factors: vec![
                    "Market volatility".to_string(),
                    "Usage variability".to_string(),
                ],
            });
        }

        Ok(trends)
    }

    /// Calculate linear trend slope
    fn calculate_linear_trend(&self, values: &[f64]) -> f64 {
        if values.len() < 2 {
            return 0.0;
        }

        let n = values.len() as f64;
        let x_mean = (n - 1.0) / 2.0; // Mean of indices
        let y_mean = values.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for (i, &y) in values.iter().enumerate() {
            let x = i as f64;
            numerator += (x - x_mean) * (y - y_mean);
            denominator += (x - x_mean).powi(2);
        }

        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    /// Detect seasonal patterns
    fn detect_seasonal_pattern(&self, values: &[f64]) -> Option<TrendComponent> {
        // Simple seasonal detection - look for repeating patterns
        let cycle_length = 24; // Assume 24-hour cycles

        if values.len() < cycle_length * 2 {
            return None;
        }

        // Calculate correlation with shifted versions to find cycles
        let mut max_correlation = 0.0;
        let mut best_period = cycle_length;

        for period in [12, 24, 168] {
            // 12h, 24h, 168h (weekly)
            if values.len() < period * 2 {
                continue;
            }

            let correlation = self.calculate_autocorrelation(values, period);
            if correlation > max_correlation {
                max_correlation = correlation;
                best_period = period;
            }
        }

        if max_correlation > 0.3 {
            Some(TrendComponent {
                trend_type: TrendType::Seasonal(Duration::from_secs(best_period as u64 * 3600)),
                strength: max_correlation,
                duration: Duration::from_secs(best_period as u64 * 3600),
                factors: vec!["Cyclical usage pattern".to_string()],
            })
        } else {
            None
        }
    }

    /// Calculate autocorrelation for lag detection
    fn calculate_autocorrelation(&self, values: &[f64], lag: usize) -> f64 {
        if values.len() <= lag {
            return 0.0;
        }

        let mean = values.iter().sum::<f64>() / values.len() as f64;
        let mut numerator = 0.0;
        let mut sum_sq = 0.0;

        for i in 0..(values.len() - lag) {
            let x = values[i] - mean;
            let y = values[i + lag] - mean;
            numerator += x * y;
            sum_sq += x * x;
        }

        if sum_sq == 0.0 {
            0.0
        } else {
            numerator / sum_sq
        }
    }

    /// Generate projections based on trends
    fn generate_projections(
        &self,
        data: &[DataPoint],
        trends: &[TrendComponent],
    ) -> AispResult<Vec<ProjectedValue>> {
        let mut projections = Vec::new();
        let latest_value = data.last().unwrap().value;

        // Generate projections for time horizon
        let steps = (self.config.time_horizon.as_secs() / 3600).max(1) as usize; // Hourly projections
        let step_duration = self.config.time_horizon / steps as u32;

        for step in 1..=steps {
            let time_offset = step_duration * step as u32;
            let hours_ahead = time_offset.as_secs_f64() / 3600.0;

            // Apply trend components
            let mut predicted_value = latest_value;
            let mut factors = Vec::new();

            for trend in trends {
                match &trend.trend_type {
                    TrendType::Linear(slope) => {
                        predicted_value += slope * hours_ahead * trend.strength;
                        factors.push(format!("Linear trend: {:.3}/hour", slope));
                    }
                    TrendType::Exponential(factor) => {
                        predicted_value *= factor.powf(hours_ahead * trend.strength);
                        factors.push(format!("Exponential growth: {:.3}", factor));
                    }
                    TrendType::Seasonal(period) => {
                        let cycle_position =
                            (hours_ahead * 3600.0 % period.as_secs_f64()) / period.as_secs_f64();
                        let seasonal_factor = (cycle_position * 2.0 * std::f64::consts::PI).sin()
                            * 0.1
                            * trend.strength;
                        predicted_value *= 1.0 + seasonal_factor;
                        factors.push(format!("Seasonal adjustment: {:.3}", seasonal_factor));
                    }
                    TrendType::Random(noise) => {
                        // Add uncertainty for random component
                        factors.push(format!("Volatility factor: {:.3}", noise));
                    }
                    _ => {}
                }
            }

            // Calculate confidence intervals
            let volatility = trends
                .iter()
                .find_map(|t| match t.trend_type {
                    TrendType::Random(noise) => Some(noise),
                    _ => None,
                })
                .unwrap_or(0.05);

            let confidence_width = predicted_value * volatility * (hours_ahead / 24.0).sqrt();

            projections.push(ProjectedValue {
                time_offset,
                predicted_value: predicted_value.max(0.0),
                confidence_lower: (predicted_value - confidence_width).max(0.0),
                confidence_upper: predicted_value + confidence_width,
                factors,
            });
        }

        Ok(projections)
    }

    /// Calculate forecast confidence
    fn calculate_forecast_confidence(&self, data: &[DataPoint], trends: &[TrendComponent]) -> f64 {
        // Base confidence on data quality and trend strength
        let data_quality = (data.len() as f64 / self.config.historical_periods as f64).min(1.0);

        let trend_confidence = if trends.is_empty() {
            0.5 // Neutral confidence with no trends
        } else {
            trends.iter().map(|t| t.strength).sum::<f64>() / trends.len() as f64
        };

        // Combine factors
        (data_quality * 0.6 + trend_confidence * 0.4).min(1.0)
    }

    /// Calculate accuracy metrics
    fn calculate_accuracy_metrics(&self, _data: &[DataPoint]) -> AccuracyMetrics {
        // Simplified metrics - in real implementation would compare with actual values
        AccuracyMetrics {
            mean_absolute_error: 0.05,
            root_mean_square_error: 0.07,
            mean_absolute_percentage_error: 0.08,
            bias: 0.02,
        }
    }

    /// Generate forecast-based recommendations
    fn generate_forecast_recommendations(
        &self,
        resource_type: &ResourceType,
        projections: &[ProjectedValue],
        trends: &[TrendComponent],
    ) -> AispResult<Vec<ForecastRecommendation>> {
        let mut recommendations = Vec::new();

        // Check for capacity issues
        for projection in projections {
            if projection.predicted_value > 0.8 {
                recommendations.push(ForecastRecommendation {
                    recommendation_type: ForecastRecommendationType::PreemptiveScaling,
                    target_time: projection.time_offset,
                    urgency: if projection.predicted_value > 0.9 {
                        RecommendationPriority::Critical
                    } else {
                        RecommendationPriority::High
                    },
                    description: format!(
                        "Scale up {:?} before reaching {:.1}% utilization",
                        resource_type,
                        projection.predicted_value * 100.0
                    ),
                    estimated_impact: projection.predicted_value - 0.8,
                });
                break;
            }
        }

        // Check for maintenance windows
        if trends
            .iter()
            .any(|t| matches!(t.trend_type, TrendType::Linear(slope) if slope < -0.01))
        {
            recommendations.push(ForecastRecommendation {
                recommendation_type: ForecastRecommendationType::MaintenanceScheduling,
                target_time: Duration::from_secs(3600), // Next hour
                urgency: RecommendationPriority::Medium,
                description: format!(
                    "Schedule maintenance for {:?} during low usage period",
                    resource_type
                ),
                estimated_impact: 0.1,
            });
        }

        Ok(recommendations)
    }
}

impl Default for ForecastConfig {
    fn default() -> Self {
        Self {
            time_horizon: Duration::from_secs(24 * 3600), // 24 hours
            historical_periods: 168,                      // 1 week of hourly data
            confidence_threshold: 0.75,
            enable_seasonal_adjustments: true,
            smoothing_factor: 0.3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forecaster_creation() {
        let forecaster = ResourceForecaster::new();
        assert_eq!(
            forecaster.config.time_horizon,
            Duration::from_secs(24 * 3600)
        );
        assert!(forecaster.config.enable_seasonal_adjustments);
        assert_eq!(forecaster.config.confidence_threshold, 0.75);
    }

    #[test]
    fn test_add_data_point() {
        let mut forecaster = ResourceForecaster::new();

        let data_point = DataPoint {
            timestamp: std::time::Instant::now(),
            value: 0.75,
            metadata: HashMap::new(),
        };

        forecaster.add_data_point(ResourceType::Memory, data_point);
        assert_eq!(forecaster.historical_data.len(), 1);
        assert_eq!(forecaster.historical_data[&ResourceType::Memory].len(), 1);
    }

    #[test]
    fn test_linear_trend_calculation() {
        let forecaster = ResourceForecaster::new();

        // Test increasing trend
        let increasing_values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let slope = forecaster.calculate_linear_trend(&increasing_values);
        assert!(
            slope > 0.9 && slope < 1.1,
            "Expected slope ~1.0, got {}",
            slope
        );

        // Test decreasing trend
        let decreasing_values = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        let slope = forecaster.calculate_linear_trend(&decreasing_values);
        assert!(
            slope < -0.9 && slope > -1.1,
            "Expected slope ~-1.0, got {}",
            slope
        );

        // Test flat trend
        let flat_values = vec![3.0, 3.0, 3.0, 3.0, 3.0];
        let slope = forecaster.calculate_linear_trend(&flat_values);
        assert!(slope.abs() < 0.01, "Expected slope ~0.0, got {}", slope);
    }

    #[test]
    fn test_forecast_with_sufficient_data() {
        let mut forecaster = ResourceForecaster::new();
        forecaster.config.time_horizon = Duration::from_secs(3600); // 1 hour for testing

        // Add trending data points
        let base_time = std::time::Instant::now();
        for i in 0..10 {
            let data_point = DataPoint {
                timestamp: base_time + Duration::from_secs(i * 360), // 6-minute intervals
                value: 0.5 + (i as f64 * 0.02),                      // Slight upward trend
                metadata: HashMap::new(),
            };
            forecaster.add_data_point(ResourceType::CPU, data_point);
        }

        let forecast = forecaster.forecast_resource(&ResourceType::CPU);
        assert!(forecast.is_ok());

        let result = forecast.unwrap();
        assert_eq!(result.resource_type, ResourceType::CPU);
        assert!(!result.projected_values.is_empty());
        assert!(result.confidence > 0.0);
        assert!(!result.trends.is_empty());
    }

    #[test]
    fn test_forecast_with_insufficient_data() {
        let mut forecaster = ResourceForecaster::new();

        // Add only one data point
        let data_point = DataPoint {
            timestamp: std::time::Instant::now(),
            value: 0.75,
            metadata: HashMap::new(),
        };
        forecaster.add_data_point(ResourceType::Memory, data_point);

        let forecast = forecaster.forecast_resource(&ResourceType::Memory);
        assert!(forecast.is_err());
    }

    #[test]
    fn test_autocorrelation_calculation() {
        let forecaster = ResourceForecaster::new();

        // Create periodic data
        let mut values = Vec::new();
        for i in 0..48 {
            let hour = i % 24;
            let value = if (8..=18).contains(&hour) {
                0.8 // High during day
            } else {
                0.3 // Low during night
            };
            values.push(value);
        }

        // Check for 24-hour correlation
        let correlation_24h = forecaster.calculate_autocorrelation(&values, 24);
        assert!(
            correlation_24h > 0.5,
            "Expected high correlation for 24h period, got {}",
            correlation_24h
        );

        // Check for 12-hour correlation (should be negative)
        let correlation_12h = forecaster.calculate_autocorrelation(&values, 12);
        assert!(
            correlation_12h < -0.3,
            "Expected negative correlation for 12h period, got {}",
            correlation_12h
        );
    }

    #[test]
    fn test_forecast_recommendations() {
        let forecaster = ResourceForecaster::new();

        // Create projections that will trigger scaling recommendation
        let projections = vec![ProjectedValue {
            time_offset: Duration::from_secs(3600),
            predicted_value: 0.85, // High utilization
            confidence_lower: 0.8,
            confidence_upper: 0.9,
            factors: vec!["Test trend".to_string()],
        }];

        let trends = vec![TrendComponent {
            trend_type: TrendType::Linear(0.05), // Increasing trend
            strength: 0.7,
            duration: Duration::from_secs(3600),
            factors: vec!["Growth pattern".to_string()],
        }];

        let recommendations = forecaster
            .generate_forecast_recommendations(&ResourceType::CPU, &projections, &trends)
            .unwrap();

        assert!(!recommendations.is_empty());
        let scaling_rec = recommendations
            .iter()
            .find(|r| {
                matches!(
                    r.recommendation_type,
                    ForecastRecommendationType::PreemptiveScaling
                )
            })
            .unwrap();

        assert_eq!(scaling_rec.urgency, RecommendationPriority::High);
        assert!(scaling_rec.description.contains("Scale up"));
    }
}
