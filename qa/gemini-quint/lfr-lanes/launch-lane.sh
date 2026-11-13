#!/usr/bin/env bash
# Launch one L_Fr Gemini Quint lane (argon-run, gemini-4-argon-high) as a durable user unit,
# the lane itself in heavy.slice through heavy-run. Usage: launch-lane.sh GROUP (bbalu | poseidon | tape)
# The lane works in ~/.cache/gemini-lanes/lfr-GROUP: a plain copy (git archive) of
# crates/sherlock-lfr (openvm branch e2-lfr), crates/stark-backend/src/interaction (backend e2-lfr),
# the E2 RESULTS.md and copies of the quint skills in .lane-skills.
set -eu
g="${1:?group}"
L=$HOME/.cache/gemini-lanes/lfr-$g
QB=$HOME/.local/share/mise/installs/node/26.13.0/bin:$HOME/.local/share/mise/installs/antigravity-cli/latest
exec systemd-run --user --unit="gq-lfr-$g" --collect -q \
  --working-directory="$L" --setenv=PATH="$QB:$HOME/.local/bin:/usr/local/bin:/usr/bin:/bin" --setenv=HOME="$HOME" \
  -- "$HOME/.local/bin/heavy-run" --mem 6G --name "lane-lfr-$g" -- "$HOME/.local/bin/argon-run" \
  --prompt-file "$L/qa/gemini-quint/lfr-$g/TASK.md" \
  --log "$HOME/.cache/gemini-lanes/lfr-$g.log" \
  --done "$L/qa/gemini-quint/lfr-$g/REPORT.md:600" \
  --model gemini-4-argon-high \
  --max-turns 40
