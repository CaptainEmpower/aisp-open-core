#!/usr/bin/env bash
# Acceptance test for the CLI path. For each lemma it prints the verdict and
# whether it agrees with the lemma's `expect≜`; exits 1 on any disagreement.
#
# A verdict counts only if the lemma's ⟦Γ⟧ rule was examined, which the CLI's
# JSON does not report directly. So each lemma is also run with its ⟦Γ⟧ block
# removed, and the rule counts as examined only when removing it changes the
# formal status or the number of properties checked.
#
#   proved  = examined ∧ status AllVerified
#   refuted = examined ∧ status VerificationFailed ∧ valid false
#
# Anything else (PartiallyVerified, no formal result, a parse failure) is not
# a verdict, matching the README's "Unknown counts as not refuted".
set -euo pipefail
cd "$(dirname "$0")/lemmas"
B="${AISP_CLI:-aisp-cli}"
command -v "$B" >/dev/null || { echo "run.sh: aisp-cli not found (set AISP_CLI)" >&2; exit 2; }
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT

# Prints "valid status checked proven" for one document. aisp-cli exits 1 for
# an invalid document, which is what a refutation produces, so its exit
# status must not abort the run: the verdict is read from the JSON instead.
probe() {
  local out j
  out=$("$B" -l formal --formal-verification -f json "$1" 2>/dev/null) || true
  j=$(sed -n '/^{/,/^}/p' <<<"$out")
  jq -r '[ (if .valid == null then "none" else (.valid|tostring) end),
           (.formal_verification.status // "none"),
           (.formal_verification.properties_checked // "-" | tostring),
           (.formal_verification.properties_proven // "-" | tostring) ] | join(" ")' \
     <<<"$j" 2>/dev/null || echo "none none - -"
}

fail=0
printf '| lemma | expected | valid | formal status | proven/checked | rule examined | agrees |\n|---|---|---|---|---|---|---|\n'
for f in *.aisp; do
  exp=$(sed -n 's/^ *expect≜//p' "$f")
  read -r v s c p <<<"$(probe "$f")"
  # Same document without its ⟦Γ:Rules⟧{...} block.
  awk '/^⟦Γ/{skip=1} !skip{print} skip&&/^}/{skip=0}' "$f" >"$tmp/no-rule.aisp"
  read -r _ s0 c0 _ <<<"$(probe "$tmp/no-rule.aisp")"
  if [[ $s != none && ( $s != "$s0" || $c != "$c0" ) ]]; then e=yes; else e=no; fi
  if [[ $e == yes ]] && { [[ $exp == proved && $s == AllVerified ]] ||
       [[ $exp == refuted && $s == VerificationFailed && $v == false ]]; }; then
    a=yes
  else
    a=**no**; fail=1
  fi
  printf '| %s | %s | %s | %s | %s/%s | %s | %s |\n' "${f%.aisp}" "$exp" "$v" "$s" "$p" "$c" "$e" "$a"
done
exit "$fail"
