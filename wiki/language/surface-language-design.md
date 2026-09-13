---
id: language.surface-design
type: language
title: Moriarty surface language design
status: draft
updated_at: 2026-09-13T07:00:00Z
sources:
  - SRC-0114
  - SRC-0115
  - SRC-0116
  - SRC-0117
  - SRC-0118
  - SRC-0119
created: 2026-09-13
updated: 2026-09-13
tags:
  - moriarty
  - research
  - language
---

# Moriarty surface language design

Three independent designs, produced 2026-09-13 against the constraints in
[[wiki/language/zk-language-survey|the survey]] and
[[wiki/language/jet-discipline|the jet analysis]]. Full documents in
`docs/design/`: `surface-syntax-2026-09-13.md`, `surface-sugar-2026-09-13.md`,
`developer-workflow-2026-09-13.md`.

The governing question put to every designer was: **what are the timeless
practices a developer returns to again and again?** Constructs that served only
novelty were to be cut.

## The measured result

**CLM-0994 — The loan lifecycle goes from 178 lines to 72.** Measured including
blank lines. The 39 `ensures` become 14: frame conditions are generated, and the
`outstanding` assertions are dropped because the financial slice proves
`outstanding = principal + accrued` is a maintained invariant. The remaining
assertions state facts about the loan rather than facts about the frame
(CLM-0994; `docs/design/surface-syntax-2026-09-13.md`; analysis; reproduced; high; S4).

## What is genuinely new, after rejecting what is not

**CLM-0995 — Party identity is not new and was rejected as a novelty claim.** It
is an enumerated type, a practice from the 1970s. Claiming it as invention would
be the reskin the brief warned against
(CLM-0995; design; reproduced; high; S4).

**CLM-0996 — Asset-scoped precision with context-typed literals.** `asset Cash
scale 2` plus a literal `100 Cash` resolving to `Amount` or `Quantity` by
expected type. The elaborated `Quantity<Units<Cash,1>,2>` survives intact while
never being written. F# has units of measure but not currency scale; Rust and
TypeScript have neither (CLM-0996; design; reproduced; high; S4).

**CLM-0997 — Phase-typed state.** `pre`, `next` and `post` as three environments
where the `ensure` block sees a *different type* than the body. Cairo has one
axis, read versus write; Moriarty has three views with distinct capabilities, and
[[wiki/language/jet-discipline|CLM-0990]] establishes the third view is a type
difference rather than a scope rule. The soundness property becomes unforgeable
by spelling (CLM-0997; design; reproduced; high; S4).

## Frame inference

**CLM-0998 — The frame is generated per domain cell, not assumed.** For every
cell not in the written set and not in the footprint of any emitted operation,
the elaborator appends the unchanged condition. Touched cells get no generated
condition. An author denies the frame explicitly with a `modifies` clause naming
regions, which is a promise the author is making
(CLM-0998; `docs/design/surface-sugar-2026-09-13.md`; analysis; reproduced; high; S4).

**CLM-0999 — The operation footprint table does not yet exist in one place.** It
is derivable from `financial-lifecycle.ts` but is not stated anywhere as a single
artifact. Pinning it is the first implementation task, and an operation with no
row cannot be emitted
(CLM-0999; design; reproduced; high; S4).

## Workflow

**CLM-1000 — Error spans already exist and are unused.** Every diagnostic is
built with a primary byte span, but the message field is set to the code itself.
The gap between today's `TYPE_MISMATCH` and a Rust-grade diagnostic is rendering,
not analysis (CLM-1000; `experiments/moriarty-language/src/diagnostics.ts:7`;
source fact; reproduced; high; S4).

**CLM-1001 — A lockfile precedent already exists.** The K toolchain is pinned by
version, Nix path, NAR hash and per-executable SHA-256. The same shape extends to
the Moriarty and Compact toolchains
(CLM-1001; `experiments/moriarty-language/formal/k/toolchain.lock.json`;
source fact; reproduced; high; S4).

**CLM-1002 — `test` is the most-run command, not `build`.** The workflow designer
disagreed with the brief on this and was right: the test command writes the
evidence record, so receipts exist when audits need them. Doing the rigorous
thing becomes the easy thing
(CLM-1002; `docs/design/developer-workflow-2026-09-13.md`; design decision; reproduced; high; S4).

## Corrections the designers made to the brief

**CLM-1004 — "Everything not assigned is unchanged" was incomplete.** Emits change
ledger cells without any assignment, so the default must be "not assigned *and*
not in the footprint of an emitted operation". This is why pinning the footprint
table is the first task rather than a detail
(CLM-1004; `docs/design/surface-sugar-2026-09-13.md`; correction; reproduced; high; S4).

**CLM-1005 — The frame does not reproduce all 39 original conditions, and should
not.** Eleven of them assert *fixture values* on untouched cells, such as
`post_allowance_remaining<Cash>("Borrower") == amount<Cash>(110)`. The frame
replaces those with `post == pre`, which is strictly stronger. A contract should
not know its test fixture
(CLM-1005; design; correction; reproduced; high; S4).

**CLM-1006 — Flow syntax cannot close the funder hole by itself.** The 5.1a
handover established there is no source-level slot to bind authority. Grammar can
make a transfer without a payer unwritable and emit a funding claim, but the
claim must be discharged at the Compact seam. Describing the grammar as closing
the hole overstates it: it makes the obligation an artifact the seam must
consume (CLM-1006; design; correction; reproduced; high; S4).

**CLM-1007 — Two proposed sugars were rejected as unfounded.** `Loan1.accrue(at)`
and `10% per 60s` infer from context and their expansions need rate-scale
decisions the repository has not made
(CLM-1007; design; correction; reproduced; high; S4).

## The work-meter problem, solved by construction

**CLM-1008 — Term-count parity with a hand-written twin removes the need for a
declared-cost rule.** Every construct is defined by a twin written in the current
profile, and is admissible only if both elaborate to byte-identical Core. The
work consequence is then zero relative to the twin by construction, so
[[wiki/language/jet-discipline|CLM-0980]]'s declared-cost requirement does not
arise for sugar. Nothing expands into a boolean operator or a conditional
(CLM-1008; design; reproduced; high; S4).

**CLM-1009 — Substitution, never binding.** Because the static rule at
`financial-expression-v1.ts:347` requires a syntactically literal divisor, no
expansion may introduce a binding. Defaults and unit literals are copied as
literal nodes at each site. This is [[wiki/language/jet-discipline|CLM-0985]]
designed around rather than deferred
(CLM-1009; design; reproduced; high; S4).

**CLM-1010 — Measured result of the sugar.** The origination action goes from 42
lines to 11, and its suffix work from 78 units to 30
(CLM-1010; design; measurement; reproduced; high; S4).

## Honesty markers carried from the designers

**CLM-1003 — The worked session transcript is designed output, not captured.** No
`moriarty` binary exists. The designer labelled it explicitly rather than
presenting it as a recording
(CLM-1003; design; reproduced; high; S4).
