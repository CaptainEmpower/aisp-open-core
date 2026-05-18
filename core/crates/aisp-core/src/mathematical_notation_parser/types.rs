//! Mathematical Notation Parser Types
//!
//! Core type definitions and configuration for mathematical notation parsing.

use crate::error::{AispError, AispResult};
use std::collections::HashMap;
use thiserror::Error;

/// Mathematical notation parsing errors
#[derive(Debug, Clone, Error)]
pub enum MathNotationError {
    #[error("Unknown mathematical symbol: '{symbol}' at position {position}")]
    UnknownSymbol { symbol: String, position: usize },

    #[error("Invalid mathematical expression: {expression} - {reason}")]
    InvalidExpression { expression: String, reason: String },

    #[error("Unsupported Unicode mathematical block: U+{codepoint:04X}")]
    UnsupportedUnicodeBlock { codepoint: u32 },

    #[error("Complex mathematical structure parsing failed: {structure_type}")]
    ComplexStructureFailure { structure_type: String },

    #[error("Parsing depth limit exceeded: {depth}")]
    DepthLimitExceeded { depth: usize },

    #[error("Invalid category theory construct: {construct}")]
    InvalidCategoryConstruct { construct: String },

    #[error("Malformed quantifier expression: {reason}")]
    MalformedQuantifier { reason: String },
}

/// Enhanced mathematical expression
#[derive(Debug, Clone, PartialEq)]
pub enum EnhancedMathExpression {
    /// Basic mathematical symbols
    BasicSymbol(String),
    /// Unicode mathematical operators
    UnicodeOperator {
        symbol: String,
        unicode_name: String,
        category: String,
    },
    /// Category theory constructs
    CategoryTheory { construct: CategoryConstruct },
    /// Complex mathematical structures
    ComplexStructure {
        structure_type: String,
        components: Vec<EnhancedMathExpression>,
    },
    /// Lambda calculus expressions
    Lambda {
        parameter: String,
        body: Box<EnhancedMathExpression>,
    },
    /// Quantified expressions
    Quantified {
        quantifier: Quantifier,
        variable: String,
        domain: String,
        body: Box<EnhancedMathExpression>,
    },
    /// Function application
    Application {
        function: Box<EnhancedMathExpression>,
        argument: Box<EnhancedMathExpression>,
    },
    /// Mathematical constants and sets
    Constant {
        name: String,
        symbol: String,
        set_type: Option<SetType>,
    },
    /// Binary operations
    BinaryOperation {
        operator: String,
        left: Box<EnhancedMathExpression>,
        right: Box<EnhancedMathExpression>,
        precedence: i32,
    },
    /// Unary operations
    UnaryOperation {
        operator: String,
        operand: Box<EnhancedMathExpression>,
    },
    /// Subscript/superscript notation
    ScriptNotation {
        base: Box<EnhancedMathExpression>,
        script_type: ScriptType,
        script: Box<EnhancedMathExpression>,
    },
}

/// Category theory constructs
#[derive(Debug, Clone, PartialEq)]
pub enum CategoryConstruct {
    /// Functor: F: C ⇒ D
    Functor {
        name: String,
        source: String,
        target: String,
    },
    /// Natural transformation: η: F ⇒ G
    NaturalTransformation {
        name: String,
        source_functor: String,
        target_functor: String,
    },
    /// Adjunction: L ⊣ R
    Adjunction {
        left_adjoint: String,
        right_adjoint: String,
    },
    /// Category: ⟨Objects, Morphisms, ∘, id⟩
    Category {
        name: String,
        objects: String,
        morphisms: String,
        composition: String,
        identity: String,
    },
    /// Monad: ⟨T, η, μ⟩
    Monad {
        endofunctor: String,
        unit: String,
        multiplication: String,
    },
    /// Morphism: f: A → B
    Morphism {
        name: String,
        source: String,
        target: String,
    },
    /// Composition: g ∘ f
    Composition { functions: Vec<String> },
    /// Identity morphism: id_A
    Identity { object: String },
}

/// Quantifier types
#[derive(Debug, Clone, PartialEq)]
pub enum Quantifier {
    /// Universal quantifier: ∀
    Forall,
    /// Existential quantifier: ∃
    Exists,
    /// Unique existence: ∃!
    ExistsUnique,
    /// Lambda abstraction: λ
    Lambda,
    /// Counting quantifier: ∃^n
    Counting(usize),
}

/// Mathematical set types
#[derive(Debug, Clone, PartialEq)]
pub enum SetType {
    /// Natural numbers: ℕ
    Natural,
    /// Integers: ℤ
    Integer,
    /// Rational numbers: ℚ
    Rational,
    /// Real numbers: ℝ
    Real,
    /// Complex numbers: ℂ
    Complex,
    /// Boolean algebra: 𝔹
    Boolean,
    /// Custom set type
    Custom(String),
}

/// Script notation types
#[derive(Debug, Clone, PartialEq)]
pub enum ScriptType {
    /// Subscript: x₁
    Subscript,
    /// Superscript: x¹
    Superscript,
    /// Both subscript and superscript
    Combined {
        subscript: Option<Box<EnhancedMathExpression>>,
        superscript: Option<Box<EnhancedMathExpression>>,
    },
}

/// Operator associativity
#[derive(Debug, Clone, PartialEq)]
pub enum Associativity {
    Left,
    Right,
    None,
}

/// Configuration for mathematical parsing
#[derive(Debug, Clone)]
pub struct MathParsingConfig {
    /// Enable category theory parsing
    pub enable_category_theory: bool,
    /// Enable advanced Unicode support
    pub enable_advanced_unicode: bool,
    /// Enable lambda calculus parsing
    pub enable_lambda_calculus: bool,
    /// Maximum parsing depth for complex expressions
    pub max_parsing_depth: usize,
    /// Enable strict parsing mode
    pub strict_mode: bool,
    /// Custom symbol registry
    pub custom_symbols: HashMap<String, UnicodeSymbolInfo>,
    /// Precedence rules
    pub precedence_rules: HashMap<String, i32>,
}

/// Information about Unicode mathematical symbols
#[derive(Debug, Clone)]
pub struct UnicodeSymbolInfo {
    /// Unicode symbol
    pub symbol: String,
    /// Official Unicode name
    pub unicode_name: String,
    /// Mathematical category
    pub category: String,
    /// LaTeX equivalent
    pub latex_equivalent: Option<String>,
    /// Parsing precedence
    pub precedence: i32,
    /// Associativity
    pub associativity: Associativity,
    /// Additional metadata
    pub metadata: SymbolMetadata,
}

/// Additional symbol metadata
#[derive(Debug, Clone)]
pub struct SymbolMetadata {
    /// Unicode block
    pub unicode_block: String,
    /// Usage frequency
    pub frequency: f64,
    /// Complexity level
    pub complexity: ComplexityLevel,
    /// Alternative representations
    pub alternatives: Vec<String>,
}

/// Symbol complexity levels
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum ComplexityLevel {
    Basic,
    Intermediate,
    Advanced,
    Expert,
    Specialized,
}

/// Information about category theory symbols
#[derive(Debug, Clone)]
pub struct CategorySymbolInfo {
    /// Symbol representation
    pub symbol: String,
    /// Category theory meaning
    pub meaning: String,
    /// Usage context
    pub context: String,
    /// Related concepts
    pub related_concepts: Vec<String>,
    /// Formal definition
    pub formal_definition: Option<String>,
}

/// Parsing context for state management
#[derive(Debug, Clone)]
pub struct ParsingContext {
    /// Current position in input
    pub position: usize,
    /// Current parsing depth
    pub depth: usize,
    /// Active scopes
    pub scopes: Vec<ParseScope>,
    /// Symbol bindings
    pub bindings: HashMap<String, EnhancedMathExpression>,
    /// Parsing errors
    pub errors: Vec<MathNotationError>,
    /// Warnings
    pub warnings: Vec<String>,
}

/// Parsing scope for variable bindings
#[derive(Debug, Clone)]
pub struct ParseScope {
    /// Scope type
    pub scope_type: ScopeType,
    /// Bound variables
    pub variables: HashMap<String, VariableBinding>,
    /// Scope start position
    pub start_position: usize,
}

/// Types of parsing scopes
#[derive(Debug, Clone, PartialEq)]
pub enum ScopeType {
    Global,
    Lambda,
    Quantifier,
    Category,
    LocalDefinition,
}

/// Variable binding information
#[derive(Debug, Clone)]
pub struct VariableBinding {
    /// Variable name
    pub name: String,
    /// Variable type
    pub var_type: Option<String>,
    /// Binding scope
    pub scope: String,
    /// Definition position
    pub definition_position: usize,
}

impl Default for MathParsingConfig {
    fn default() -> Self {
        Self {
            enable_category_theory: true,
            enable_advanced_unicode: true,
            enable_lambda_calculus: true,
            max_parsing_depth: 50,
            strict_mode: false,
            custom_symbols: HashMap::new(),
            precedence_rules: Self::default_precedence_rules(),
        }
    }
}

impl MathParsingConfig {
    /// Create configuration with strict parsing
    pub fn strict() -> Self {
        Self {
            strict_mode: true,
            max_parsing_depth: 25,
            ..Self::default()
        }
    }

    /// Create configuration for category theory only
    pub fn category_theory_only() -> Self {
        Self {
            enable_category_theory: true,
            enable_advanced_unicode: false,
            enable_lambda_calculus: false,
            ..Self::default()
        }
    }

    /// Create default precedence rules
    fn default_precedence_rules() -> HashMap<String, i32> {
        let mut rules = HashMap::new();

        // Logical operators
        rules.insert("¬".to_string(), 100); // Negation (highest)
        rules.insert("∧".to_string(), 90); // Conjunction
        rules.insert("∨".to_string(), 80); // Disjunction
        rules.insert("→".to_string(), 70); // Implication
        rules.insert("↔".to_string(), 60); // Biconditional

        // Set operators
        rules.insert("∩".to_string(), 85); // Intersection
        rules.insert("∪".to_string(), 75); // Union
        rules.insert("⊆".to_string(), 55); // Subset
        rules.insert("∈".to_string(), 50); // Membership

        // Category theory
        rules.insert("∘".to_string(), 95); // Composition
        rules.insert("⇒".to_string(), 65); // Natural transformation
        rules.insert("⊣".to_string(), 45); // Adjunction

        // Arithmetic (for reference)
        rules.insert("*".to_string(), 120);
        rules.insert("/".to_string(), 120);
        rules.insert("+".to_string(), 110);
        rules.insert("-".to_string(), 110);

        rules
    }

    /// Add custom symbol
    pub fn add_symbol(&mut self, symbol: String, info: UnicodeSymbolInfo) {
        self.custom_symbols.insert(symbol, info);
    }

    /// Set precedence for operator
    pub fn set_precedence(&mut self, operator: String, precedence: i32) {
        self.precedence_rules.insert(operator, precedence);
    }
}

impl ParsingContext {
    /// Create new parsing context
    pub fn new() -> Self {
        Self {
            position: 0,
            depth: 0,
            scopes: vec![ParseScope::global()],
            bindings: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Enter new scope
    pub fn enter_scope(&mut self, scope_type: ScopeType) {
        self.scopes.push(ParseScope::new(scope_type, self.position));
    }

    /// Exit current scope
    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Add variable binding
    pub fn bind_variable(&mut self, name: String, binding: VariableBinding) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.variables.insert(name.clone(), binding);
        }
        self.bindings.insert(
            name,
            EnhancedMathExpression::BasicSymbol("variable".to_string()),
        );
    }

    /// Look up variable binding
    pub fn lookup_variable(&self, name: &str) -> Option<&VariableBinding> {
        for scope in self.scopes.iter().rev() {
            if let Some(binding) = scope.variables.get(name) {
                return Some(binding);
            }
        }
        None
    }

    /// Add parsing error
    pub fn add_error(&mut self, error: MathNotationError) {
        self.errors.push(error);
    }

    /// Add warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

impl ParseScope {
    /// Create global scope
    pub fn global() -> Self {
        Self {
            scope_type: ScopeType::Global,
            variables: HashMap::new(),
            start_position: 0,
        }
    }

    /// Create new scope
    pub fn new(scope_type: ScopeType, start_position: usize) -> Self {
        Self {
            scope_type,
            variables: HashMap::new(),
            start_position,
        }
    }
}

impl VariableBinding {
    /// Create new variable binding
    pub fn new(name: String, var_type: Option<String>, scope: String, position: usize) -> Self {
        Self {
            name,
            var_type,
            scope,
            definition_position: position,
        }
    }
}

impl UnicodeSymbolInfo {
    /// Create basic symbol info
    pub fn basic(symbol: String, unicode_name: String, category: String) -> Self {
        Self {
            symbol,
            unicode_name,
            category,
            latex_equivalent: None,
            precedence: 50,
            associativity: Associativity::None,
            metadata: SymbolMetadata::default(),
        }
    }

    /// Create operator symbol info
    pub fn operator(
        symbol: String,
        unicode_name: String,
        precedence: i32,
        associativity: Associativity,
    ) -> Self {
        Self {
            symbol,
            unicode_name,
            category: "Operator".to_string(),
            latex_equivalent: None,
            precedence,
            associativity,
            metadata: SymbolMetadata::default(),
        }
    }
}

impl Default for SymbolMetadata {
    fn default() -> Self {
        Self {
            unicode_block: "Unknown".to_string(),
            frequency: 0.0,
            complexity: ComplexityLevel::Basic,
            alternatives: Vec::new(),
        }
    }
}

impl CategorySymbolInfo {
    /// Create new category symbol info
    pub fn new(symbol: String, meaning: String, context: String) -> Self {
        Self {
            symbol,
            meaning,
            context,
            related_concepts: Vec::new(),
            formal_definition: None,
        }
    }

    /// Add related concept
    pub fn with_related_concept(mut self, concept: String) -> Self {
        self.related_concepts.push(concept);
        self
    }

    /// Add formal definition
    pub fn with_definition(mut self, definition: String) -> Self {
        self.formal_definition = Some(definition);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_parsing_config_defaults() {
        let config = MathParsingConfig::default();
        assert!(config.enable_category_theory);
        assert!(config.enable_advanced_unicode);
        assert!(config.enable_lambda_calculus);
        assert_eq!(config.max_parsing_depth, 50);
        assert!(!config.strict_mode);
    }

    #[test]
    fn test_strict_config() {
        let config = MathParsingConfig::strict();
        assert!(config.strict_mode);
        assert_eq!(config.max_parsing_depth, 25);
    }

    #[test]
    fn test_category_theory_config() {
        let config = MathParsingConfig::category_theory_only();
        assert!(config.enable_category_theory);
        assert!(!config.enable_advanced_unicode);
        assert!(!config.enable_lambda_calculus);
    }

    #[test]
    fn test_precedence_rules() {
        let config = MathParsingConfig::default();
        assert!(
            config.precedence_rules.get("¬").unwrap() > config.precedence_rules.get("∧").unwrap()
        );
        assert!(
            config.precedence_rules.get("∧").unwrap() > config.precedence_rules.get("∨").unwrap()
        );
        assert!(
            config.precedence_rules.get("∘").unwrap() > config.precedence_rules.get("∧").unwrap()
        );
    }

    #[test]
    fn test_parsing_context() {
        let mut context = ParsingContext::new();
        assert_eq!(context.depth, 0);
        assert_eq!(context.position, 0);
        assert_eq!(context.scopes.len(), 1);
        assert!(context.errors.is_empty());

        context.enter_scope(ScopeType::Lambda);
        assert_eq!(context.scopes.len(), 2);

        context.exit_scope();
        assert_eq!(context.scopes.len(), 1);
    }

    #[test]
    fn test_variable_binding() {
        let mut context = ParsingContext::new();
        let binding = VariableBinding::new(
            "x".to_string(),
            Some("Real".to_string()),
            "lambda".to_string(),
            10,
        );

        context.bind_variable("x".to_string(), binding);
        let found_binding = context.lookup_variable("x");
        assert!(found_binding.is_some());
        assert_eq!(found_binding.unwrap().name, "x");
    }

    #[test]
    fn test_expression_types() {
        let basic = EnhancedMathExpression::BasicSymbol("x".to_string());
        assert!(matches!(basic, EnhancedMathExpression::BasicSymbol(_)));

        let constant = EnhancedMathExpression::Constant {
            name: "pi".to_string(),
            symbol: "π".to_string(),
            set_type: Some(SetType::Real),
        };
        assert!(matches!(constant, EnhancedMathExpression::Constant { .. }));
    }

    #[test]
    fn test_category_constructs() {
        let functor = CategoryConstruct::Functor {
            name: "F".to_string(),
            source: "C".to_string(),
            target: "D".to_string(),
        };
        assert!(matches!(functor, CategoryConstruct::Functor { .. }));

        let monad = CategoryConstruct::Monad {
            endofunctor: "T".to_string(),
            unit: "η".to_string(),
            multiplication: "μ".to_string(),
        };
        assert!(matches!(monad, CategoryConstruct::Monad { .. }));
    }

    #[test]
    fn test_quantifier_types() {
        assert_eq!(Quantifier::Forall, Quantifier::Forall);
        assert_ne!(Quantifier::Forall, Quantifier::Exists);

        let counting = Quantifier::Counting(5);
        assert!(matches!(counting, Quantifier::Counting(5)));
    }

    #[test]
    fn test_unicode_symbol_info() {
        let symbol = UnicodeSymbolInfo::basic(
            "∀".to_string(),
            "FOR ALL".to_string(),
            "Quantifier".to_string(),
        );
        assert_eq!(symbol.symbol, "∀");
        assert_eq!(symbol.category, "Quantifier");
        assert_eq!(symbol.precedence, 50);

        let operator = UnicodeSymbolInfo::operator(
            "∧".to_string(),
            "LOGICAL AND".to_string(),
            90,
            Associativity::Left,
        );
        assert_eq!(operator.precedence, 90);
        assert_eq!(operator.associativity, Associativity::Left);
    }

    #[test]
    fn test_complexity_levels() {
        assert!(ComplexityLevel::Basic < ComplexityLevel::Specialized);
        assert!(ComplexityLevel::Intermediate < ComplexityLevel::Advanced);
        assert!(ComplexityLevel::Advanced < ComplexityLevel::Expert);
    }
}
