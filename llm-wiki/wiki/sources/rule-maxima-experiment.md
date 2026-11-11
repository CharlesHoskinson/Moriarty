---
title: "Per-rule memory maxima and the one-arena non-interleaving check"
type: source
tags: [experiments, memory, arenas, leaf-sizing]
sources: [rule-maxima-experiment]
last_updated: 2026-11-11
date: 2026-11-11
source_file: ~/Projects/Moriarty/research/sherlock-final-design-20261111/EXPERIMENTS-MAXIMA.md
source_url: local
---

# Per-rule memory maxima and the one-arena non-interleaving check

## Summary

Experiment asked for by decisions R-6, L-2, L-9 and E-3 ([[final-design-decisions]]), run-length audit finding 6 ([[runlength-consensus-audit]]) and lookup audit blocker 1 and finding 8 ([[lookup-cache-audit]]). A test-only module in `mori-core` (`rule_maxima_tests.rs` under `owner_adt_runtime`) derives per-rule maxima of one step two ways: a rule-sliced text scan of the Core family, fragment, ADT-runtime and guard code (labelled STATIC, an upper bound), and real owner steps over a 28-program corpus through a logging `PreImageView` (labelled OBSERVED, a lower bound). It also checks the one-arena non-interleaving claim. Other labels in the source: READ (code reading) and DERIVED (formula). No circuit changed.

## Key Claims

- Rule set: 120 enabled step rules, the 108 of `supported_rules()` plus the 12 `XAdtHelperBuild*` and `XAdtHelperPlan*` rules the 108 leave out (STATIC, asserted). 90 of the 108 and 102 of the 120 fired on the corpus; the 18 that did not are failure paths and have zero appends (STATIC).
- Maximum appends in one step: 3 cells (`BHelperScalarExit`: 2 FIELDS + 1 DATA) (STATIC = OBSERVED). Per kind 1 DATA, 2 FIELDS, 1 ENV, 1 SLOTS. Only `BHelperScalarExit` and `XAdtHelperBuildField` append two kinds in one step.
- Stack operations: at most one frame operation, or one task push or pop plus one task rewrite-top; no step has both a frame and a task operation (STATIC).
- Heap reads: at most 4 distinct records in one step (`BHelperEqualField`) (OBSERVED 4, READ 4; the STATIC call-site bound is 9). Every heap read can be older than the leaf window (READ); all 16 rules with heap reads read a flushed segment at S = 1 on the corpus, read age up to 507 steps (OBSERVED).
- Segments: at most 2 one-arena segments appended into (OBSERVED; DERIVED bound for 3 cells), old segments read 2 OBSERVED with a READ bound of 4, all segments touched 3 OBSERVED with a DERIVED bound of 6.
- ROM: at most 3 family ROM rows in 3 tables (OBSERVED), plus at most 2 PROG expression rows during selection (READ).
- Hash calls (DERIVED from the design): at most 3 per step without cold reads; each cold read adds 1 record hash and `TOP + 3` node hashes.
- Non-interleaving: holds for all 120 rules, no counterexample. Property (A): each kind's appends in a step are one consecutive run. Property (B): every reachable FIELDS or SLOTS slice is at consecutive unified addresses. Premises S1 (apply order), S2 (only the Append helper appends SLOTS, only Build writes Build fields, helper rules append only their kind), S3 (entries start at the active length) and S4 (helper exclusivity in `owner_select`) are checked mechanically; the induction from them to (B) is a code reading. Dynamically: 6,668 applied steps and 9,031 slices with no violation.
- Leaf sizing: one arena with `W_flush = 1` per step is safe (DERIVED); at S = 1 a step needs up to 4 old heap records, so `K_page >= 4` in one arena or 4 scalar cold reads (READ bound).

## Key Quotes

- "The one-arena non-interleaving property holds for all 120 enabled rules. No counterexample."
- "So `K_page >= 4` (one arena) is required for a step to fit with line caching, or 4 scalar cold reads per step with `cold_read`."

## Connections

- Inputs: [[final-design-decisions]], [[runlength-consensus-audit]], [[lookup-cache-audit]], [[runlength-consensus]].
- Memory design: [[DenseTailMemory]], [[BlockCache]], [[c1-dense-memory-experiment]], [[ChipLeaf]].
- Status: [[WorkstreamStatus]], [[OpenOwnerQuestions]].

## Contradictions

- The run-length audit (finding 4) and the lookup consensus default to four arenas until the non-interleaving invariant is mechanised; this experiment finds the invariant holds, with a code-reading step left ([[runlength-consensus-audit]], [[lookup-cache-consensus]]).
- The 108 rules of `supported_rules()` leave out the Build and Plan helpers, but those rules run on every ADT program; `XAdtHelperBuildField` is one of the two two-kind steps and `XAdtHelperPlanWalk` reads old memory.
- The lookup consensus fixed `K_page = 4` without a per-rule table; this experiment gives 4 as a code-reading bound, not a measured leaf ([[lookup-cache-consensus]]).
