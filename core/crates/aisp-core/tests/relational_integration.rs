//! Relational analysis integration tests (Level 4)
//!
//! This module tests Level 4 relational logic analysis including type
//! relationships, dependency analysis, and relational consistency checking.
//!
//! Note: These tests use deprecated relational analysis APIs.

// Skip this entire test file - it uses deprecated APIs
#![cfg(feature = "relational-integration-deprecated")]

use aisp_core::{
    AispDocument, AispParser, CircularDependency, ConflictDetectionResult, CycleSeverity,
    DependencyAnalysisResult, RelationType, RelationalAnalysisResult, RelationalAnalyzer,
    TypeGraphResult,
};

/// Builder for creating relational analysis test scenarios
pub struct RelationalTestBuilder {
    document_source: String,
    expected_consistency: Option<f64>,
    expected_conflicts: usize,
    expected_circular_deps: usize,
}

impl RelationalTestBuilder {
    pub fn new(document_source: &str) -> Self {
        Self {
            document_source: document_source.to_string(),
            expected_consistency: None,
            expected_conflicts: 0,
            expected_circular_deps: 0,
        }
    }

    pub fn expecting_consistency(mut self, consistency: f64) -> Self {
        self.expected_consistency = Some(consistency);
        self
    }

    pub fn expecting_conflicts(mut self, count: usize) -> Self {
        self.expected_conflicts = count;
        self
    }

    pub fn expecting_circular_dependencies(mut self, count: usize) -> Self {
        self.expected_circular_deps = count;
        self
    }

    pub fn test_relational_analysis(self) -> RelationalResult {
        let parser = AispParser::new();
        let document = parser
            .parse(&self.document_source)
            .expect("Document should parse successfully for relational analysis");

        let mut analyzer = RelationalAnalyzer::new();
        let result = analyzer.analyze(&document);

        // Verify consistency score if specified
        if let Some(expected_consistency) = self.expected_consistency {
            let actual_consistency = result.consistency_score;
            if (actual_consistency - expected_consistency).abs() > 0.1 {
                panic!(
                    "Expected consistency score ~{} but got {}",
                    expected_consistency, actual_consistency
                );
            }
        }

        // Verify conflict count
        if result.conflicts.len() != self.expected_conflicts {
            panic!(
                "Expected {} conflicts but got {}: {:?}",
                self.expected_conflicts,
                result.conflicts.len(),
                result.conflicts
            );
        }

        // Verify circular dependency count
        if result.circular_dependencies.len() != self.expected_circular_deps {
            panic!(
                "Expected {} circular dependencies but got {}: {:?}",
                self.expected_circular_deps,
                result.circular_dependencies.len(),
                result.circular_dependencies
            );
        }

        RelationalResult::new(document, result)
    }
}

/// Helper for asserting relational analysis results
pub struct RelationalResult {
    _document: AispDocument,
    analysis: RelationalAnalysisResult,
}

impl RelationalResult {
    pub fn new(document: AispDocument, analysis: RelationalAnalysisResult) -> Self {
        Self {
            _document: document,
            analysis,
        }
    }

    pub fn has_type_relationships(self, count: usize) -> Self {
        assert_eq!(
            self.analysis.type_relationships.len(),
            count,
            "Expected {} type relationships but got {}",
            count,
            self.analysis.type_relationships.len()
        );
        self
    }

    pub fn has_dependency_depth(self, component: &str, expected_depth: usize) -> Self {
        let actual_depth = self
            .analysis
            .dependency_depths
            .get(component)
            .expect(&format!(
                "Component '{}' should have dependency depth",
                component
            ));
        assert_eq!(
            *actual_depth, expected_depth,
            "Expected depth {} for '{}' but got {}",
            expected_depth, component, actual_depth
        );
        self
    }

    pub fn has_topological_order_before(self, first: &str, second: &str) -> Self {
        let first_pos = self
            .analysis
            .topological_order
            .iter()
            .position(|x| x == first)
            .expect(&format!("'{}' should be in topological order", first));
        let second_pos = self
            .analysis
            .topological_order
            .iter()
            .position(|x| x == second)
            .expect(&format!("'{}' should be in topological order", second));

        assert!(
            first_pos < second_pos,
            "'{}' should come before '{}' in topological order",
            first,
            second
        );
        self
    }

    pub fn has_conflict_containing(self, message_fragment: &str) -> Self {
        let found = self
            .analysis
            .conflicts
            .iter()
            .any(|conflict| conflict.description.contains(message_fragment));
        assert!(
            found,
            "Expected conflict containing '{}' but conflicts were: {:?}",
            message_fragment, self.analysis.conflicts
        );
        self
    }

    pub fn has_circular_dependency(self, components: &[&str]) -> Self {
        let found = self
            .analysis
            .circular_dependencies
            .iter()
            .any(|circular_dep| {
                circular_dep.cycle.len() == components.len()
                    && components
                        .iter()
                        .all(|comp| circular_dep.cycle.contains(&comp.to_string()))
            });
        assert!(
            found,
            "Expected circular dependency involving {:?} but found: {:?}",
            components, self.analysis.circular_dependencies
        );
        self
    }

    pub fn has_consistency_above(self, threshold: f64) -> Self {
        assert!(
            self.analysis.consistency_score >= threshold,
            "Expected consistency >= {} but got {}",
            threshold,
            self.analysis.consistency_score
        );
        self
    }
}

#[test]
fn test_simple_type_relationships() {
    let document = r#"𝔸5.1.SimpleRelations@2026-01-25

⟦Σ:Types⟧{
  BaseType≜ℕ
  DerivedType≜BaseType
  ContainerType≜BaseType[10]
}

⟦Ω:Meta⟧{
  domain≜simple_relations
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_consistency(1.0)
        .expecting_conflicts(0)
        .test_relational_analysis()
        .has_type_relationships(2) // DerivedType->BaseType, ContainerType->BaseType
        .has_consistency_above(0.9);
}

#[test]
fn test_dependency_analysis_ordering() {
    let document = r#"𝔸5.1.DependencyTest@2026-01-25

⟦Σ:Types⟧{
  A≜ℕ
  B≜A
  C≜B
  D≜{value:C, meta:A}
}

⟦Ω:Meta⟧{
  domain≜dependency_test
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_consistency(1.0)
        .test_relational_analysis()
        .has_dependency_depth("A", 1) // No dependencies
        .has_dependency_depth("B", 2) // Depends on A
        .has_dependency_depth("C", 3) // Depends on B->A
        .has_dependency_depth("D", 4) // Depends on C->B->A and A
        .has_topological_order_before("A", "B")
        .has_topological_order_before("B", "C")
        .has_topological_order_before("C", "D");
}

#[test]
fn test_circular_dependency_detection() {
    let document = r#"𝔸5.1.CircularTest@2026-01-25

⟦Σ:Types⟧{
  TypeA≜TypeB
  TypeB≜TypeC
  TypeC≜TypeA
  IndependentType≜ℕ
}

⟦Ω:Meta⟧{
  domain≜circular_test
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_circular_dependencies(1)
        .expecting_conflicts(1) // Circular dependency creates conflict
        .test_relational_analysis()
        .has_circular_dependency(&["TypeA", "TypeB", "TypeC"])
        .has_conflict_containing("circular");
}

#[test]
fn test_complex_type_relationships() {
    let document = r#"𝔸5.1.ComplexRelations@2026-01-25

⟦Σ:Types⟧{
  Primitive≜ℕ
  Enhanced≜{value:Primitive, metadata:𝕊}
  Collection≜Enhanced[5]
  Transform≜Primitive→Enhanced
  Processor≜Collection→Transform
  Result≜{input:Collection, output:Transform, processor:Processor}
}

⟦Λ:Funcs⟧{
  enhance≜λ(p:Primitive).Create(p)
  collect≜λ(items:Enhanced[]).ToCollection(items)
  process≜λ(c:Collection).Transform(c)
}

⟦Γ:Rules⟧{
  ∀p:Primitive→Valid(p)
  ∀e:Enhanced→Consistent(e.value,e.metadata)
  ∀c:Collection→Length(c)≤5
}

⟦Ω:Meta⟧{
  domain≜complex_relations
  version≜"1.0.0"
}

⟦Ε⟧⟨δ≜0.85⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_consistency(1.0)
        .expecting_conflicts(0)
        .test_relational_analysis()
        .has_type_relationships(6) // Multiple complex relationships
        .has_dependency_depth("Primitive", 1)
        .has_dependency_depth("Enhanced", 2)
        .has_dependency_depth("Collection", 3)
        .has_dependency_depth("Result", 4) // Depends on all others
        .has_consistency_above(0.9);
}

#[test]
fn test_function_type_relationships() {
    let document = r#"𝔸5.1.FunctionRelations@2026-01-25

⟦Σ:Types⟧{
  Input≜ℕ
  Output≜𝔹
  SimpleFunc≜Input→Output
  HigherOrderFunc≜SimpleFunc→SimpleFunc
  CombinedFunc≜(Input,SimpleFunc)→Output
}

⟦Λ:Funcs⟧{
  basic≜λ(x:Input).x>0
  transform≜λ(f:SimpleFunc).λy.f(y)∧True
  combine≜λ(x:Input,f:SimpleFunc).f(x)
}

⟦Γ:Rules⟧{
  ∀f:SimpleFunc→∀x:Input→f(x)∈Output
  ∀h:HigherOrderFunc→∀g:SimpleFunc→h(g)∈SimpleFunc
}

⟦Ω:Meta⟧{
  domain≜function_relations
}

⟦Ε⟧⟨δ≜0.82⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_consistency(1.0)
        .test_relational_analysis()
        .has_type_relationships(4) // Function type relationships
        .has_topological_order_before("Input", "SimpleFunc")
        .has_topological_order_before("Output", "SimpleFunc")
        .has_topological_order_before("SimpleFunc", "HigherOrderFunc")
        .has_consistency_above(0.9);
}

#[test]
fn test_relational_conflict_detection() {
    let document = r#"𝔸5.1.ConflictTest@2026-01-25

⟦Σ:Types⟧{
  BaseType≜ℕ
  ConflictType≜{valid:𝔹, invalid:𝔹}
  InconsistentType≜BaseType
  InconsistentType≜𝔹  # Duplicate definition
}

⟦Λ:Funcs⟧{
  conflictFunc≜λx:UndefinedType.Process(x)
  validFunc≜λy:BaseType.IsValid(y)
}

⟦Γ:Rules⟧{
  ∀x:BaseType→x≥0
  ∀x:BaseType→x<0  # Contradictory constraint
  ∀c:ConflictType→c.valid∧¬c.invalid
}

⟦Ω:Meta⟧{
  domain≜conflict_test
}

⟦Ε⟧⟨δ≜0.5⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_conflicts(3) // Multiple conflicts expected
        .test_relational_analysis()
        .has_conflict_containing("duplicate")
        .has_conflict_containing("undefined")
        .has_conflict_containing("contradiction");
}

#[test]
fn test_deep_dependency_chains() {
    let document = r#"𝔸5.1.DeepDependencies@2026-01-25

⟦Σ:Types⟧{
  Level1≜ℕ
  Level2≜Level1
  Level3≜Level2
  Level4≜Level3
  Level5≜Level4
  Level6≜Level5
  Level7≜Level6
  Level8≜Level7
  ComplexType≜{l1:Level1, l4:Level4, l8:Level8}
}

⟦Λ:Funcs⟧{
  processL1≜λx:Level1.Basic(x)
  processL4≜λx:Level4.Intermediate(x)
  processL8≜λx:Level8.Advanced(x)
  processComplex≜λc:ComplexType.Combine(c.l1,c.l4,c.l8)
}

⟦Ω:Meta⟧{
  domain≜deep_dependencies
}

⟦Ε⟧⟨δ≜0.8⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_consistency(1.0)
        .test_relational_analysis()
        .has_dependency_depth("Level1", 1)
        .has_dependency_depth("Level4", 4)
        .has_dependency_depth("Level8", 8)
        .has_dependency_depth("ComplexType", 9) // Max(1,4,8) + 1
        .has_topological_order_before("Level1", "Level8")
        .has_topological_order_before("Level8", "ComplexType");
}

#[test]
fn test_multiple_circular_dependencies() {
    let document = r#"𝔸5.1.MultiCircular@2026-01-25

⟦Σ:Types⟧{
  # First circular group
  CircleA1≜CircleA2
  CircleA2≜CircleA1
  
  # Second circular group  
  CircleB1≜CircleB2
  CircleB2≜CircleB3
  CircleB3≜CircleB1
  
  # Independent type
  Independent≜ℕ
}

⟦Ω:Meta⟧{
  domain≜multi_circular
}

⟦Ε⟧⟨δ≜0.6⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_circular_dependencies(2) // Two separate cycles
        .expecting_conflicts(2) // Each cycle creates a conflict
        .test_relational_analysis()
        .has_circular_dependency(&["CircleA1", "CircleA2"])
        .has_circular_dependency(&["CircleB1", "CircleB2", "CircleB3"]);
}

#[test]
fn test_relational_consistency_metrics() {
    let document = r#"𝔸5.1.ConsistencyMetrics@2026-01-25

⟦Σ:Types⟧{
  WellFormedType≜{id:ℕ, name:𝕊, active:𝔹}
  ConsistentType≜WellFormedType
  ValidatedType≜ConsistentType
  QualityType≜{base:ValidatedType, score:ℝ}
}

⟦Λ:Funcs⟧{
  validate≜λ(w:WellFormedType).Check(w)
  enhance≜λ(c:ConsistentType).Improve(c)
  score≜λ(v:ValidatedType).Calculate(v)
  quality≜λ(q:QualityType).Assess(q.base,q.score)
}

⟦Γ:Rules⟧{
  ∀w:WellFormedType→w.id>0∧Length(w.name)>0
  ∀c:ConsistentType→Valid(c)
  ∀v:ValidatedType→Verified(v)
  ∀q:QualityType→q.score≥0∧q.score≤1
}

⟦Ω:Meta⟧{
  domain≜consistency_metrics
  version≜"1.0.0"
  description≜"Testing relational consistency calculations"
  ∀T∈Types:WellFormed(T)
  ∀F∈Functions:TypeSafe(F)
  ∀R∈Rules:Consistent(R)
}

⟦Ε⟧⟨δ≜0.92;φ≜120⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_consistency(1.0)
        .expecting_conflicts(0)
        .test_relational_analysis()
        .has_type_relationships(6) // Clean dependency chain
        .has_dependency_depth("WellFormedType", 1)
        .has_dependency_depth("ConsistentType", 2)
        .has_dependency_depth("ValidatedType", 3)
        .has_dependency_depth("QualityType", 4)
        .has_consistency_above(0.95);
}

#[test]
fn test_relational_analysis_with_generics() {
    let document = r#"𝔸5.1.GenericRelations@2026-01-25

⟦Σ:Types⟧{
  Element≜ℕ
  Container≜Element[]
  Pair≜(Element,Element)
  Transformer≜Element→Element
  GenericProcessor≜(Container,Transformer)→Container
}

⟦Λ:Funcs⟧{
  createPair≜λ(a:Element,b:Element).(a,b)
  transform≜λ(c:Container,t:Transformer).Map(c,t)
  process≜λ(elements:Container).Sort(elements)
}

⟦Γ:Rules⟧{
  ∀e:Element→e≥0
  ∀c:Container→Length(c)≥0
  ∀p:Pair→p.0≤p.1
  ∀t:Transformer→∀x:Element→t(x)≥x
}

⟦Ω:Meta⟧{
  domain≜generic_relations
}

⟦Ε⟧⟨δ≜0.87⟩"#;

    RelationalTestBuilder::new(document)
        .expecting_consistency(1.0)
        .test_relational_analysis()
        .has_type_relationships(4) // Generic type relationships
        .has_topological_order_before("Element", "Container")
        .has_topological_order_before("Element", "Pair")
        .has_topological_order_before("Element", "Transformer")
        .has_topological_order_before("Transformer", "GenericProcessor")
        .has_consistency_above(0.9);
}
