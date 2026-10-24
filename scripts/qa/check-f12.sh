#!/usr/bin/env bash
# F12: spawn degrades with the bus off/missing/failing/hung, strips API keys, enforces the
# per-vendor flock governor and never feeds bus text to a prompt.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)/models/doctorwatson"
exec python3 -m unittest factory.tests.test_spawn
