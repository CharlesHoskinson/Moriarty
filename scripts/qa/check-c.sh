#!/usr/bin/env bash
# C1-C6: pure composite. Passing is gated entirely by the depends list of U predicates, which the runner requires to pass first.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
echo "composite: a C item has no check of its own; it passes only when every depended U predicate has passed (enforced by depends)"
exit 0
