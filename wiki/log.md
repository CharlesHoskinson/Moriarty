# Wiki log

## [2026-09-03] design | Moriarty semantics and intent research prompt

Added a focused XML assignment for Moriarty semantics, intent correctness,
verified compilation, Compact and ZKIR realization, and the standard developer
interface. Acquired the CAKE framework and 28 official ERC and EIP pages with
Scrapling. Preserved exact status, source date, path, and SHA-256 evidence. The
assignment uses twelve evidence-gated sprints and makes no semantic change.

## [2026-09-03] audit | Evidence and SDK gate hardening

Closed the independent review findings. The disclosure validator now handles
whitespace, excludes comments and strings, and rejects unnamed compound
expressions. Complete compiler-interface metadata is checked against the
Moriarty manifest. Clean reproduction and current-checkout evidence now have
separate identities. Sprint manifests bind package, scope, outputs, self-hash,
and package-specific gates. The SDK validator applies its schemas, requires
exact counts, closes component references, and recomputes its complete index.

## [2026-09-03] experiment | Clean Compact reproduction and complete SDK contracts

Reproduced E00 from a clean archive and fresh environment. Forty-three focused
tests passed, 1,000 traces had zero divergence, and all semantic, Compact,
compiler-manifest, ZKIR, and negative-control digests matched. Recorded the
Compact source-map path-sensitivity footgun. Added an independent Moriarty
visibility-manifest validator with missing and additional disclosure controls.
Specified 65 SDK components, 28 canonical data contracts, two JSON contract
schemas, and a 17-component minimum safety spine. Preserved the Grok, Sol, and
exact Fable 5.1 Council advisory and its requested changes.

## [2026-09-03] decision | Evidence-gated sprints and complete SDK scope

Superseded calendar-based progress with evidence-gated sprints. Added the
research journal and an iteration-scoped semantic ledger. Classified E00 as a
narrow Compact DSL feasibility result rather than general language evidence.
Expanded the SDK boundary to the complete authoring, compiler, analysis,
packaging, verification, wallet, chain, and operations development system.

## [2026-09-03] experiment | Moriarty Core atomic-swap stop test

Implemented the finite E00 Core subset and canonical two-token swap. Generated
fixed-state Compact, an artifact manifest, and an independent transition
machine. One thousand unique traces produced zero divergence and zero invariant
failures across 18 required deadline-boundary cells and both terminal-expiry
rejections. Follow-up semantic review added manifest-driven time guards and
timeout priority, canonical refund ordering for all accepted party names,
small-deadline coverage, and exact Compact integer validation. The first Compact
compile reproduced an undeclared-disclosure
footgun for the decision argument. The corrected source lists and applies the
public disclosure explicitly; the failing source and diagnostic digest are now
preserved. Compact emitted four ZKIR 3.0 circuits, and the pinned mock compiler
accepted all four. Constructor values, real keys, and proof generation remain
open.

## [2026-09-03] decision | DeFi Kernel prompt, council, and graph

Reproduced the 47/72 1-NN, 50/72 3-NN, Jaccard, and exact pair-rate results.
Ran tool-free round-1 proposals with Grok, exact Fable 5.1, and GPT-5.6 Sol,
then a blinded round 2; recorded Fable's budget-exhausted second round as an
abstention. Added the 12-workstream XML prompt, gated 90-day sprint, seven-family
Marlowe mapping, and the library-only stop path. Built a six-source Moriarty
decision graph with 46 nodes, 45 directed edges, and zero missing endpoints.

## [2026-09-03] ingest | Repository-verified taxonomy update

Preserved the decision-bearing claims from the user-supplied updated run as
SRC-0017. Reproduced the DeFiFormal pair, canonicalization, obligation, roster,
and v3 self-test counts at commit
`8ae0bbfaa3193078d1cabf6999db1382985b7f95`. Replaced M4+ as the top-level
decision with M2+M3 human-facing families and facets plus an M5 formal behavior
profile. Added a 72-row family/facet crosswalk while preserving the M4+ file as
historical input.

## 2026-09-02 — Moriarty decision packet

- Added the consolidated decision study and the standalone stakeholder
  pre-read.
- Added an experiment handoff for the compiled Compact/ZKIR escrow vertical
  slice.
- Kept Marlowe as the upstream and migration name; Moriarty is the new language
  and repository name.

## [2026-09-02] ingest | Marlowe organization and live documentation graph

Refreshed and pinned all 36 `marlowe-lang` repositories, acquired all 100 live
documentation sitemap paths with Scrapling, added the official online entry
points, and built the combined AST, semantic, documentation, and provenance
graph. The graph contains 8,864 nodes, 10,888 edges, and 120 hyperedges. Its
1,510 dangling endpoints and 265 undirected relation collapses are recorded as
health limitations rather than hidden.

## [2026-09-02] ingest | DeFi taxonomy report and local corpus reconciliation

Preserved and extracted the user-supplied 25-page taxonomy PDF. Reconciled its
public-access limitation against the authorized clean local DeFiFormal checkout
at commit `8ae0bbfaa3193078d1cabf6999db1382985b7f95`. Reproduced the 72-row corpus,
60 constructions, 1,259 obligations, 570 covered obligations, and 689 residue;
generated a one-to-one construction roster and draft M4+ crosswalk.

## [2026-09-02] design | Canonical Moriarty patterns for the 72-row roster

Assigned every DeFiFormal row to a bounded canonical application pattern and
added 13 surface-language strawmen. D01–D11 each receive one reference pattern;
D12 splits into an event-contingent market and a delegated-curator vault. The
sketches separate provable Core obligations from oracle, bridge, custody,
identity, solver, validator, and legal capabilities.

## [2026-09-02] ingest | Research assignment and LLM Wiki method

Initialized the research repository around the upstream Marlowe modernization
assignment, preserved the controlling prompt, installed the acquisition
environment, and ingested Karpathy's original LLM Wiki idea file as `SRC-0001`.

## [2026-09-02] decision | Rename to Moriarty

Renamed the repository and proposed language to Moriarty. Marlowe now refers
only to the upstream source language, implementation, and migration baseline.

## [2026-09-02] ingest | Midnight and active Compact repositories

Acquired the official Midnight documentation corpus with Scrapling, enumerated
and cloned all 74 public `midnightntwrk` repositories, followed the official
Compact relocation, and cloned all three public LFDT Minokawa repositories.

## [2026-09-02] experiment | Moriarty escrow to Compact and ZKIR 3

Built Compact compiler 0.34.100 from pinned source, compiled a finite Moriarty
escrow lowering with the ZKIR 3 backend, passed six acceptance tests and 44 ZKIR
library tests, and mock-compiled all three generated circuits. Full proof tests
remain dependent on external `MIDNIGHT_PP` parameter files.

## [2026-09-03] ingest | K Framework, ZKIR specification, and Midnight K tooling

Crawled kframework.org with Scrapling (SRC-0024) and confirmed
`runtimeverification/k` as the sole canonical repository; cloned it at
v7.1.337 (SRC-0023) and recorded the lock. Acquired `input-output-hk/arc-zkir`
(SRC-0025), the Agda mechanization and textual specification of ZKIR v2 and v3,
and fetched its pinned `midnight-ledger` `ledger-9` commit `92e8bdd3`. Built a
315-node graphify graph of the K documentation with agy `gemini-3.8-flash-high`
as extractor. Wrote seven K pages, seven ZKIR pages, and the ZKIR-in-K plan (advisory consult routed to agy after Claude overload, preserved in raw/notes/);
recorded the ledger-8 versus ledger-9 pin conflict and the 34 versus 42
instruction drift as contradictions. Claude subagents were abandoned for this
ingest after repeated API overload failures; all reading was done by agy.

## [2026-09-05] toolchain | K v7.1.337 installed and checked (milestone 1)

Installed `kup` 0.2.6 into the Nix profile and K v7.1.337 at the pinned commit
4a46d123 from the K binary cache (2m 29s, no source build). Lesson 1.2 of the K
tutorial compiles and runs on the LLVM and Haskell backends; pyk 7.1.337 from
PyPI (`kframework`, uv group `zkir-k`) round-trips KAST through KORE and runs a
program. Check script and receipt: `experiments/zkir-k/toolchain-check/`,
`evidence/k-toolchain-install-2026-09-05.md` (CLM-0723). Plan page milestone 1
marked done; next is milestone 2, module `ZKIR-SYNTAX` and the pyk
JSON-to-KAST preprocessor.
Also repaired the five ZKIR pages from the 2026-09-03 ingest that linked
sibling pages and arc-zkir Agda modules by absolute `file:///home/charl/...`
URIs; they now use relative paths (`../../repos/...` for pinned sources).

## [2026-09-05] semantics | ZKIR-SYNTAX, ZKIR-WF and the pyk preprocessor (milestone 2)

Wrote `experiments/zkir-k/semantics/zkir-syntax.k` (abstract syntax of the
34-instruction, 13-type surface at 92e8bdd3, `reads`/`writes`, static
well-formedness) and `tools/zkir_kast.py` (JSON to K term, serde-faithful).
Assembled a 56-program version-3 corpus under `experiments/zkir-k/corpus/`
(the crate's inline test programs, midnight-zkir micro-dao precompiles, Moriarty
artifacts) plus 7 handmade negatives; `tools/check_corpus.py` passes 63/63
(CLM-0724, evidence/zkir-k-milestone2-corpus-check-2026-09-05.txt). Finding: the
ledger's precompiles at 92e8bdd3 are still ZKIR v2 (CLM-0725).

## [2026-09-05] semantics | ZKIR VM, constraint checker, oracles (milestones 3 to 6)

Built the executable K definition under `experiments/zkir-k/semantics/`
(fields, curves, values and encodings, Poseidon, hash-to-curve, SHA-256,
Keccak-256, SHA-512, the VM with constraint emission, the constraint checker,
the `ZKIR-EXT` surface of midnight-zkir 2ffe2d1) and the pyk tooling under
`tools/`. Two Rust oracles (`zkir-oracle` harnesses in worktrees of
midnight-ledger 92e8bdd3 and midnight-zkir 2ffe2d1) give the crate's own
`preprocess`. Results: 41/41 value unit checks, 18/18 hash known answers,
314/314 differential agreements at 92e8bdd3 and 366/366 at 2ffe2d1, all gates
holding on every successful run, 14/14 divergence cases (the review's findings
plus two new ones: K1, Jubjub `from_coordinates` uses only the parity of `x`
off-circuit; K2, a short transcript panics the crate). The arc-zkir v3 Agda
development type-checks but cannot execute programs. New page
`wiki/zkir/zkir-k-definition.md`; plan milestones marked done; K1 recorded in
`contradictions.md`. Receipts: `evidence/zkir-k-*-2026-09-05.txt`,
`evidence/arc-zkir-agda-typecheck-2026-09-05.txt`.

## [2026-09-05] review | eight-reviewer audit and the second iteration

Eight formal-methods reviews (four Claude Fable 5.1, four GPT-6 Astra via
Codex read-only) of the K definition, briefs and reports under
`experiments/zkir-k/review-2026-09-05/`, 34 findings consolidated; verdicts one
APPROVED, four WARNING, three BLOCKED, with cross-vendor agreement on every
major defect. All fixed on branch `zkir-k-iter2`: stuck runs, missing run-time
checks, `test_eq` dispatch, alignment options, error-class comparison, the
commitment gate, unsatisfiable versus unbuildable, public-input indices,
native range checks, panic status, sequential resolution, chip and width
checks, serde fidelity, totality. New receipts (`evidence/*-2026-09-05b.txt`):
42/42, 18/18, 63/63, 20/20 divergence cases, 358/358 and 418/418 differential
agreements. Three more upstream candidates K3 to K5 in `contradictions.md`.
Also: full kframework.org crawl (SRC-0026) and `k-framework/k-best-practices.md`;
semantics graph (237 nodes) in evidence.
- [2026-09-05] docs | fifteen-chapter documentation of the ZKIR K definition under experiments/zkir-k/docs, drafted by three model families and audited cross-vendor as developer and formal methods expert; four semantics defects (K6 Bytes32 strict decoding, sha512 gate alignment, overlapping test_eq rules, check --ext) and three tool gaps fixed; all six check layers rerun green (receipts 2026-09-05c)
