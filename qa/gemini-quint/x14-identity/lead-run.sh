#!/usr/bin/env bash
# Lead Quint runs of the x14-identity lane model (T13, 2026-11-13). quint 0.32.0, each command through heavy-slot.
# The lane model is unchanged. Seeds are fixed; the lane gave no --max-steps, the lead uses 5.
set -u
cd "$(dirname "$0")"
QB=$HOME/.local/share/mise/installs/node/26.13.0/bin
out=lead-runs; mkdir -p "$out"; : > "$out/summary.txt"; n=0
go() {
  n=$((n + 1)); local log="$out/$(printf '%02d' $n).log"
  "$HOME/.local/bin/heavy-slot" --mem 4G --name "t13-x14id-$n" -- env PATH="$QB:$PATH" timeout 1800 quint "$@" >"$log" 2>&1
  local rc=$? tag=other
  grep -q '\[violation\]' "$log" && tag='[violation]'; grep -q '\[ok\]' "$log" && tag='[ok]'
  grep -q 'passing' "$log" && tag="$(grep -Eo '[0-9]+ passing' "$log" | tail -1)"
  grep -Eq '[0-9]+ failed' "$log" && tag="$tag; $(grep -Eo '[0-9]+ failed' "$log" | tail -1)"
  [ $rc = 124 ] && tag=timeout
  echo "exit=$rc $tag :: quint $*" | tee -a "$out/summary.txt"
}
M=x14_identity.qnt; T=x14_identity_test.qnt
go typecheck "$M"; go typecheck "$T"
go test --main x14_identity_test --match 'Test$' --seed 0x14b000 "$T"
R=(--max-samples 1000 --max-steps 5)
go run "$M" --main x14_identity --invariant invRow "${R[@]}" --seed 0x14b001
go run "$M" --main x14_identity --invariant invNonVacuous "${R[@]}" --seed 0x14b002
s=3
for m in m_no_zero_limbs m_e_ignores_flag m_sel_wrong m_out_flag_wrong m_out_mult_v; do
  go run "$M" --main "$m" --invariant invRow "${R[@]}" --seed "0x14b00$s"; s=$((s + 1))
done
