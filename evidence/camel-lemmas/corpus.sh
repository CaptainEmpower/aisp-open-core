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
  local bad=0 f id exp pt line
  if grep -q $'\r' MANIFEST.tsv; then echo "corpus: MANIFEST.tsv has CR line endings" >&2; bad=1; fi
  while IFS= read -r line; do
    [[ -z $line || $line == \#* ]] && continue
    # Exactly three tab-separated fields; IFS=$'\t' read would silently merge
    # consecutive tabs, so count the fields first.
    if [[ $(awk -F'\t' '{print NF}' <<<"$line") != 3 ]]; then
      echo "corpus: bad manifest line: $line" >&2; bad=1; continue
    fi
    IFS=$'\t' read -r id exp pt <<<"$line"
    [[ $exp == proved || $exp == refuted ]] || { echo "corpus: $id: bad verdict $exp" >&2; bad=1; }
    EXPECT[$id]=$exp; PARTNER[$id]=$pt
  done < MANIFEST.tsv
  for f in *.aisp; do
    [[ -n ${EXPECT[${f%.aisp}]+x} ]] || { echo "corpus: ${f%.aisp} is not in MANIFEST.tsv" >&2; bad=1; }
  done
  for id in "${!EXPECT[@]}"; do
    pt=${PARTNER[$id]}
    [[ -f $id.aisp ]] || { echo "corpus: $id is in MANIFEST.tsv but $id.aisp is missing" >&2; bad=1; continue; }
    [[ -f $pt.aisp && ${PARTNER[$pt]-} == "$id" ]] || { echo "corpus: $id's partner $pt is missing or not mutual" >&2; bad=1; continue; }
    [[ ${EXPECT[$pt]-} != "${EXPECT[$id]}" ]] || { echo "corpus: $id and $pt expect the same verdict" >&2; bad=1; }
    if grep -q 'expect≜' "$id.aisp"; then echo "corpus: $id.aisp carries its expected verdict" >&2; bad=1; fi
    if grep -q $'\r' "$id.aisp"; then echo "corpus: $id.aisp has CR line endings" >&2; bad=1; fi
    if ! rules_well_formed "$id.aisp"; then
      echo "corpus: $id.aisp must hold exactly one ⟦Γ:Rules⟧{ line, closed by a line that is exactly }, with no ⟦ header inside" >&2; bad=1
    fi
    if ! diff -q <(strip_rules "$id.aisp") <(strip_rules "$pt.aisp") >/dev/null; then
      echo "corpus: $id and $pt differ outside ⟦Γ⟧" >&2; bad=1
    fi
  done
  (( bad == 0 )) || { echo "corpus: malformed, no verdicts read" >&2; exit 2; }
}
# The ⟦Γ⟧ block's form is fixed so that it can be stripped exactly: one line
# that is exactly `⟦Γ:Rules⟧{`, rule lines, and one line that is exactly `}`.
# Anything else (a one-line block, an indented brace) is refused rather than
# guessed at. driver/src/main.rs implements the same two functions.
rules_well_formed() {
  awk 'BEGIN{n=0; open=0; ok=1}
       $0=="⟦Γ:Rules⟧{" { if (open) ok=0; n++; open=1; next }
       /^⟦Γ/            { ok=0 }
       open && /^⟦/     { ok=0 }
       open && $0=="}"  { open=0; next }
       END { exit !(ok && n==1 && !open) }' "$1"
}
strip_rules() { awk '$0=="⟦Γ:Rules⟧{"{s=1} !s{print} s&&$0=="}"{s=0}' "$1"; }
