#!/usr/bin/env bash
# Tests for .githooks/pre-commit using throwaway git repos with a stubbed t0.sh.
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
hook_src="$root/.githooks/pre-commit"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

fails=0
ok() { printf 'ok    %s\n' "$1"; }
bad() {
  printf 'FAIL  %s\n' "$1"
  fails=$((fails + 1))
}

# new_repo NAME : a repo with the hook installed and a t0 stub that records its calls.
new_repo() {
  local d="$tmp/$1"
  mkdir -p "$d/.githooks" "$d/qa/tiers"
  git init -q "$d"
  git -C "$d" config user.email t@example.invalid
  git -C "$d" config user.name tester
  git -C "$d" config commit.gpgsign false
  cp "$hook_src" "$d/.githooks/pre-commit"
  # shellcheck disable=SC2016
  printf '#!/usr/bin/env bash\necho "$@" >> "$(git rev-parse --show-toplevel)/.t0-calls"\nexit "${STUB_T0_EXIT:-0}"\n' >"$d/qa/tiers/t0.sh"
  chmod +x "$d/.githooks/pre-commit" "$d/qa/tiers/t0.sh"
  git -C "$d" config core.hooksPath .githooks
  # Keep the stub's call log out of the index.
  printf '.t0-calls\n' >"$d/.gitignore"
  git -C "$d" add .gitignore
  DW_SKIP_T0=1 git -C "$d" commit -q -m init 2>/dev/null
  echo "$d"
}

# try_commit DIR : run the commit with the current environment, echo "rc|stderr".
try_commit() {
  local rc=0 err
  err="$(git -C "$1" commit -q -m test 2>&1 >/dev/null)" || rc=$?
  printf '%s|%s' "$rc" "$err"
}

# 1. holdout paths are rejected unless DW_ROLE=evaluator.
d="$(new_repo holdout)"
mkdir -p "$d/qa/holdout"
echo x >"$d/qa/holdout/s1.json"
git -C "$d" add qa/holdout/s1.json
res="$(try_commit "$d")"
if [[ "${res%%|*}" != 0 && "$res" == *"holdout is evaluator-owned"* ]]; then ok "holdout rejected for non-evaluator"; else bad "holdout rejected for non-evaluator: $res"; fi
res="$(DW_ROLE=evaluator try_commit "$d")"
if [[ "${res%%|*}" == 0 ]]; then ok "holdout accepted for evaluator"; else bad "holdout accepted for evaluator: $res"; fi

# 2. docs-only commits are rejected unless DW_DOCS_COMMIT=1; mixed commits pass.
d="$(new_repo docs)"
mkdir -p "$d/docs" "$d/wiki-llm" "$d/src"
echo a >"$d/docs/a.md"
echo b >"$d/wiki-llm/b.md"
git -C "$d" add docs wiki-llm
res="$(try_commit "$d")"
if [[ "${res%%|*}" != 0 && "$res" == *"docs-only commit in the inner loop"* ]]; then ok "docs-only rejected"; else bad "docs-only rejected: $res"; fi
res="$(DW_DOCS_COMMIT=1 try_commit "$d")"
if [[ "${res%%|*}" == 0 ]]; then ok "docs-only accepted with DW_DOCS_COMMIT=1"; else bad "docs-only accepted with DW_DOCS_COMMIT=1: $res"; fi
echo c >"$d/docs/c.md"
echo code >"$d/src/main.rs"
git -C "$d" add docs src
res="$(try_commit "$d")"
if [[ "${res%%|*}" == 0 ]]; then ok "mixed docs+code accepted"; else bad "mixed docs+code accepted: $res"; fi

# 3. T0 runs with --staged, its failure blocks, DW_SKIP_T0=1 skips it with a warning.
d="$(new_repo t0)"
echo code >"$d/a.rs"
git -C "$d" add a.rs
rm -f "$d/.t0-calls"
res="$(STUB_T0_EXIT=1 try_commit "$d")"
if [[ "${res%%|*}" != 0 && -f "$d/.t0-calls" && "$(cat "$d/.t0-calls")" == "--staged" ]]; then ok "failing t0 --staged blocks the commit"; else bad "failing t0 blocks the commit: $res"; fi
res="$(DW_SKIP_T0=1 STUB_T0_EXIT=1 try_commit "$d")"
if [[ "${res%%|*}" == 0 && "$res" == *"WARNING"* ]]; then ok "DW_SKIP_T0=1 skips t0 with a warning"; else bad "DW_SKIP_T0=1 skips t0: $res"; fi
echo more >>"$d/a.rs"
git -C "$d" add a.rs
rm -f "$d/.t0-calls"
res="$(try_commit "$d")"
if [[ "${res%%|*}" == 0 && -f "$d/.t0-calls" ]]; then ok "passing t0 lets the commit through"; else bad "passing t0 lets the commit through: $res"; fi

# 4. DW_SKIP_T0 must not bypass the holdout or docs-only rules.
d="$(new_repo skipt0)"
mkdir -p "$d/qa/holdout" "$d/docs"
echo x >"$d/qa/holdout/s.json"
git -C "$d" add qa/holdout/s.json
res="$(DW_SKIP_T0=1 try_commit "$d")"
if [[ "${res%%|*}" != 0 && "$res" == *"holdout is evaluator-owned"* ]]; then ok "DW_SKIP_T0 does not bypass holdout rule"; else bad "DW_SKIP_T0 bypasses holdout rule: $res"; fi
git -C "$d" reset -q
echo y >"$d/docs/y.md"
git -C "$d" add docs/y.md
res="$(DW_SKIP_T0=1 try_commit "$d")"
if [[ "${res%%|*}" != 0 && "$res" == *"docs-only"* ]]; then ok "DW_SKIP_T0 does not bypass docs-only rule"; else bad "DW_SKIP_T0 bypasses docs-only rule: $res"; fi

# 6. dw_step does not leak hook git variables into steps (a step running git in a
#    scratch repo must not write into the outer repository's index).
d="$tmp/scrub"
git init -q "$d"
mkdir -p "$d/qa/tiers"
cp "$root/qa/tiers/lib.sh" "$d/qa/tiers/"
git -C "$d" -c user.email=t@t -c user.name=t commit -q --allow-empty -m base
leak="$(cd "$d" && GIT_INDEX_FILE="$d/.git/index" GIT_DIR="$d/.git" bash -c '
  source qa/tiers/lib.sh
  dw_step probe bash -c "env | grep -cE \"^GIT_(DIR|INDEX_FILE|WORK_TREE|OBJECT_DIRECTORY|COMMON_DIR|PREFIX)=\" || true" >/dev/null
  cat "$DW_TMP"/step-0.log')"
if [[ "$leak" == "0" ]]; then ok "dw_step scrubs GIT_* hook variables"; else bad "dw_step leaks GIT_* variables: $leak"; fi

# 5. install-hooks.sh is idempotent.
d="$tmp/install"
mkdir -p "$d/scripts/qa" "$d/.githooks"
git init -q "$d"
cp "$root/scripts/qa/install-hooks.sh" "$d/scripts/qa/"
echo '#!/bin/sh' >"$d/.githooks/pre-commit"
(cd "$d" && scripts/qa/install-hooks.sh >/dev/null && scripts/qa/install-hooks.sh >/dev/null)
if [[ "$(git -C "$d" config --get core.hooksPath)" == ".githooks" ]]; then ok "install-hooks idempotent"; else bad "install-hooks idempotent"; fi

if [[ $fails -gt 0 ]]; then
  echo "$fails test(s) failed"
  exit 1
fi
echo "all tier tests passed"
