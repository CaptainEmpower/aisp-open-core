# Shared by run.sh and ground-truth.sh: loads lemmas/MANIFEST.tsv and checks
# that the corpus is well formed before any verdict is read. Source it from
# inside lemmas/.
#
# The manifest, not the lemma document, holds each lemma's expected verdict and
# its partner, so no verifier can read the answer it is being tested on. Each
# pair must be byte-identical outside its ⟦Γ⟧ block (no expect≜, the same header
# and name), so anything other than the rule that a verifier keys on gives both
# partners the same input and therefore the same verdict.
declare -A EXPECT PARTNER
corpus_load() {
  local id exp pt n=0
  while IFS=$'\t' read -r id exp pt; do
    [[ -z $id || $id == \#* ]] && continue
    EXPECT[$id]=$exp; PARTNER[$id]=$pt; n=$((n+1))
  done < MANIFEST.tsv
  local bad=0 f
  for f in *.aisp; do
    [[ -n ${EXPECT[${f%.aisp}]+x} ]] || { echo "corpus: ${f%.aisp} is not in MANIFEST.tsv" >&2; bad=1; }
  done
  for id in "${!EXPECT[@]}"; do
    pt=${PARTNER[$id]}
    [[ -f $id.aisp ]] || { echo "corpus: $id is in MANIFEST.tsv but $id.aisp is missing" >&2; bad=1; continue; }
    [[ -f $pt.aisp && ${PARTNER[$pt]-} == "$id" ]] || { echo "corpus: $id's partner $pt is missing or not mutual" >&2; bad=1; continue; }
    [[ ${EXPECT[$pt]-} != "${EXPECT[$id]}" ]] || { echo "corpus: $id and $pt expect the same verdict" >&2; bad=1; }
    grep -q 'expect≜' "$id.aisp" && { echo "corpus: $id.aisp carries its expected verdict" >&2; bad=1; }
    if ! diff -q <(strip_rules "$id.aisp") <(strip_rules "$pt.aisp") >/dev/null; then
      echo "corpus: $id and $pt differ outside ⟦Γ⟧" >&2; bad=1
    fi
  done
  (( bad == 0 )) || { echo "corpus: malformed, no verdicts read" >&2; exit 2; }
}
strip_rules() { awk '/^⟦Γ/{s=1} !s{print} s&&/^}/{s=0}' "$1"; }
