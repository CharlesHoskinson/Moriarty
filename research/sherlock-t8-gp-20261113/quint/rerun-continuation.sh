#!/usr/bin/env bash
# N-4 continuation.qnt: lead-style run set (typecheck, every test module, the sound invariant,
# reach witnesses, every mutant sampled). Each quint command runs under heavy-run (4G, 900 s).
# Usage: rerun-continuation.sh OUTDIR [SAMPLES] [STEPS]
set -u
M=$HOME/Projects/Moriarty-wt/t8-gp/models/sherlock/quint
out="${1:?outdir}"; S="${2:-20000}"; K="${3:-14}"
mkdir -p "$out"; : > "$out/summary.txt"
QB=$HOME/.local/share/mise/installs/node/26.13.0/bin
SEED=0x20261113; n=0
go() {
  n=$((n + 1))
  local log="$out/$(printf '%02d' $n).log"
  ( cd "$M" && "$HOME/.local/bin/heavy-run" --mem 4G --name "quint-n4-$n" -- env PATH="$QB:$PATH" timeout 900 quint "$@" ) >"$log" 2>&1
  local rc=$? tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'
  grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'passing' "$log" && tag="$(grep -Eo '[0-9]+ passing' "$log" | tail -1)"
  grep -Eq '[0-9]+ failed' "$log" && tag="$tag; $(grep -Eo '[0-9]+ failed' "$log" | tail -1)"
  echo "exit=$rc $tag :: quint $*" | tee -a "$out/summary.txt"
}
go --version
go typecheck continuation.qnt; go typecheck continuation_test.qnt
for m in continuation_test t_noStitch t_noInterval t_noGenesis t_noTerminal t_recordTagRun t_consumeAnyRun t_anyRunUntagged t_pauseTerminal; do
  go test --match '^(pos_|cont_|record_|tag_)' --main "$m" continuation_test.qnt
done
RUN=(--max-samples "$S" --max-steps "$K" --seed "$SEED")
go run continuation.qnt --main cont_sound --invariant soundAll "${RUN[@]}"
for inv in ResumeProducesIdenticalClaimD OutOfOrderWorkerCompletionPreservesSegInterval CheckpointRejectedBySealAndGP2 MaxStackDepthLeLogLPlus2 AdminStepsKeptApart RecordNeverParsesAsClaim NoTerminalCheckpoint; do
  go run continuation.qnt --main cont_sound --invariant "$inv" "${RUN[@]}"
done
go run continuation.qnt --main cont_recordTagRun --invariant circuitSide "${RUN[@]}"
for w in reachSealed reachSealedAfterResume reachConsumed; do go run continuation.qnt --main cont_sound --invariant "$w" "${RUN[@]}"; done
for mu in noStitch noInterval noGenesis noTerminal recordTagRun anyRunUntagged pauseTerminal consumeAnyRun; do
  go run continuation.qnt --main "cont_$mu" --invariant soundAll "${RUN[@]}"
done
echo DONE >> "$out/summary.txt"
