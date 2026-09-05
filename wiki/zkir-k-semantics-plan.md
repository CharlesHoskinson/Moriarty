---
id: zkir.k-semantics.plan
type: semantics
title: ZKIR semantics in K, plan
status: active
updated_at: 2026-09-05T17:05:00Z
sources:
  - SRC-0023
  - SRC-0025
  - SRC-0006
  - SRC-0007
---

# ZKIR semantics in K, plan

This page is the synthesis of the 2026-09-03 ingest. It states how the executable K Framework semantics of ZKIR will be written, which sources it targets, and what will count as evidence that it is right. Everything here is a recommendation or inference unless marked otherwise; the facts it rests on live in the linked pages.

## What ZKIR is, in one paragraph

ZKIR v3 is a straight-line, single-assignment intermediate representation over the BLS12-381 scalar field, with named registers, no loops and no branches; conditional behaviour is expressed with branch-free `cond_select` and guarded transcript instructions (`impact`, `public_input`, `private_input`) (CLM-0700; [zkir-vm-semantics.md](zkir/zkir-vm-semantics.md) and [zkir-instruction-set.md](zkir/zkir-instruction-set.md); source fact; not reproduced; high; S5). Every instruction has two semantics that must agree: an off-circuit witness computation (`preprocess`) that fills registers and builds the public-input vector, and an in-circuit constraint synthesis that emits Halo2 PLONKish gates over the same registers (CLM-0701; SRC-0025 docs/zkir-v3-spec.md; source fact; not reproduced; high; S4). The arc-zkir Agda development proves that the two agree (circuit faithfulness, statement soundness, extraction uniqueness) for the 34-instruction, 13-type surface at `midnight-ledger` `ledger-9` commit `92e8bdd3` (CLM-0702; [zkir-formal-spec-agda.md](zkir/zkir-formal-spec-agda.md); source fact; not reproduced; high; S4). The standalone `midnight-zkir` repository has since grown to 42 instructions and 15 types, which the mechanization does not yet cover (CLM-0703; [contradictions.md](contradictions.md); repository observation; reproduced; high; S5).

## Decision: shape of the K definition

The definition is one K definition with one configuration that carries both semantics at once, so that every instruction rule updates the witness store and appends to the constraint set in the same rewrite (CLM-0704; inference; high; S2). This mirrors the Agda model's paired `preprocess` and `synth` functions and makes the faithfulness property a checkable statement inside K: after a run, every emitted constraint must evaluate to true under the witness store (CLM-0704). Splitting witness and constraint semantics into two definitions was rejected because the divergence review shows that the interesting bugs are exactly the places where the two disagree, and a single configuration makes each disagreement a visible, testable difference between two cells of the same state (CLM-0705; [zkir-v3-divergence-review.md](zkir/zkir-v3-divergence-review.md); inference; high; S2). A mode cell that switches rules on and off was rejected because it multiplies rules without adding information; constraint emission is cheap data and can always be on, and "check mode" is a function over the final state rather than a third semantics (CLM-0705).

One refinement follows from the advisory consult recorded in `raw/notes/zkir-k-plan-advisory-2026-09-03.md`: constraint emission must not depend on the witness computation succeeding. A witness error (failed assertion, transcript mismatch, undefined register) sets the `status` cell and freezes the witness cells, but the synthesis half of every remaining rule still fires, so the constraint set at the end of a run is always the constraint set of the whole circuit, exactly as the real synthesiser produces it. This is what makes the review's findings 2 and 3 testable: a circuit whose preprocess halts but whose constraints all pass, or an inactive guard whose registers may take arbitrary values, both show up as a disagreement between `status` and the constraint check rather than being masked by an early stop (CLM-0720; raw/notes/zkir-k-plan-advisory-2026-09-03.md and [zkir-v3-divergence-review.md](zkir/zkir-v3-divergence-review.md); inference; high; S2).

The configuration, in K syntax, is planned as follows (CLM-0706; inference; medium; S2):

```k
configuration
  <zkir>
    <k> $PGM:Program </k>
    <mem> .Map </mem>              // Identifier |-> Value, write-once
    <pi> .List </pi>               // public-input statement vector
    <skips> .List </skips>         // impact skip markers
    <cursors> <pubIn> 0 </pubIn> <pubOut> 0 </pubOut> <priv> 0 </priv> </cursors>
    <outputs> .List </outputs>
    <preimage> ... </preimage>     // inputs and transcripts, read-only
    <constraints> .List </constraints>   // emitted algebraic constraints
    <status> ok </status>          // ok | error(String)
  </zkir>
```

The seven-tuple state that the VM page derives from `ir_vm.rs` maps one-to-one onto the cells above, so every transition rule already written informally on that page becomes one K rule (CLM-0706; [zkir-vm-semantics.md](zkir/zkir-vm-semantics.md)).

## Decision: which surface to target

The first definition targets the 34-instruction, 13-type surface at `92e8bdd3`, because that is the only surface with a machine-checked specification to compare against and the textual spec is written for it (CLM-0707; inference; high; S2). The eight additional instructions and the `Bool`, `Byte` and `Bytes(n)` types of `2ffe2d17` go into a separate module, `ZKIR-EXT`, imported only by a second main module, so that a program using them is rejected by the spec-aligned definition and accepted by the extended one (CLM-0707). Every rule carries the commit it was derived from in a comment; the ledger-8 versus ledger-9 pin conflict is recorded in [contradictions.md](contradictions.md) and does not need resolving to start (CLM-0708; inference; high; S2).

## Decision: sorts and the field

Field elements are represented as K `Int` values kept in canonical range by `modInt` with the BLS12-381 scalar modulus as a constant, because `Int` is arbitrary precision, `modInt` is Euclidean so results are never negative, and `^%Int` gives modular exponentiation for inversion by Fermat's theorem (CLM-0709; [k-builtins.md](k-framework/k-builtins.md); source fact and inference; not reproduced; high; S6). `MInt{N}` was rejected because the field modulus is not a power of two and fixed-width wraparound would be wrong (CLM-0709). Curve points are constructors over field elements with an explicit identity flag for the Weierstrass curves and affine `(0, 1)` identity for the Edwards curves, following the type page (CLM-0710; [zkir-type-system.md](zkir/zkir-type-system.md); inference; medium; S2). Bytes32 values use the K `Bytes` sort with explicit endianness in the conversion rules (CLM-0710). Registers are a `Map` from `Identifier` to `Value`; a rule that writes an identifier already present in the map is an error, which encodes single assignment (CLM-0711; inference; high; S2).

## Decision: how programs enter K

ZKIR programs are JSON documents. They enter K through a small pyk preprocessor that reads the JSON artifact and emits the corresponding `Program` term as KAST, so that the `ZKIR-SYNTAX` module holds only the abstract sorts and their well-formedness checks (CLM-0712; [k-backends-and-tools.md](k-framework/k-backends-and-tools.md) and [zkir-instruction-set.md](zkir/zkir-instruction-set.md); inference; high; S2). Parsing the JSON with K's builtin `JSON` module and translating it with rewrite rules was the first choice, because it would keep the serialisation format inside the semantics, but the advisory consult judged rewriting over large JSON trees slow and fragile, and the JSON shape is already documented field by field on the instruction page; the in-K route stays available as a later optional module (CLM-0712; raw/notes/zkir-k-plan-advisory-2026-09-03.md). pyk also drives `krun` over the precompile corpus and compares outputs for the test harness (CLM-0712).

## Decision: toolchain and compatibility

The definition is written for canonical K v7.1.337 at commit `4a46d123` and executed with the LLVM backend for concrete runs and the Haskell backend for claims (CLM-0713; [k-framework-overview.md](k-framework/k-framework-overview.md); inference; high; S6). That is the same K commit that Midnight's `k-rust` validates against, so compatibility with `k-rust` is a stated goal but not a day-one gate: the definition avoids features that `k-rust` documents as unsupported where the cost is zero, and a `k-rust` run is a later milestone (CLM-0714; [midnight-k-tooling.md](zkir/midnight-k-tooling.md); inference; medium; S4). K is not installed on this machine yet; the install route is `kup`, which requires Nix on WSL2 (CLM-0715; [k-framework-overview.md](k-framework/k-framework-overview.md); source fact; not reproduced; high; S6).

## Evidence that the semantics is right

Three oracles, in increasing strength (CLM-0716; inference; high; S2):

1. Differential testing against the Rust `preprocess`: for every program in `zkir-precompiles/` and every fixture in `zkir/tests/`, the K run must produce the same register store, public-input vector and skip vector as the crate, and the same error on the crate's error inputs. The crate is the ground truth the spec itself defers to.
2. Constraint satisfaction: after each successful K run, every entry of the `constraints` cell must hold under the `mem` cell. Failures are candidates for the divergence list, and the 13 known divergences are the first negative tests.
3. Correspondence with the Agda model: the Agda `preprocess` and `synth` functions can be run on the same programs once the Nix toolchain in arc-zkir is installed, giving a third independent trace. This is the only oracle that speaks to the theorems, and it has not been run.

The E00 experiment already holds ZKIR 3 artifacts compiled from the Moriarty escrow and swap, which become the first application-level test programs once their instruction surface is confirmed to be within the 34 (CLM-0717; [open-questions.md](open-questions.md); repository observation; reproduced; medium; S3).

## Milestones

1. Install K v7.1.337 with `kup` on WSL2; run the tutorial's first lesson to confirm the toolchain. Done 2026-09-05: `kup install k --version v7.1.337` resolved to the pinned commit 4a46d123 from the K binary cache in under three minutes; lesson 1.2 compiles and runs under both the LLVM and the Haskell backend, and pyk 7.1.337 (PyPI `kframework`, uv dependency group `zkir-k`) parses, converts KAST to KORE and back, and runs a program against the compiled definition. Rerunnable as `experiments/zkir-k/toolchain-check/check_k_toolchain.sh` (CLM-0723; evidence/k-toolchain-install-2026-09-05.md; executed test; reproduced; high; S1). Two facts for milestone 2: pyk reads `compiled.json`, so every ZKIR definition must be compiled with `--emit-json`; and kup does not put pyk on the path, so the Python side runs through `uv run --group zkir-k`.
2. Module `ZKIR-SYNTAX`: abstract sorts and well-formedness checks, plus the pyk JSON-to-KAST preprocessor (single assignment, declared types, input count). Test: every precompile parses.
3. Module `ZKIR-FIELD` and `ZKIR-TYPES`: field arithmetic, curve constructors, Bytes32 conversions with unit claims for inverse and encode/decode round trips.
4. Module `ZKIR-VM`: the 34 instruction rules, each updating `mem`, `pi`, `skips`, cursors and `constraints`. Test: oracle 1 over the precompile corpus.
5. Constraint checker and the 13 divergence tests (oracle 2).
6. `ZKIR-EXT` module for the `2ffe2d17` surface; `k-rust` compatibility run; Agda correspondence run (oracle 3).

## Risks

The largest risk is that the crate and the spec disagree on an instruction and the K definition silently follows one of them; the mitigation is oracle 2 plus the divergence list as explicit negative tests (CLM-0718; inference; high; S2). The second risk is chip gating: non-native operations are only constrained when their chip is initialised, so a K constraint set that always emits constraints will be stricter than the real circuit in exactly the cases the divergence review calls unconstrained; those cases must be modelled as an explicit `used_chips` cell rather than papered over (CLM-0719; [zkir-v3-divergence-review.md](zkir/zkir-v3-divergence-review.md); inference; high; S2).

Two further risks come from the advisory consult (CLM-0721; raw/notes/zkir-k-plan-advisory-2026-09-03.md; recommendation; not reproduced; high; S2). First, symbolic execution with the Haskell backend will not decide non-linear modular field arithmetic or hash functions; claims over the field need `simplification` lemmas, and hashes must stay uninterpreted function symbols in proofs. Second, oracle 2 only establishes completeness, that the honest witness satisfies the emitted constraints; it cannot show that no other witness does, so the K definition must not be described as proving soundness or the absence of underconstrained circuits. Soundness remains the arc-zkir theorems' territory, and the K work feeds them test programs, not proofs.

## Provenance of the decisions

The five decisions above were reviewed on 2026-09-03 by an advisory consult that agreed with the surface pin, the field representation and the compatibility stance, and disagreed with coupled constraint emission and in-K JSON parsing; both disagreements were accepted and are reflected in the text. The consult ran on `agy` with `gemini-3.8-flash-high` because the configured Claude advisor failed with API overload errors; the verbatim verdict is preserved in `raw/notes/zkir-k-plan-advisory-2026-09-03.md` (CLM-0722; raw/notes/zkir-k-plan-advisory-2026-09-03.md; repository observation; reproduced; high; S2).
