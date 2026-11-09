#!/usr/bin/env bash
# Blind vendor, model, CLI and author names in the bundle copies. Identity only; substance unchanged.
set -euo pipefail
B=/home/hoskinson/.cache/council-2.14/bundle
find "$B" -type f | while read -r f; do
  perl -i -pe '
    s/gemini-4-argon[-a-z]*/model-A/gi;
    s/gpt-6\.1-sol/model-B/gi;
    s/claude-opus-5-5/model-C/gi;
    s/gemini-quint/indep-quint/gi;
    s/gemini-lanes/review-lanes/gi;
    s/argon-run/lane-runner/gi;
    s/\bastra-?([0-9])/member-a$1/gi;
    s/\bfable-([0-9])/member-f$1/gi;
    s/\bclaude-s1\b/vendorY-s1/gi;
    s/\bcodex-s1\b/vendorX-s1/gi;
    s/lane-claude/lane-vendorY/gi;
    s/lane-codex/lane-vendorX/gi;
    s/claude-1000/user-1000/gi;
    s{/tmp/claude-\*}{/tmp/user-*}g;
    s/\bclaude\b/vendorY/gi;
    s/\bcodex\b/vendorX/gi;
    s/\bgemini\b/vendorZ/gi;
    s/\bargon\b/model-A/gi;
    s/\bagy\b/cliZ/gi;
    s/\bantigravity\b/cliZ/gi;
    s/\banthropic\b/vendorY/gi;
    s/\bopenai\b/vendorX/gi;
    s/\bxai\b/vendorW/gi;
    s/\bgoogle\b/vendorZ/gi;
    s/\bgrok\b/vendorW/gi;
    s/\bmuse\b/vendorV/gi;
    s/\bopus\b/model-C/gi;
    s/\bsonnet\b/model-C2/gi;
  ' "$f"
done
grep -rnoiE "(gemini|argon|claude|opus|fable|codex|gpt-|grok|muse|anthropic|openai|xai|google|sonnet|agy|antigravity|astra)" "$B" | awk -F: '{print tolower($NF)}' | sort | uniq -c || true
