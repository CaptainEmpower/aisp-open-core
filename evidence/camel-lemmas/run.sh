#!/usr/bin/env bash
# Acceptance test for the CLI path. For each lemma it prints the verdict and
# whether it agrees with the lemma's `expect≜`; exits 1 on any disagreement.
#
# Verdict, from the CLI's formal-verification counts (the status string is a
# confidence band, aisp-cli/src/main.rs:409-415, so it is shown but not used):
#   proved  = a formal result with properties_checked > 0 and proven == checked
#   refuted = a formal result with proven < checked
#   none    = anything else: no formal result, a parse failure, no output.
#
# A verdict counts only if the lemma's rule decided it. The CLI does not report
# a per-rule result, so each lemma is paired with the one that shares its
# skeleton and differs only in the rule (T_n ↔ F_n, C2 ↔ C1, C4 ↔ C3). A row
# agrees only if its verdict matches its expectation AND differs from its
# partner's. Output that does not depend on the rule's content (a heuristic
# score, a hard-coded Proven, ⟦Σ⟧-derived properties) gives both members the
# same verdict and so fails both rows.
#
# The driver (driver/src/main.rs) applies the same criterion to FormalVerifier.
set -euo pipefail
cd "$(dirname "$0")/lemmas"
B="${AISP_CLI:-aisp-cli}"
command -v "$B" >/dev/null || { echo "run.sh: aisp-cli not found (set AISP_CLI)" >&2; exit 2; }

# Prints "valid status checked proven verdict" for one document. aisp-cli exits
# 1 for an invalid document, which a refutation may produce, so its exit status
# must not abort the run: the verdict is read from the JSON instead.
probe() {
  local out j r
  out=$("$B" -l formal --formal-verification -f json "$1" 2>/dev/null) || true
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

partner() {
  case $1 in
    T*) echo "F${1#T}" ;; F*) echo "T${1#F}" ;;
    C1-*|C2-*|C3-*|C4-*)
      local n=${1:1:1} p
      case $n in 1) p=2 ;; 2) p=1 ;; 3) p=4 ;; 4) p=3 ;; esac
      ls C"$p"-*.aisp 2>/dev/null | head -1 | sed 's/\.aisp$//' ;;
  esac
}

declare -A V
for f in *.aisp; do V[${f%.aisp}]=$(probe "$f"); done

fail=0
printf '| lemma | expected | valid | formal status | proven/checked | verdict | partner verdict | agrees |\n|---|---|---|---|---|---|---|---|\n'
for f in *.aisp; do
  id=${f%.aisp}
  exp=$(sed -n 's/^ *expect≜//p' "$f")
  read -r v s c p verdict <<<"${V[$id]}"
  pid=$(partner "$id")
  if [[ -n $pid && -n ${V[$pid]+x} ]]; then pv=$(awk '{print $5}' <<<"${V[$pid]}"); else pv=missing; fi
  if [[ $verdict == "$exp" && $pv != missing && $pv != "$verdict" ]]; then a=yes; else a=**no**; fail=1; fi
  printf '| %s | %s | %s | %s | %s/%s | %s | %s | %s |\n' "$id" "$exp" "$v" "$s" "$p" "$c" "$verdict" "$pv" "$a"
done
exit "$fail"
