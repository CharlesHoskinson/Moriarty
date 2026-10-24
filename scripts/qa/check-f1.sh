#!/usr/bin/env bash
# F1: governance charter in place: factory title, and no review-routing headings in AGENTS.md.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
head -1 AGENTS.md | grep -q '^# Moriarty: factory charter' || { echo "AGENTS.md is not the factory charter"; exit 1; }
if grep -nEi '^#{1,6} .*(routing|review|audit|Opus|Muse|Argon|Gemini|adoption|vote)' AGENTS.md; then
  echo "AGENTS.md still has review-routing/governance headings"; exit 1
fi
