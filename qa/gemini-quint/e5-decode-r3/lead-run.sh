#!/bin/bash
# Lead runs of the r3 lane model (quint 0.32.0; each command in heavy.slice via heavy-run, no lock).
# The default rust backend stops with "recursion limit exceeded" on this model (lead-runs-rust.log),
# so every test and run uses --backend typescript.
cd "$(dirname "$0")"
QB=$HOME/.local/share/mise/installs/node/26.13.0/bin
q() { "$HOME/.local/bin/heavy-run" --mem 4G --name "quint-e5r3" -- env PATH="$QB:$PATH" timeout 3600 quint "$@"; }
{
echo "## quint test seed 1"; q test --backend typescript --main decode_test --match '_test$' --seed 1 decode_test.qnt 2>&1 | tail -80
for s in 1 2; do echo "## run SOUND_AND_COMPLETE seed $s"; q run --backend typescript --invariant SOUND_AND_COMPLETE --max-samples 5000 --max-steps 10 --seed $s decode.qnt 2>&1 | tail -6; done
for m in free_mult no_m_range no_inactive_zero no_has1 no_chain first_cap_wrong ungated_y1; do echo "## run SOUND_mut_$m seed 1"; q run --backend typescript --invariant SOUND_mut_$m --max-samples 5000 --max-steps 10 --seed 1 decode.qnt 2>&1 | tail -4; done
} > lead-runs.log 2>&1
