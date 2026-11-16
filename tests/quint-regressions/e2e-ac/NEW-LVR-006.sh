#!/usr/bin/env bash
# NEW-LVR-006 regression (text against code). LVR-006: "When a ledger call is built, the settle runner SHALL run
# the native ledger 9.1.0.1 validator on the call before the submit." Correct behaviour: with no validator
# configured the runner refuses to submit A (no skip path), and the t23.py Preview plan passes the validator.
# Pinned at check time: t23-post-t25@cf7a86473 tools/sherlock-sdk/t10a-ledger-run.mjs:420-421,799
# (`if (!validateCallBin) return {skipped: ...}`; gate `if (!nat.skipped && nat.accepted !== true)`), and
# tools/t23-suite/t23.py:385 (plan env without E6_VALIDATE_CALL / T10A_VALIDATE_CALL). Same in t17-preview@9e1e7f398:359,711.
set -u
W=/home/hoskinson/Projects/Moriarty-wt/t23-post-t25
R=$W/tools/sherlock-sdk/t10a-ledger-run.mjs
D=$W/tools/t23-suite/t23.py
rc=0
if grep -q 'if (!validateCallBin) return {skipped' "$R" && grep -q 'if (!nat.skipped && nat.accepted !== true)' "$R"; then
  echo "FAIL runner: an unset validator skips the native check and A is still submitted"; rc=1
else echo "PASS runner"; fi
plan=$(sed -n '/^def settle_plan/,/^def /p' "$D")
if grep -qE 'VALIDATE_CALL' <<<"$plan"; then echo "PASS t23.py plan env names the validator"
else echo "FAIL t23.py: settle_plan env has no E6_VALIDATE_CALL/T10A_VALIDATE_CALL"; rc=1; fi
exit $rc
