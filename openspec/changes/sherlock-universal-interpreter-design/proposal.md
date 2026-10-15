# Sherlock universal interpreter design

Status: DRAFT / SPECIFIED-ONLY. Candidate A is the concrete subject of review, not an adopted product architecture. Prepared 2026-10-15 against `feat/universal-interpreter-spec-20261015` at `9c63048a18315d7eb3cf114b983bd68c0d842dc6`. Eventual location: `openspec/changes/sherlock-universal-interpreter-design/`; this runtime package does not register or change that location.

## Why

A general compiler target needs one execution relation that interprets different supported programs as input. Fixed financial fixtures, a lifecycle state machine, and a circuit generated separately for each program do not by themselves demonstrate that property. Recursive functions, recursive immutable data, explicit return contexts and eventual durable suspension must have inspectable semantics before lowering and optimization claims.

## What changes

Propose a strict left-to-right first-order CEK-style reference machine with named self/mutually recursive functions, lexical environments, explicit frames, checked UInt128/Bool operations, typed immutable recursive constructors and matching, and finite execution budgets. Program code and declarations are inputs to unchanged transition rules. Physical sharing is an implementation option with structural meaning; source-visible pointer identity and cyclic data are outside this draft.

The concrete K source fragment is a generic pure first-order function-table evaluator with dedicated recursive UInt list/tree sorts and eight unexecuted fixtures. Quint provides three algorithm-specific finite transitions, scalar15 and eight unexecuted scenarios, including a local pending-return checkpoint; it is not a generic code interpreter. Their unequal representations, fuel/tick schedules and omissions require a named projection. See [model-coverage.md](model-coverage.md). Neither source draft establishes execution, refinement, compiler correspondence, circuit correctness, native proof acceptance or ledger results.

Financial suspension, ordered effects, duties and failure behavior remain required future extensions. The existing Core/2 typed-await successful-prefix boundary constrains that adapter; exact universal Commit/Yield/Await opcodes remain OPEN. A technical proof chunk does not change financial semantics. Stable `/1` and `/2` remain unchanged.

## Impact

Future work spans parser/type admission and lowering, host state/serialization/trace tooling, model adapters, and native circuit transition/dispatch/data/frame constraints. The route reuses Midnight native recursion and the existing ledger-9 custom-decider integration. This statement records the user-supplied integration premise, not a fresh reproduction. VerifyProof and ledger10 are not prerequisites; no foreign proof-system port is proposed. Private implementation material is not reproduced here.

The package adds 26 draft nonsecurity UIREQ clauses and twenty normal demonstration contracts within existing U0–U7/M1–M17/MC/SP/G/ZR/MNR obligations. It creates no new milestone, public developer review requirement, campaign, resource counter, source profile or execution authority. Existing ACTUS/DeFi and composition denominators remain intact.

## Review and evidence

The ten research lanes are inputs, not votes selecting architecture. Candidate A, direct AST/native K-stack execution, bytecode and per-program/specialized circuit realization alternatives are compared in [design.md](design.md). Root owns design freeze, integration, canonical registration and review. The second ten-agent optimization wave starts only after root freezes the concrete design/model interfaces; optimization must preserve the observations and costs stated in UIREQ-024. Opus owns the separate [security draft](security-requirements.md), unchanged by this package. Every task is unchecked; see [README.md](README.md) and [traceability.md](traceability.md).

The five unchanged Opus proposals are additional hashed inputs; their disagreement over initial universal versus per-program circuit realization is retained for review. Optional Clean/Lean correspondence research is an OPEN extension only; recursive data types are supported by the concrete dedicated K sorts, with no existing Sherlock interpreter proof claimed.

Required development path: K reference semantics → Quint bounded design validation → Lean semantic/representation definitions plus Clean bounded-step circuit and correspondence obligations → compiler/native integration. [UIREQ-026](specs/universal-interpreter/spec.md) and [clean-stage.md](clean-stage.md) make this a required design stage, with all proof/export/backend work explicitly unproved. Existing public developer permissionless behavior and stable source profiles remain unchanged.

Clean integration deliverables belong in the existing private Fenrir fork/worktree, while this Sherlock language/OpenSpec package remains Moriarty-owned. Cross-repository pins and evidence must bind the actual private integration to the public semantics; no private source/path disclosure or new public Clean fork is proposed.
