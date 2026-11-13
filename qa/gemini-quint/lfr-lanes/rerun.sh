#!/usr/bin/env bash
# Lead quint runs of one L_Fr lane model (each command: heavy-run, no lock, timeout 900, via x12/q.sh).
# Usage: lfr-lanes/rerun.sh GROUP SOUND_MODULE "INVS..." "MUTANT_MODULES..." [SAMPLES] [STEPS] [BACKEND]
set -u
g="$1"; sound="$2"; invs="$3"; muts="$4"; S="${5:-4000}"; K="${6:-4}"; BK="${7:-rust}"
cd "$(dirname "$0")/../lfr-$g"
out=lead-runs/run; mkdir -p "$out"; : > "$out/summary.txt"; n=0
RUN=(--max-samples "$S" --max-steps "$K" --seed 0x20261112 --backend "$BK")
go() {
  n=$((n + 1)); local log="$out/$(printf '%02d' $n).log"
  ../x12/q.sh "lfr-$g-$n" "$@" >"$log" 2>&1; local rc=$? tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'; grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'QNT600\|outside i64' "$log" && tag="$tag (i64 limit)"
  echo "exit=$rc $tag :: quint $*" | tee -a "$out/summary.txt"
}
for inv in $invs; do go run "lfr_$g.qnt" --main "$sound" --invariant "$inv" "${RUN[@]}"; done
for m in $muts; do go run "lfr_$g.qnt" --main "$m" --invariant invRefine "${RUN[@]}"; done
