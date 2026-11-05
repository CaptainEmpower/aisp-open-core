#!/usr/bin/env bash
# Runs every lemma at the formal level and prints: id, expected, valid, status, proven/checked.
set -euo pipefail
cd "$(dirname "$0")/lemmas"
B="${AISP_CLI:-aisp-cli}"
printf '| lemma | expected | valid | formal status | proven/checked | agrees |\n|---|---|---|---|---|---|\n'
for f in *.aisp; do
  exp=$(sed -n 's/^ *expect≜//p' "$f")
  j=$("$B" -l formal --formal-verification -f json "$f" 2>/dev/null | sed -n '/^{/,/^}/p')
  v=$(jq -r '.valid' <<<"$j"); s=$(jq -r '.formal_verification.status // "none"' <<<"$j")
  p=$(jq -r '"\(.formal_verification.properties_proven // "-")/\(.formal_verification.properties_checked // "-")"' <<<"$j")
  if [[ $exp == proved && $s == AllVerified ]] || [[ $exp == refuted && $s != AllVerified && $v == false ]]; then a=yes; else a=**no**; fi
  printf '| %s | %s | %s | %s | %s | %s |\n' "${f%.aisp}" "$exp" "$v" "$s" "$p" "$a"
done
