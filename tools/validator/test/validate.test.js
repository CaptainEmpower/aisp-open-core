/**
 * Smoke tests for the core validate() path
 */

import { describe, it, before } from 'node:test';
import assert from 'node:assert';
import { init, validate, isValid, getTier } from '../src/index.js';

const VALID_DOC = `𝔸1.0.test@2026-01-16
γ≔test
⟦Ω:Meta⟧{ ∀D:Ambig(D)<0.02 }
⟦Σ:Types⟧{ T≜ℕ }
⟦Γ:Rules⟧{ ∀x:T:x≥0 }
⟦Λ:Funcs⟧{ f≜λx.x }
⟦Ε⟧⟨δ≜0.75;τ≜◊⁺⁺⟩`;

describe('validate() smoke tests', () => {
    before(async () => {
        await init();
    });

    it('validates a well-formed document', () => {
        const result = validate(VALID_DOC);
        assert.strictEqual(typeof result, 'object');
        assert.strictEqual(result.valid, true);
        assert.ok(result.tier, 'result should include a tier');
        assert.strictEqual(typeof result.delta, 'number');
    });

    it('rejects an empty document', () => {
        const result = validate('');
        assert.strictEqual(result.valid, false);
    });

    it('rejects plain prose with no AISP structure', () => {
        const result = validate('just some ordinary text with no blocks');
        assert.strictEqual(result.valid, false);
    });

    it('isValid() agrees with validate().valid', () => {
        assert.strictEqual(isValid(VALID_DOC), validate(VALID_DOC).valid);
    });

    it('getTier() returns a tier string for a valid document', () => {
        const tier = getTier(VALID_DOC);
        assert.strictEqual(typeof tier, 'string');
        assert.ok(tier.length > 0);
    });
});
