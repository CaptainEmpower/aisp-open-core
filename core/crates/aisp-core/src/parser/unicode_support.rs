// Unicode Mathematical Symbol Support for AISP Parser
// Comprehensive registry and validation for mathematical notation

use std::collections::HashMap;
use std::fmt;

/// Mathematical symbol type classification
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MathematicalSymbolType {
    // Logic and quantifiers
    Quantifier,
    LogicalConnective,
    LogicalConstant,

    // Set theory
    SetRelation,
    SetOperation,

    // Type theory
    TypeOperator,
    TypeRelation,

    // Functions and lambda calculus
    FunctionNotation,
    LambdaCalculus,

    // Block delimiters
    BlockDelimiter,
    EvidenceDelimiter,

    // Mathematical operators
    ComparisonOperator,
    ArithmeticOperator,

    // Greek letters (variables and constants)
    GreekLetter,

    // Special AISP symbols
    QualityTier,
    EvidenceSymbol,
    DefinitionOperator,

    // Number theory
    NumberType,

    // Misc mathematical
    Infinity,
    EmptySet,

    // Security-relevant symbols
    PotentiallyDangerous,
}

/// Mathematical symbol with metadata
#[derive(Debug, Clone)]
pub struct MathematicalSymbol {
    pub unicode_char: char,
    pub unicode_code: u32,
    pub symbol_type: MathematicalSymbolType,
    pub name: &'static str,
    pub latex_equivalent: Option<&'static str>,
    pub ascii_fallback: Option<&'static str>,
    pub description: &'static str,
    pub aisp_usage: &'static str,
    pub security_level: SecurityLevel,
}

/// Security level for Unicode symbols
#[derive(Debug, Clone, PartialEq)]
pub enum SecurityLevel {
    Safe,       // Standard mathematical symbols
    Caution,    // Symbols that could be confused
    Restricted, // Symbols requiring special handling
    Dangerous,  // Symbols that pose security risks
}

/// Unicode symbol validation result
#[derive(Debug, Clone)]
pub struct SymbolValidationResult {
    pub is_valid: bool,
    pub symbol_info: Option<MathematicalSymbol>,
    pub security_warnings: Vec<SecurityWarning>,
    pub normalization_issues: Vec<NormalizationIssue>,
    pub suggestions: Vec<String>,
}

/// Security warning for symbol usage
#[derive(Debug, Clone)]
pub struct SecurityWarning {
    pub warning_type: SecurityWarningType,
    pub severity: SecuritySeverity,
    pub message: String,
    pub mitigation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecurityWarningType {
    SimilarLooking,
    NormalizationAttack,
    EncodingConfusion,
    InvisibleCharacter,
    BidirectionalOverride,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SecuritySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Unicode normalization issue
#[derive(Debug, Clone)]
pub struct NormalizationIssue {
    pub issue_type: NormalizationIssueType,
    pub original_char: char,
    pub normalized_char: Option<char>,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NormalizationIssueType {
    ComposingSequence,
    DecomposingSequence,
    CombiningCharacters,
    CompatibilityMapping,
}

/// Unicode mathematical symbol registry
pub struct UnicodeSymbolRegistry {
    symbols: HashMap<char, MathematicalSymbol>,
    by_name: HashMap<&'static str, char>,
    by_type: HashMap<MathematicalSymbolType, Vec<char>>,
    dangerous_patterns: Vec<DangerousPattern>,
}

/// Dangerous Unicode pattern detection
#[derive(Debug, Clone)]
pub struct DangerousPattern {
    pub pattern_name: &'static str,
    pub description: &'static str,
    pub detection_regex: &'static str,
    pub threat_level: SecuritySeverity,
    pub examples: Vec<&'static str>,
}

impl UnicodeSymbolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            symbols: HashMap::new(),
            by_name: HashMap::new(),
            by_type: HashMap::new(),
            dangerous_patterns: Vec::new(),
        };

        registry.initialize_symbols();
        registry.initialize_dangerous_patterns();
        registry
    }

    /// Initialize the complete AISP mathematical symbol set
    fn initialize_symbols(&mut self) {
        // Logic and Quantifiers
        self.register_symbol(
            '∀',
            "forall",
            MathematicalSymbolType::Quantifier,
            "Universal Quantifier",
            Some("\\forall"),
            Some("forall"),
            "Universal quantification over a domain",
            "∀x∈D: P(x)",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '∃',
            "exists",
            MathematicalSymbolType::Quantifier,
            "Existential Quantifier",
            Some("\\exists"),
            Some("exists"),
            "Existential quantification",
            "∃x: P(x)",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '∧',
            "and",
            MathematicalSymbolType::LogicalConnective,
            "Logical AND",
            Some("\\land"),
            Some("AND"),
            "Logical conjunction",
            "P ∧ Q",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '∨',
            "or",
            MathematicalSymbolType::LogicalConnective,
            "Logical OR",
            Some("\\lor"),
            Some("OR"),
            "Logical disjunction",
            "P ∨ Q",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '¬',
            "not",
            MathematicalSymbolType::LogicalConnective,
            "Logical NOT",
            Some("\\lnot"),
            Some("NOT"),
            "Logical negation",
            "¬P",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '→',
            "implies",
            MathematicalSymbolType::LogicalConnective,
            "Logical Implication",
            Some("\\to"),
            Some("->"),
            "Logical implication",
            "P → Q",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '↔',
            "iff",
            MathematicalSymbolType::LogicalConnective,
            "Bi-conditional",
            Some("\\leftrightarrow"),
            Some("<->"),
            "Bi-conditional",
            "P ↔ Q",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⊤',
            "top",
            MathematicalSymbolType::LogicalConstant,
            "Logical True",
            Some("\\top"),
            Some("TRUE"),
            "Logical truth",
            "⊤",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⊥',
            "bottom",
            MathematicalSymbolType::LogicalConstant,
            "Logical False",
            Some("\\bot"),
            Some("FALSE"),
            "Logical falsity",
            "⊥",
            SecurityLevel::Safe,
        );

        // Set Theory
        self.register_symbol(
            '∈',
            "in",
            MathematicalSymbolType::SetRelation,
            "Element Of",
            Some("\\in"),
            Some("in"),
            "Set membership",
            "x ∈ S",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '∉',
            "notin",
            MathematicalSymbolType::SetRelation,
            "Not Element Of",
            Some("\\notin"),
            Some("not in"),
            "Set non-membership",
            "x ∉ S",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⊆',
            "subseteq",
            MathematicalSymbolType::SetRelation,
            "Subset or Equal",
            Some("\\subseteq"),
            Some("<="),
            "Subset relation",
            "A ⊆ B",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⊇',
            "supseteq",
            MathematicalSymbolType::SetRelation,
            "Superset or Equal",
            Some("\\supseteq"),
            Some(">="),
            "Superset relation",
            "A ⊇ B",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '∩',
            "intersect",
            MathematicalSymbolType::SetOperation,
            "Intersection",
            Some("\\cap"),
            Some("intersect"),
            "Set intersection",
            "A ∩ B",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '∪',
            "union",
            MathematicalSymbolType::SetOperation,
            "Union",
            Some("\\cup"),
            Some("union"),
            "Set union",
            "A ∪ B",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '∅',
            "emptyset",
            MathematicalSymbolType::EmptySet,
            "Empty Set",
            Some("\\emptyset"),
            Some("{}"),
            "Empty set",
            "∅",
            SecurityLevel::Safe,
        );

        // Type Theory and Functions
        self.register_symbol(
            '⊕',
            "oplus",
            MathematicalSymbolType::TypeOperator,
            "Direct Sum",
            Some("\\oplus"),
            Some("(+)"),
            "Direct sum operator",
            "V_H ⊕ V_L ⊕ V_S",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⊗',
            "otimes",
            MathematicalSymbolType::TypeOperator,
            "Tensor Product",
            Some("\\otimes"),
            Some("(*)"),
            "Tensor product",
            "A ⊗ B",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⊘',
            "oslash",
            MathematicalSymbolType::TypeOperator,
            "Quotient",
            Some("\\oslash"),
            Some("(/)"),
            "Quotient operator",
            "A ⊘ B",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'λ',
            "lambda",
            MathematicalSymbolType::LambdaCalculus,
            "Lambda",
            Some("\\lambda"),
            Some("lambda"),
            "Lambda abstraction",
            "λx.f(x)",
            SecurityLevel::Safe,
        );

        // Comparison and Arithmetic
        self.register_symbol(
            '≡',
            "equiv",
            MathematicalSymbolType::ComparisonOperator,
            "Equivalent",
            Some("\\equiv"),
            Some("==="),
            "Logical equivalence",
            "A ≡ B",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '≠',
            "neq",
            MathematicalSymbolType::ComparisonOperator,
            "Not Equal",
            Some("\\neq"),
            Some("!="),
            "Not equal",
            "A ≠ B",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '≤',
            "leq",
            MathematicalSymbolType::ComparisonOperator,
            "Less or Equal",
            Some("\\leq"),
            Some("<="),
            "Less than or equal",
            "x ≤ y",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '≥',
            "geq",
            MathematicalSymbolType::ComparisonOperator,
            "Greater or Equal",
            Some("\\geq"),
            Some(">="),
            "Greater than or equal",
            "x ≥ y",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '≈',
            "approx",
            MathematicalSymbolType::ComparisonOperator,
            "Approximately",
            Some("\\approx"),
            Some("~="),
            "Approximately equal",
            "x ≈ y",
            SecurityLevel::Safe,
        );

        // AISP Block Delimiters
        self.register_symbol(
            '⟦',
            "lblock",
            MathematicalSymbolType::BlockDelimiter,
            "Left Block Delimiter",
            Some("\\llbracket"),
            Some("[["),
            "AISP block opening",
            "⟦Ω:Meta⟧",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⟧',
            "rblock",
            MathematicalSymbolType::BlockDelimiter,
            "Right Block Delimiter",
            Some("\\rrbracket"),
            Some("]]"),
            "AISP block closing",
            "⟦Ω:Meta⟧",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⟨',
            "langle",
            MathematicalSymbolType::EvidenceDelimiter,
            "Left Angle Bracket",
            Some("\\langle"),
            Some("<"),
            "Evidence block opening",
            "⟨δ≜0.01⟩",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⟩',
            "rangle",
            MathematicalSymbolType::EvidenceDelimiter,
            "Right Angle Bracket",
            Some("\\rangle"),
            Some(">"),
            "Evidence block closing",
            "⟨δ≜0.01⟩",
            SecurityLevel::Safe,
        );

        // Greek Letters (Variables and Constants)
        self.register_symbol(
            'α',
            "alpha",
            MathematicalSymbolType::GreekLetter,
            "Alpha",
            Some("\\alpha"),
            Some("alpha"),
            "Greek letter alpha",
            "α-value",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'β',
            "beta",
            MathematicalSymbolType::GreekLetter,
            "Beta",
            Some("\\beta"),
            Some("beta"),
            "Greek letter beta",
            "β-reduction",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'γ',
            "gamma",
            MathematicalSymbolType::GreekLetter,
            "Gamma",
            Some("\\gamma"),
            Some("gamma"),
            "Greek letter gamma",
            "γ≔domain",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'δ',
            "delta",
            MathematicalSymbolType::EvidenceSymbol,
            "Delta",
            Some("\\delta"),
            Some("delta"),
            "Ambiguity measure",
            "δ≜0.01",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'ε',
            "epsilon",
            MathematicalSymbolType::GreekLetter,
            "Epsilon",
            Some("\\varepsilon"),
            Some("epsilon"),
            "Greek letter epsilon",
            "ε-neighborhood",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'φ',
            "phi",
            MathematicalSymbolType::EvidenceSymbol,
            "Phi",
            Some("\\phi"),
            Some("phi"),
            "Completeness measure",
            "φ≜95",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'π',
            "pi",
            MathematicalSymbolType::GreekLetter,
            "Pi",
            Some("\\pi"),
            Some("pi"),
            "Greek letter pi",
            "π-calculus",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'ρ',
            "rho",
            MathematicalSymbolType::GreekLetter,
            "Rho",
            Some("\\rho"),
            Some("rho"),
            "Greek letter rho",
            "ρ≔⟨tags⟩",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'σ',
            "sigma",
            MathematicalSymbolType::GreekLetter,
            "Sigma",
            Some("\\sigma"),
            Some("sigma"),
            "Greek letter sigma",
            "σ-algebra",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'τ',
            "tau",
            MathematicalSymbolType::EvidenceSymbol,
            "Tau",
            Some("\\tau"),
            Some("tau"),
            "Tier indicator",
            "τ≜◊⁺",
            SecurityLevel::Safe,
        );

        // Mathematical Type Symbols
        self.register_symbol(
            'ℕ',
            "naturals",
            MathematicalSymbolType::NumberType,
            "Natural Numbers",
            Some("\\mathbb{N}"),
            Some("Nat"),
            "Set of natural numbers",
            "x:ℕ",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'ℤ',
            "integers",
            MathematicalSymbolType::NumberType,
            "Integers",
            Some("\\mathbb{Z}"),
            Some("Int"),
            "Set of integers",
            "x:ℤ",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'ℚ',
            "rationals",
            MathematicalSymbolType::NumberType,
            "Rational Numbers",
            Some("\\mathbb{Q}"),
            Some("Rat"),
            "Set of rational numbers",
            "x:ℚ",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'ℝ',
            "reals",
            MathematicalSymbolType::NumberType,
            "Real Numbers",
            Some("\\mathbb{R}"),
            Some("Real"),
            "Set of real numbers",
            "x:ℝ",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            'ℂ',
            "complex",
            MathematicalSymbolType::NumberType,
            "Complex Numbers",
            Some("\\mathbb{C}"),
            Some("Complex"),
            "Set of complex numbers",
            "x:ℂ",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '𝕊',
            "strings",
            MathematicalSymbolType::NumberType,
            "String Type",
            Some("\\mathbb{S}"),
            Some("String"),
            "String type",
            "x:𝕊",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '𝔹',
            "booleans",
            MathematicalSymbolType::NumberType,
            "Boolean Type",
            Some("\\mathbb{B}"),
            Some("Bool"),
            "Boolean type",
            "x:𝔹",
            SecurityLevel::Safe,
        );

        // AISP Definition and Evidence Symbols
        self.register_symbol(
            '≜',
            "defined",
            MathematicalSymbolType::DefinitionOperator,
            "Defined As",
            None,
            Some("=def"),
            "Definition operator",
            "f≜λx.x+1",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '⊢',
            "proves",
            MathematicalSymbolType::EvidenceSymbol,
            "Proves",
            Some("\\vdash"),
            Some("|-"),
            "Provability relation",
            "⊢P",
            SecurityLevel::Safe,
        );
        self.register_symbol(
            '◊',
            "diamond",
            MathematicalSymbolType::QualityTier,
            "Diamond",
            Some("\\diamond"),
            Some("diamond"),
            "Quality tier indicator",
            "◊⁺⁺",
            SecurityLevel::Safe,
        );

        // Special Mathematical Symbols
        self.register_symbol(
            '∞',
            "infinity",
            MathematicalSymbolType::Infinity,
            "Infinity",
            Some("\\infty"),
            Some("inf"),
            "Infinity symbol",
            "lim→∞",
            SecurityLevel::Caution,
        );

        // Potentially dangerous symbols (confusables)
        self.register_symbol(
            'А',
            "cyrillic_a",
            MathematicalSymbolType::PotentiallyDangerous,
            "Cyrillic A",
            None,
            None,
            "Cyrillic letter that looks like Latin A",
            "Visual spoofing risk",
            SecurityLevel::Dangerous,
        );
        self.register_symbol(
            'Α',
            "greek_alpha_cap",
            MathematicalSymbolType::PotentiallyDangerous,
            "Greek Capital Alpha",
            Some("\\Alpha"),
            None,
            "Greek capital alpha (looks like Latin A)",
            "Visual spoofing risk",
            SecurityLevel::Restricted,
        );
    }

    /// Register a mathematical symbol with all metadata
    fn register_symbol(
        &mut self,
        unicode_char: char,
        name: &'static str,
        symbol_type: MathematicalSymbolType,
        display_name: &'static str,
        latex_equivalent: Option<&'static str>,
        ascii_fallback: Option<&'static str>,
        description: &'static str,
        aisp_usage: &'static str,
        security_level: SecurityLevel,
    ) {
        let symbol = MathematicalSymbol {
            unicode_char,
            unicode_code: unicode_char as u32,
            symbol_type: symbol_type.clone(),
            name: display_name,
            latex_equivalent,
            ascii_fallback,
            description,
            aisp_usage,
            security_level,
        };

        self.symbols.insert(unicode_char, symbol);
        self.by_name.insert(name, unicode_char);

        self.by_type
            .entry(symbol_type)
            .or_default()
            .push(unicode_char);
    }

    /// Initialize dangerous Unicode patterns
    fn initialize_dangerous_patterns(&mut self) {
        self.dangerous_patterns.extend([
            DangerousPattern {
                pattern_name: "zero_width_characters",
                description: "Zero-width characters that can hide malicious content",
                detection_regex: r"[\u200B\u200C\u200D\u2060\uFEFF]",
                threat_level: SecuritySeverity::High,
                examples: vec!["invisible\u{200B}text", "hid\u{200C}den"],
            },
            DangerousPattern {
                pattern_name: "bidi_overrides",
                description: "Bidirectional text overrides that can reorder text",
                detection_regex: r"[\u202A-\u202E\u2066-\u2069]",
                threat_level: SecuritySeverity::Critical,
                examples: vec!["mal\u{202E}icious"],
            },
            DangerousPattern {
                pattern_name: "confusable_characters",
                description: "Characters that look similar to ASCII but are different Unicode",
                detection_regex: r"[АВЕКМНОРСТУХаеорсухуАαΑ]", // Cyrillic/Greek lookalikes
                threat_level: SecuritySeverity::Medium,
                examples: vec!["Аlpha", "αlpha"],
            },
            DangerousPattern {
                pattern_name: "combining_characters",
                description: "Excessive combining character sequences",
                detection_regex: r"[\u0300-\u036F\u1AB0-\u1AFF\u1DC0-\u1DFF\u20D0-\u20FF\uFE20-\uFE2F]{3,}",
                threat_level: SecuritySeverity::Medium,
                examples: vec!["a\u{0301}\u{0302}\u{0303}\u{0304}"],
            },
        ]);
    }

    /// Validate a Unicode character for AISP usage
    pub fn validate_symbol(&self, ch: char) -> SymbolValidationResult {
        let mut result = SymbolValidationResult {
            is_valid: false,
            symbol_info: None,
            security_warnings: Vec::new(),
            normalization_issues: Vec::new(),
            suggestions: Vec::new(),
        };

        // Check if it's a registered mathematical symbol
        if let Some(symbol) = self.symbols.get(&ch) {
            result.is_valid = true;
            result.symbol_info = Some(symbol.clone());

            // Check security level
            match symbol.security_level {
                SecurityLevel::Dangerous => {
                    result.security_warnings.push(SecurityWarning {
                        warning_type: SecurityWarningType::SimilarLooking,
                        severity: SecuritySeverity::High,
                        message: format!("Dangerous character '{}' - {}", ch, symbol.description),
                        mitigation: "Use ASCII equivalent if available".to_string(),
                    });
                    result.is_valid = false;
                }
                SecurityLevel::Restricted => {
                    result.security_warnings.push(SecurityWarning {
                        warning_type: SecurityWarningType::SimilarLooking,
                        severity: SecuritySeverity::Medium,
                        message: format!("Restricted character '{}' - {}", ch, symbol.description),
                        mitigation: "Consider ASCII equivalent".to_string(),
                    });
                }
                SecurityLevel::Caution => {
                    result.security_warnings.push(SecurityWarning {
                        warning_type: SecurityWarningType::SimilarLooking,
                        severity: SecuritySeverity::Low,
                        message: format!("Use with caution: '{}'", ch),
                        mitigation: "Verify intended usage".to_string(),
                    });
                }
                _ => {}
            }

            // Add ASCII fallback suggestion if available
            if let Some(ascii) = symbol.ascii_fallback {
                result
                    .suggestions
                    .push(format!("ASCII alternative: {}", ascii));
            }
        } else {
            // Check if it's a safe ASCII character (letters, digits, basic punctuation)
            if ch.is_ascii() && (ch.is_alphanumeric() || ".:>=<()[]{}+-*/=!?".contains(ch)) {
                result.is_valid = true;
            } else {
                // Unknown character - check against dangerous patterns
                let ch_str = ch.to_string();
                for pattern in &self.dangerous_patterns {
                    if regex::Regex::new(pattern.detection_regex)
                        .map(|re| re.is_match(&ch_str))
                        .unwrap_or(false)
                    {
                        result.security_warnings.push(SecurityWarning {
                            warning_type: SecurityWarningType::EncodingConfusion,
                            severity: pattern.threat_level.clone(),
                            message: format!(
                                "Character '{}' matches dangerous pattern: {}",
                                ch, pattern.description
                            ),
                            mitigation: "Remove or replace with safe equivalent".to_string(),
                        });
                    }
                }

                result
                    .suggestions
                    .push("Use only registered AISP mathematical symbols".to_string());
            }
        }

        // Check for normalization issues
        result.normalization_issues = self.check_normalization_issues(ch);

        result
    }

    /// Validate an entire string for AISP usage
    pub fn validate_string(&self, text: &str) -> Vec<SymbolValidationResult> {
        text.chars().map(|ch| self.validate_symbol(ch)).collect()
    }

    /// Get symbol by name
    pub fn get_symbol_by_name(&self, name: &str) -> Option<&MathematicalSymbol> {
        self.by_name.get(name).and_then(|&ch| self.symbols.get(&ch))
    }

    /// Get symbols by type
    pub fn get_symbols_by_type(
        &self,
        symbol_type: &MathematicalSymbolType,
    ) -> Vec<&MathematicalSymbol> {
        self.by_type
            .get(symbol_type)
            .map(|chars| {
                chars
                    .iter()
                    .filter_map(|&ch| self.symbols.get(&ch))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Check for Unicode normalization issues
    fn check_normalization_issues(&self, ch: char) -> Vec<NormalizationIssue> {
        let mut issues = Vec::new();

        // Check for combining characters
        if unicode_normalization::char::is_combining_mark(ch) {
            issues.push(NormalizationIssue {
                issue_type: NormalizationIssueType::CombiningCharacters,
                original_char: ch,
                normalized_char: None,
                description: "Combining character that modifies preceding character".to_string(),
            });
        }

        // Check for decomposable characters
        let nfd_result: String =
            unicode_normalization::UnicodeNormalization::nfd(ch.to_string().as_str()).collect();
        if nfd_result.len() > 1 {
            issues.push(NormalizationIssue {
                issue_type: NormalizationIssueType::DecomposingSequence,
                original_char: ch,
                normalized_char: nfd_result.chars().next(),
                description: "Character decomposes into multiple code points".to_string(),
            });
        }

        issues
    }

    /// Get all safe mathematical symbols
    pub fn get_safe_symbols(&self) -> Vec<&MathematicalSymbol> {
        self.symbols
            .values()
            .filter(|symbol| symbol.security_level == SecurityLevel::Safe)
            .collect()
    }

    /// Check if character is safe for AISP usage
    pub fn is_safe_character(&self, ch: char) -> bool {
        self.symbols
            .get(&ch)
            .map(|symbol| symbol.security_level == SecurityLevel::Safe)
            .unwrap_or(false)
    }

    /// Generate security report for text
    pub fn generate_security_report(&self, text: &str) -> SecurityReport {
        let validations = self.validate_string(text);

        let mut total_chars = 0;
        let mut safe_chars = 0;
        let mut dangerous_chars = 0;
        let mut unknown_chars = 0;
        let mut all_warnings = Vec::new();

        for validation in validations {
            total_chars += 1;

            if validation.is_valid {
                if let Some(symbol) = &validation.symbol_info {
                    if symbol.security_level == SecurityLevel::Safe {
                        safe_chars += 1;
                    }
                } else {
                    // ASCII characters that are valid but not registered as mathematical symbols
                    safe_chars += 1;
                }
            } else {
                if validation.symbol_info.is_some() {
                    dangerous_chars += 1;
                } else {
                    unknown_chars += 1;
                }
            }

            all_warnings.extend(validation.security_warnings);
        }

        SecurityReport {
            total_characters: total_chars,
            safe_characters: safe_chars,
            dangerous_characters: dangerous_chars,
            unknown_characters: unknown_chars,
            security_warnings: all_warnings,
            overall_safety: if dangerous_chars == 0 && unknown_chars == 0 {
                SecuritySeverity::Info
            } else if dangerous_chars > 0 {
                SecuritySeverity::High
            } else {
                SecuritySeverity::Medium
            },
        }
    }
}

/// Security report for text analysis
#[derive(Debug, Clone)]
pub struct SecurityReport {
    pub total_characters: usize,
    pub safe_characters: usize,
    pub dangerous_characters: usize,
    pub unknown_characters: usize,
    pub security_warnings: Vec<SecurityWarning>,
    pub overall_safety: SecuritySeverity,
}

impl Default for UnicodeSymbolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Add regex dependency for pattern matching
#[cfg(not(feature = "regex"))]
mod regex {
    pub struct Regex;
    impl Regex {
        pub fn new(_pattern: &str) -> Result<Self, ()> {
            Err(())
        }
        pub fn is_match(&self, _text: &str) -> bool {
            false
        }
    }
}

#[cfg(feature = "regex")]
use regex;

// Add unicode-normalization dependency for proper Unicode handling
#[cfg(not(feature = "unicode-normalization"))]
mod unicode_normalization {
    pub trait UnicodeNormalization {
        fn nfd(s: &str) -> std::iter::Empty<char> {
            std::iter::empty()
        }
    }
    impl UnicodeNormalization for str {}

    pub mod char {
        pub fn is_combining_mark(_ch: char) -> bool {
            false
        }
    }
}

#[cfg(feature = "unicode-normalization")]
use unicode_normalization;

impl fmt::Display for MathematicalSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ('{}') - {}",
            self.unicode_char, self.name, self.description
        )
    }
}

impl fmt::Display for SecurityReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Security Report: {}/{} safe characters, {} warnings, overall: {:?}",
            self.safe_characters,
            self.total_characters,
            self.security_warnings.len(),
            self.overall_safety
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_creation() {
        let registry = UnicodeSymbolRegistry::new();
        assert!(!registry.symbols.is_empty());
        assert!(!registry.by_name.is_empty());
        assert!(!registry.by_type.is_empty());
    }

    #[test]
    fn test_safe_symbol_validation() {
        let registry = UnicodeSymbolRegistry::new();

        let result = registry.validate_symbol('∀');
        assert!(result.is_valid);
        assert!(result.security_warnings.is_empty());
        assert!(result.symbol_info.is_some());
    }

    #[test]
    fn test_dangerous_symbol_detection() {
        let registry = UnicodeSymbolRegistry::new();

        let result = registry.validate_symbol('А'); // Cyrillic A
        assert!(!result.is_valid);
        assert!(!result.security_warnings.is_empty());
    }

    #[test]
    fn test_symbol_lookup_by_name() {
        let registry = UnicodeSymbolRegistry::new();

        let symbol = registry.get_symbol_by_name("forall");
        assert!(symbol.is_some());
        assert_eq!(symbol.unwrap().unicode_char, '∀');
    }

    #[test]
    fn test_symbols_by_type() {
        let registry = UnicodeSymbolRegistry::new();

        let quantifiers = registry.get_symbols_by_type(&MathematicalSymbolType::Quantifier);
        assert!(!quantifiers.is_empty());
        assert!(quantifiers.iter().any(|s| s.unicode_char == '∀'));
        assert!(quantifiers.iter().any(|s| s.unicode_char == '∃'));
    }

    #[test]
    fn test_string_validation() {
        let registry = UnicodeSymbolRegistry::new();

        let test_string = "∀x∈ℕ:x≥0";
        let results = registry.validate_string(test_string);

        assert_eq!(results.len(), test_string.chars().count());
        assert!(results.iter().all(|r| r.is_valid));
    }

    #[test]
    fn test_security_report() {
        let registry = UnicodeSymbolRegistry::new();

        let safe_text = "∀x∈ℕ";
        let report = registry.generate_security_report(safe_text);

        let expected_chars = safe_text.chars().count();
        assert_eq!(report.total_characters, expected_chars);
        assert_eq!(report.safe_characters, expected_chars);
        assert_eq!(report.dangerous_characters, 0);
        assert!(matches!(report.overall_safety, SecuritySeverity::Info));
    }
}
