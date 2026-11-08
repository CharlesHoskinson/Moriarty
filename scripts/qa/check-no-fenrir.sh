#!/usr/bin/env bash
# F17 (SWF-014, SSRD-030): fail, naming file and line, when a tracked file under
# crates/, tools/, scripts/, models/ or qa/ names a Fenrir checkout or the private
# rollup research tree. qa/holdout/ (evaluator-owned) and the append-only run
# history qa/sherlock/runs.jsonl are excluded; qa/sherlock/predicates.yaml is checked.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
# The patterns are split so that this script does not match itself.
pattern='project''fenrir|rollup''-subarch'
if hits="$(git grep -n -E "$pattern" -- crates tools scripts models qa \
  ':(exclude)qa/holdout/**' ':(exclude)qa/sherlock/runs.jsonl')"; then
  printf '%s\n' "$hits"
  echo "F17 FAIL: $(printf '%s\n' "$hits" | wc -l) tracked line(s) name a Fenrir or research-tree path" >&2
  exit 1
fi
echo "F17 PASS: no tracked Sherlock file names a Fenrir or research-tree path"
