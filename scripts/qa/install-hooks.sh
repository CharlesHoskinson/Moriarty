#!/usr/bin/env bash
# Enable the repo's git hooks (idempotent): git config core.hooksPath .githooks
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

chmod +x .githooks/* qa/tiers/*.sh 2>/dev/null || true
current="$(git config --get core.hooksPath || true)"
if [[ "$current" != ".githooks" ]]; then
  git config core.hooksPath .githooks
fi
echo "hooks installed: core.hooksPath=$(git config --get core.hooksPath)"
