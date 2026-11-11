#!/usr/bin/env bash
# Launch one chipleaf-final Gemini Quint lane (argon-run, gemini-4-argon-high) in heavy.slice.
# Usage: launch-lane.sh LANE   (LANE = packsel | range | m3)
# The lane works in ~/.cache/gemini-lanes/chipleaf-final-LANE, a plain copy of the sources
# (crates/mori-chip/src, the research notes, design.md, the earlier lanes, .lane-skills).
set -eu
n="${1:?lane}"
D="$(cd "$(dirname "$0")" && pwd)"
L=$HOME/.cache/gemini-lanes/chipleaf-final-$n
cp "$D/TASK-$n.md" "$L/qa/gemini-quint/chipleaf-final/TASK-$n.md"
cd "$L"
exec heavy-run --mem 8G --name "lane-final-$n" -- argon-run \
  --prompt-file "$L/qa/gemini-quint/chipleaf-final/TASK-$n.md" \
  --log "$HOME/.cache/gemini-lanes/chipleaf-final-$n.log" \
  --done "$L/qa/gemini-quint/chipleaf-final/REPORT-$n.md:400" \
  --model gemini-4-argon-high \
  --max-turns 40
