//! Unicode Mathematical Symbol Parser
//!
//! Specialized parser for Unicode mathematical symbols and operators.

use super::types::*;
use crate::error::{AispError, AispResult};
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

/// Unicode mathematical symbol parser
pub struct UnicodeParser {
    /// Symbol registry
    symbol_registry: HashMap<String, UnicodeSymbolInfo>,
    /// Block mappings
    unicode_blocks: HashMap<String, UnicodeBlock>,
    /// Parsing configuration
    config: UnicodeParsingConfig,
}

/// Unicode parsing configuration
#[derive(Debug, Clone)]
pub struct UnicodeParsingConfig {
    /// Enable advanced mathematical symbols
    pub enable_advanced_symbols: bool,
    /// Enable letterlike symbols
    pub enable_letterlike: bool,
    /// Enable arrows
    pub enable_arrows: bool,
    /// Enable geometric shapes
    pub enable_geometric: bool,
    /// Strict Unicode validation
    pub strict_validation: bool,
}

/// Unicode block information
#[derive(Debug, Clone)]
pub struct UnicodeBlock {
    /// Block name
    pub name: String,
    /// Start codepoint
    pub start: u32,
    /// End codepoint
    pub end: u32,
    /// Description
    pub description: String,
    /// Common symbols
    pub common_symbols: Vec<String>,
}

/// Mathematical symbol categories
#[derive(Debug, Clone, PartialEq)]
pub enum MathSymbolCategory {
    /// Operators: +, -, ×, ÷, etc.
    Operator,
    /// Relations: =, ≠, <, >, ≤, ≥, etc.
    Relation,
    /// Arrows: →, ←, ↑, ↓, ⇒, etc.
    Arrow,
    /// Quantifiers: ∀, ∃, etc.
    Quantifier,
    /// Set theory: ∈, ⊆, ∪, ∩, etc.
    SetTheory,
    /// Logic: ∧, ∨, ¬, etc.
    Logic,
    /// Greek letters: α, β, γ, etc.
    GreekLetter,
    /// Scripts: 𝒜, 𝒷, 𝒸, etc.
    Script,
    /// Constants: π, e, ℏ, etc.
    Constant,
    /// Geometry: ∠, ⊥, ∥, etc.
    Geometry,
    /// Miscellaneous
    Miscellaneous,
}

impl Default for UnicodeParser {
    fn default() -> Self {
        Self::new()
    }
}

impl UnicodeParser {
    /// Create new Unicode parser
    pub fn new() -> Self {
        Self {
            symbol_registry: Self::create_symbol_registry(),
            unicode_blocks: Self::create_unicode_blocks(),
            config: UnicodeParsingConfig::default(),
        }
    }

    /// Create parser with custom configuration
    pub fn with_config(config: UnicodeParsingConfig) -> Self {
        Self {
            symbol_registry: Self::create_symbol_registry(),
            unicode_blocks: Self::create_unicode_blocks(),
            config,
        }
    }

    /// Parse Unicode mathematical symbol
    pub fn parse_unicode_symbol(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let ch = chars
            .next()
            .ok_or_else(|| AispError::validation_error("Unexpected end of input".to_string()))?;

        let symbol = ch.to_string();
        context.position += 1;

        // Look up symbol in registry
        if let Some(symbol_info) = self.symbol_registry.get(&symbol) {
            self.parse_registered_symbol(&symbol, symbol_info, chars, context)
        } else {
            // Try to identify symbol by Unicode properties
            self.parse_unregistered_symbol(ch, chars, context)
        }
    }

    /// Parse mathematical operator
    pub fn parse_mathematical_operator(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let operator = chars.next().unwrap().to_string();
        context.position += 1;

        // Handle multi-character operators
        let full_operator = self.try_parse_multi_char_operator(&operator, chars, context)?;

        if let Some(symbol_info) = self.symbol_registry.get(&full_operator) {
            Ok(EnhancedMathExpression::UnicodeOperator {
                symbol: full_operator.clone(),
                unicode_name: symbol_info.unicode_name.clone(),
                category: symbol_info.category.clone(),
            })
        } else {
            // Fallback for unknown operators
            context.add_warning(format!("Unknown operator: {}", full_operator));
            Ok(EnhancedMathExpression::BasicSymbol(full_operator))
        }
    }

    /// Parse mathematical constant
    pub fn parse_mathematical_constant(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let constant_char = chars.next().unwrap();
        context.position += 1;

        let (name, symbol, set_type) = match constant_char {
            'ℕ' => ("Natural numbers", "ℕ", SetType::Natural),
            'ℤ' => ("Integers", "ℤ", SetType::Integer),
            'ℚ' => ("Rationals", "ℚ", SetType::Rational),
            'ℝ' => ("Reals", "ℝ", SetType::Real),
            'ℂ' => ("Complex", "ℂ", SetType::Complex),
            '𝔹' => ("Booleans", "𝔹", SetType::Boolean),
            '𝔸' => ("Custom A", "𝔸", SetType::Custom("A".to_string())),
            '𝕊' => ("Custom S", "𝕊", SetType::Custom("S".to_string())),
            '𝕃' => ("Custom L", "𝕃", SetType::Custom("L".to_string())),
            _ => {
                context.add_error(MathNotationError::UnknownSymbol {
                    symbol: constant_char.to_string(),
                    position: context.position,
                });
                return Ok(EnhancedMathExpression::BasicSymbol(
                    constant_char.to_string(),
                ));
            }
        };

        Ok(EnhancedMathExpression::Constant {
            name: name.to_string(),
            symbol: symbol.to_string(),
            set_type: Some(set_type),
        })
    }

    /// Parse script symbol (subscript/superscript)
    pub fn parse_script_symbol(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let script_char = chars.next().unwrap();
        context.position += 1;

        let (script_type, normalized) = self.normalize_script_character(script_char)?;

        // Parse the base expression (look backwards in context if needed)
        let base = Box::new(EnhancedMathExpression::BasicSymbol("base".to_string()));
        let script = Box::new(EnhancedMathExpression::BasicSymbol(normalized));

        Ok(EnhancedMathExpression::ScriptNotation {
            base,
            script_type,
            script,
        })
    }

    /// Parse Greek letters
    pub fn parse_greek_letter(
        &self,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let greek_char = chars.next().unwrap();
        context.position += 1;

        let letter_name = self.get_greek_letter_name(greek_char);

        if let Some(symbol_info) = self.symbol_registry.get(&greek_char.to_string()) {
            Ok(EnhancedMathExpression::UnicodeOperator {
                symbol: greek_char.to_string(),
                unicode_name: symbol_info.unicode_name.clone(),
                category: "Greek Letter".to_string(),
            })
        } else {
            Ok(EnhancedMathExpression::BasicSymbol(letter_name))
        }
    }

    /// Parse registered symbol
    fn parse_registered_symbol(
        &self,
        symbol: &str,
        symbol_info: &UnicodeSymbolInfo,
        _chars: &mut Peekable<Chars>,
        _context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        Ok(EnhancedMathExpression::UnicodeOperator {
            symbol: symbol.to_string(),
            unicode_name: symbol_info.unicode_name.clone(),
            category: symbol_info.category.clone(),
        })
    }

    /// Parse unregistered symbol by Unicode properties
    fn parse_unregistered_symbol(
        &self,
        ch: char,
        _chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<EnhancedMathExpression> {
        let codepoint = ch as u32;
        let category = self.determine_symbol_category(codepoint);

        if self.config.strict_validation {
            context.add_error(MathNotationError::UnknownSymbol {
                symbol: ch.to_string(),
                position: context.position,
            });
        } else {
            context.add_warning(format!(
                "Unknown Unicode symbol: {} (U+{:04X})",
                ch, codepoint
            ));
        }

        Ok(EnhancedMathExpression::UnicodeOperator {
            symbol: ch.to_string(),
            unicode_name: format!("Unknown symbol U+{:04X}", codepoint),
            category: format!("{:?}", category),
        })
    }

    /// Try to parse multi-character operator
    fn try_parse_multi_char_operator(
        &self,
        start: &str,
        chars: &mut Peekable<Chars>,
        context: &mut ParsingContext,
    ) -> AispResult<String> {
        let mut operator = start.to_string();

        // Check for common multi-character combinations
        match start {
            ":" => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    context.position += 1;
                    operator = ":=".to_string();
                }
            }
            "!" => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    context.position += 1;
                    operator = "!=".to_string();
                }
            }
            "=" => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    context.position += 1;
                    operator = "==".to_string();
                } else if chars.peek() == Some(&'>') {
                    chars.next();
                    context.position += 1;
                    operator = "=>".to_string();
                }
            }
            "<" => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    context.position += 1;
                    operator = "<=".to_string();
                } else if chars.peek() == Some(&'-') {
                    chars.next();
                    context.position += 1;
                    operator = "<-".to_string();
                }
            }
            ">" => {
                if chars.peek() == Some(&'=') {
                    chars.next();
                    context.position += 1;
                    operator = ">=".to_string();
                }
            }
            "-" => {
                if chars.peek() == Some(&'>') {
                    chars.next();
                    context.position += 1;
                    operator = "->".to_string();
                }
            }
            _ => {}
        }

        Ok(operator)
    }

    /// Normalize script character to regular form
    fn normalize_script_character(&self, ch: char) -> AispResult<(ScriptType, String)> {
        match ch {
            // Subscripts
            '₀' => Ok((ScriptType::Subscript, "0".to_string())),
            '₁' => Ok((ScriptType::Subscript, "1".to_string())),
            '₂' => Ok((ScriptType::Subscript, "2".to_string())),
            '₃' => Ok((ScriptType::Subscript, "3".to_string())),
            '₄' => Ok((ScriptType::Subscript, "4".to_string())),
            '₅' => Ok((ScriptType::Subscript, "5".to_string())),
            '₆' => Ok((ScriptType::Subscript, "6".to_string())),
            '₇' => Ok((ScriptType::Subscript, "7".to_string())),
            '₈' => Ok((ScriptType::Subscript, "8".to_string())),
            '₉' => Ok((ScriptType::Subscript, "9".to_string())),

            // Superscripts
            '⁰' => Ok((ScriptType::Superscript, "0".to_string())),
            '¹' => Ok((ScriptType::Superscript, "1".to_string())),
            '²' => Ok((ScriptType::Superscript, "2".to_string())),
            '³' => Ok((ScriptType::Superscript, "3".to_string())),
            '⁴' => Ok((ScriptType::Superscript, "4".to_string())),
            '⁵' => Ok((ScriptType::Superscript, "5".to_string())),
            '⁶' => Ok((ScriptType::Superscript, "6".to_string())),
            '⁷' => Ok((ScriptType::Superscript, "7".to_string())),
            '⁸' => Ok((ScriptType::Superscript, "8".to_string())),
            '⁹' => Ok((ScriptType::Superscript, "9".to_string())),
            '⁺' => Ok((ScriptType::Superscript, "+".to_string())),
            '⁻' => Ok((ScriptType::Superscript, "-".to_string())),

            _ => Err(AispError::validation_error(format!(
                "Unknown script character: {}",
                ch
            ))),
        }
    }

    /// Get Greek letter name
    fn get_greek_letter_name(&self, ch: char) -> String {
        match ch {
            'α' => "alpha",
            'β' => "beta",
            'γ' => "gamma",
            'δ' => "delta",
            'ε' => "epsilon",
            'ζ' => "zeta",
            'η' => "eta",
            'θ' => "theta",
            'ι' => "iota",
            'κ' => "kappa",
            'λ' => "lambda",
            'μ' => "mu",
            'ν' => "nu",
            'ξ' => "xi",
            'ο' => "omicron",
            'π' => "pi",
            'ρ' => "rho",
            'σ' => "sigma",
            'τ' => "tau",
            'υ' => "upsilon",
            'φ' => "phi",
            'χ' => "chi",
            'ψ' => "psi",
            'ω' => "omega",
            // Uppercase
            'Α' => "Alpha",
            'Β' => "Beta",
            'Γ' => "Gamma",
            'Δ' => "Delta",
            'Ε' => "Epsilon",
            'Ζ' => "Zeta",
            'Η' => "Eta",
            'Θ' => "Theta",
            'Ι' => "Iota",
            'Κ' => "Kappa",
            'Λ' => "Lambda",
            'Μ' => "Mu",
            'Ν' => "Nu",
            'Ξ' => "Xi",
            'Ο' => "Omicron",
            'Π' => "Pi",
            'Ρ' => "Rho",
            'Σ' => "Sigma",
            'Τ' => "Tau",
            'Υ' => "Upsilon",
            'Φ' => "Phi",
            'Χ' => "Chi",
            'Ψ' => "Psi",
            'Ω' => "Omega",
            _ => "unknown_greek",
        }
        .to_string()
    }

    /// Determine symbol category by Unicode codepoint
    fn determine_symbol_category(&self, codepoint: u32) -> MathSymbolCategory {
        match codepoint {
            // Mathematical Operators
            0x2200..=0x22FF => MathSymbolCategory::Operator,
            // Arrows
            0x2190..=0x21FF => MathSymbolCategory::Arrow,
            // Mathematical Symbols-A
            0x27C0..=0x27EF => MathSymbolCategory::Miscellaneous,
            // Mathematical Symbols-B
            0x2980..=0x29FF => MathSymbolCategory::Miscellaneous,
            // Greek and Coptic
            0x0370..=0x03FF => MathSymbolCategory::GreekLetter,
            // Mathematical Script Capital Letters
            0x1D49C..=0x1D4CF => MathSymbolCategory::Script,
            _ => MathSymbolCategory::Miscellaneous,
        }
    }

    /// Create symbol registry
    fn create_symbol_registry() -> HashMap<String, UnicodeSymbolInfo> {
        let mut registry = HashMap::new();

        // Logical operators
        registry.insert(
            "∀".to_string(),
            UnicodeSymbolInfo::operator(
                "∀".to_string(),
                "FOR ALL".to_string(),
                100,
                Associativity::None,
            ),
        );
        registry.insert(
            "∃".to_string(),
            UnicodeSymbolInfo::operator(
                "∃".to_string(),
                "THERE EXISTS".to_string(),
                100,
                Associativity::None,
            ),
        );
        registry.insert(
            "∧".to_string(),
            UnicodeSymbolInfo::operator(
                "∧".to_string(),
                "LOGICAL AND".to_string(),
                90,
                Associativity::Left,
            ),
        );
        registry.insert(
            "∨".to_string(),
            UnicodeSymbolInfo::operator(
                "∨".to_string(),
                "LOGICAL OR".to_string(),
                80,
                Associativity::Left,
            ),
        );
        registry.insert(
            "¬".to_string(),
            UnicodeSymbolInfo::operator(
                "¬".to_string(),
                "NOT SIGN".to_string(),
                100,
                Associativity::Right,
            ),
        );

        // Set theory operators
        registry.insert(
            "∈".to_string(),
            UnicodeSymbolInfo::operator(
                "∈".to_string(),
                "ELEMENT OF".to_string(),
                50,
                Associativity::None,
            ),
        );
        registry.insert(
            "∉".to_string(),
            UnicodeSymbolInfo::operator(
                "∉".to_string(),
                "NOT AN ELEMENT OF".to_string(),
                50,
                Associativity::None,
            ),
        );
        registry.insert(
            "⊆".to_string(),
            UnicodeSymbolInfo::operator(
                "⊆".to_string(),
                "SUBSET OF OR EQUAL TO".to_string(),
                55,
                Associativity::None,
            ),
        );
        registry.insert(
            "⊊".to_string(),
            UnicodeSymbolInfo::operator(
                "⊊".to_string(),
                "SUBSET OF".to_string(),
                55,
                Associativity::None,
            ),
        );
        registry.insert(
            "∪".to_string(),
            UnicodeSymbolInfo::operator(
                "∪".to_string(),
                "UNION".to_string(),
                75,
                Associativity::Left,
            ),
        );
        registry.insert(
            "∩".to_string(),
            UnicodeSymbolInfo::operator(
                "∩".to_string(),
                "INTERSECTION".to_string(),
                85,
                Associativity::Left,
            ),
        );
        registry.insert(
            "∅".to_string(),
            UnicodeSymbolInfo::basic(
                "∅".to_string(),
                "EMPTY SET".to_string(),
                "Set Theory".to_string(),
            ),
        );

        // Arrows
        registry.insert(
            "→".to_string(),
            UnicodeSymbolInfo::operator(
                "→".to_string(),
                "RIGHTWARDS ARROW".to_string(),
                70,
                Associativity::Right,
            ),
        );
        registry.insert(
            "←".to_string(),
            UnicodeSymbolInfo::operator(
                "←".to_string(),
                "LEFTWARDS ARROW".to_string(),
                70,
                Associativity::Left,
            ),
        );
        registry.insert(
            "⇒".to_string(),
            UnicodeSymbolInfo::operator(
                "⇒".to_string(),
                "RIGHTWARDS DOUBLE ARROW".to_string(),
                65,
                Associativity::Right,
            ),
        );
        registry.insert(
            "⇐".to_string(),
            UnicodeSymbolInfo::operator(
                "⇐".to_string(),
                "LEFTWARDS DOUBLE ARROW".to_string(),
                65,
                Associativity::Left,
            ),
        );

        // Mathematical operators
        registry.insert(
            "≤".to_string(),
            UnicodeSymbolInfo::operator(
                "≤".to_string(),
                "LESS-THAN OR EQUAL TO".to_string(),
                60,
                Associativity::None,
            ),
        );
        registry.insert(
            "≥".to_string(),
            UnicodeSymbolInfo::operator(
                "≥".to_string(),
                "GREATER-THAN OR EQUAL TO".to_string(),
                60,
                Associativity::None,
            ),
        );
        registry.insert(
            "≠".to_string(),
            UnicodeSymbolInfo::operator(
                "≠".to_string(),
                "NOT EQUAL TO".to_string(),
                60,
                Associativity::None,
            ),
        );

        // Category theory
        registry.insert(
            "∘".to_string(),
            UnicodeSymbolInfo::operator(
                "∘".to_string(),
                "RING OPERATOR".to_string(),
                95,
                Associativity::Left,
            ),
        );
        registry.insert(
            "⊣".to_string(),
            UnicodeSymbolInfo::operator(
                "⊣".to_string(),
                "LEFT TACK".to_string(),
                45,
                Associativity::None,
            ),
        );

        registry
    }

    /// Create Unicode blocks mapping
    fn create_unicode_blocks() -> HashMap<String, UnicodeBlock> {
        let mut blocks = HashMap::new();

        blocks.insert(
            "Mathematical Operators".to_string(),
            UnicodeBlock {
                name: "Mathematical Operators".to_string(),
                start: 0x2200,
                end: 0x22FF,
                description: "Mathematical operators and symbols".to_string(),
                common_symbols: vec![
                    "∀".to_string(),
                    "∃".to_string(),
                    "∈".to_string(),
                    "∉".to_string(),
                ],
            },
        );

        blocks.insert(
            "Arrows".to_string(),
            UnicodeBlock {
                name: "Arrows".to_string(),
                start: 0x2190,
                end: 0x21FF,
                description: "Arrow symbols".to_string(),
                common_symbols: vec![
                    "←".to_string(),
                    "→".to_string(),
                    "⇐".to_string(),
                    "⇒".to_string(),
                ],
            },
        );

        blocks.insert(
            "Greek and Coptic".to_string(),
            UnicodeBlock {
                name: "Greek and Coptic".to_string(),
                start: 0x0370,
                end: 0x03FF,
                description: "Greek letters used in mathematics".to_string(),
                common_symbols: vec![
                    "α".to_string(),
                    "β".to_string(),
                    "γ".to_string(),
                    "π".to_string(),
                ],
            },
        );

        blocks
    }

    /// Check if character is in mathematical block
    pub fn is_mathematical_symbol(&self, ch: char) -> bool {
        let codepoint = ch as u32;

        // Check common mathematical Unicode blocks
        matches!(codepoint,
            0x2190..=0x21FF | // Arrows
            0x2200..=0x22FF | // Mathematical Operators
            0x2300..=0x23FF | // Miscellaneous Technical
            0x27C0..=0x27EF | // Mathematical Symbols-A
            0x2980..=0x29FF | // Mathematical Symbols-B
            0x1D400..=0x1D7FF // Mathematical Alphanumeric Symbols
        )
    }

    /// Get symbol information
    pub fn get_symbol_info(&self, symbol: &str) -> Option<&UnicodeSymbolInfo> {
        self.symbol_registry.get(symbol)
    }

    /// Add custom symbol
    pub fn add_symbol(&mut self, symbol: String, info: UnicodeSymbolInfo) {
        self.symbol_registry.insert(symbol, info);
    }
}

impl Default for UnicodeParsingConfig {
    fn default() -> Self {
        Self {
            enable_advanced_symbols: true,
            enable_letterlike: true,
            enable_arrows: true,
            enable_geometric: true,
            strict_validation: false,
        }
    }
}

impl UnicodeParsingConfig {
    /// Create strict Unicode parsing configuration
    pub fn strict() -> Self {
        Self {
            strict_validation: true,
            ..Self::default()
        }
    }

    /// Create minimal Unicode parsing configuration
    pub fn minimal() -> Self {
        Self {
            enable_advanced_symbols: false,
            enable_letterlike: false,
            enable_arrows: false,
            enable_geometric: false,
            strict_validation: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_context() -> ParsingContext {
        ParsingContext::new()
    }

    #[test]
    fn test_unicode_parser_creation() {
        let parser = UnicodeParser::new();
        assert!(!parser.symbol_registry.is_empty());
        assert!(!parser.unicode_blocks.is_empty());
    }

    #[test]
    fn test_mathematical_constant_parsing() {
        let parser = UnicodeParser::new();
        let mut context = create_test_context();
        let mut chars = "ℝ".chars().peekable();

        let result = parser.parse_mathematical_constant(&mut chars, &mut context);
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(expr, EnhancedMathExpression::Constant { .. }));
    }

    #[test]
    fn test_script_symbol_parsing() {
        let parser = UnicodeParser::new();
        let mut context = create_test_context();
        let mut chars = "₁".chars().peekable();

        let result = parser.parse_script_symbol(&mut chars, &mut context);
        assert!(result.is_ok());

        let expr = result.unwrap();
        assert!(matches!(
            expr,
            EnhancedMathExpression::ScriptNotation { .. }
        ));
    }

    #[test]
    fn test_greek_letter_parsing() {
        let parser = UnicodeParser::new();
        let mut context = create_test_context();
        let mut chars = "α".chars().peekable();

        let result = parser.parse_greek_letter(&mut chars, &mut context);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mathematical_symbol_detection() {
        let parser = UnicodeParser::new();

        assert!(parser.is_mathematical_symbol('∀'));
        assert!(parser.is_mathematical_symbol('→'));
        assert!(parser.is_mathematical_symbol('∈'));
        assert!(!parser.is_mathematical_symbol('a'));
        assert!(!parser.is_mathematical_symbol('1'));
    }

    #[test]
    fn test_multi_char_operator_parsing() {
        let parser = UnicodeParser::new();
        let mut context = create_test_context();
        let mut chars = "=".chars().peekable();

        let result = parser.try_parse_multi_char_operator(":", &mut chars, &mut context);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ":=");
    }

    #[test]
    fn test_script_normalization() {
        let parser = UnicodeParser::new();

        let result = parser.normalize_script_character('₁');
        assert!(result.is_ok());
        let (script_type, normalized) = result.unwrap();
        assert_eq!(script_type, ScriptType::Subscript);
        assert_eq!(normalized, "1");

        let result = parser.normalize_script_character('²');
        assert!(result.is_ok());
        let (script_type, normalized) = result.unwrap();
        assert_eq!(script_type, ScriptType::Superscript);
        assert_eq!(normalized, "2");
    }

    #[test]
    fn test_symbol_category_determination() {
        let parser = UnicodeParser::new();

        assert_eq!(
            parser.determine_symbol_category(0x2200),
            MathSymbolCategory::Operator
        ); // ∀
        assert_eq!(
            parser.determine_symbol_category(0x2190),
            MathSymbolCategory::Arrow
        ); // ←
        assert_eq!(
            parser.determine_symbol_category(0x03B1),
            MathSymbolCategory::GreekLetter
        ); // α
    }

    #[test]
    fn test_symbol_registry_lookup() {
        let parser = UnicodeParser::new();

        let symbol_info = parser.get_symbol_info("∀");
        assert!(symbol_info.is_some());
        assert_eq!(symbol_info.unwrap().unicode_name, "FOR ALL");

        let unknown_symbol = parser.get_symbol_info("unknown");
        assert!(unknown_symbol.is_none());
    }

    #[test]
    fn test_custom_symbol_addition() {
        let mut parser = UnicodeParser::new();

        let custom_symbol = UnicodeSymbolInfo::basic(
            "⊕".to_string(),
            "CIRCLED PLUS".to_string(),
            "Custom".to_string(),
        );

        parser.add_symbol("⊕".to_string(), custom_symbol);

        let symbol_info = parser.get_symbol_info("⊕");
        assert!(symbol_info.is_some());
        assert_eq!(symbol_info.unwrap().category, "Custom");
    }

    #[test]
    fn test_unicode_parsing_config() {
        let default_config = UnicodeParsingConfig::default();
        assert!(default_config.enable_advanced_symbols);
        assert!(!default_config.strict_validation);

        let strict_config = UnicodeParsingConfig::strict();
        assert!(strict_config.strict_validation);

        let minimal_config = UnicodeParsingConfig::minimal();
        assert!(!minimal_config.enable_advanced_symbols);
        assert!(!minimal_config.enable_arrows);
    }
}
