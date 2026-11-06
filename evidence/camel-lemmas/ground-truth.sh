#!/usr/bin/env bash
# Checks each lemma's expected verdict against Z3 directly, independently of
# aisp-core. Each lemma is hand-encoded in SMT-LIB below; its negation must be
# `unsat` when expect≜proved and `sat` when expect≜refuted. A lemma with no
# encoding, or an `unknown`, fails: an expected verdict the solver cannot
# decide is not a known answer.
#
# ℕ is Int with every variable ≥ 0. ℕ subtraction (T3) is left as Int
# subtraction: the hypothesis d≤b-s is only reachable with s≤b, where the two
# agree. Sets of ℕ are (Array Int Bool), with ∪ as a pointwise `or`.
set -uo pipefail
cd "$(dirname "$0")"
Z3="${Z3:-z3}"
command -v "$Z3" >/dev/null || { echo "ground-truth.sh: z3 not found (set Z3)" >&2; exit 2; }

nat='(declare-const b Int)(declare-const s Int)(declare-const d Int)(declare-const t Int)(declare-const n Int)(declare-const x Int)(assert (and (>= b 0)(>= s 0)(>= d 0)(>= t 0)(>= n 0)(>= x 0)))'
bool='(declare-const h Bool)(declare-const deny Bool)(declare-const confirm Bool)'
sets='(declare-const A (Array Int Bool))(declare-const C (Array Int Bool))(define-fun U ((p (Array Int Bool))(r (Array Int Bool))) (Array Int Bool) ((_ map or) p r))(define-fun E () (Array Int Bool) ((as const (Array Int Bool)) false))'
charge='(define-fun Charge ((b Int)(s Int)(d Int)) Bool (<= (+ s d) b))'
step='(or (and (<= (+ s d) b) (= n (+ s d))) (and (> (+ s d) b) (= n s)))'

# id → "declarations|formula"
declare -A L=(
  [C1-contradiction]="|(< x x)"
  [C2-tautology]="|(<= x x)"
  [C3-uses-lambda]="$charge|(=> (Charge b s d) (< (+ s d) b))"
  [C4-needs-lambda]="$charge|(=> (Charge b s d) (<= (+ s d) b))"
  [T1-charge-sound]="|(=> (<= (+ s d) b) (<= (+ s d) b))"
  [F1-charge-sound]="|(=> (<= (+ s d) b) (< (+ s d) b))"
  [T2-exhausted-stays]="|(=> (and (> (+ s d) b) (<= s t)) (> (+ t d) b))"
  [F2-exhausted-stays]="|(=> (> (+ s d) b) (> (+ t d) b))"
  [T3-fits-charged]="|(=> (and (<= s b) (<= d (- b s))) (<= (+ s d) b))"
  [F3-fits-charged]="|(=> (<= s b) (<= (+ s d) b))"
  [T4-run-stays-in-budget]="|(=> (and (<= s b) $step) (<= n b))"
  [F4-run-stays-in-budget]="|(=> $step (<= n b))"
  [T5-join-commutes]="$sets|(= (U A C) (U C A))"
  [F5-join-commutes]="$sets|(= (U A C) A)"
  [T6-trust-meet]="$sets|(= (= (U A C) E) (and (= A E) (= C E)))"
  [F6-trust-meet]="$sets|(= (= (U A C) E) (or (= A E) (= C E)))"
  [T7-deny-beats-confirm]="|(=> (and (= deny h) (= confirm (not h))) (=> h (not confirm)))"
  [F7-deny-beats-confirm]="|(=> (and (= deny h) (= confirm (not h))) (=> h confirm))"
)

fail=0
printf '| lemma | expected | z3 on the negation | agrees |\n|---|---|---|---|\n'
for f in lemmas/*.aisp; do
  id=$(basename "$f" .aisp)
  exp=$(sed -n 's/^ *expect≜//p' "$f")
  if [[ -z ${L[$id]+x} ]]; then r="no encoding"
  else
    r=$("$Z3" -T:20 -in <<<"$nat $bool ${L[$id]%%|*} (assert (not ${L[$id]#*|})) (check-sat)" 2>&1 | head -1)
  fi
  if [[ $exp == proved && $r == unsat ]] || [[ $exp == refuted && $r == sat ]]; then a=yes; else a=**no**; fail=1; fi
  printf '| %s | %s | %s | %s |\n' "$id" "$exp" "$r" "$a"
done
exit "$fail"
