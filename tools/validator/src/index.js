/**
 * AISP Validator - Node.js Entry Point
 * Validates AISP 5.1 documents using the WASM kernel
 */

import { readFile } from 'fs/promises';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';
import { validateWithZ3 } from './z3-bridge.js';
import { RelationalAnalyzer } from './relational-analyzer.js';
import { TemporalAnalyzer } from './temporal-analyzer.js';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/**
 * AISP Symbol Set (Σ_512 subset for density calculation)
 * Formal symbols that define AISP semantic content
 */
const AISP_SYMBOLS = [
    // Block delimiters
    '⟦', '⟧',
    // Operators
    '≜', '≔', '≡', '≢',
    // Quantifiers
    '∀', '∃',
    // Lambda
    'λ',
    // Logic
    '⇒', '⇔', '→', '↔', '∧', '∨', '¬', '⊕',
    // Sets
    '∈', '∉', '⊆', '⊇', '∩', '∪', '∅', '𝒫',
    // Relations
    '≤', '≥', '<', '>',
    // Types
    'ℕ', 'ℤ', 'ℝ', '𝔹', '𝕊',
    // Document
    '𝔸',
    // Tier symbols
    '◊', '⊘',
    // Angle brackets (tuples)
    '⟨', '⟩',
    // Greek letters (commonly used)
    'α', 'β', 'γ', 'δ', 'ε', 'φ', 'τ', 'ρ', 'Ω', 'Σ', 'Γ', 'Λ', 'Ε', 'Θ', 'Χ', 'Δ', 'Π'
];

/**
 * Calculate pure density: |AISP_symbols| ÷ |non_ws_tokens|
 * @param {string} text - AISP source
 * @returns {object} { pureDensity, symbolCount, tokenCount }
 */
function calculatePureDensity(text) {
    // Count AISP symbols
    let symbolCount = 0;
    for (const symbol of AISP_SYMBOLS) {
        const matches = text.split(symbol).length - 1;
        symbolCount += matches;
    }

    // Count non-whitespace tokens (split on whitespace, filter empty)
    const tokens = text.split(/\s+/).filter(t => t.length > 0);
    const tokenCount = tokens.length;

    // Pure density: ratio of AISP symbols to tokens
    const pureDensity = tokenCount > 0 ? symbolCount / tokenCount : 0;

    return {
        pureDensity,
        symbolCount,
        tokenCount
    };
}

/**
 * Calculate semantic density using Block Coverage + Binding Density
 * δ = (blockScore × 0.4) + (bindingScore × 0.6)
 * @param {string} text - AISP source
 * @returns {object} { delta, blockScore, bindingScore, breakdown, pureDensity }
 */
function calculateSemanticDensity(text) {
    // Required blocks present?
    const blocks = ['⟦Ω', '⟦Σ', '⟦Γ', '⟦Λ', '⟦Ε'];
    const blocksFound = blocks.filter(b => text.includes(b));
    const blockScore = blocksFound.length / blocks.length;

    // Binding density (semantic operators)
    const definitions = (text.match(/≜/g) || []).length;
    const assignments = (text.match(/≔/g) || []).length;
    const quantifiers = (text.match(/[∀∃]/g) || []).length;
    const lambdas = (text.match(/λ/g) || []).length;
    const implications = (text.match(/[⇒⇔→↔]/g) || []).length;
    const setOps = (text.match(/[∈⊆∩∪∅]/g) || []).length;

    const totalBindings = definitions + assignments + quantifiers + lambdas + implications + setOps;
    const bindingScore = Math.min(1, totalBindings / 20);

    // Combined score (semantic delta)
    const delta = (blockScore * 0.4) + (bindingScore * 0.6);

    // Pure density (symbol/token ratio)
    const pureResult = calculatePureDensity(text);

    return {
        delta,
        blockScore,
        bindingScore,
        pureDensity: pureResult.pureDensity,
        breakdown: {
            blocksFound: blocksFound.length,
            blocksRequired: blocks.length,
            definitions,
            assignments,
            quantifiers,
            lambdas,
            implications,
            setOps,
            totalBindings,
            symbolCount: pureResult.symbolCount,
            tokenCount: pureResult.tokenCount
        }
    };
}

/**
 * Get tier from delta value
 * @param {number} delta
 * @returns {object} { tier, tierValue, tierName }
 */
function getTierFromDelta(delta) {
    if (delta >= 0.75) return { tier: '◊⁺⁺', tierValue: 4, tierName: 'Platinum' };
    if (delta >= 0.60) return { tier: '◊⁺', tierValue: 3, tierName: 'Gold' };
    if (delta >= 0.40) return { tier: '◊', tierValue: 2, tierName: 'Silver' };
    if (delta >= 0.20) return { tier: '◊⁻', tierValue: 1, tierName: 'Bronze' };
    return { tier: '⊘', tierValue: 0, tierName: 'Reject' };
}

/**
 * Supported file extensions for AISP documents
 */
const SUPPORTED_EXTENSIONS = ['.aisp', '.md', '.txt', '.spec', '.aisp5'];

/**
 * Default and max document size limits
 */
const SIZE_LIMITS = {
    DEFAULT_MAX: 64 * 1024,  // 64KB default
    ABSOLUTE_MAX: 1024 * 1024, // 1MB absolute max
    WASM_MAX: 1024  // WASM kernel limit
};

const AISP = {
    _instance: null,
    _memory: null,
    _allocPtr: 0x1000,
    _initialized: false,
    _maxDocSize: SIZE_LIMITS.DEFAULT_MAX,

    /**
     * Initialize AISP kernel
     * @param {string|object} [options] - Path to aisp.wasm or options object
     * @param {string} [options.wasmPath] - Path to aisp.wasm
     * @param {number} [options.maxDocSize] - Maximum document size in bytes (default: 64KB)
     * @returns {Promise<number>} 0 on success
     */
    async init(options) {
        if (this._initialized) return 0;

        // Handle both string (legacy) and object (new) options
        let wasmPath;
        if (typeof options === 'string') {
            wasmPath = options;
        } else if (options && typeof options === 'object') {
            wasmPath = options.wasmPath;
            if (options.maxDocSize) {
                this._maxDocSize = Math.min(options.maxDocSize, SIZE_LIMITS.ABSOLUTE_MAX);
            }
        }

        const path = wasmPath || join(__dirname, '..', 'wasm', 'aisp.wasm');
        const bytes = await readFile(path);

        const { instance } = await WebAssembly.instantiate(bytes, {
            env: {
                host_alloc: (size, align) => {
                    const aligned = (this._allocPtr + align - 1) & ~(align - 1);
                    this._allocPtr = aligned + size;
                    return aligned;
                }
            }
        });

        this._instance = instance.exports;
        this._memory = new Uint8Array(instance.exports.memory.buffer);
        this._initialized = true;

        return this._instance.aisp_init();
    },

    /**
     * Set maximum document size
     * @param {number} size - Maximum size in bytes
     */
    setMaxDocSize(size) {
        this._maxDocSize = Math.min(size, SIZE_LIMITS.ABSOLUTE_MAX);
    },

    /**
     * Validate AISP document
     * @param {string} source - AISP source code
     * @param {object} [options] - Validation options
     * @param {boolean} [options.strict] - Use strict WASM validation (limited to 1KB)
     * @param {boolean} [options.z3] - Enable Z3 formal verification
     * @returns {object} Validation result
     */
    validate(source, options = {}) {
        if (!this._instance) {
            throw new Error('AISP not initialized. Call init() first.');
        }

        const encoder = new TextEncoder();
        const bytes = encoder.encode(source);

        // Check size against configurable limit
        if (bytes.length > this._maxDocSize) {
            return {
                valid: false,
                error: `Document too large (${bytes.length} bytes, max ${this._maxDocSize} bytes)`,
                errorCode: -4
            };
        }

        // Use semantic density calculation (works for all sizes)
        const densityResult = calculateSemanticDensity(source);
        const tierResult = getTierFromDelta(densityResult.delta);

        // For documents within WASM limit, use WASM validation
        if (bytes.length <= SIZE_LIMITS.WASM_MAX && options.strict !== false) {
            const ptr = 0x1000;
            this._memory = new Uint8Array(this._instance.memory.buffer);
            this._memory.set(bytes, ptr);

            const docId = this._instance.aisp_parse(ptr, bytes.length);
            if (docId < 0) {
                return {
                    valid: false,
                    error: `Parse error at offset ${this._instance.aisp_error_offset()}`,
                    errorCode: this._instance.aisp_error_code(),
                    tier: tierResult.tier,
                    tierValue: tierResult.tierValue,
                    tierName: tierResult.tierName,
                    delta: densityResult.delta,
                    pureDensity: densityResult.pureDensity
                };
            }

            const parseResult = this._instance.aisp_validate(docId);

            const result = {
                valid: parseResult === 0,
                tier: tierResult.tier,
                tierValue: tierResult.tierValue,
                tierName: tierResult.tierName,
                delta: densityResult.delta,
                pureDensity: densityResult.pureDensity,
                ambiguity: this._instance.aisp_ambig(docId),
                errorCode: parseResult,
                mode: 'wasm'
            };

            // Note: Z3 validation is async, use validateAsync() for Z3 support
            if (options.z3) {
                result.z3 = { 
                    available: false, 
                    message: 'Use validateAsync() for Z3 formal verification' 
                };
            }

            return result;
        }

        // For larger documents, use pure JS validation
        const jsValidation = this._validatePureJS(source, densityResult);

        const result = {
            valid: jsValidation.valid,
            tier: tierResult.tier,
            tierValue: tierResult.tierValue,
            tierName: tierResult.tierName,
            delta: densityResult.delta,
            pureDensity: densityResult.pureDensity,
            ambiguity: jsValidation.ambiguity,
            errorCode: jsValidation.valid ? 0 : -3,
            mode: 'js',
            docSize: bytes.length
        };

        // Note: Z3 validation is async, use validateAsync() for Z3 support
        if (options.z3) {
            result.z3 = { 
                available: false, 
                message: 'Use validateAsync() for Z3 formal verification' 
            };
        }

        return result;
    },

    /**
     * Validate AISP document with async Z3, relational, and temporal analysis support
     * @param {string} source - AISP source code
     * @param {object} [options] - Validation options
     * @param {boolean} [options.strict] - Use strict WASM validation (limited to 1KB)
     * @param {boolean} [options.z3] - Enable Z3 formal verification
     * @param {boolean} [options.relational] - Enable Level 4 relational logic analysis
     * @param {boolean} [options.temporal] - Enable Level 5 temporal logic analysis
     * @returns {Promise<object>} Validation result
     */
    async validateAsync(source, options = {}) {
        // First run normal validation
        const result = this.validate(source, { ...options, z3: false });
        
        // Add Z3 validation if requested and document is valid
        if (options.z3 && result.valid) {
            await this._addZ3Validation(source, result, options);
        }
        
        // Add relational analysis if requested and document is valid
        if (options.relational && result.valid) {
            await this._addRelationalAnalysis(source, result, options);
        }
        
        // Add temporal analysis if requested and document is valid
        if (options.temporal && result.valid) {
            await this._addTemporalAnalysis(source, result, options);
        }
        
        return result;
    },

    /**
     * Add Z3 formal verification to validation result
     * @private
     */
    async _addZ3Validation(source, result, options) {
        try {
            const z3Result = await validateWithZ3(source, options);
            
            result.z3 = {
                available: z3Result.z3Available,
                satisfiable: z3Result.satisfiable,
                constructs: z3Result.constructs || 0,
                runtime: z3Result.runtime || 0
            };
            
            if (!z3Result.valid) {
                result.z3.error = z3Result.error;
                result.z3.warning = z3Result.warning;
            }
            
            if (options.debug) {
                result.z3.smtlib = z3Result.smtlib;
            }
            
            // Z3 validation failure doesn't invalidate the document,
            // but we note it for transparency
            if (z3Result.satisfiable === false) {
                result.z3.inconsistent = true;
            }
            
        } catch (error) {
            result.z3 = {
                available: false,
                error: `Z3 integration error: ${error.message}`
            };
        }
    },

    /**
     * Add Level 4 relational logic analysis to validation result
     * @private
     */
    async _addRelationalAnalysis(source, result, options) {
        try {
            const analyzer = new RelationalAnalyzer();
            const relationalResult = await analyzer.analyze(source, options);
            
            result.relational = {
                level: relationalResult.level,
                name: relationalResult.name,
                runtime: relationalResult.runtime,
                rules: relationalResult.rules,
                dependencies: relationalResult.dependencies,
                metrics: relationalResult.metrics,
                complexity: relationalResult.complexity,
                completeness: relationalResult.completeness
            };
            
            // Include detailed analysis in debug mode
            if (options.debug) {
                result.relational.dependencyGraph = relationalResult.dependencyGraph;
                result.relational.consistencyChains = relationalResult.consistencyChains;
                result.relational.invariants = relationalResult.invariants;
            }
            
            // Add depth measurement
            result.relational.depth = this._calculateLogicDepth(relationalResult);
            
        } catch (error) {
            result.relational = {
                available: false,
                error: `Relational analysis error: ${error.message}`
            };
        }
    },

    /**
     * Add Level 5 temporal logic analysis to validation result
     * @private
     */
    async _addTemporalAnalysis(source, result, options) {
        try {
            const analyzer = new TemporalAnalyzer();
            const temporalResult = await analyzer.analyze(source, options);
            
            result.temporal = {
                level: temporalResult.level,
                name: temporalResult.name,
                runtime: temporalResult.runtime,
                stateModel: {
                    states: temporalResult.stateModel.states.length,
                    transitions: temporalResult.stateModel.transitions.length,
                    stateSpace: temporalResult.stateModel.stateSpace
                },
                properties: temporalResult.properties.length,
                modelChecking: temporalResult.modelCheckingResults,
                consistency: temporalResult.consistencyAnalysis,
                complexity: temporalResult.temporalComplexity,
                metrics: temporalResult.metrics
            };
            
            // Include detailed analysis in debug mode
            if (options.debug) {
                result.temporal.detailedStateModel = temporalResult.stateModel;
                result.temporal.detailedProperties = temporalResult.properties;
                result.temporal.relationalFoundation = temporalResult.relationalFoundation;
            }
            
            // Update depth measurement to Level 5
            result.temporal.depth = this._calculateTemporalLogicDepth(temporalResult);
            
        } catch (error) {
            result.temporal = {
                available: false,
                error: `Temporal analysis error: ${error.message}`
            };
        }
    },

    /**
     * Calculate logic measurement depth including temporal analysis
     * @private
     */
    _calculateTemporalLogicDepth(temporalResult) {
        let depth = 5; // Level 5 temporal logic achieved
        
        // Verify Level 5 indicators
        const hasTemporalOperators = temporalResult.properties.length > 0;
        const hasStateModel = temporalResult.stateModel.states.length > 0;
        const hasModelChecking = temporalResult.modelCheckingResults.verified + 
                                 temporalResult.modelCheckingResults.violated > 0;
        
        if (!hasTemporalOperators || !hasStateModel || !hasModelChecking) {
            depth = 4; // Fall back to Level 4 if temporal analysis incomplete
        }
        
        return {
            level: depth,
            maxLevel: 7,
            percentage: (depth / 7 * 100).toFixed(1) + '%',
            description: this._getDepthDescription(depth),
            indicators: {
                temporalOperators: hasTemporalOperators,
                stateModel: hasStateModel,
                modelChecking: hasModelChecking
            }
        };
    },

    /**
     * Calculate logic measurement depth from relational analysis
     * @private
     */
    _calculateLogicDepth(relationalResult) {
        let depth = 3; // Base level (we have syntactic + structural + basic semantic)
        
        // Level 4 indicators
        if (relationalResult.dependencies > 0) depth = 4;
        if (relationalResult.consistencyChains?.length > 0) depth = 4;
        if (relationalResult.invariants?.length > 0) depth = 4;
        
        // Future level indicators (placeholder)
        // if (hasTemporalOperators) depth = 5;
        // if (hasGameTheoreticRules) depth = 6;
        // if (hasHigherOrderLogic) depth = 7;
        
        return {
            level: depth,
            maxLevel: 7,
            percentage: (depth / 7 * 100).toFixed(1) + '%',
            description: this._getDepthDescription(depth)
        };
    },

    /**
     * Get human-readable description of logic depth level
     * @private  
     */
    _getDepthDescription(level) {
        const descriptions = {
            1: 'Surface Syntax',
            2: 'Structural Logic', 
            3: 'Basic Semantic Logic',
            4: 'Relational Logic',
            5: 'Temporal Logic',
            6: 'Game-Theoretic Logic',
            7: 'Higher-Order Logic'
        };
        
        return descriptions[level] || 'Unknown';
    },

    /**
     * Pure JavaScript validation for larger documents
     * @private
     */
    _validatePureJS(source, densityResult) {
        // Check for AISP header
        if (!source.trim().startsWith('𝔸')) {
            return { valid: false, ambiguity: 1.0, error: 'Missing AISP header (𝔸)' };
        }

        // Check for required blocks
        const requiredBlocks = ['⟦Ω', '⟦Σ', '⟦Γ', '⟦Λ', '⟦Ε'];
        const missingBlocks = requiredBlocks.filter(b => !source.includes(b));

        if (missingBlocks.length > 0) {
            return {
                valid: false,
                ambiguity: 0.5,
                error: `Missing required blocks: ${missingBlocks.join(', ')}`
            };
        }

        // Valid if all required blocks present and density >= 0.20 (Bronze tier minimum)
        const valid = densityResult.delta >= 0.20;
        const ambiguity = valid ? 0.01 : 0.5;

        return { valid, ambiguity };
    },

    /**
     * Get detailed density breakdown for debugging
     * @param {string} source - AISP source
     * @returns {object} Detailed breakdown
     */
    debug(source) {
        const densityResult = calculateSemanticDensity(source);
        const tierResult = getTierFromDelta(densityResult.delta);

        return {
            ...tierResult,
            delta: densityResult.delta,
            pureDensity: densityResult.pureDensity,
            blockScore: densityResult.blockScore,
            bindingScore: densityResult.bindingScore,
            breakdown: densityResult.breakdown
        };
    },

    /**
     * Get tier symbol from numeric value
     * @private
     */
    _getTierSymbol(tier) {
        const symbols = ['⊘', '◊⁻', '◊', '◊⁺', '◊⁺⁺'];
        return symbols[tier] || '⊘';
    },

    /**
     * Quick validation check
     * @param {string} source - AISP source
     * @returns {boolean} true if valid
     */
    isValid(source) {
        return this.validate(source).valid;
    },

    /**
     * Get density score
     * @param {string} source - AISP source
     * @returns {number} density δ [0, 1]
     */
    getDensity(source) {
        return this.validate(source).delta;
    },

    /**
     * Get quality tier
     * @param {string} source - AISP source
     * @returns {string} tier symbol
     */
    getTier(source) {
        return this.validate(source).tier;
    },

    /**
     * Validate file
     * @param {string} filePath - Path to AISP file
     * @returns {Promise<object>} Validation result
     */
    async validateFile(filePath) {
        const content = await readFile(filePath, 'utf-8');
        return this.validate(content);
    },

    /**
     * Debug file - show density breakdown
     * @param {string} filePath - Path to AISP file
     * @returns {Promise<object>} Debug breakdown
     */
    async debugFile(filePath) {
        const content = await readFile(filePath, 'utf-8');
        return this.debug(content);
    }
};

export default AISP;
// Bind named exports so `import { validate } from 'aisp-validator'` keeps
// working — plain destructuring would detach methods from AISP and break
// their `this` references.
export const init = AISP.init.bind(AISP);
export const validate = AISP.validate.bind(AISP);
export const validateAsync = AISP.validateAsync.bind(AISP);
export const isValid = AISP.isValid.bind(AISP);
export const getDensity = AISP.getDensity.bind(AISP);
export const getTier = AISP.getTier.bind(AISP);
export const validateFile = AISP.validateFile.bind(AISP);
export const debug = AISP.debug.bind(AISP);
export const debugFile = AISP.debugFile.bind(AISP);
export const setMaxDocSize = AISP.setMaxDocSize.bind(AISP);
export { calculateSemanticDensity, calculatePureDensity, getTierFromDelta, AISP_SYMBOLS, SUPPORTED_EXTENSIONS, SIZE_LIMITS };
