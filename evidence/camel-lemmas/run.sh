#!/usr/bin/env bash
# Acceptance test for the CLI path. For each lemma it prints the verdict and
# whether it agrees with lemmas/MANIFEST.tsv; exits 1 on any disagreement and 2
# on a malformed corpus.
#
# Expected verdicts and partners come from lemmas/MANIFEST.tsv, never from the
# document under test, and corpus.sh refuses (exit 2) a corpus whose partners
# differ anywhere outside their ⟦Γ⟧ block.
#
# Verdict on this path, from the CLI's formal-verification counts (the status
# string is a confidence band, aisp-cli/src/main.rs:409-415, shown but unused):
#   proved  = a formal result with properties_checked > 0 and proven == checked
#   refuted = a formal result with proven < checked
#   none    = anything else: no formal result, no output.
# This is coarser than the driver's: the CLI's JSON reports only how many
# components passed, so "refuted" here means some component failed, whatever
# the reason, where the driver requires a counterexample. The CLI also hides
# parse errors, which the driver turns into "none".
#
# A verdict counts only if the rule decided it: a row agrees only if its
# verdict matches the manifest AND differs from its partner's. Partners are
# identical outside ⟦Γ⟧, so output that does not depend on the rule's content
# (a heuristic score, a hard-coded Proven, ⟦Σ⟧-derived properties, a dropped
# ⟦Γ⟧ block) gives both the same verdict and fails both rows.
set -euo pipefail
cd "$(dirname "$0")/lemmas"
B="${AISP_CLI:-aisp-cli}"
command -v "$B" >/dev/null || { echo "run.sh: aisp-cli not found (set AISP_CLI)" >&2; exit 2; }

# Prints "valid status checked proven verdict" for one document. aisp-cli exits
# 1 for an invalid document, which a refutation may produce, so its exit status
# must not abort the run: the verdict is read from the JSON instead.
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
probe() {
  local out j r
  # Every lemma is handed to the CLI under the same path, so its name cannot
  # tell a verifier which member of a pair it is looking at.
  cp "$1" "$tmp/lemma.aisp"
  out=$("$B" -l formal --formal-verification -f json "$tmp/lemma.aisp" 2>/dev/null) || true
  j=$(sed -n '/^{/,/^}/p' <<<"$out")
  r=$(jq -r '[ (if .valid == null then "none" else (.valid|tostring) end),
               (.formal_verification.status // "none"),
               (.formal_verification.properties_checked // "-" | tostring),
               (.formal_verification.properties_proven // "-" | tostring) ] | join(" ")' \
       <<<"$j" 2>/dev/null) || r=""
  [[ -n $r ]] || r="none none - -"
  local v s c p; read -r v s c p <<<"$r"
  local verdict=none
  if [[ $c =~ ^[0-9]+$ && $p =~ ^[0-9]+$ ]]; then
    if (( c > 0 && p == c )); then verdict=proved; elif (( p < c )); then verdict=refuted; fi
  fi
  echo "$v $s $c $p $verdict"
}

source ../corpus.sh
corpus_load

declare -A V
for f in *.aisp; do V[${f%.aisp}]=$(probe "$f"); done

fail=0
printf '| lemma | expected | valid | formal status | proven/checked | verdict | partner verdict | agrees |\n|---|---|---|---|---|---|---|---|\n'
for f in *.aisp; do
  id=${f%.aisp}
  exp=${EXPECT[$id]}
  read -r v s c p verdict <<<"${V[$id]}"
  read -r _ _ _ _ pv <<<"${V[${PARTNER[$id]}]}"
  if [[ $verdict == "$exp" && $pv != missing && $pv != "$verdict" ]]; then a=yes; else a=**no**; fail=1; fi
  printf '| %s | %s | %s | %s | %s/%s | %s | %s | %s |\n' "$id" "$exp" "$v" "$s" "$p" "$c" "$verdict" "$pv" "$a"
done
exit "$fail"
