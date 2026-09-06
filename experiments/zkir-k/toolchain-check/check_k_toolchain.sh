#!/usr/bin/env bash
# Milestone 1 toolchain check for the ZKIR-in-K plan (wiki/zkir-k-semantics-plan.md).
# Compiles K tutorial lesson 1.2 with the LLVM backend, runs both programs, and
# type-checks the same definition with the Haskell backend. Exits non-zero on any
# mismatch. Writes a receipt to stdout; redirect it into evidence/.
set -euo pipefail
cd "$(dirname "$0")"
EXPECTED_VERSION="${EXPECTED_K_VERSION:-7.1.337}"
rm -rf lesson-02-a-kompiled lesson-02-a-haskell-kompiled

echo "date: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
echo "host: $(uname -srm)"
echo "kompile: $(command -v kompile)"
KV="$(kompile --version)"
echo "kompile --version: $(tr '\n' ' ' <<<"$KV")"
grep -q "$EXPECTED_VERSION" <<<"$KV" || { echo "FAIL: expected K $EXPECTED_VERSION, got: $KV"; exit 1; }

echo "--- kompile (llvm)"
kompile lesson-02-a.k --backend llvm --emit-json 2>&1 | grep -v "^\[Warning\] Compiler: Could not find main syntax module" || true
[ -d lesson-02-a-kompiled ] || { echo "FAIL: no kompiled dir"; exit 1; }

echo "--- krun banana.color"
OUT_B=$(krun banana.color --definition lesson-02-a-kompiled)
echo "$OUT_B"
echo "$OUT_B" | grep -q "Yellow ( )" || { echo "FAIL: expected Yellow"; exit 1; }

echo "--- krun blueberry.color"
OUT_BB=$(krun blueberry.color --definition lesson-02-a-kompiled)
echo "$OUT_BB"
echo "$OUT_BB" | grep -q "Blue ( )" || { echo "FAIL: expected Blue"; exit 1; }

echo "--- kompile (haskell)"
kompile lesson-02-a.k --backend haskell --output-definition lesson-02-a-haskell-kompiled 2>&1 | grep -v "^\[Warning\] Compiler: Could not find main syntax module" || true
[ -f lesson-02-a-haskell-kompiled/definition.kore ] || { echo "FAIL: no haskell definition.kore"; exit 1; }
echo "--- krun banana.color (haskell)"
OUT_H=$(krun banana.color --definition lesson-02-a-haskell-kompiled)
echo "$OUT_H"
echo "$OUT_H" | grep -q "Yellow ( )" || { echo "FAIL: haskell backend expected Yellow"; exit 1; }

echo "--- pyk (uv dependency group zkir-k of the Moriarty project)"
REPO_ROOT="$(git rev-parse --show-toplevel)"
PYK_VERSION=$(cd "$REPO_ROOT" && uv run --group zkir-k python -c 'import pyk; print(pyk.__version__)') || { echo "FAIL: pyk import"; exit 1; }
echo "pyk: $PYK_VERSION"
[ "$PYK_VERSION" = "$EXPECTED_VERSION" ] || { echo "FAIL: pyk $PYK_VERSION != K $EXPECTED_VERSION"; exit 1; }
echo "--- pyk kast round trip"
(cd "$REPO_ROOT" && uv run --group zkir-k python "$OLDPWD/pyk_roundtrip.py" "$OLDPWD/lesson-02-a-kompiled") || { echo "FAIL: pyk round trip"; exit 1; }

echo "RESULT: PASS"
