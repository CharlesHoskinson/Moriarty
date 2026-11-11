---
title: "AGENTS.md factory charter"
type: source
tags: [owner-rule, factory]
sources: [agents-charter]
last_updated: 2026-11-10
date: 2026-11-10
source_file: ~/Projects/Moriarty/AGENTS.md
source_url: local
---

# AGENTS.md factory charter

## Summary

The charter of the DoctorWatson factory that develops Moriarty. Moriarty is a bounded financial language for Midnight, and the current goal is to finish the Sherlock recursion processor. The charter gives the work loop and the fixed configuration. Earlier routing, review, adoption-vote, campaign, lane and evidence rules are retired.

## Key Claims

- The loop: run `scripts/sherlock-next`, make the first failing predicate pass, run heavy work through `scripts/doctorwatson heavy`, run `scripts/doctorwatson check <ID>`, commit (the pre-commit hook runs T0), then run `scripts/doctorwatson stop-rule`.
- Reviews are inputs. Nothing waits on a reviewer. The one review point is the T3 stagenet promotion, a cold review through `codex exec -s read-only` that proceeds after one hour without an answer.
- Commits go to branch `main`, with no AI `Co-authored-by` trailers and no Claude attribution in commits or PRs.
- Each inner-loop commit touches code or tests. `qa/holdout/` is evaluator-owned.
- Rust is pinned to 1.99.0 and must pass `cargo fmt`, `cargo clippy` with `-D warnings` and `cargo test`.
- No credentials, wallet seeds, private witnesses or unredacted environments are committed.
- Maintainer tooling never becomes a prerequisite for public developers.
- Model CLIs use subscription logins only.

## Key Quotes

- "Reviews are inputs. Nothing waits on a reviewer."
- "If it says STOP, bring the owner the trace and your best hypothesis. Don't write another document."

## Connections

- [[OwnerRules]] lists these rules beside the GP rules of [[general-purpose-definition]].
- [[WorkstreamStatus]] tracks the work that the loop produces.

## Contradictions

None found.
