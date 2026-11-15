#!/usr/bin/env bash
# Sync formal/sherlock-e2e to odin and run `lake build --wfail` for the given contexts.
# Usage: odin-build.sh <context>...   Full log: odin ~/sherlock/formal-e2e/<context>/build.log
set -uo pipefail
here="$(cd "$(dirname "$0")/.." && pwd)"
rsync -a --exclude .lake --exclude traces "$here/" odin:sherlock/formal-e2e/
for c in "$@"; do
  echo "== $c"
  ssh odin "cd ~/sherlock/formal-e2e/$c/lean && systemd-run --user --scope -q -p MemoryMax=24G nice -n 10 ~/.elan/bin/lake build --wfail > ../build.log 2>&1; echo rc=\$?; grep -nE '^(error|warning)' ../build.log | head -${MAXERR:-15}"
done
