# Sherlock required Clean stage

Status: REQUIRED DESIGN STAGE / SPECIFIED-ONLY. The stage is required by the user. Sherlock proofs, circuits, exporters and native integration remain incomplete. No Lean/Clean build, model execution, proof, key or backend command is performed by this package.

## Position in the existing delivery path

K reference semantics → Quint bounded design validation → **Lean semantic/representation definitions and Clean bounded step-circuit specification with correspondence obligations** → compiler/native integration. These are work stages within existing U0–U7/ZR/MNR, not new roadmap milestones or public developer approval gates. Lean/Clean is a required development/design correspondence stage; this does not require each public Moriarty developer to install Lean or obtain a project review receipt to compile/prove/deploy supported programs.

K remains the executable semantic reference candidate. Quint remains a finite design abstraction; its actual three algorithms are not a full Sherlock evaluator. Lean definitions must name the relation to that K reference, rather than silently become a different source language. The Clean circuit must state its relation to those Lean definitions. The compiler/native implementation then needs its own mapping to the modeled relation. Each arrow is an explicit unproved obligation, not an implication from shared names.

## Recursive semantic types versus bounded circuit representation

Define recursive inductive Lean semantic values for the admitted UInt128/Bool/list/tree fragment, with later generic ADTs separately admitted. Define expressions, named function tables, environments, frames, control, outcomes and resource observations at the semantic level. Recursive inductive values are supported; source pointer identity and cyclic data remain outside this draft.

Define a small-step relation and a run relation indexed by finite fuel. The run definition can recurse on decreasing fuel; it must not require every user's recursive function to recurse structurally on its data. Named self/mutual calls, non-tail calls and even nonterminating source recursions are interpreted through finite step attempts, with an explicit Exhausted outcome. This is distinct from proving that every source program terminates.

Define a separate finite encoding for the Clean step inputs/outputs: tagged fixed-capacity node records, bounded code/frame/value arrays, indices, counters and declared scalar representation. Lean's recursive semantic List/Tree type cannot simply be inserted as an unbounded circuit field. Clean's fixed-size `ProvableType`/`ProvableStruct` interface requires an explicit capacity configuration. Sharing erases to structural meaning; the representation relation must state its supported domain, bounds and observations. Exact UInt128 limb/field encoding, reference accesses and helper-step granularity require further design and separately owned security requirements.

## Source-pinned reuse and new work

Public upstream: https://github.com/Verified-zkEVM/clean.git. Public-core research examined commit `bba820f8bdd07d6ea70c43051f4fdf96b2f5186f`; that is not a pin for the canonical private integration. The existing private Fenrir Clean fork is the integration destination. Its actual worktree/revision and relevant changed-byte inventory must be frozen in a private correspondence record; HEAD alone does not identify local changes. Only separately inventoried public core APIs and public file hashes may support public reuse claims. No private path, extension source or private project detail is copied into this package. The [public-only API inventory](public-clean-api-inventory.json) records the shareable source locators/hashes supplied by lane08 and rechecked here. The separate private integration inventory is not linked, copied or included in public input membership.

Inspected public core candidates include `Clean/Circuit/Formal.lean`, `Provable.lean`, witness/export interfaces and the FemtoCairo machine specification example. These motivate fixed-width encodings, bundled circuit interfaces, and finite transition/execution specifications; they are not Sherlock proofs. The inventory locates `Circuit`, `assertZero`, `lookup` and `witnessIR` in Basic.lean; `ProvableType`/`ProvableStruct`/`ProvableVector` in Provable.lean; and finite `step`/table constraints in Table/Inductive.lean. It also records WitnessIR/export and Circom R1CS interfaces; the latter explicitly has unsupported lookup/interact cases. These are reuse candidates, not completed Sherlock components. Existing witness JSON/Circom/Plonky3 paths do not establish Midnight/ZKIR compatibility.

New required work includes Sherlock semantic definitions, its encoding/decoding relation, bounded code/frame/data/scalar transitions, explicit cost accounting, normal fixtures and a circuit-to-native inventory. Financial effects, duties, durable suspension and target phases remain additional required extensions absent from the current K/Quint fragments. No existing generic Clean Sherlock interpreter, complete UInt128 step gadget, Midnight exporter or compatible lowering is assumed.

## Cross-repository delivery boundary

Sherlock language/OpenSpec design stays in Moriarty. Required Clean integration deliverables—including Lean semantic/representation definitions, bounded step-circuit development and their actual integration artifacts—belong in the existing private Fenrir Clean fork/worktree. This is not a request to create or publish a new Clean fork. Public upstream contributions, if later separately requested, need their own scope; none is assumed here.

The correspondence record must bind the public Sherlock semantics/profile and model hashes to the actual private integration revision, source membership, encoding/configuration, theorem/proof membership and compiler/native consumer. Keep the private pin/content evidence in its authorized private destination. The public package may record the dependency, claim/evidence class and an opaque evidence reference without publishing private source paths or code. Public APIs, an upstream commit, or a separately prepared unrelated checkout cannot substitute for that actual cross-repository binding.

## Unproved correspondence and evidence tasks

| Obligation | Required artifact | Present evidence / owner boundary |
|---|---|---|
| K reference → Lean semantics | Construct/rule inventory and explicit simulation relation, including outcomes and costs | UNPROVED; semantic definitions not authored here |
| Semantic value → finite arena | Encode/decode domain and structural-meaning correspondence under declared capacities | UNPROVED; assumptions and security acceptance belong to designated Opus work |
| Step → fuel-indexed run | Finite-run composition and step/cost correspondence | UNPROVED; no universal source termination claim |
| Lean relation → Clean step | A bounded circuit specification and named proof obligations for the same operation/outcome | NOT IMPLEMENTED; security theorem statements/acceptance, soundness judgments and hostile tests are Opus-owned |
| Clean model → actual emitted/native constraints | Private Fenrir integration pin plus source-pinned extractor/lowering or explicitly qualified mapping, naming omissions | OPEN; exporter/backend compatibility unestablished |
| Composition → native recursion/ledger-9 endpoint | Named state/statement projection and retained duties/effects/costs | OPEN; use existing route, no VerifyProof/ledger10 prerequisite |

Normal candidate evidence must include the independently frozen factorial3/list123/tree45 results, unchanged semantic rules across input function tables, pending frames and complete state observations. Exact Lean theorem membership, premises, standard axioms, imported definitions, admitted capacities and completed proof receipts must be reported if later produced. Source stubs, declarations, fixture outputs and a successful Lean compilation are distinct from the required cross-layer theorem claims. This package has none of those proof receipts.

Opus owns the separate Clean-specific security design. This nonsecurity stage description neither writes nor assesses its soundness/acceptance theorems or hostile tests. Existing Opus documents remain unchanged and unresolved design alternatives remain visible.
