---
id: language.compilation-path
type: decision
title: Mori to ZKIR compilation path
status: draft
updated_at: 2026-09-13T15:30:00Z
sources:
  - SRC-0120
created: 2026-09-13
updated: 2026-09-13
tags:
  - moriarty
  - research
  - language
  - formal
---

# Mori to ZKIR compilation path

## The gap, established

**CLM-1011 — There is no compiler from Core to Compact today.** What stands in
the gap is a validator plus a string template. `lowerLifecycleSourceBinding`
rejects any input whose SHA-256 is not one pinned constant, checks the emits are
exactly Transfer, Originate, Fee in that order, checks party names against
literal strings, and returns a digest bundle marked `compiled: false`. The
Compact itself is assembled from JavaScript template literals containing
hand-written circuit text. The file says so in its own comment: "This is one
selected source, not a general-purpose financial lowerer"
(CLM-1011; `experiments/moriarty-midnight-financial/ledger/lifecycle-source-binding.mjs:8-10`;
`custody/generate-lifecycle.mjs`; source fact; reproduced; high; S4).

**CLM-1012 — K semantics already exist at both ends.** 1,037 lines for Moriarty
Core and 2,815 lines for ZKIR, the latter including a VM, a constraint system,
field arithmetic and curve operations. The missing formal artifact is the middle,
not the ends
(CLM-1012; `experiments/moriarty-language/formal/k/`; `graphs/zkir-k/corpus/`;
measurement; reproduced; high; S4).

## Why not formalise a Compact subset

**CLM-1013 — Every JavaScript mechanisation died at the standard it started
with.** JSCert has had no substantive commit since December 2016 and still
requires Coq 8.4.6; its domain now serves a casino site. JSExplain is marked
unmaintained. LambdaJS targeted ES3, last real commit 2013, archived 2022. S5
targeted ES5, last commit October 2015. KJS last commit February 2015.
Gillian-JS is actively maintained as a framework but its JavaScript instantiation
remains ES5. **None crossed a standard boundary.** JSCert's own paper predicted
straightforward updating and hoped to reach ES7
(CLM-1013; `deliverables/research-2026-09-13/safe-subset.md`; retrieved sources;
reproduced; high; S4).

**CLM-1014 — Compact moves faster than JavaScript did, and moves invisibly.**
Toolchain 0.31.0 in April 2026 to 0.34.0 in August 2026, language 0.23.0 to
0.26.0, ZKIR v2 to v3. Decisively: 0.31 changed the ZKIR representation of
conditional-branch operations **with no syntax change**, so a syntactic subset
would have continued to typecheck while the circuit semantics moved underneath it
(CLM-1014; research; reproduced; high; S4).

**CLM-1015 — No subset desugaring in that lineage was ever proven correct.**
λJS, S5 and JaVerT all validate desugaring by test suite rather than proof.
JSCert's authors note λJS's encoding could not be proven because no source
semantics existed to prove it against
(CLM-1015; research; reproduced; high; S4).

## Why image-of-the-lowerer instead

**CLM-1016 — The precedent is strong and alive.** CompCert specifies each
intermediate language in 300 to 600 lines of Coq against 1,100 for Clight, with
theorems stated over source programs so unreachable intermediate states fall out
of scope. WebAssembly's designers attribute their small specification to
designing a producer-only target with formal semantics from the start, and Watt's
700-line Isabelle mechanisation found two type-system unsoundnesses that were
then fixed, **because the authors owned the spec**. Racket's fully-expanded-program
grammar is the same pattern
(CLM-1016; research; reproduced; high; S4).

**CLM-1017 — The Plutus translation-certification work gives the technique.**
Krijnen and colleagues, 2022, for keeping a certified lowering robust to compiler
change (CLM-1017; research; reproduced; high; S4).

## The counterargument, recorded

**CLM-1018 — Safe subsets have failed before, and the dependence does not
vanish.** ADsafety found new exploits in an audited safe subset; Google archived
Caja in 2021 with unpatched vulnerabilities. The image strategy shrinks but does
not remove dependence on Compact's semantics, which the 0.31 branch change
demonstrates directly
(CLM-1018; research; reproduced; high; S4).

## Provisional decision

Define the target as **the image of our own lowerer**, enforced by a lint pass,
with the lowering specified as translation relations. Attach semantics to that
image only per pinned Compact toolchain version, validated by differential
testing. Do not formalise a Compact fragment. Do not bypass Compact yet.

Pending: the translation-validation, K-equivalence and circuit-verification
research reports, which may qualify this.
