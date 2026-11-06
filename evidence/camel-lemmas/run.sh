#!/usr/bin/env bash
# Runs every lemma at the formal level and prints: id, expected, valid, status, proven/checked.
# Exits 1 if any lemma's verdict disagrees with its expected one (the acceptance test).
set -euo pipefail
cd "$(dirname "$0")/lemmas"
B="${AISP_CLI:-aisp-cli}"
command -v "$B" >/dev/null || { echo "run.sh: aisp-cli not found (set AISP_CLI)" >&2; exit 2; }
fail=0
printf '| lemma | expected | valid | formal status | proven/checked | agrees |\n|---|---|---|---|---|---|\n'
for f in *.aisp; do
  exp=$(sed -n 's/^ *expect≜//p' "$f")
  # aisp-cli exits 1 when a document is invalid, which is the outcome a refuted
  # lemma should produce, so its status must not abort the loop; the verdict is
  # read from the JSON instead.
  out=$("$B" -l formal --formal-verification -f json "$f" 2>/dev/null) || true
  j=$(sed -n '/^{/,/^}/p' <<<"$out")
  v=$(jq -r '.valid // "none"' <<<"$j" 2>/dev/null || echo none)
  s=$(jq -r '.formal_verification.status // "none"' <<<"$j" 2>/dev/null || echo none)
  p=$(jq -r '"\(.formal_verification.properties_proven // "-")/\(.formal_verification.properties_checked // "-")"' <<<"$j" 2>/dev/null || echo -/-)
  if [[ $exp == proved && $s == AllVerified ]] || [[ $exp == refuted && $s != AllVerified && $v == false ]]; then a=yes; else a=**no**; fail=1; fi
  printf '| %s | %s | %s | %s | %s | %s |\n' "${f%.aisp}" "$exp" "$v" "$s" "$p" "$a"
done
exit "$fail"
