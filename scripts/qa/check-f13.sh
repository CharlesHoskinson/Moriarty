#!/usr/bin/env bash
# F13: live bus smoke. Skips (exit 0) when there is no installed bus, broker or ops/doctor identity.
# Mints two throwaway identities in run dw-f13, publishes a status.flip as one and reads it via
# `bus inbox` as the other, then revokes both. Never touches the repo's own identity or state.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
skip() { echo "SKIP F13: $1"; exit 0; }
command -v agentbus >/dev/null || skip "agentbus not installed"
timeout 20 agentbus --run ops --as doctor doctor >/dev/null 2>&1 || skip "no healthy broker or ops/doctor identity"
tmp="$(mktemp -d)"
a="f13-a-$$"; b="f13-b-$$"
cleanup() {
  agentbus --run dw-f13 --as "$a" id revoke >/dev/null 2>&1 || true
  agentbus --run dw-f13 --as "$b" id revoke >/dev/null 2>&1 || true
  rm -rf "$tmp"
}
trap cleanup EXIT
timeout 30 agentbus id new --run dw-f13 --as "$a" --role implementer --chan '*' --claims slot: --identity-dir "$tmp/a" --ttl 3h --no-register >/dev/null
timeout 30 agentbus id new --run dw-f13 --as "$b" --role lead --chan '*' --identity-dir "$tmp/b" --ttl 3h --no-register >/dev/null
export DW_BUS_RUN=dw-f13 DW_BUS_STATE="$tmp/state" DW_BUS_TIMEOUT=5
AGENTBUS_IDENTITY_DIR="$tmp/a" scripts/doctorwatson bus doctor
AGENTBUS_IDENTITY_DIR="$tmp/a" scripts/doctorwatson bus publish --type status.flip --data '{"id":"F13"}'
out="$(AGENTBUS_IDENTITY_DIR="$tmp/b" scripts/doctorwatson bus inbox)"
echo "$out"
grep -q 'type=status.flip' <<<"$out" || { echo "status.flip did not arrive"; exit 1; }
echo "F13 live smoke ok"
