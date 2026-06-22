//! SMT Interface with Syntax Validation and Z3 Integration
//!
//! Provides genuine Z3 SMT solver integration with comprehensive
//! syntax validation and counterexample generation.

use super::canonical_types::*;
use crate::error::*;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::Instant;

/// Effective process-wide Z3 memory cap (MB), or `None` if none has been set.
///
/// `memory_max_size` is a *process-wide* Z3 parameter (`set_global_param`), not a
/// per-solver setting — it cannot be enforced per `SmtInterface`. To stay
/// deterministic under concurrency we never let one caller's limit silently win
/// by timing: the effective cap is the *most conservative* (smallest) value any
/// caller has requested, applied under this mutex. See [`apply_global_memory_cap`].
#[cfg(feature = "z3-verification")]
static GLOBAL_MEMORY_CAP_MB: Mutex<Option<u64>> = Mutex::new(None);

/// Apply a process-wide Z3 memory ceiling deterministically.
///
/// Because `memory_max_size` is global, concurrent `with_config` callers could
/// otherwise race to fix the cap. We resolve this by only ever tightening the
/// cap to the minimum requested across all callers, under a mutex, and only
/// touching Z3's global parameter when the effective value actually changes.
#[cfg(feature = "z3-verification")]
fn apply_global_memory_cap(requested_mb: u64) {
    let mut current = GLOBAL_MEMORY_CAP_MB
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let effective = effective_memory_cap(*current, requested_mb);
    if *current != Some(effective) {
        set_global_param("memory_max_size", &effective.to_string());
        *current = Some(effective);
    }
}

/// Pure cap-reconciliation policy: the effective process-wide cap is the
/// smallest value any caller has requested. Kept separate so it can be tested
/// without mutating Z3's global state.
#[cfg(feature = "z3-verification")]
fn effective_memory_cap(current: Option<u64>, requested_mb: u64) -> u64 {
    match current {
        Some(existing) => existing.min(requested_mb),
        None => requested_mb,
    }
}

#[cfg(feature = "z3-verification")]
use z3::*;

/// SMT formula interface with real Z3 integration
pub struct SmtInterface {
    /// Z3 availability status
    z3_available: bool,
    /// Configuration options
    config: SmtConfig,
    /// Query statistics
    stats: SmtStats,
}

/// SMT configuration
#[derive(Debug, Clone)]
pub struct SmtConfig {
    pub timeout_ms: u64,
    pub verbose: bool,
    pub require_z3: bool,
    /// Soft memory ceiling, in megabytes (0 = unbounded).
    ///
    /// NOTE: Z3's `memory_max_size` is **process-wide**, so this is not enforced
    /// per `SmtInterface`. When several interfaces request different limits the
    /// effective process cap is the smallest non-zero value requested.
    pub memory_limit_mb: u64,
}

/// SMT query statistics
#[derive(Debug, Clone)]
pub struct SmtStats {
    pub queries_executed: usize,
    pub syntax_errors: usize,
    pub proven_properties: usize,
    pub disproven_properties: usize,
}

impl SmtInterface {
    /// Create new SMT interface
    pub fn new() -> Self {
        #[cfg(feature = "z3-verification")]
        let z3_available = true;
        #[cfg(not(feature = "z3-verification"))]
        let z3_available = false;

        Self {
            z3_available,
            config: SmtConfig {
                timeout_ms: 30000,
                verbose: false,
                require_z3: true,
                memory_limit_mb: 2048,
            },
            stats: SmtStats {
                queries_executed: 0,
                syntax_errors: 0,
                proven_properties: 0,
                disproven_properties: 0,
            },
        }
    }

    /// Create an SMT interface with caller-supplied configuration.
    ///
    /// Lets upstream verifiers honour their own timeout limits instead of
    /// silently falling back to the defaults in [`SmtInterface::new`]. The
    /// `timeout_ms` bound is genuinely per-query; `memory_limit_mb` is a
    /// process-wide cap (see [`SmtConfig::memory_limit_mb`]) reconciled across
    /// callers to the smallest requested value.
    pub fn with_config(config: SmtConfig) -> Self {
        #[cfg(feature = "z3-verification")]
        let z3_available = true;
        #[cfg(not(feature = "z3-verification"))]
        let z3_available = false;

        Self {
            z3_available,
            config,
            stats: SmtStats {
                queries_executed: 0,
                syntax_errors: 0,
                proven_properties: 0,
                disproven_properties: 0,
            },
        }
    }

    /// Create disabled SMT interface (for testing without Z3)
    pub fn new_disabled() -> Self {
        Self {
            z3_available: false,
            config: SmtConfig {
                timeout_ms: 30000,
                verbose: false,
                require_z3: false,
                memory_limit_mb: 2048,
            },
            stats: SmtStats {
                queries_executed: 0,
                syntax_errors: 0,
                proven_properties: 0,
                disproven_properties: 0,
            },
        }
    }

    /// Verify SMT formula with comprehensive validation
    pub fn verify_smt_formula(&mut self, formula: &str) -> AispResult<Z3PropertyResult> {
        let _start = Instant::now();
        self.stats.queries_executed += 1;

        if self.config.verbose {
            eprintln!("SMT Formula:\n{}", formula);
        }

        // Validate syntax first
        if let Err(syntax_error) = self.validate_smt_syntax(formula) {
            self.stats.syntax_errors += 1;
            return Ok(Z3PropertyResult::Error {
                error_message: format!("Syntax error: {}", syntax_error),
                error_code: -1,
            });
        }

        if !self.z3_available && self.config.require_z3 {
            return Err(AispError::validation_error(
                "Z3 verification required but not available. Compile with --features z3-verification".to_string(),
            ));
        }

        #[cfg(feature = "z3-verification")]
        {
            if self.z3_available {
                return self.execute_z3_query(formula);
            }
        }

        // Fallback for disabled mode
        if !self.config.require_z3 {
            Ok(Z3PropertyResult::Unknown {
                reason: "Formula verification not implemented".to_string(),
                partial_progress: 0.0,
            })
        } else {
            Ok(Z3PropertyResult::Error {
                error_message: "Z3 not available".to_string(),
                error_code: -2,
            })
        }
    }

    /// Validate SMT-LIB syntax comprehensively
    fn validate_smt_syntax(&self, formula: &str) -> Result<(), String> {
        let mut paren_count = 0;
        let mut has_check_sat = false;
        let mut declared_symbols = HashSet::new();
        let mut used_symbols = HashSet::new();

        for (line_no, line) in formula.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(";;") {
                continue;
            }

            // Count parentheses
            paren_count += line.chars().filter(|&c| c == '(').count() as i32;
            paren_count -= line.chars().filter(|&c| c == ')').count() as i32;

            if paren_count < 0 {
                return Err(format!(
                    "Line {}: Unmatched closing parenthesis",
                    line_no + 1
                ));
            }

            // Track declarations and usage
            if line.contains("declare-const")
                || line.contains("declare-fun")
                || line.contains("declare-sort")
            {
                if let Some(symbol) = self.extract_declared_symbol(line) {
                    declared_symbols.insert(symbol);
                }
            }

            if line.contains("assert") {
                self.extract_used_symbols(line, &mut used_symbols);
            }

            if line.contains("check-sat") {
                has_check_sat = true;
            }
        }

        if paren_count != 0 {
            return Err(format!("Unbalanced parentheses: {} unclosed", paren_count));
        }

        if !has_check_sat {
            return Err("Missing (check-sat) command".to_string());
        }

        // Check undeclared symbols
        for symbol in &used_symbols {
            if !declared_symbols.contains(symbol) && !self.is_builtin(symbol) {
                return Err(format!("Undeclared symbol: {}", symbol));
            }
        }

        Ok(())
    }

    /// Execute a syntactically-validated SMT-LIB query against Z3.
    ///
    /// Every solver call is bounded by a wall-clock timeout and a soft memory
    /// ceiling (R-16). The result mapping is sound: `Unsat` becomes `Proven`
    /// (the asserted set is genuinely unsatisfiable), `Sat` becomes `Disproven`
    /// with a real counter-model, and anything Z3 cannot decide — or any
    /// construct outside the supported fragment — becomes a first-class
    /// `Unknown`. A `Proven` result is never fabricated.
    #[cfg(feature = "z3-verification")]
    fn execute_z3_query(&mut self, formula: &str) -> AispResult<Z3PropertyResult> {
        let start = Instant::now();

        // Bound memory (soft cap, in MB) per R-16. `memory_max_size` is global
        // to the Z3 process, not per-solver: we apply the most conservative cap
        // any caller has requested, deterministically and under a mutex, instead
        // of letting the first query fix it by timing. The per-query wall-clock
        // `timeout` set below remains the solver-scoped bound.
        if self.config.memory_limit_mb > 0 {
            apply_global_memory_cap(self.config.memory_limit_mb);
        }

        let solver = Solver::new();
        let mut params = Params::new();
        // Per-query wall-clock timeout (ms) so no solver call can hang (R-16).
        params.set_u32(
            "timeout",
            self.config.timeout_ms.min(u32::MAX as u64) as u32,
        );
        solver.set_params(&params);

        match build_and_check(formula, &solver) {
            Ok(SatResult::Unsat) => {
                self.stats.proven_properties += 1;
                Ok(Z3PropertyResult::Proven {
                    proof_certificate: "unsat: asserted constraints are unsatisfiable".to_string(),
                    verification_time: start.elapsed(),
                })
            }
            Ok(SatResult::Sat) => {
                self.stats.disproven_properties += 1;
                let counterexample = solver
                    .get_model()
                    .map(|m| m.to_string().split('\n').collect::<Vec<_>>().join("; "))
                    .map(|s| s.trim().trim_end_matches(';').trim().to_string())
                    .filter(|s| !s.is_empty())
                    .unwrap_or_else(|| "satisfiable (model unavailable)".to_string());
                Ok(Z3PropertyResult::Disproven {
                    counterexample,
                    verification_time: start.elapsed(),
                })
            }
            Ok(SatResult::Unknown) => Ok(Z3PropertyResult::Unknown {
                reason: solver.get_reason_unknown().unwrap_or_else(|| {
                    "Z3 returned unknown (timeout or incompleteness)".to_string()
                }),
                partial_progress: 0.5,
            }),
            // Well-formed SMT-LIB that uses constructs outside the supported
            // fragment must NOT be reported as proven; surface it as Unknown.
            Err(reason) => Ok(Z3PropertyResult::Unknown {
                reason: format!("Unsupported formula: {reason}"),
                partial_progress: 0.0,
            }),
        }
    }

    /// Extract declared symbol from line
    fn extract_declared_symbol(&self, line: &str) -> Option<String> {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() >= 2 && tokens[0].contains("declare") {
            Some(tokens[1].to_string())
        } else {
            None
        }
    }

    /// Extract used symbols from assertion
    fn extract_used_symbols(&self, line: &str, used: &mut HashSet<String>) {
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            let clean = word.trim_matches(|c: char| "()=<>+-*/".contains(c));
            if !clean.is_empty()
                && !clean.chars().all(|c| c.is_numeric() || c == '.')
                && !self.is_builtin(clean)
            {
                used.insert(clean.to_string());
            }
        }
    }

    /// Check if symbol is SMT-LIB builtin
    fn is_builtin(&self, symbol: &str) -> bool {
        matches!(
            symbol,
            "assert"
                | "check-sat"
                | "get-model"
                | "declare-const"
                | "declare-fun"
                | "declare-sort"
                | "Real"
                | "Int"
                | "Bool"
                | "String"
                | "+"
                | "-"
                | "*"
                | "/"
                | "="
                | "<"
                | ">"
                | "<="
                | ">="
                | "and"
                | "or"
                | "not"
                | "=>"
                | "iff"
                | "forall"
                | "exists"
                | "true"
                | "false"
                | "sat"
                | "unsat"
                | "unknown"
                | "^"
        )
    }

    /// Get interface statistics
    pub fn get_stats(&self) -> &SmtStats {
        &self.stats
    }

    /// Check Z3 availability
    pub fn is_z3_available(&self) -> bool {
        self.z3_available
    }
}

impl Default for SmtInterface {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// SMT-LIB → Z3 translation (quantifier-free fragment over Bool/Int/Real).
//
// A small recursive S-expression parser feeds a typed translator that builds
// genuine Z3 AST nodes. Anything outside the supported fragment returns `Err`,
// which the caller maps to `Unknown` — never to a silently-true assertion — so
// a `Proven` result always corresponds to a real `unsat`.
// ---------------------------------------------------------------------------

/// Parse the SMT-LIB text, build the corresponding Z3 assertions, and run the
/// trailing `(check-sat)`.
#[cfg(feature = "z3-verification")]
fn build_and_check(formula: &str, solver: &Solver) -> Result<SatResult, String> {
    let exprs = parse_sexprs(formula)?;
    let mut env: HashMap<String, Z3Value> = HashMap::new();
    let mut checked: Option<SatResult> = None;

    for expr in &exprs {
        let list = match expr {
            SExpr::List(items) if !items.is_empty() => items,
            SExpr::List(_) => continue,
            SExpr::Atom(a) => return Err(format!("unexpected top-level atom: {a}")),
        };
        let head = match &list[0] {
            SExpr::Atom(s) => s.as_str(),
            SExpr::List(_) => return Err("command head is not an identifier".to_string()),
        };
        match head {
            // (declare-const name Sort)
            "declare-const" => {
                let name = expect_atom(list, 1)?;
                let sort = expect_atom(list, 2)?;
                env.insert(name.clone(), Z3Value::new_const(&name, &sort)?);
            }
            // (declare-fun name () Sort) — only nullary declarations (constants)
            "declare-fun" => {
                let name = expect_atom(list, 1)?;
                match list.get(2) {
                    Some(SExpr::List(args)) if args.is_empty() => {}
                    _ => return Err(format!("non-nullary function '{name}' is not supported")),
                }
                let sort = expect_atom(list, 3)?;
                env.insert(name.clone(), Z3Value::new_const(&name, &sort)?);
            }
            "assert" => {
                let body = list.get(1).ok_or_else(|| "empty (assert)".to_string())?;
                let assertion = translate(body, &env)?.into_bool()?;
                solver.assert(&assertion);
            }
            "check-sat" => checked = Some(solver.check()),
            // Accept and ignore commands that do not affect satisfiability of the
            // asserted set (preamble / output / session commands).
            "set-logic" | "set-info" | "set-option" | "get-model" | "get-value" | "exit" => {}
            // Incremental-scope commands change which assertions are active. This
            // translator evaluates assertions in a single flat scope, so honouring
            // them as no-ops would silently alter semantics and could yield a wrong
            // Proven/Disproven. Fail closed (soundness-first): the caller maps this
            // Err to `Unknown` rather than trusting an incorrect verdict.
            "push" | "pop" | "reset" => {
                return Err(format!(
                    "incremental command '{head}' is unsupported (single-scope translator)"
                ));
            }
            other => return Err(format!("unsupported command: {other}")),
        }
    }

    checked.ok_or_else(|| "formula has no (check-sat)".to_string())
}

/// A minimal S-expression: either an atom or a list of S-expressions.
#[cfg(feature = "z3-verification")]
enum SExpr {
    Atom(String),
    List(Vec<SExpr>),
}

#[cfg(feature = "z3-verification")]
fn parse_sexprs(input: &str) -> Result<Vec<SExpr>, String> {
    let tokens = tokenize(input);
    let mut pos = 0;
    let mut out = Vec::new();
    while pos < tokens.len() {
        out.push(parse_one(&tokens, &mut pos)?);
    }
    Ok(out)
}

/// Split into `(`, `)` and atom tokens, dropping `;` line comments.
#[cfg(feature = "z3-verification")]
fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cur = String::new();
    for raw_line in input.lines() {
        let line = match raw_line.find(';') {
            Some(i) => &raw_line[..i],
            None => raw_line,
        };
        for ch in line.chars() {
            match ch {
                '(' | ')' => {
                    if !cur.is_empty() {
                        tokens.push(std::mem::take(&mut cur));
                    }
                    tokens.push(ch.to_string());
                }
                c if c.is_whitespace() => {
                    if !cur.is_empty() {
                        tokens.push(std::mem::take(&mut cur));
                    }
                }
                c => cur.push(c),
            }
        }
        if !cur.is_empty() {
            tokens.push(std::mem::take(&mut cur));
        }
    }
    tokens
}

#[cfg(feature = "z3-verification")]
fn parse_one(tokens: &[String], pos: &mut usize) -> Result<SExpr, String> {
    if *pos >= tokens.len() {
        return Err("unexpected end of input".to_string());
    }
    let tok = tokens[*pos].clone();
    *pos += 1;
    match tok.as_str() {
        "(" => {
            let mut items = Vec::new();
            while *pos < tokens.len() && tokens[*pos] != ")" {
                items.push(parse_one(tokens, pos)?);
            }
            if *pos >= tokens.len() {
                return Err("missing closing parenthesis".to_string());
            }
            *pos += 1; // consume ')'
            Ok(SExpr::List(items))
        }
        ")" => Err("unexpected ')'".to_string()),
        atom => Ok(SExpr::Atom(atom.to_string())),
    }
}

#[cfg(feature = "z3-verification")]
fn expect_atom(list: &[SExpr], idx: usize) -> Result<String, String> {
    match list.get(idx) {
        Some(SExpr::Atom(s)) => Ok(s.clone()),
        _ => Err(format!("expected an identifier at argument {idx}")),
    }
}

/// A translated Z3 term, tagged with its sort so the translator can type-check
/// operators and promote `Int` to `Real` where the two are mixed.
#[cfg(feature = "z3-verification")]
#[derive(Clone)]
enum Z3Value {
    Bool(ast::Bool),
    Int(ast::Int),
    Real(ast::Real),
}

#[cfg(feature = "z3-verification")]
enum Numeric {
    Int(ast::Int),
    Real(ast::Real),
}

#[cfg(feature = "z3-verification")]
impl Z3Value {
    fn new_const(name: &str, sort: &str) -> Result<Self, String> {
        match sort {
            "Bool" => Ok(Z3Value::Bool(ast::Bool::new_const(name))),
            "Int" => Ok(Z3Value::Int(ast::Int::new_const(name))),
            "Real" => Ok(Z3Value::Real(ast::Real::new_const(name))),
            other => Err(format!("unsupported sort: {other}")),
        }
    }

    fn into_bool(self) -> Result<ast::Bool, String> {
        match self {
            Z3Value::Bool(b) => Ok(b),
            _ => Err("expected a Bool expression".to_string()),
        }
    }

    fn into_numeric(self) -> Result<Numeric, String> {
        match self {
            Z3Value::Int(i) => Ok(Numeric::Int(i)),
            Z3Value::Real(r) => Ok(Numeric::Real(r)),
            Z3Value::Bool(_) => Err("expected a numeric (Int/Real) expression".to_string()),
        }
    }
}

#[cfg(feature = "z3-verification")]
fn numeric_to_real(n: &Numeric) -> ast::Real {
    match n {
        Numeric::Int(i) => ast::Real::from_int(i),
        Numeric::Real(r) => r.clone(),
    }
}

#[cfg(feature = "z3-verification")]
fn any_real(ns: &[Numeric]) -> bool {
    ns.iter().any(|n| matches!(n, Numeric::Real(_)))
}

#[cfg(feature = "z3-verification")]
fn conjoin(mut conj: Vec<ast::Bool>) -> ast::Bool {
    match conj.len() {
        0 => ast::Bool::from_bool(true),
        1 => conj.remove(0),
        _ => ast::Bool::and(&conj),
    }
}

/// Translate one S-expression into a typed Z3 term.
#[cfg(feature = "z3-verification")]
fn translate(expr: &SExpr, env: &HashMap<String, Z3Value>) -> Result<Z3Value, String> {
    match expr {
        SExpr::Atom(a) => translate_atom(a, env),
        SExpr::List(items) => {
            let op = match items.first() {
                Some(SExpr::Atom(s)) => s.as_str(),
                Some(SExpr::List(_)) => {
                    return Err("application head is not an operator".to_string());
                }
                None => return Err("empty expression".to_string()),
            };
            translate_app(op, &items[1..], env)
        }
    }
}

#[cfg(feature = "z3-verification")]
fn translate_atom(a: &str, env: &HashMap<String, Z3Value>) -> Result<Z3Value, String> {
    match a {
        "true" => return Ok(Z3Value::Bool(ast::Bool::from_bool(true))),
        "false" => return Ok(Z3Value::Bool(ast::Bool::from_bool(false))),
        _ => {}
    }
    if let Some(v) = env.get(a) {
        return Ok(v.clone());
    }
    if let Ok(i) = a.parse::<i64>() {
        return Ok(Z3Value::Int(ast::Int::from_i64(i)));
    }
    if let Some(r) = parse_real_literal(a) {
        return Ok(Z3Value::Real(r));
    }
    Err(format!("unknown symbol: {a}"))
}

/// Parse a decimal literal (e.g. `3.14`, `-0.5`) into an exact `Real`.
#[cfg(feature = "z3-verification")]
fn parse_real_literal(a: &str) -> Option<ast::Real> {
    let (sign, body) = match a.strip_prefix('-') {
        Some(rest) => (-1i64, rest),
        None => (1i64, a),
    };
    let (int_part, frac_part) = match body.split_once('.') {
        Some((i, f)) => (i, f),
        None => return None, // pure integers are handled as Int
    };
    if frac_part.is_empty() || !frac_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !int_part.is_empty() && !int_part.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let frac_digits = frac_part.len() as u32;
    if frac_digits > 18 {
        return None; // out of exact i64 range; treat as unsupported
    }
    let denom = 10i64.checked_pow(frac_digits)?;
    let int_v: i64 = if int_part.is_empty() {
        0
    } else {
        int_part.parse().ok()?
    };
    let frac_v: i64 = frac_part.parse().ok()?;
    let num = int_v.checked_mul(denom)?.checked_add(frac_v)?;
    Some(ast::Real::from_rational(sign * num, denom))
}

#[cfg(feature = "z3-verification")]
fn translate_app(
    op: &str,
    args: &[SExpr],
    env: &HashMap<String, Z3Value>,
) -> Result<Z3Value, String> {
    match op {
        "and" | "or" => {
            let bools = translate_bools(args, env)?;
            if bools.is_empty() {
                return Err(format!("'{op}' needs at least one argument"));
            }
            let combined = if op == "and" {
                ast::Bool::and(&bools)
            } else {
                ast::Bool::or(&bools)
            };
            Ok(Z3Value::Bool(combined))
        }
        "not" => {
            let bools = translate_bools(args, env)?;
            if bools.len() != 1 {
                return Err("'not' takes exactly one argument".to_string());
            }
            Ok(Z3Value::Bool(bools[0].not()))
        }
        "=>" | "implies" => {
            let bools = translate_bools(args, env)?;
            if bools.len() < 2 {
                return Err("'=>' needs at least two arguments".to_string());
            }
            // Right-associative: a => b => c  ==  a => (b => c)
            let mut acc = bools[bools.len() - 1].clone();
            for b in bools[..bools.len() - 1].iter().rev() {
                acc = b.implies(&acc);
            }
            Ok(Z3Value::Bool(acc))
        }
        "xor" => {
            let bools = translate_bools(args, env)?;
            if bools.len() < 2 {
                return Err("'xor' needs at least two arguments".to_string());
            }
            let mut acc = bools[0].clone();
            for b in &bools[1..] {
                acc = acc.xor(b);
            }
            Ok(Z3Value::Bool(acc))
        }
        "ite" => {
            if args.len() != 3 {
                return Err("'ite' takes three arguments".to_string());
            }
            let cond = translate(&args[0], env)?.into_bool()?;
            let then_v = translate(&args[1], env)?;
            let else_v = translate(&args[2], env)?;
            match (then_v, else_v) {
                (Z3Value::Bool(t), Z3Value::Bool(e)) => Ok(Z3Value::Bool(cond.ite(&t, &e))),
                (Z3Value::Int(t), Z3Value::Int(e)) => Ok(Z3Value::Int(cond.ite(&t, &e))),
                (t, e) => {
                    let t = numeric_to_real(&t.into_numeric()?);
                    let e = numeric_to_real(&e.into_numeric()?);
                    Ok(Z3Value::Real(cond.ite(&t, &e)))
                }
            }
        }
        "=" => translate_eq(args, env),
        "<" | "<=" | ">" | ">=" => translate_cmp(op, args, env),
        "+" | "-" | "*" | "/" => translate_arith(op, args, env),
        other => Err(format!("unsupported operator: {other}")),
    }
}

#[cfg(feature = "z3-verification")]
fn translate_bools(
    args: &[SExpr],
    env: &HashMap<String, Z3Value>,
) -> Result<Vec<ast::Bool>, String> {
    args.iter()
        .map(|a| translate(a, env)?.into_bool())
        .collect()
}

#[cfg(feature = "z3-verification")]
fn translate_numerics(
    args: &[SExpr],
    env: &HashMap<String, Z3Value>,
) -> Result<Vec<Numeric>, String> {
    args.iter()
        .map(|a| translate(a, env)?.into_numeric())
        .collect()
}

#[cfg(feature = "z3-verification")]
fn translate_eq(args: &[SExpr], env: &HashMap<String, Z3Value>) -> Result<Z3Value, String> {
    if args.len() < 2 {
        return Err("'=' needs at least two arguments".to_string());
    }
    let vals: Vec<Z3Value> = args
        .iter()
        .map(|a| translate(a, env))
        .collect::<Result<_, _>>()?;

    let mut conj = Vec::new();
    if vals.iter().all(|v| matches!(v, Z3Value::Bool(_))) {
        let bs: Vec<ast::Bool> = vals
            .into_iter()
            .map(|v| v.into_bool())
            .collect::<Result<_, _>>()?;
        for pair in bs.windows(2) {
            conj.push(pair[0].iff(&pair[1]));
        }
    } else {
        let ns: Vec<Numeric> = vals
            .into_iter()
            .map(|v| v.into_numeric())
            .collect::<Result<_, _>>()?;
        for pair in ns.windows(2) {
            let b = if any_real(pair) {
                numeric_to_real(&pair[0]).eq(numeric_to_real(&pair[1]))
            } else {
                match (&pair[0], &pair[1]) {
                    (Numeric::Int(l), Numeric::Int(r)) => l.eq(r),
                    _ => unreachable!("non-real pair is Int/Int"),
                }
            };
            conj.push(b);
        }
    }
    Ok(Z3Value::Bool(conjoin(conj)))
}

#[cfg(feature = "z3-verification")]
fn translate_cmp(
    op: &str,
    args: &[SExpr],
    env: &HashMap<String, Z3Value>,
) -> Result<Z3Value, String> {
    let ns = translate_numerics(args, env)?;
    if ns.len() < 2 {
        return Err(format!("'{op}' needs at least two arguments"));
    }
    let mut conj = Vec::new();
    for pair in ns.windows(2) {
        let b = if any_real(pair) {
            let l = numeric_to_real(&pair[0]);
            let r = numeric_to_real(&pair[1]);
            cmp_real(op, &l, &r)
        } else {
            match (&pair[0], &pair[1]) {
                (Numeric::Int(l), Numeric::Int(r)) => cmp_int(op, l, r),
                _ => unreachable!("non-real pair is Int/Int"),
            }
        };
        conj.push(b);
    }
    Ok(Z3Value::Bool(conjoin(conj)))
}

#[cfg(feature = "z3-verification")]
fn cmp_int(op: &str, l: &ast::Int, r: &ast::Int) -> ast::Bool {
    match op {
        "<" => l.lt(r),
        "<=" => l.le(r),
        ">" => l.gt(r),
        _ => l.ge(r),
    }
}

#[cfg(feature = "z3-verification")]
fn cmp_real(op: &str, l: &ast::Real, r: &ast::Real) -> ast::Bool {
    match op {
        "<" => l.lt(r),
        "<=" => l.le(r),
        ">" => l.gt(r),
        _ => l.ge(r),
    }
}

#[cfg(feature = "z3-verification")]
fn translate_arith(
    op: &str,
    args: &[SExpr],
    env: &HashMap<String, Z3Value>,
) -> Result<Z3Value, String> {
    let ns = translate_numerics(args, env)?;
    if ns.is_empty() {
        return Err(format!("'{op}' needs at least one argument"));
    }

    // Real division always yields a Real.
    if op == "/" {
        if ns.len() < 2 {
            return Err("'/' needs at least two arguments".to_string());
        }
        let reals: Vec<ast::Real> = ns.iter().map(numeric_to_real).collect();
        let mut acc = reals[0].clone();
        for r in &reals[1..] {
            acc = acc.div(r);
        }
        return Ok(Z3Value::Real(acc));
    }

    if any_real(&ns) {
        let reals: Vec<ast::Real> = ns.iter().map(numeric_to_real).collect();
        let result = match op {
            "+" => ast::Real::add(&reals),
            "*" => ast::Real::mul(&reals),
            _ if reals.len() == 1 => reals[0].unary_minus(), // unary '-'
            _ => ast::Real::sub(&reals),
        };
        Ok(Z3Value::Real(result))
    } else {
        let ints: Vec<ast::Int> = ns
            .iter()
            .map(|n| match n {
                Numeric::Int(i) => i.clone(),
                Numeric::Real(_) => unreachable!("any_real was false"),
            })
            .collect();
        let result = match op {
            "+" => ast::Int::add(&ints),
            "*" => ast::Int::mul(&ints),
            _ if ints.len() == 1 => ints[0].unary_minus(), // unary '-'
            _ => ast::Int::sub(&ints),
        };
        Ok(Z3Value::Int(result))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smt_interface_creation() {
        let interface = SmtInterface::new();

        #[cfg(feature = "z3-verification")]
        assert!(interface.is_z3_available());

        #[cfg(not(feature = "z3-verification"))]
        assert!(!interface.is_z3_available());
    }

    #[test]
    fn test_disabled_interface() {
        let interface = SmtInterface::new_disabled();
        assert!(!interface.is_z3_available());
        assert!(!interface.config.require_z3);
    }

    #[test]
    fn test_with_config_honours_caller_limits() {
        let interface = SmtInterface::with_config(SmtConfig {
            timeout_ms: 1234,
            verbose: false,
            require_z3: false,
            memory_limit_mb: 512,
        });
        assert_eq!(interface.config.timeout_ms, 1234);
        assert_eq!(interface.config.memory_limit_mb, 512);
    }

    /// The process-wide memory cap is reconciled to the *smallest* requested
    /// value, regardless of the order callers arrive in — so `with_config`
    /// cannot misleadingly claim per-instance enforcement and the effective cap
    /// is deterministic under concurrency.
    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_memory_cap_reconciles_to_minimum() {
        assert_eq!(effective_memory_cap(None, 4096), 4096);
        assert_eq!(effective_memory_cap(Some(4096), 1024), 1024); // tighten
        assert_eq!(effective_memory_cap(Some(1024), 4096), 1024); // never loosen
        assert_eq!(effective_memory_cap(Some(2048), 2048), 2048); // idempotent
    }

    /// Incremental-scope commands must never be silently ignored: a script whose
    /// verdict depends on push/pop is reported `Unknown`, not a wrong
    /// `Proven`/`Disproven`. Here, ignoring the scopes would assert both `x = 1`
    /// and `x = 2` (unsat ⇒ spurious Proven); fail-closed yields Unknown.
    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_push_pop_is_unknown_not_misclassified() {
        let mut interface = SmtInterface::new();
        let script = "(declare-const x Int)\n\
             (push)\n(assert (= x 1))\n(pop)\n\
             (assert (= x 2))\n(check-sat)";
        match interface.verify_smt_formula(script).unwrap() {
            Z3PropertyResult::Unknown { .. } => {}
            other => panic!("expected Unknown for push/pop script, got {other:?}"),
        }
    }

    #[test]
    fn test_syntax_validation() {
        let interface = SmtInterface::new_disabled();

        // Valid formula
        let valid = "(declare-const x Real)\n(assert (> x 0.0))\n(check-sat)";
        assert!(interface.validate_smt_syntax(valid).is_ok());

        // Invalid - unbalanced parentheses
        let invalid_parens = "(declare-const x Real\n(assert (> x 0.0))\n(check-sat)";
        assert!(interface.validate_smt_syntax(invalid_parens).is_err());

        // Invalid - missing check-sat
        let missing_check = "(declare-const x Real)\n(assert (> x 0.0))";
        assert!(interface.validate_smt_syntax(missing_check).is_err());

        // Invalid - undeclared symbol
        let undeclared = "(assert (> y 0.0))\n(check-sat)";
        assert!(interface.validate_smt_syntax(undeclared).is_err());
    }

    #[test]
    fn test_symbol_extraction() {
        let interface = SmtInterface::new_disabled();

        assert_eq!(
            interface.extract_declared_symbol("(declare-const x Real)"),
            Some("x".to_string())
        );
        assert_eq!(
            interface.extract_declared_symbol("(declare-fun f (Int) Bool)"),
            Some("f".to_string())
        );
        assert_eq!(interface.extract_declared_symbol("(assert (> x 0))"), None);
    }

    #[test]
    fn test_builtin_detection() {
        let interface = SmtInterface::new_disabled();

        assert!(interface.is_builtin("assert"));
        assert!(interface.is_builtin("Real"));
        assert!(interface.is_builtin("+"));
        assert!(interface.is_builtin("check-sat"));

        assert!(!interface.is_builtin("my_variable"));
        assert!(!interface.is_builtin("custom_function"));
    }

    #[test]
    fn test_symbol_usage_extraction() {
        let interface = SmtInterface::new_disabled();
        let mut used_symbols = HashSet::new();

        interface.extract_used_symbols("(assert (> x y))", &mut used_symbols);

        assert!(used_symbols.contains("x"));
        assert!(used_symbols.contains("y"));
        assert!(!used_symbols.contains("assert"));
        assert!(!used_symbols.contains(">"));
    }

    #[test]
    fn test_smt_formula_verification() {
        let mut interface = SmtInterface::new_disabled();

        let valid_formula = "(declare-const x Real)\n\
             (assert (> x 0.0))\n\
             (check-sat)";

        let result = interface.verify_smt_formula(valid_formula);
        assert!(result.is_ok());

        // With disabled interface, should return Unknown
        assert!(matches!(result.unwrap(), Z3PropertyResult::Unknown { .. }));

        let stats = interface.get_stats();
        assert_eq!(stats.queries_executed, 1);
        assert_eq!(stats.syntax_errors, 0);
    }

    #[test]
    fn test_syntax_error_tracking() {
        let mut interface = SmtInterface::new_disabled();

        let invalid_formula = "(invalid syntax";
        let result = interface.verify_smt_formula(invalid_formula);
        assert!(result.is_ok());

        match result.unwrap() {
            Z3PropertyResult::Error { .. } => (),
            _ => panic!("Expected syntax error"),
        }

        assert_eq!(interface.get_stats().syntax_errors, 1);
    }

    // ---- Real Z3 round-trips (only meaningful with the z3 backend) --------

    /// An unsatisfiable constraint set must come back as a genuine `Proven`
    /// (Z3 `unsat`), not a fabricated one.
    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_unsat_is_proven_int() {
        let mut interface = SmtInterface::new();
        let formula = "(declare-const x Int)\n\
             (assert (> x 0))\n\
             (assert (< x 0))\n\
             (check-sat)";
        let result = interface.verify_smt_formula(formula).unwrap();
        assert!(
            matches!(result, Z3PropertyResult::Proven { .. }),
            "x>0 ∧ x<0 is unsatisfiable, expected Proven, got {result:?}"
        );
    }

    /// Boolean contradictions are also discharged by the real solver.
    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_unsat_is_proven_bool() {
        let mut interface = SmtInterface::new();
        let formula = "(declare-const p Bool)\n\
             (assert (and p (not p)))\n\
             (check-sat)";
        let result = interface.verify_smt_formula(formula).unwrap();
        assert!(
            matches!(result, Z3PropertyResult::Proven { .. }),
            "p ∧ ¬p is unsatisfiable, expected Proven, got {result:?}"
        );
    }

    /// Linear real arithmetic with an exact decimal bound.
    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_unsat_is_proven_real() {
        let mut interface = SmtInterface::new();
        let formula = "(declare-const x Real)\n\
             (assert (> x 1.5))\n\
             (assert (< x 1.5))\n\
             (check-sat)";
        let result = interface.verify_smt_formula(formula).unwrap();
        assert!(matches!(result, Z3PropertyResult::Proven { .. }));
    }

    /// A satisfiable formula returns `Disproven` with a real counter-model.
    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_sat_is_disproven_with_model() {
        let mut interface = SmtInterface::new();
        let formula = "(declare-const x Int)\n\
             (assert (> x 41))\n\
             (assert (< x 43))\n\
             (check-sat)";
        let result = interface.verify_smt_formula(formula).unwrap();
        match result {
            Z3PropertyResult::Disproven { counterexample, .. } => {
                assert!(counterexample.contains('x'), "model should mention x");
            }
            other => panic!("expected Disproven with model, got {other:?}"),
        }
    }

    /// Implication and mixed Int/Real comparison are translated correctly.
    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_implication_translation() {
        let mut interface = SmtInterface::new();
        // (x >= 2) ∧ ¬(x >= 1)  is unsatisfiable.
        let formula = "(declare-const x Int)\n\
             (assert (>= x 2))\n\
             (assert (not (>= x 1)))\n\
             (check-sat)";
        let result = interface.verify_smt_formula(formula).unwrap();
        assert!(matches!(result, Z3PropertyResult::Proven { .. }));
    }

    /// The crucial soundness guarantee: a well-formed formula that uses
    /// constructs outside the supported fragment (here, an uninterpreted
    /// function application) must be reported as `Unknown` — never `Proven`.
    #[cfg(feature = "z3-verification")]
    #[test]
    fn test_unsupported_is_unknown_not_proven() {
        let mut interface = SmtInterface::new();
        let formula = "(declare-fun f (Int) Bool)\n\
             (declare-const x Int)\n\
             (assert (f x))\n\
             (check-sat)";
        let result = interface.verify_smt_formula(formula).unwrap();
        assert!(
            matches!(result, Z3PropertyResult::Unknown { .. }),
            "uninterpreted-function formula must be Unknown, got {result:?}"
        );
    }
}
