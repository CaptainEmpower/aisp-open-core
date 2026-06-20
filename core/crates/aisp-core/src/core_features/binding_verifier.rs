//! F₄: Four-State Binding Verifier
//!
//! Implements the four-state binding system: Δ⊗λ∈{0,1,2,3}

use super::types::*;
use crate::error::AispResult;
use std::collections::HashMap;

/// Four-state binding verifier with formal guarantees
pub struct FourStateBindingVerifier {
    /// Deterministic binding rules
    binding_rules: HashMap<(TypeSignature, TypeSignature), BindingState>,
    /// Logical consistency constraints
    consistency_checker: LogicalConsistencyChecker,
    /// Socket compatibility database
    socket_registry: SocketCompatibilityRegistry,
    /// Adaptation strategy repository
    adaptation_strategies: AdaptationStrategyRepository,
}

/// Logical consistency checker for crash detection
pub struct LogicalConsistencyChecker {
    /// Known contradictory patterns
    contradiction_patterns: Vec<ContradictionPattern>,
    /// Logical axioms for validation
    logical_axioms: Vec<LogicalAxiom>,
}

/// Socket compatibility registry
pub struct SocketCompatibilityRegistry {
    /// Socket interface definitions
    socket_interfaces: HashMap<String, SocketInterface>,
    /// Compatibility matrix
    compatibility_matrix: HashMap<(String, String), CompatibilityLevel>,
}

/// Adaptation strategy repository
pub struct AdaptationStrategyRepository {
    /// Available adaptation patterns
    strategies: HashMap<(String, String), AdaptationStrategy>,
    /// Success rate tracking
    success_rates: HashMap<String, f64>,
}

impl FourStateBindingVerifier {
    /// Create new binding verifier with formal rules
    pub fn new() -> Self {
        Self {
            binding_rules: HashMap::new(),
            consistency_checker: LogicalConsistencyChecker::new(),
            socket_registry: SocketCompatibilityRegistry::new(),
            adaptation_strategies: AdaptationStrategyRepository::new(),
        }
    }

    /// Verify binding state between two components
    /// Implements: ∀A,B:|{Δ⊗λ(A,B)}|≡1 (deterministic binding)
    pub fn verify_binding(
        &self,
        component_a: &TypeSignature,
        component_b: &TypeSignature,
    ) -> AispResult<BindingState> {
        // Check cache for pre-computed binding
        if let Some(&cached_state) = self
            .binding_rules
            .get(&(component_a.clone(), component_b.clone()))
        {
            return Ok(cached_state);
        }

        // Phase 1: Logical consistency check (crash detection)
        if self.has_logical_contradiction(component_a, component_b)? {
            return Ok(BindingState::Crash);
        }

        // Phase 2: Socket compatibility check (null detection)
        let socket_compatibility = self.check_socket_compatibility(component_a, component_b)?;
        if socket_compatibility == CompatibilityLevel::Incompatible {
            return Ok(BindingState::Null);
        }

        // Phase 3: Type compatibility check (adaptation vs zero-cost)
        let type_compatibility = self.check_type_compatibility(component_a, component_b)?;

        let binding_state = match (socket_compatibility, type_compatibility) {
            (CompatibilityLevel::Perfect, true) => BindingState::Zero,
            (CompatibilityLevel::Perfect, false) => BindingState::Adapt,
            (CompatibilityLevel::Adaptable, _) => BindingState::Adapt,
            (CompatibilityLevel::Incompatible, _) => BindingState::Null,
        };

        Ok(binding_state)
    }

    /// Cache binding rule for future lookups
    pub fn cache_binding_rule(
        &mut self,
        component_a: TypeSignature,
        component_b: TypeSignature,
        binding_state: BindingState,
    ) {
        self.binding_rules
            .insert((component_a, component_b), binding_state);
    }

    /// Get adaptation strategy for components requiring adaptation
    pub fn get_adaptation_strategy(
        &self,
        source_type: &str,
        target_type: &str,
    ) -> Option<&AdaptationStrategy> {
        self.adaptation_strategies
            .strategies
            .get(&(source_type.to_string(), target_type.to_string()))
    }

    /// Add custom socket interface
    pub fn register_socket_interface(&mut self, interface: SocketInterface) {
        self.socket_registry
            .socket_interfaces
            .insert(interface.interface_id.clone(), interface);
    }

    /// Set compatibility between socket types
    pub fn set_socket_compatibility(
        &mut self,
        interface_a: String,
        interface_b: String,
        compatibility: CompatibilityLevel,
    ) {
        self.socket_registry
            .compatibility_matrix
            .insert((interface_a, interface_b), compatibility);
    }

    /// Add logical contradiction pattern
    pub fn add_contradiction_pattern(&mut self, pattern: ContradictionPattern) {
        self.consistency_checker
            .contradiction_patterns
            .push(pattern);
    }

    /// Check for logical contradictions
    fn has_logical_contradiction(
        &self,
        component_a: &TypeSignature,
        component_b: &TypeSignature,
    ) -> AispResult<bool> {
        for pattern in &self.consistency_checker.contradiction_patterns {
            if self.matches_contradiction_pattern(component_a, component_b, pattern) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Check socket compatibility
    fn check_socket_compatibility(
        &self,
        component_a: &TypeSignature,
        component_b: &TypeSignature,
    ) -> AispResult<CompatibilityLevel> {
        let interface_a = component_a.base_type.clone();
        let interface_b = component_b.base_type.clone();

        // Identical types are always perfectly compatible
        if interface_a == interface_b {
            return Ok(CompatibilityLevel::Perfect);
        }

        Ok(self
            .socket_registry
            .compatibility_matrix
            .get(&(interface_a, interface_b))
            .cloned()
            .unwrap_or(CompatibilityLevel::Incompatible))
    }

    /// Check type compatibility
    fn check_type_compatibility(
        &self,
        component_a: &TypeSignature,
        component_b: &TypeSignature,
    ) -> AispResult<bool> {
        // Perfect match: identical types and parameters
        if component_a.base_type == component_b.base_type
            && component_a.parameters == component_b.parameters
        {
            return Ok(true);
        }

        // Check if types are structurally compatible
        if self.are_structurally_compatible(component_a, component_b) {
            return Ok(true);
        }

        // Check constraints compatibility
        self.check_constraints_compatibility(component_a, component_b)
    }

    /// Check if types are structurally compatible
    fn are_structurally_compatible(
        &self,
        component_a: &TypeSignature,
        component_b: &TypeSignature,
    ) -> bool {
        // Simple heuristic: check if parameter count matches
        component_a.parameters.len() == component_b.parameters.len()
    }

    /// Check constraints compatibility
    fn check_constraints_compatibility(
        &self,
        component_a: &TypeSignature,
        component_b: &TypeSignature,
    ) -> AispResult<bool> {
        // Check if constraints are mutually satisfiable
        for constraint_a in &component_a.constraints {
            for constraint_b in &component_b.constraints {
                if self.are_constraints_conflicting(constraint_a, constraint_b) {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    /// Check if constraints are conflicting
    fn are_constraints_conflicting(&self, constraint_a: &str, constraint_b: &str) -> bool {
        // Simple conflict detection
        (constraint_a.contains("Pure") && constraint_b.contains("Impure"))
            || (constraint_a.contains("Immutable") && constraint_b.contains("Mutable"))
            || (constraint_a.contains("Sync") && constraint_b.contains("Async"))
    }

    /// Check if components match contradiction pattern
    fn matches_contradiction_pattern(
        &self,
        component_a: &TypeSignature,
        component_b: &TypeSignature,
        pattern: &ContradictionPattern,
    ) -> bool {
        let a_matches_antecedent = component_a.base_type.contains(&pattern.antecedent);
        let b_matches_consequent = component_b.base_type.contains(&pattern.consequent);

        a_matches_antecedent && b_matches_consequent
    }

    /// Get binding statistics
    pub fn get_binding_statistics(&self) -> BindingStatistics {
        let mut stats = BindingStatistics::default();

        for &binding_state in self.binding_rules.values() {
            match binding_state {
                BindingState::Crash => stats.crash_count += 1,
                BindingState::Null => stats.null_count += 1,
                BindingState::Adapt => stats.adapt_count += 1,
                BindingState::Zero => stats.zero_count += 1,
            }
        }

        stats.total_bindings = self.binding_rules.len();
        stats
    }
}

/// Binding statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct BindingStatistics {
    pub total_bindings: usize,
    pub crash_count: usize,
    pub null_count: usize,
    pub adapt_count: usize,
    pub zero_count: usize,
}

impl LogicalConsistencyChecker {
    /// Create new consistency checker
    pub fn new() -> Self {
        let mut checker = Self {
            contradiction_patterns: Vec::new(),
            logical_axioms: Vec::new(),
        };
        checker.load_default_patterns();
        checker
    }

    /// Load default contradiction patterns
    fn load_default_patterns(&mut self) {
        // Add standard logical contradictions
        self.contradiction_patterns.push(ContradictionPattern {
            pattern_name: "Sync-Async Contradiction".to_string(),
            antecedent: "Sync".to_string(),
            consequent: "Async".to_string(),
            contradiction_proof: "Synchronous and asynchronous operations are mutually exclusive"
                .to_string(),
        });

        self.contradiction_patterns.push(ContradictionPattern {
            pattern_name: "Pure-Impure Contradiction".to_string(),
            antecedent: "Pure".to_string(),
            consequent: "Impure".to_string(),
            contradiction_proof: "Pure functions cannot have side effects".to_string(),
        });

        self.contradiction_patterns.push(ContradictionPattern {
            pattern_name: "Immutable-Mutable Contradiction".to_string(),
            antecedent: "Immutable".to_string(),
            consequent: "Mutable".to_string(),
            contradiction_proof: "Immutable values cannot be modified".to_string(),
        });
    }

    /// Add logical axiom
    pub fn add_axiom(&mut self, axiom: LogicalAxiom) {
        self.logical_axioms.push(axiom);
    }

    /// Validate logical consistency
    pub fn validate_consistency(&self, type_sig: &TypeSignature) -> AispResult<bool> {
        // Check internal consistency of type signature
        for constraint in &type_sig.constraints {
            if self.is_self_contradictory(constraint) {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Check if constraint is self-contradictory
    fn is_self_contradictory(&self, constraint: &str) -> bool {
        constraint.contains("Pure") && constraint.contains("Impure")
            || constraint.contains("Sync") && constraint.contains("Async")
            || constraint.contains("Immutable") && constraint.contains("Mutable")
    }
}

impl SocketCompatibilityRegistry {
    /// Create new socket registry
    pub fn new() -> Self {
        let mut registry = Self {
            socket_interfaces: HashMap::new(),
            compatibility_matrix: HashMap::new(),
        };
        registry.load_default_interfaces();
        registry
    }

    /// Load default socket interfaces
    fn load_default_interfaces(&mut self) {
        let rest_interface = SocketInterface {
            interface_id: "REST".to_string(),
            required_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
            ],
            provided_capabilities: vec!["HTTP".to_string(), "JSON".to_string()],
            communication_protocol: "HTTP/1.1".to_string(),
        };

        let graphql_interface = SocketInterface {
            interface_id: "GraphQL".to_string(),
            required_methods: vec![
                "query".to_string(),
                "mutation".to_string(),
                "subscription".to_string(),
            ],
            provided_capabilities: vec!["GraphQL".to_string(), "JSON".to_string()],
            communication_protocol: "HTTP/1.1".to_string(),
        };

        self.socket_interfaces
            .insert("REST".to_string(), rest_interface);
        self.socket_interfaces
            .insert("GraphQL".to_string(), graphql_interface);

        // Set compatibility levels
        self.compatibility_matrix.insert(
            ("REST".to_string(), "GraphQL".to_string()),
            CompatibilityLevel::Adaptable,
        );
        self.compatibility_matrix.insert(
            ("GraphQL".to_string(), "REST".to_string()),
            CompatibilityLevel::Adaptable,
        );
    }

    /// Get interface definition
    pub fn get_interface(&self, interface_id: &str) -> Option<&SocketInterface> {
        self.socket_interfaces.get(interface_id)
    }

    /// Get compatibility level
    pub fn get_compatibility(&self, interface_a: &str, interface_b: &str) -> CompatibilityLevel {
        self.compatibility_matrix
            .get(&(interface_a.to_string(), interface_b.to_string()))
            .cloned()
            .unwrap_or(CompatibilityLevel::Incompatible)
    }
}

impl AdaptationStrategyRepository {
    /// Create new strategy repository
    pub fn new() -> Self {
        let mut repo = Self {
            strategies: HashMap::new(),
            success_rates: HashMap::new(),
        };
        repo.load_default_strategies();
        repo
    }

    /// Load default adaptation strategies
    fn load_default_strategies(&mut self) {
        let rest_to_graphql = AdaptationStrategy {
            strategy_id: "REST_to_GraphQL".to_string(),
            source_type: "REST".to_string(),
            target_type: "GraphQL".to_string(),
            adaptation_code: "transform_rest_to_graphql".to_string(),
            cost_estimate: 2.5,
            success_probability: 0.8,
        };

        self.strategies
            .insert(("REST".to_string(), "GraphQL".to_string()), rest_to_graphql);
        self.success_rates
            .insert("REST_to_GraphQL".to_string(), 0.8);
    }

    /// Add adaptation strategy
    pub fn add_strategy(&mut self, strategy: AdaptationStrategy) {
        let key = (strategy.source_type.clone(), strategy.target_type.clone());
        self.success_rates
            .insert(strategy.strategy_id.clone(), strategy.success_probability);
        self.strategies.insert(key, strategy);
    }

    /// Get strategy success rate
    pub fn get_success_rate(&self, strategy_id: &str) -> Option<f64> {
        self.success_rates.get(strategy_id).copied()
    }

    /// Update success rate based on actual results
    pub fn update_success_rate(&mut self, strategy_id: &str, success: bool) {
        if let Some(current_rate) = self.success_rates.get_mut(strategy_id) {
            // Simple exponential moving average
            let alpha = 0.1;
            let new_value = if success { 1.0 } else { 0.0 };
            *current_rate = alpha * new_value + (1.0 - alpha) * *current_rate;
        }
    }
}

impl Default for FourStateBindingVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binding_verifier_creation() {
        let verifier = FourStateBindingVerifier::new();
        assert_eq!(verifier.binding_rules.len(), 0);
    }

    #[test]
    fn test_identical_type_binding() {
        let verifier = FourStateBindingVerifier::new();
        let type_sig = TypeSignature::new("String".to_string());

        let result = verifier.verify_binding(&type_sig, &type_sig);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BindingState::Zero);
    }

    #[test]
    fn test_contradictory_constraints() {
        let verifier = FourStateBindingVerifier::new();
        let type_a = TypeSignature::new("Function".to_string()).with_constraint("Pure".to_string());
        let type_b =
            TypeSignature::new("Function".to_string()).with_constraint("Impure".to_string());

        let compatibility = verifier
            .check_constraints_compatibility(&type_a, &type_b)
            .unwrap();
        assert!(!compatibility);
    }

    #[test]
    fn test_socket_compatibility() {
        let verifier = FourStateBindingVerifier::new();
        let rest_type = TypeSignature::new("REST".to_string());
        let graphql_type = TypeSignature::new("GraphQL".to_string());

        let compatibility = verifier
            .check_socket_compatibility(&rest_type, &graphql_type)
            .unwrap();
        assert_eq!(compatibility, CompatibilityLevel::Adaptable);
    }

    #[test]
    fn test_binding_cache() {
        let mut verifier = FourStateBindingVerifier::new();
        let type_a = TypeSignature::new("Type1".to_string());
        let type_b = TypeSignature::new("Type2".to_string());

        verifier.cache_binding_rule(type_a.clone(), type_b.clone(), BindingState::Adapt);

        let result = verifier.verify_binding(&type_a, &type_b).unwrap();
        assert_eq!(result, BindingState::Adapt);
    }

    #[test]
    fn test_logical_consistency_checker() {
        let checker = LogicalConsistencyChecker::new();
        assert!(!checker.contradiction_patterns.is_empty());

        let valid_type =
            TypeSignature::new("Function".to_string()).with_constraint("Pure".to_string());
        assert!(checker.validate_consistency(&valid_type).unwrap());

        let invalid_type =
            TypeSignature::new("Function".to_string()).with_constraint("Pure Impure".to_string());
        assert!(!checker.validate_consistency(&invalid_type).unwrap());
    }

    #[test]
    fn test_socket_registry() {
        let registry = SocketCompatibilityRegistry::new();

        let rest_interface = registry.get_interface("REST");
        assert!(rest_interface.is_some());
        assert_eq!(rest_interface.unwrap().interface_id, "REST");

        let compatibility = registry.get_compatibility("REST", "GraphQL");
        assert_eq!(compatibility, CompatibilityLevel::Adaptable);
    }

    #[test]
    fn test_adaptation_strategy_repository() {
        let mut repo = AdaptationStrategyRepository::new();

        let strategy = repo
            .strategies
            .get(&("REST".to_string(), "GraphQL".to_string()));
        assert!(strategy.is_some());

        let success_rate = repo.get_success_rate("REST_to_GraphQL");
        assert!(success_rate.is_some());
        assert_eq!(success_rate.unwrap(), 0.8);

        // Test success rate update
        repo.update_success_rate("REST_to_GraphQL", false);
        let updated_rate = repo.get_success_rate("REST_to_GraphQL").unwrap();
        assert!(updated_rate < 0.8);
    }

    #[test]
    fn test_binding_statistics() {
        let mut verifier = FourStateBindingVerifier::new();

        // Add some test bindings
        verifier.cache_binding_rule(
            TypeSignature::new("A".to_string()),
            TypeSignature::new("B".to_string()),
            BindingState::Zero,
        );
        verifier.cache_binding_rule(
            TypeSignature::new("C".to_string()),
            TypeSignature::new("D".to_string()),
            BindingState::Adapt,
        );
        verifier.cache_binding_rule(
            TypeSignature::new("E".to_string()),
            TypeSignature::new("F".to_string()),
            BindingState::Crash,
        );

        let stats = verifier.get_binding_statistics();
        assert_eq!(stats.total_bindings, 3);
        assert_eq!(stats.zero_count, 1);
        assert_eq!(stats.adapt_count, 1);
        assert_eq!(stats.crash_count, 1);
        assert_eq!(stats.null_count, 0);
    }

    #[test]
    fn test_contradiction_pattern_matching() {
        let verifier = FourStateBindingVerifier::new();
        let sync_type = TypeSignature::new("SyncFunction".to_string());
        let async_type = TypeSignature::new("AsyncFunction".to_string());

        let pattern = &verifier.consistency_checker.contradiction_patterns[0];
        let matches = verifier.matches_contradiction_pattern(&sync_type, &async_type, pattern);
        assert!(matches);
    }
}
