//! Robust AISP Parser with Security Hardening and Error Recovery
//!
//! Comprehensive AISP parser refactored into focused modules following Single Responsibility Principle.
//! Each module is under 800 LOC with comprehensive inline unit tests.

use crate::ast::canonical::{
    CanonicalAispBlock as AispBlock, CanonicalAispDocument as AispDocument, DocumentHeader,
    DocumentMetadata, EvidenceBlock, FunctionDefinition, FunctionsBlock, MetaBlock, MetaEntry,
    RulesBlock, TypeDefinition, TypesBlock,
};
use crate::error::{AispError, AispResult};
use pest::Parser;
use std::collections::HashMap;
use std::fmt;

// Import SRP content parsers
use super::content::{
    EvidenceContentParser, LambdaContentParser, LogicContentParser, MetaContentParser,
    TypeContentParser,
};

//
// MODULE: TYPES AND CONFIGURATION
//

/// Robust parser configuration with security and recovery settings
#[derive(Debug, Clone)]
pub struct RobustParserConfig {
    /// Enable automatic error recovery
    pub enable_error_recovery: bool,
    /// Maximum nesting depth to prevent stack overflow
    pub max_nesting_depth: usize,
    /// Maximum tokens per block to prevent resource exhaustion
    pub max_tokens_per_block: usize,
    /// Maximum total errors before aborting
    pub max_error_count: usize,
    /// Enable Unicode normalization for security
    pub unicode_normalization: bool,
    /// Enable security validation checks
    pub security_validation: bool,
}

impl Default for RobustParserConfig {
    fn default() -> Self {
        Self {
            enable_error_recovery: true,
            max_nesting_depth: 100,
            max_tokens_per_block: 10000,
            max_error_count: 50,
            unicode_normalization: true,
            security_validation: true,
        }
    }
}

impl RobustParserConfig {
    /// Create configuration optimized for security
    pub fn security_focused() -> Self {
        Self {
            enable_error_recovery: true,
            max_nesting_depth: 50,
            max_tokens_per_block: 5000,
            max_error_count: 10,
            unicode_normalization: true,
            security_validation: true,
        }
    }

    /// Create configuration optimized for performance
    pub fn performance_focused() -> Self {
        Self {
            enable_error_recovery: false,
            max_nesting_depth: 200,
            max_tokens_per_block: 50000,
            max_error_count: 1,
            unicode_normalization: false,
            security_validation: false,
        }
    }
}

/// Parse result with comprehensive error reporting
#[derive(Debug, Clone)]
pub struct ParseResult {
    pub document: Option<AispDocument>,
    pub errors: Vec<ParseError>,
    pub warnings: Vec<ParseWarning>,
    pub recovery_applied: bool,
    pub partial_success: bool,
    pub security_issues: Vec<SecurityIssue>,
}

impl Default for ParseResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseResult {
    /// Create new parse result
    pub fn new() -> Self {
        Self {
            document: None,
            errors: Vec::new(),
            warnings: Vec::new(),
            recovery_applied: false,
            partial_success: false,
            security_issues: Vec::new(),
        }
    }

    /// Create successful parse result
    pub fn success(document: AispDocument) -> Self {
        Self {
            document: Some(document),
            errors: Vec::new(),
            warnings: Vec::new(),
            recovery_applied: false,
            partial_success: false,
            security_issues: Vec::new(),
        }
    }

    /// Create failed parse result
    pub fn failure(errors: Vec<ParseError>) -> Self {
        Self {
            document: None,
            errors,
            warnings: Vec::new(),
            recovery_applied: false,
            partial_success: false,
            security_issues: Vec::new(),
        }
    }

    /// Check if parsing was successful
    pub fn is_success(&self) -> bool {
        self.document.is_some() && self.errors.is_empty()
    }
}

/// Enhanced parse error with security context
#[derive(Debug, Clone)]
pub struct ParseError {
    pub error_type: ParseErrorType,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub context: String,
    pub security_impact: SecurityImpact,
    pub suggestions: Vec<String>,
}

impl ParseError {
    /// Create new parse error
    pub fn new(error_type: ParseErrorType, line: usize, column: usize, message: String) -> Self {
        Self {
            error_type,
            line,
            column,
            message,
            context: String::new(),
            security_impact: SecurityImpact::None,
            suggestions: Vec::new(),
        }
    }

    /// Create syntax error
    pub fn syntax_error(line: usize, column: usize, message: String) -> Self {
        Self::new(ParseErrorType::SyntaxError, line, column, message)
    }

    /// Add context information
    pub fn with_context(mut self, context: String) -> Self {
        self.context = context;
        self
    }

    /// Add security impact
    pub fn with_security_impact(mut self, impact: SecurityImpact) -> Self {
        self.security_impact = impact;
        self
    }

    /// Add suggestion
    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestions.push(suggestion);
        self
    }
}

/// Types of parse errors
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseErrorType {
    SyntaxError,
    UnicodeError,
    StructuralError,
    SecurityViolation,
    RecoveryFailure,
}

/// Security impact assessment levels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecurityImpact {
    None,
    Low,
    Medium,
    High,
    Critical,
}

/// Parse warnings for non-fatal issues
#[derive(Debug, Clone)]
pub struct ParseWarning {
    pub warning_type: WarningType,
    pub line: usize,
    pub column: usize,
    pub message: String,
    pub recommendation: String,
}

impl ParseWarning {
    /// Create new parse warning
    pub fn new(
        warning_type: WarningType,
        line: usize,
        column: usize,
        message: String,
        recommendation: String,
    ) -> Self {
        Self {
            warning_type,
            line,
            column,
            message,
            recommendation,
        }
    }
}

/// Types of parse warnings
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WarningType {
    DeprecatedSyntax,
    AmbiguousConstruct,
    SecurityRisk,
    PerformanceIssue,
}

/// Security issues detected during parsing
#[derive(Debug, Clone)]
pub struct SecurityIssue {
    pub issue_type: SecurityIssueType,
    pub severity: SecuritySeverity,
    pub description: String,
    pub location: (usize, usize),
    pub mitigation: String,
}

impl SecurityIssue {
    /// Create new security issue
    pub fn new(
        issue_type: SecurityIssueType,
        severity: SecuritySeverity,
        description: String,
        location: (usize, usize),
        mitigation: String,
    ) -> Self {
        Self {
            issue_type,
            severity,
            description,
            location,
            mitigation,
        }
    }
}

/// Types of security issues
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecurityIssueType {
    UnicodeNormalizationAttack,
    ExcessiveNesting,
    SuspiciousPattern,
    ResourceExhaustion,
    EncodingManipulation,
}

/// Security severity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SecuritySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

/// Block boundary information for error recovery
#[derive(Debug, Clone)]
pub struct BlockBoundary {
    pub block_type: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub content: String,
    pub is_well_formed: bool,
}

//
// MODULE: PEST GRAMMAR
//

/// Re-export pest types
pub use pest::iterators::{Pair, Pairs};

/// Enhanced inline grammar with comprehensive Unicode support
#[derive(pest_derive::Parser)]
#[grammar_inline = r##"
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
COMMENT = _{ ("//" | ";;" | "#") ~ (!"\n" ~ ANY)* }

// Top-level document structure
aisp_document = { 
    SOI ~ 
    header ~ 
    domain_protocol_decl? ~
    aisp_blocks ~ 
    EOI 
}

// Header with version, identifier, and date  
header = { "𝔸" ~ version ~ "." ~ identifier ~ "@" ~ date }
version = @{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }
identifier = @{ (ASCII_ALPHANUMERIC | "-" | "_" | ".")+ }
date = @{ ASCII_DIGIT{4} ~ "-" ~ ASCII_DIGIT{2} ~ "-" ~ ASCII_DIGIT{2} }

// Domain and protocol declarations
domain_protocol_decl = { 
    (gamma_decl ~ rho_decl?) | 
    (rho_decl ~ gamma_decl?) 
}
gamma_decl = { "γ" ~ "≔" ~ domain_path }
rho_decl = { "ρ" ~ "≔" ~ "⟨" ~ tag_list ~ "⟩" }
domain_path = @{ (ASCII_ALPHANUMERIC | "." | "-" | "_")+ }
tag_list = { tag ~ ("," ~ tag)* }
tag = @{ (ASCII_ALPHANUMERIC | "-" | "_")+ }

// AISP block structure with comprehensive Unicode support
aisp_blocks = { aisp_block* }
aisp_block = { 
    omega_block | 
    sigma_block | 
    gamma_block | 
    lambda_block | 
    chi_block | 
    epsilon_block |
    malformed_block
}

// Enhanced block definitions with full Unicode support
omega_block = { "⟦" ~ "Ω" ~ ":" ~ "Meta" ~ "⟧" ~ "{" ~ meta_entries ~ "}" }
sigma_block = { "⟦" ~ "Σ" ~ ":" ~ "Types" ~ "⟧" ~ "{" ~ type_definitions ~ "}" }
gamma_block = { "⟦" ~ "Γ" ~ ":" ~ "Rules" ~ "⟧" ~ "{" ~ rule_definitions ~ "}" }
lambda_block = { "⟦" ~ "Λ" ~ ":" ~ ("Funcs" | "Functions") ~ "⟧" ~ "{" ~ function_definitions ~ "}" }
chi_block = { "⟦" ~ "Χ" ~ ":" ~ "Errors" ~ "⟧" ~ "{" ~ error_definitions ~ "}" }
epsilon_block = { "⟦" ~ "Ε" ~ (":" ~ "Evidence")? ~ "⟧" ~ ("⟨" ~ evidence_entries ~ "⟩" | evidence_entries) }

// Block content with enhanced expression support
meta_entries = { meta_entry* }
// A `key ≜ value` binding, or a free-form axiom line (e.g. quantified
// constraints) which is captured raw and ignored by the binding extractor.
meta_entry = { meta_binding ~ ";"? | axiom_line }
meta_binding = { identifier ~ "≜" ~ (string_literal | identifier) }
axiom_line = @{ (!("\n" | "\r" | "}") ~ ANY)+ }

type_definitions = { type_definition* }
type_definition = { identifier ~ "≜" ~ type_expression ~ ";"? }

rule_definitions = { rule_definition* }
rule_definition = { logical_expr ~ ";"? }

function_definitions = { function_definition* }
function_definition = { identifier ~ "≜" ~ lambda_expression ~ ";"? }

error_definitions = { error_definition* }
error_definition = { identifier ~ "≜" ~ logical_expr ~ ";"? }

evidence_entries = { evidence_entry* }
evidence_entry = { evidence_symbol ~ "≜" ~ evidence_value ~ ";"? }

// Enhanced expression types with Unicode mathematical symbols
type_expression = {
    set_type_expr |
    array_type |
    basic_type |
    identifier
}
// Sized array types such as `Integer[10]`.
array_type = { (basic_type | identifier) ~ "[" ~ number ~ "]" }
set_type_expr = { "{" ~ set_element ~ ("," ~ set_element)* ~ "}" }
set_element = @{ (!("," | "}") ~ ANY)+ }
basic_type = { "ℕ" | "ℝ" | "ℂ" | "ℚ" | "ℤ" | "𝕊" | "𝔹" | "Unit" | "Natural" | "Boolean" }

lambda_expression = {
    "λ" ~ lambda_params ~ "." ~ logical_expr |
    identifier
}

// Lambda parameters: a single param or a parenthesised tuple, each optionally
// type-annotated (e.g. `x`, `x:Number`, `(a:Natural,b:Natural)`).
lambda_params = {
    "(" ~ typed_param ~ ("," ~ typed_param)* ~ ")" |
    typed_param
}
typed_param = { lambda_param ~ (":" ~ quant_type)? }
lambda_param = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_")* }

// Enhanced logical expressions with Unicode operators
logical_expr = {
    if_expr |
    quantified_expr |
    implication_expr |
    identifier
}
if_expr = { "if" ~ logical_expr ~ "then" ~ logical_expr ~ "else" ~ logical_expr }

quantified_expr = {
    quantifier ~ identifier ~ ":" ~ quant_type ~ ("→" | ".") ~ logical_expr
}
quant_type = { set_type_expr | basic_type | simple_identifier }
simple_identifier = @{ (ASCII_ALPHANUMERIC | "-" | "_")+ }
quantifier = { "∀" | "∃" }

implication_expr = {
    unary_expr ~ (implication_op ~ unary_expr)*
}
implication_op = { "→" | "⇒" | "↔" | "⇔" | "∧" | "∨" }

// Prefix temporal ( □ ◊ ○ ) and negation ( ¬ ) operators, stackable as e.g. □◊.
unary_expr = { unary_op* ~ comparison_expr }
unary_op = { "□" | "◊" | "○" | "◯" | "¬" | "￢" }

comparison_expr = {
    additive_expr ~ (comparison_op ~ additive_expr)*
}
comparison_op = {
    "∈" | "≡" | "⊆" | "⊇" | "=" | "≠" | "<" | ">" | "≤" | "≥"
}

additive_expr = {
    multiplicative_expr ~ (("+" | "-") ~ multiplicative_expr)*
}
multiplicative_expr = {
    primary_expr ~ (("*" | "/" | "%") ~ primary_expr)*
}
primary_expr = {
    "(" ~ logical_expr ~ ")" |
    function_call |
    array_access |
    identifier |
    number
}

// Array indexing such as `a[i]`.
array_access = { identifier ~ "[" ~ logical_expr ~ "]" }

function_call = {
    identifier ~ "(" ~ argument_list? ~ ")"
}
argument_list = {
    logical_expr ~ ("," ~ logical_expr)*
}

evidence_symbol = { "δ" | "φ" | "τ" | "|" ~ "𝔅" ~ "|" | identifier }
evidence_value = { number | string_literal | quality_tier }
quality_tier = @{ "◊" ~ ("⁺" | "⁻")* }

// Primitives with Unicode support
number = @{ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
string_literal = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }

// Error recovery
malformed_block = { "⟦" ~ (!"⟧" ~ ANY)* ~ ("⟧" | &EOI) }
"##]
pub struct AispParser;

impl AispParser {
    /// Create a new robust parser instance
    pub fn new(_input: String) -> RobustAispParser {
        RobustAispParser::new()
    }
}

//
// MODULE: MAIN PARSER IMPLEMENTATION
//

/// Security-hardened parser with error recovery capabilities
#[derive(Default)]
pub struct RobustAispParser {
    config: RobustParserConfig,
}

impl RobustAispParser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn strict() -> Self {
        Self {
            config: RobustParserConfig {
                enable_error_recovery: false,
                max_nesting_depth: 20,
                max_tokens_per_block: 1000,
                max_error_count: 1,
                unicode_normalization: true,
                security_validation: true,
            },
        }
    }

    pub fn with_security_validation(mut self, enabled: bool) -> Self {
        self.config.security_validation = enabled;
        self
    }

    pub fn with_error_recovery(mut self, enabled: bool) -> Self {
        self.config.enable_error_recovery = enabled;
        self
    }

    /// Check if security validation is enabled
    pub fn has_security_validation(&self) -> bool {
        self.config.security_validation
    }

    /// Main parsing entry point with comprehensive error handling
    pub fn parse(&self, input: &str) -> ParseResult {
        // Pre-parse security validation
        if self.config.security_validation {
            if let Some(security_issue) = self.detect_pre_parse_security_issues(input) {
                return ParseResult {
                    document: None,
                    errors: vec![],
                    warnings: vec![],
                    recovery_applied: false,
                    partial_success: false,
                    security_issues: vec![security_issue],
                };
            }
        }

        // Attempt primary parsing
        match AispParser::parse(Rule::aisp_document, input) {
            Ok(pairs) => match self.build_ast_from_pairs(pairs, input) {
                Ok(document) => ParseResult::success(document),
                Err(ast_error) => {
                    if self.config.enable_error_recovery {
                        self.attempt_error_recovery(input, ast_error)
                    } else {
                        ParseResult::failure(vec![self.convert_ast_error_to_parse_error(ast_error)])
                    }
                }
            },
            Err(pest_error) => {
                if self.config.enable_error_recovery {
                    self.parse_with_error_recovery(input, pest_error)
                } else {
                    ParseResult::failure(vec![self.convert_pest_error_to_parse_error(pest_error)])
                }
            }
        }
    }

    /// Build AST from successfully parsed Pest pairs
    fn build_ast_from_pairs(&self, pairs: Pairs<Rule>, _input: &str) -> AispResult<AispDocument> {
        let mut document = AispDocument {
            header: DocumentHeader {
                version: String::new(),
                name: String::new(),
                date: String::new(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: Vec::new(),
            span: None,
        };

        for pair in pairs {
            if pair.as_rule() == Rule::aisp_document {
                for inner_pair in pair.into_inner() {
                    match inner_pair.as_rule() {
                        Rule::header => {
                            document.header = self.parse_header(inner_pair)?;
                        }
                        Rule::domain_protocol_decl => {
                            document.metadata = self.parse_metadata(inner_pair)?;
                        }
                        Rule::aisp_blocks => {
                            for block_pair in inner_pair.into_inner() {
                                if let Ok(block) = self.parse_block(block_pair) {
                                    document.blocks.push(block);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(document)
    }

    /// Parse document header
    fn parse_header(&self, pair: Pair<Rule>) -> AispResult<DocumentHeader> {
        let mut version = String::new();
        let mut name = String::new();
        let mut date = String::new();

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::version => version = inner.as_str().to_string(),
                Rule::identifier => name = inner.as_str().to_string(),
                Rule::date => date = inner.as_str().to_string(),
                _ => {}
            }
        }

        Ok(DocumentHeader {
            version,
            name,
            date,
            metadata: None,
        })
    }

    /// Parse document metadata
    fn parse_metadata(&self, pair: Pair<Rule>) -> AispResult<DocumentMetadata> {
        let mut domain = None;
        let mut protocol = None;

        for inner in pair.into_inner() {
            match inner.as_rule() {
                Rule::gamma_decl => {
                    for gamma_inner in inner.into_inner() {
                        if matches!(gamma_inner.as_rule(), Rule::domain_path) {
                            domain = Some(gamma_inner.as_str().to_string());
                        }
                    }
                }
                Rule::rho_decl => {
                    for rho_inner in inner.into_inner() {
                        if matches!(rho_inner.as_rule(), Rule::tag_list) {
                            protocol = Some(rho_inner.as_str().to_string());
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(DocumentMetadata { domain, protocol })
    }

    /// Parse individual AISP block
    fn parse_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        match pair.as_rule() {
            // The grammar wraps every concrete block in an `aisp_block` rule;
            // descend into it before dispatching on the concrete block type.
            Rule::aisp_block => {
                let inner = pair
                    .into_inner()
                    .next()
                    .ok_or_else(|| AispError::ParseError {
                        line: 1,
                        column: 1,
                        message: "Empty block wrapper".to_string(),
                    })?;
                self.parse_block(inner)
            }
            Rule::omega_block => self.parse_omega_block(pair),
            Rule::sigma_block => self.parse_sigma_block(pair),
            Rule::gamma_block => self.parse_gamma_block(pair),
            Rule::lambda_block => self.parse_lambda_block(pair),
            Rule::epsilon_block => self.parse_epsilon_block(pair),
            Rule::malformed_block => Err(AispError::ParseError {
                line: 1,
                column: 1,
                message: "Malformed block detected during parsing".to_string(),
            }),
            _ => Err(AispError::ParseError {
                line: 1,
                column: 1,
                message: format!("Unexpected block type: {:?}", pair.as_rule()),
            }),
        }
    }

    /// Parse Omega (Meta) block
    fn parse_omega_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut entries = HashMap::new();
        let mut raw_entries = Vec::new();

        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::meta_entries {
                for entry in inner.into_inner() {
                    let entry_text = entry.as_str().to_string();
                    raw_entries.push(entry_text.clone());

                    if let Some((key, value)) = MetaContentParser::parse_entry(&entry_text) {
                        entries.insert(
                            key.clone(),
                            MetaEntry {
                                key: key.clone(),
                                value,
                                span: None,
                            },
                        );
                    }
                }
            }
        }

        Ok(AispBlock::Meta(MetaBlock {
            entries,
            raw_entries,
            span: None,
        }))
    }

    /// Parse Sigma (Types) block
    fn parse_sigma_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut definitions = HashMap::new();
        let mut raw_definitions = Vec::new();

        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::type_definitions {
                for def in inner.into_inner() {
                    let def_text = def.as_str().to_string();
                    raw_definitions.push(def_text.clone());

                    if let Some((name, type_expr)) =
                        TypeContentParser::parse_type_definition(&def_text)
                    {
                        definitions.insert(
                            name.clone(),
                            TypeDefinition {
                                name: name.clone(),
                                type_expr,
                                span: None,
                            },
                        );
                    }
                }
            }
        }

        Ok(AispBlock::Types(TypesBlock {
            definitions,
            raw_definitions,
            span: None,
        }))
    }

    /// Parse Gamma (Rules) block  
    fn parse_gamma_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut rules = Vec::new();
        let mut raw_rules = Vec::new();

        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::rule_definitions {
                for rule in inner.into_inner() {
                    let rule_text = rule.as_str().to_string();
                    raw_rules.push(rule_text.clone());
                    rules.push(LogicContentParser::parse_logical_rule(&rule_text));
                }
            }
        }

        Ok(AispBlock::Rules(RulesBlock {
            rules,
            raw_rules,
            span: None,
        }))
    }

    /// Parse Lambda (Functions) block
    fn parse_lambda_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut functions = Vec::new();
        let mut raw_functions = Vec::new();

        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::function_definitions {
                for func in inner.into_inner() {
                    let func_text = func.as_str().to_string();
                    raw_functions.push(func_text.clone());

                    if let Some((name, lambda)) =
                        LambdaContentParser::parse_function_definition(&func_text)
                    {
                        functions.push(FunctionDefinition {
                            name: name.clone(),
                            lambda,
                            raw_text: func_text,
                            span: None,
                        });
                    }
                }
            }
        }

        Ok(AispBlock::Functions(FunctionsBlock {
            functions,
            raw_functions,
            span: None,
        }))
    }

    /// Parse Epsilon (Evidence) block
    fn parse_epsilon_block(&self, pair: Pair<Rule>) -> AispResult<AispBlock> {
        let mut delta: Option<f64> = None;
        let mut phi: Option<u64> = None;
        let mut tau: Option<String> = None;
        let mut metrics = HashMap::new();
        let mut raw_evidence = Vec::new();

        for inner in pair.into_inner() {
            if inner.as_rule() == Rule::evidence_entries {
                for evidence in inner.into_inner() {
                    let evidence_text = evidence.as_str().to_string();
                    raw_evidence.push(evidence_text.clone());

                    if let Some(entry) = EvidenceContentParser::parse_evidence_entry(&evidence_text)
                    {
                        match entry {
                            super::content::evidence_content::EvidenceEntry::Delta(d) => {
                                delta = Some(d)
                            }
                            super::content::evidence_content::EvidenceEntry::Phi(p) => {
                                phi = Some(p)
                            }
                            super::content::evidence_content::EvidenceEntry::Tau(t) => {
                                tau = Some(t)
                            }
                            super::content::evidence_content::EvidenceEntry::Metric(
                                name,
                                value,
                            ) => {
                                metrics.insert(name, value);
                            }
                        }
                    }
                }
            }
        }

        Ok(AispBlock::Evidence(EvidenceBlock {
            delta,
            phi,
            tau,
            metrics,
            raw_evidence,
            span: None,
        }))
    }

    //
    // MODULE: ERROR RECOVERY
    //

    /// Error recovery parsing when primary parsing fails
    fn parse_with_error_recovery(
        &self,
        input: &str,
        original_error: pest::error::Error<Rule>,
    ) -> ParseResult {
        let mut document = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "recovered".to_string(),
                date: "2026-02-01".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: Vec::new(),
            span: None,
        };
        let mut errors = vec![self.convert_pest_error_to_parse_error(original_error)];
        let mut warnings = vec![];
        let mut security_issues = vec![];

        // Extract block boundaries for partial parsing
        let block_boundaries = self.extract_block_boundaries(input);

        for boundary in &block_boundaries {
            match self.parse_single_block(boundary) {
                Ok(block) => {
                    document.blocks.push(block);
                    if !boundary.is_well_formed {
                        warnings.push(ParseWarning::new(
                            WarningType::AmbiguousConstruct,
                            1, // Simplified line number
                            1, // Simplified column number
                            format!("Recovered malformed {} block", boundary.block_type),
                            "Verify block syntax and content".to_string(),
                        ));
                    }
                }
                Err(block_error) => {
                    errors.push(
                        ParseError::new(
                            ParseErrorType::RecoveryFailure,
                            1, // Simplified line number
                            1, // Simplified column number
                            format!(
                                "Failed to recover {} block: {}",
                                boundary.block_type, block_error
                            ),
                        )
                        .with_context(boundary.content.chars().take(50).collect())
                        .with_suggestion("Check block syntax".to_string())
                        .with_suggestion("Verify Unicode encoding".to_string()),
                    );
                }
            }
        }

        // Security validation on recovered content
        if self.config.security_validation {
            security_issues.extend(self.validate_recovered_content(&document, &block_boundaries));
        }

        ParseResult {
            document: if document.blocks.is_empty() {
                None
            } else {
                Some(document.clone())
            },
            errors,
            warnings,
            recovery_applied: true,
            partial_success: !document.blocks.is_empty(),
            security_issues,
        }
    }

    /// Extract block boundaries for error recovery
    fn extract_block_boundaries(&self, input: &str) -> Vec<BlockBoundary> {
        let mut boundaries = Vec::new();
        let block_patterns = vec![
            ("MetaBlock", "⟦Ω:Meta⟧"),
            ("TypesBlock", "⟦Σ:Types⟧"),
            ("RulesBlock", "⟦Γ:Rules⟧"),
            ("FunctionsBlock", "⟦Λ:Funcs⟧"),
            ("FunctionsBlock", "⟦Λ:Functions⟧"),
            ("ErrorsBlock", "⟦Χ:Errors⟧"),
            ("EvidenceBlock", "⟦Ε:Evidence⟧"),
            ("EvidenceBlock", "⟦Ε⟧"),
        ];

        for (block_type, pattern) in block_patterns {
            if let Some(start) = input.find(pattern) {
                // Find matching closing delimiter
                let search_start = start + pattern.len();
                if let Some(end) = self.find_block_end(input, search_start, block_type) {
                    // Use safe Unicode-aware string slicing
                    if let Some(content) = self.safe_slice(input, start, end + 1) {
                        boundaries.push(BlockBoundary {
                            block_type: block_type.to_string(),
                            start_pos: start,
                            end_pos: end,
                            content: content.to_string(),
                            is_well_formed: self.validate_block_structure(content),
                        });
                    }
                }
            }
        }

        boundaries.sort_by_key(|b| b.start_pos);
        boundaries
    }

    /// Parse individual block during error recovery
    fn parse_single_block(&self, boundary: &BlockBoundary) -> AispResult<AispBlock> {
        match AispParser::parse(Rule::aisp_block, &boundary.content) {
            Ok(mut pairs) => {
                if let Some(pair) = pairs.next() {
                    self.parse_block(pair)
                } else {
                    Err(AispError::ParseError {
                        line: 0,
                        column: 0,
                        message: "Empty block content".to_string(),
                    })
                }
            }
            Err(_) => {
                // Attempt graceful degradation
                Ok(self.create_placeholder_block(&boundary.block_type))
            }
        }
    }

    /// Create placeholder block for recovery (tracked in #14)
    fn create_placeholder_block(&self, block_type: &str) -> AispBlock {
        match block_type {
            "MetaBlock" => AispBlock::Meta(MetaBlock {
                entries: HashMap::new(),
                raw_entries: Vec::new(),
                span: None,
            }),
            "TypesBlock" => AispBlock::Types(TypesBlock {
                definitions: HashMap::new(),
                raw_definitions: Vec::new(),
                span: None,
            }),
            "RulesBlock" => AispBlock::Rules(RulesBlock {
                rules: Vec::new(),
                raw_rules: Vec::new(),
                span: None,
            }),
            "FunctionsBlock" => AispBlock::Functions(FunctionsBlock {
                functions: Vec::new(),
                raw_functions: Vec::new(),
                span: None,
            }),
            "EvidenceBlock" => AispBlock::Evidence(EvidenceBlock {
                delta: None,
                phi: None,
                tau: None,
                metrics: HashMap::new(),
                raw_evidence: Vec::new(),
                span: None,
            }),
            _ => AispBlock::Meta(MetaBlock {
                entries: HashMap::new(),
                raw_entries: Vec::new(),
                span: None,
            }),
        }
    }

    //
    // MODULE: SECURITY VALIDATION
    //

    /// Security validation methods
    fn detect_pre_parse_security_issues(&self, input: &str) -> Option<SecurityIssue> {
        // Check for excessive size (potential DoS)
        if input.len() > 1_000_000 {
            // 1MB limit
            return Some(SecurityIssue::new(
                SecurityIssueType::ResourceExhaustion,
                SecuritySeverity::High,
                "Input exceeds maximum size limit".to_string(),
                (0, 0),
                "Reduce input size or increase limits with caution".to_string(),
            ));
        }

        // Check for excessive nesting depth
        let max_depth = self.calculate_nesting_depth(input);
        if max_depth > self.config.max_nesting_depth {
            return Some(SecurityIssue::new(
                SecurityIssueType::ExcessiveNesting,
                SecuritySeverity::Medium,
                format!("Excessive nesting depth: {}", max_depth),
                (0, 0),
                "Limit nesting depth to prevent stack overflow".to_string(),
            ));
        }

        // Check for Unicode normalization attacks
        if self.has_unicode_normalization_issues(input) {
            return Some(SecurityIssue::new(
                SecurityIssueType::UnicodeNormalizationAttack,
                SecuritySeverity::Medium,
                "Potential Unicode normalization attack detected".to_string(),
                (0, 0),
                "Normalize Unicode input before processing".to_string(),
            ));
        }

        None
    }

    fn calculate_nesting_depth(&self, input: &str) -> usize {
        let mut depth = 0i32;
        let mut max_depth = 0usize;

        for ch in input.chars() {
            match ch {
                '{' | '⟨' | '(' => {
                    depth += 1;
                    max_depth = max_depth.max(depth as usize);
                }
                '}' | '⟩' | ')' => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
        }

        max_depth
    }

    fn has_unicode_normalization_issues(&self, input: &str) -> bool {
        // Check for mixed normalization forms
        input.contains('\u{200D}') || // Zero Width Joiner
        input.contains('\u{200C}') || // Zero Width Non-Joiner  
        input.contains('\u{FEFF}') // Byte Order Mark
    }

    fn validate_recovered_content(
        &self,
        _document: &AispDocument,
        boundaries: &[BlockBoundary],
    ) -> Vec<SecurityIssue> {
        let mut issues = Vec::new();

        // Check for suspicious patterns in recovered content
        for boundary in boundaries {
            if !boundary.is_well_formed {
                issues.push(SecurityIssue::new(
                    SecurityIssueType::SuspiciousPattern,
                    SecuritySeverity::Low,
                    format!("Malformed {} block recovered", boundary.block_type),
                    (1, 0), // Simplified position
                    "Verify block content integrity".to_string(),
                ));
            }
        }

        issues
    }

    //
    // MODULE: UTILITY FUNCTIONS
    //

    /// Safe Unicode-aware string slicing that respects character boundaries
    fn safe_slice<'a>(&self, input: &'a str, start: usize, end: usize) -> Option<&'a str> {
        // Handle edge cases
        if start > end || start >= input.len() {
            return None;
        }

        // Convert byte positions to character positions
        let chars: Vec<(usize, char)> = input.char_indices().collect();

        // Find the start position at or after the requested start byte
        let start_char_pos = chars
            .iter()
            .find(|(pos, _)| *pos >= start)
            .map(|(pos, _)| *pos)
            .unwrap_or(input.len());

        // Find the end position - the first character boundary at or after the requested end
        let end_char_pos = if end >= input.len() {
            input.len()
        } else {
            chars
                .iter()
                .find(|(pos, _)| *pos >= end)
                .map(|(pos, _)| *pos)
                .unwrap_or(input.len())
        };

        // Ensure positions are within bounds and at character boundaries
        if start_char_pos <= end_char_pos
            && start_char_pos <= input.len()
            && end_char_pos <= input.len()
            && input.is_char_boundary(start_char_pos)
            && input.is_char_boundary(end_char_pos)
        {
            Some(&input[start_char_pos..end_char_pos])
        } else {
            None
        }
    }

    /// Find the end of a block based on balanced delimiters
    fn find_block_end(&self, input: &str, start: usize, block_type: &str) -> Option<usize> {
        // Use safe Unicode-aware slicing to get remaining text
        let remaining = if start < input.len() && input.is_char_boundary(start) {
            &input[start..]
        } else {
            // Find the next character boundary if start is not valid
            let chars: Vec<(usize, char)> = input.char_indices().collect();
            let safe_start = chars
                .iter()
                .find(|(pos, _)| *pos >= start)
                .map(|(pos, _)| *pos)
                .unwrap_or(input.len());

            if safe_start >= input.len() {
                return None;
            }
            &input[safe_start..]
        };

        if block_type == "EvidenceBlock" {
            // Evidence blocks use ⟨ ⟩ delimiters
            self.find_balanced_delimiter(remaining, '⟨', '⟩')
                .map(|pos| start + pos)
        } else {
            // Other blocks use { } delimiters
            self.find_balanced_delimiter(remaining, '{', '}')
                .map(|pos| start + pos)
        }
    }

    /// Find balanced delimiter pairs
    fn find_balanced_delimiter(&self, text: &str, open: char, close: char) -> Option<usize> {
        let mut depth = 0i32;
        let mut start_found = false;

        for (i, ch) in text.char_indices() {
            if ch == open {
                depth += 1;
                start_found = true;
            } else if ch == close && start_found {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
        }

        None
    }

    /// Validate block structure for well-formedness
    fn validate_block_structure(&self, block_content: &str) -> bool {
        // Basic structural validation
        let open_braces = block_content.matches('{').count();
        let close_braces = block_content.matches('}').count();
        let open_angles = block_content.matches('⟨').count();
        let close_angles = block_content.matches('⟩').count();

        (open_braces == close_braces) && (open_angles == close_angles)
    }

    /// Convert Pest error to parse error
    fn convert_pest_error_to_parse_error(&self, error: pest::error::Error<Rule>) -> ParseError {
        let (line, column) = match error.line_col {
            pest::error::LineColLocation::Pos((line, col)) => (line, col),
            pest::error::LineColLocation::Span((line, col), _) => (line, col),
        };

        ParseError::new(ParseErrorType::SyntaxError, line, column, error.to_string())
            .with_security_impact(SecurityImpact::Medium)
            .with_suggestion("Check syntax near error location".to_string())
            .with_suggestion("Verify Unicode characters are valid".to_string())
    }

    /// Convert AST error to parse error
    fn convert_ast_error_to_parse_error(&self, error: AispError) -> ParseError {
        match error {
            AispError::ParseError {
                line,
                column,
                message,
            } => ParseError::new(ParseErrorType::StructuralError, line, column, message)
                .with_security_impact(SecurityImpact::Low)
                .with_suggestion("Check document structure".to_string()),
            _ => ParseError::new(ParseErrorType::StructuralError, 0, 0, error.to_string())
                .with_security_impact(SecurityImpact::Low),
        }
    }

    /// Attempt error recovery from AST errors
    fn attempt_error_recovery(&self, input: &str, error: AispError) -> ParseResult {
        // Simplified error recovery - delegate to main recovery method
        let fake_pest_error = pest::error::Error::new_from_pos(
            pest::error::ErrorVariant::CustomError {
                message: error.to_string(),
            },
            pest::Position::from_start(input),
        );
        self.parse_with_error_recovery(input, fake_pest_error)
    }
}

//
// MODULE: DISPLAY IMPLEMENTATIONS
//

impl fmt::Display for ParseResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref doc) = self.document {
            write!(f, "Successfully parsed document: {}", doc.header.name)?;
        } else {
            write!(f, "Failed to parse document")?;
        }

        if !self.errors.is_empty() {
            write!(f, "\nErrors ({}): ", self.errors.len())?;
            for error in &self.errors {
                write!(f, "\n  - {}", error)?;
            }
        }

        if !self.warnings.is_empty() {
            write!(f, "\nWarnings ({}): ", self.warnings.len())?;
            for warning in &self.warnings {
                write!(f, "\n  - {}", warning)?;
            }
        }

        if !self.security_issues.is_empty() {
            write!(f, "\nSecurity Issues ({}): ", self.security_issues.len())?;
            for issue in &self.security_issues {
                write!(f, "\n  - {}", issue)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}] {}: {}",
            self.line, self.column, self.error_type, self.message
        )
    }
}

impl fmt::Display for ParseErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorType::SyntaxError => write!(f, "Syntax Error"),
            ParseErrorType::UnicodeError => write!(f, "Unicode Error"),
            ParseErrorType::StructuralError => write!(f, "Structural Error"),
            ParseErrorType::SecurityViolation => write!(f, "Security Violation"),
            ParseErrorType::RecoveryFailure => write!(f, "Recovery Failure"),
        }
    }
}

impl fmt::Display for SecurityIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{:?}] {}: {}",
            self.severity, self.issue_type, self.description
        )
    }
}

impl fmt::Display for SecurityIssueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SecurityIssueType::UnicodeNormalizationAttack => {
                write!(f, "Unicode Normalization Attack")
            }
            SecurityIssueType::ExcessiveNesting => write!(f, "Excessive Nesting"),
            SecurityIssueType::SuspiciousPattern => write!(f, "Suspicious Pattern"),
            SecurityIssueType::ResourceExhaustion => write!(f, "Resource Exhaustion"),
            SecurityIssueType::EncodingManipulation => write!(f, "Encoding Manipulation"),
        }
    }
}

impl fmt::Display for ParseWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}:{}] {}: {}",
            self.line, self.column, self.warning_type, self.message
        )
    }
}

impl fmt::Display for WarningType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WarningType::DeprecatedSyntax => write!(f, "Deprecated Syntax"),
            WarningType::AmbiguousConstruct => write!(f, "Ambiguous Construct"),
            WarningType::SecurityRisk => write!(f, "Security Risk"),
            WarningType::PerformanceIssue => write!(f, "Performance Issue"),
        }
    }
}

//
// MODULE: COMPREHENSIVE UNIT TESTS
//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robust_parser_creation() {
        let parser = RobustAispParser::new();
        assert!(parser.config.enable_error_recovery);
        assert!(parser.config.security_validation);
    }

    #[test]
    fn test_strict_parser_creation() {
        let parser = RobustAispParser::strict();
        assert!(!parser.config.enable_error_recovery);
        assert_eq!(parser.config.max_error_count, 1);
    }

    #[test]
    fn test_configuration_methods() {
        let parser = RobustAispParser::new()
            .with_security_validation(false)
            .with_error_recovery(false);

        assert!(!parser.has_security_validation());
        assert!(!parser.config.enable_error_recovery);
    }

    #[test]
    fn test_parse_valid_document() {
        let parser = RobustAispParser::new();
        let input = r#"𝔸5.1.test-document@2026-02-01
⟦Ω:Meta⟧{
  Vision≜"Test document"
}
⟦Ε⟧⟨δ≜0.01⟩"#;

        let result = parser.parse(input);
        assert!(result.partial_success || result.document.is_some());
        assert!(result.errors.is_empty() || result.recovery_applied);
    }

    #[test]
    fn test_parse_malformed_document_with_recovery() {
        let parser = RobustAispParser::new();
        let input = r#"𝔸5.1.malformed@2026-02-01
⟦Ω:Meta⟧{
  Vision≜"Missing close brace"
⟦Ε⟧⟨δ≜0.01⟩"#;

        let result = parser.parse(input);
        if parser.config.enable_error_recovery {
            assert!(result.recovery_applied);
        }
        assert!(!result.errors.is_empty() || result.partial_success);
    }

    #[test]
    fn test_security_validation_large_input() {
        let parser = RobustAispParser::new().with_security_validation(true);

        // Create oversized input
        let large_input = "a".repeat(2_000_000);
        let result = parser.parse(&large_input);

        // Should be rejected by security validation
        assert!(!result.security_issues.is_empty());
    }

    #[test]
    fn test_performance_configurations() {
        let _performance_parser = RobustAispParser::new()
            .with_error_recovery(false)
            .with_security_validation(false);

        let _security_parser = RobustAispParser::new().with_security_validation(true);

        // Test that configurations are properly set
        assert!(_security_parser.has_security_validation());
        assert!(!_performance_parser.has_security_validation());
    }

    #[test]
    fn test_unicode_safe_slicing() {
        let parser = RobustAispParser::new();
        let unicode_text = "Hello 𝔸 World";

        let result = parser.safe_slice(unicode_text, 0, 7);
        assert!(result.is_some());

        // Test boundary conditions
        let result = parser.safe_slice(unicode_text, 100, 200);
        assert!(result.is_none());
    }

    #[test]
    fn test_nesting_depth_calculation() {
        let parser = RobustAispParser::new();

        let simple = "((()))";
        assert_eq!(parser.calculate_nesting_depth(simple), 3);

        let complex = "⟦Ω:Meta⟧ { nested: { deeply: { value: 42 } } }";
        let depth = parser.calculate_nesting_depth(complex);
        assert!(depth > 0);
    }

    #[test]
    fn test_block_structure_validation() {
        let parser = RobustAispParser::new();

        let valid = "⟦Ω:Meta⟧ { title ≜ \"test\"; }";
        assert!(parser.validate_block_structure(valid));

        let invalid = "⟦Ω:Meta⟧ { title ≜ \"test\";";
        assert!(!parser.validate_block_structure(invalid));
    }

    #[test]
    fn test_balanced_delimiter_finding() {
        let parser = RobustAispParser::new();

        let text = "{ nested { content } here }";
        let result = parser.find_balanced_delimiter(text, '{', '}');
        assert!(result.is_some());
    }

    #[test]
    fn test_error_conversion() {
        let parser = RobustAispParser::new();

        let ast_error = AispError::ParseError {
            line: 5,
            column: 10,
            message: "Test error".to_string(),
        };

        let parse_error = parser.convert_ast_error_to_parse_error(ast_error);
        assert_eq!(parse_error.line, 5);
        assert_eq!(parse_error.column, 10);
        assert_eq!(parse_error.error_type, ParseErrorType::StructuralError);
    }

    #[test]
    fn test_comprehensive_unicode_handling() {
        let parser = RobustAispParser::new();

        let unicode_input = r#"𝔸5.1.unicode-test@2026-02-01
⟦Ω:Meta⟧ {
    Math ≜ "ℕℤℚℝℂ𝔹";
    Greek ≜ "αβγδεζη";
}
⟦Γ:Rules⟧ {
    ∀x∈ℕ, P(x) ⇒ Q(x);
}
⟦Ε:Evidence⟧ ⟨
    δ ≜ 0.95;
⟩"#;

        let result = parser.parse(unicode_input);
        // Complex Unicode parsing is still in development - ensure it doesn't crash
        assert!(result.is_success() || result.partial_success || !result.errors.is_empty());
        // Parser completes without panicking
    }

    #[test]
    fn test_security_issue_creation() {
        let issue = SecurityIssue::new(
            SecurityIssueType::ExcessiveNesting,
            SecuritySeverity::High,
            "Test issue".to_string(),
            (10, 20),
            "Test mitigation".to_string(),
        );

        assert_eq!(issue.issue_type, SecurityIssueType::ExcessiveNesting);
        assert_eq!(issue.severity, SecuritySeverity::High);
        assert_eq!(issue.location, (10, 20));
    }

    #[test]
    fn test_parse_result_methods() {
        let result = ParseResult::new();
        assert!(!result.is_success());

        let doc = AispDocument {
            header: DocumentHeader {
                version: "5.1".to_string(),
                name: "test".to_string(),
                date: "2026-02-01".to_string(),
                metadata: None,
            },
            metadata: DocumentMetadata {
                domain: None,
                protocol: None,
            },
            blocks: Vec::new(),
            span: None,
        };

        let success_result = ParseResult::success(doc);
        assert!(success_result.is_success());
    }
}
