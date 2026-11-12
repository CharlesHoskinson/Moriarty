# Morning report, 2026-11-11

The night plan (`NIGHT-PLAN.md`) is done except two items that cannot finish before the code they check exists: C1 is partial (the dense backend is built; the second backend is not, by decision) and C2 is partial (no boundary model yet). See "Not done". Everything is on `main` and pushed (last commit `5c090e9d2`). Every decision taken while you slept is in `DECISIONS.md`, and the ones that need you are listed at the end.

## What you have now

1. **One merged architecture**: [`ARCHITECTURE.md`](ARCHITECTURE.md), 3,536 lines, 23 module kits. Each kit gives its purpose, bus interface, invariants, row budget, tests and named mutants, Quint model, Lean obligations, code skeleton, and what one implementing agent reads. It has a build order (F, 0, R, 1 to 4, L, 5 to 8), the prerequisite experiments X1 to X15, and the open decisions.
2. **An implementation spec**: the new OpenSpec change [`sherlock-modular-leaf-20261111`](../../openspec/changes/sherlock-modular-leaf-20261111/proposal.md). 23 capability specs, 119 requirements, 203 WHEN/THEN scenarios, 84 tasks. `openspec validate --strict` passes with no warnings.
3. **GP-1 and GP-001 rewritten** in `sherlock-vm-consensus-20261107` without narrowing: every current sentence kept, the meaning of 2^64 added (the step counter's range, not a circuit size). `openspec validate` and the F14 traceability check pass.

## Validation behind it

| What | Result |
|---|---|
| Run-length study (5 Gemini 4 Carbon lanes) | Consensus; Fable 5.2 audit: adopt with changes (3 blocker, 10 major, 7 minor), fixes taken |
| Lookup-cache study (5 Gemini 4 Carbon lanes) | Consensus; Fable 5.2 audit: adopt with changes (3 blocker, 11 major, 9 minor), fixes taken |
| Six architecture drafts (3 Astra, 3 Gemini) | Merged by Fable 5.2 |
| Architecture audit (GPT-6 Astra) | Adopt with changes (4 blocker, 15 major, 1 minor); all 20 checked against the code and fixed in the design text (`ARCH-AUDIT-RESPONSE.md`) |
| C1: dense memory backend built (`crates/mori-chip/src/membackend.rs`, `dense.rs`) | MockProver: honest tests pass, 22 named mutants refused. Measured: flush 8,821 rows, old-memory read 4,761 rows at depth 63 (+280 / +140 over depth 61, no new column) |
| C1 Quint lane (Gemini Argon) and lead re-run | 30 of 30 commands as expected, 34 tests pass, every mutant caught. The lane found a real gap at tree depth 1 (unreachable at 61 or 63); fixed in `f36eb714d` |
| C3: Lean (`PartitionIndependence.lean`, `ModularComposition.lean`) | 52 theorems, no `sorry`, no new axiom, `lake build` green: memory state is the same for every cut of a run into leaves of at most 8 appends; the old D3 layout is not partition-independent; composition holds for an abstract module graph with one ordered bus. Larger leaves and the landed two-lane bus are open obligations (Astra findings 7 and 8) |
| X1, X2: per-rule maxima and one-arena check (120 rules) | At most 3 appends per step; one arena holds for all 120 rules (premises mechanical, last induction by code reading) |
| 3.x dispatch branch | Merged (`e39bc7af8`); dispatch receipt 6/6 under the SDK check; five fragment groups pass. No task ticked: the U023a/b/c checks are still stubs, and the 3.4 receipts and inventory flips are not written |

## What the design is, in short

- 2^64 is the range of the 64-bit step counter. No circuit has 2^64 rows. Proving work grows linearly with the steps; keys, proof size and verification cost stay fixed.
- Memory is dense: length grows only with real allocations, a partial segment of 8 cells travels as a tail, full segments go into a Poseidon Merkle tree. One arena. Depth 63 (2^66 cells) covers 3 allocations per step for 2^64 steps.
- Frames and tasks are Poseidon hash chains. Old memory is read by a Merkle opening or by 8-record cache lines.
- Boundary commitment is a Poseidon digest (header plus body). Write-once LogUp with Horner fingerprints stays behind the same interface as a benchmark only, not built before M4.
- Continuations: segment tree plus the retained-memory checkpoint; the address-free checkpoint stays research.

## Limits of the validation

- **Row fits are ESTIMATE.** The leaf fit under `k_leaf <= 16` and the node under the 950,000-row gate are estimates built from measured parts. No leaf has the dense memory wired in yet, and node rows over such a leaf are not measured (X5, X7).
- **The B2 additions have had no independent audit.** The four kits added after the Astra audit (INIT, REFERENCE BRIDGE, HOST CHECKPOINT STORE, GP-3 general verifier), the GP-2 discharge protocol and the header-plus-body state commitment were written in the revision that answered the audit.
- **The OpenSpec change is validated for form.** `openspec validate --strict` checks structure, not content. Some scenarios name evidence the architecture leaves unnamed (the seam test, per-family CORE tests, HASH differential vectors), and the host-only kits (REFERENCE BRIDGE, HOST CHECKPOINT STORE) cite host tests, not MockProver or Lean.
- **Late draft text.** The merge started while the Gemini drafts were still being written (last writes 02:47 to 02:51). The late text is now reconciled (`ARCH-AUDIT-RESPONSE.md`, section "Late draft text (B2a)"): HASH domain numbers, a compile-time profile guard for Horner, the harness mutant record and 11 conflict rows were taken; nine items were rejected against decided inputs. The architecture outline now lists a harness spec (5.2.3) that the OpenSpec change does not have yet.
- `design.md` section 2.7 of `sherlock-vm-consensus-20261107` still describes the D3 padded memory. It now carries a dated note that the proposed design replaces it, pending your adoption.

## Not done

- **C1 second backend (write-once LogUp).** Not built, by decision L-5 (benchmark-only, after M4).
- **C2 boundary model.** The Quint model covers the memory backend. The boundary commitment (header plus body) is designed but not built, so it has no model yet.
- Lean: 14 obligations proved, 53 open (listed per kit in the architecture).
- Experiments still open before the interface freeze: X3 (hash stacks), X4 to X15, among them X15 (ROM rows per step, which fixes `K_rom`) and X10 (opening the state header in the node and decider).
- Carried from earlier work: the task 2.5 worktree is still uncommitted; the T1 rerun for task 2.16 (its script needs recreating); the genesis-check prove time for task 4.13; the lead decision for task 2.8.
- Workspace clippy with `-D warnings` is red on files the night did not touch (`rfcore.rs`, `m3.rs`, `cpu.rs`, `mem.rs`, `poseidon.rs`, `mori-lexer`, `mori-recursion`).

## Questions for you

1. **AD-1: tree depth 63** (2^66 cells) so memory never binds before the step counter. Measured cost: +280 rows per flush, +140 per old read. Holds if one logical record is one cell (AD-11); two cells per record would need 64.
2. **AD-2: no claim counter in the public statement.** An occurrence count summed through joins reaches 2^64 at depth 64 with about 128 proofs, which would limit GP-4's DAGs of any depth.
3. **AD-20: GP-2 scope.** No consume rule exists in the owner machine today. Recommended built path: consume input claims at the seal. Do you also want a consume rule mid-run? If so, it is required work, not a narrowing.
4. **AD-17: input admission** by digest with an empty genesis memory (recommended), then **AD-18**: amend SRC-021 to the one-arena Init field list.
5. **AD-21: semantic reference** for the full language. The current reference machine has no constructor values or expressions. Recommended: a separate evaluator over `moriarty-sherlock-nominal/1`.
6. Carried from before: D3 wording (i)/(iv), D4 inline vs separate, the run-ceiling question in `dissent.md` (now answered by the design, pending your review), exhaust kind 7 (AD-16).

## Links

- [`NIGHT-PLAN.md`](NIGHT-PLAN.md), [`DECISIONS.md`](DECISIONS.md), [`ARCHITECTURE.md`](ARCHITECTURE.md), [`ARCH-AUDIT-astra.md`](ARCH-AUDIT-astra.md), [`ARCH-AUDIT-RESPONSE.md`](ARCH-AUDIT-RESPONSE.md)
- [`EXPERIMENTS.md`](EXPERIMENTS.md), [`EXPERIMENTS-C3.md`](EXPERIMENTS-C3.md), [`EXPERIMENTS-MAXIMA.md`](EXPERIMENTS-MAXIMA.md)
- Studies: [run-length](../sherlock-runlength-consensus-20261111/CONSENSUS.md) and [audit](../sherlock-runlength-consensus-20261111/AUDIT.md); [lookup-cache](../sherlock-lookup-cache-20261111/CONSENSUS.md) and [audit](../sherlock-lookup-cache-20261111/AUDIT.md)
- Spec: [`sherlock-modular-leaf-20261111`](../../openspec/changes/sherlock-modular-leaf-20261111/), [`sherlock-vm-consensus-20261107`](../../openspec/changes/sherlock-vm-consensus-20261107/)
- Wiki: `llm-wiki/wiki/syntheses/OpenOwnerQuestions.md`, `WorkstreamStatus.md`
