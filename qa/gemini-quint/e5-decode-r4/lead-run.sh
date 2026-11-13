#!/bin/bash
# Lead runs of the round 4 lane model (T13, 2026-11-13; quint 0.32.0; each command through heavy-slot).
# The default rust backend stops with "recursion limit exceeded" on this model (round 3 lead-runs-rust.log);
# so every run uses --backend typescript. Seeds are fixed.
# Sample count: the typescript backend takes about 30 s per sample of 10 steps on this model (../e5-decode-r3-lead/probe-10-samples.log:
# 10 samples in under 552 s including the slot wait), so 5000 samples would take about 40 h; the lead uses 20.
set -u
cd "$(dirname "$0")"
QB=$HOME/.local/share/mise/installs/node/26.13.0/bin
out=lead-runs; mkdir -p "$out"; : > "$out/summary.txt"; n=0
go() {
  n=$((n + 1)); local log="$out/$(printf '%02d' $n).log"
  "$HOME/.local/bin/heavy-slot" --mem 4G --name "t13-e5r4l-$n" -- env PATH="$QB:$PATH" timeout 3600 quint "$@" >"$log" 2>&1
  local rc=$? tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'; grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'passing' "$log" && tag="$(grep -Eo '[0-9]+ passing' "$log" | tail -1)"
  grep -Eq '[0-9]+ failed' "$log" && tag="$tag; $(grep -Eo '[0-9]+ failed' "$log" | tail -1)"
  grep -q 'recursion limit' "$log" && tag="$tag; recursion limit exceeded"
  [ $rc = 124 ] && tag=timeout
  echo "exit=$rc $tag :: quint $*" | tee -a "$out/summary.txt"
}
go typecheck decode.qnt; go typecheck decode_test.qnt; go typecheck lead_control_test.qnt
TS=(--backend typescript)
go test "${TS[@]}" --main decode_test --match '_test$' --seed 1 decode_test.qnt
go test "${TS[@]}" --main control_test --match '_test$' --seed 1 lead_control_test.qnt
for s in 1 2; do go run "${TS[@]}" --invariant SOUND_AND_COMPLETE --max-samples 20 --max-steps 10 --seed $s decode.qnt; done
for m in free_mult no_m_range no_inactive_zero no_has1 no_chain first_cap_wrong ungated_y1; do
  go run "${TS[@]}" --invariant SOUND_mut_$m --max-samples 20 --max-steps 10 --seed 1 decode.qnt
done
