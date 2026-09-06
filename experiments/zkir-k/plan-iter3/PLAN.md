# ZKIR K semantics, third iteration

## Purpose

Make the definition usable as the compilation target in a formal specification of Moriarty, which compiles to ZKIR. Three things are missing today: a circuit-side semantics whose verdicts are checked against the real circuit; a backend on which properties are proved rather than tested; and an explicit target contract, the obligations a compiler must meet for its output to be well-formed, provable and semantically defined, stated once as K functions and quoted by the Moriarty specification.

## Where the definition stands

Off-circuit (witness) semantics of both pinned surfaces, complete and differential-tested against the crate's `preprocess`: 358/358 comparisons at midnight-ledger 92e8bdd3 and 418/418 at midnight-zkir 2ffe2d1, on status, error class, register types and encodings, public inputs and skips. Static well-formedness complete. In-circuit semantics for every instruction as an instruction-level relation with chip gating and width limits, checked only for internal consistency and by twenty divergence cases; the oracle never runs the crate's `circuit`. Not modelled: auxiliary witness cells, copy wiring, the per-type assignment constraints, JubjubScalar canonicity, hash gadget internals, prover-side panics. Symbolic reasoning: one Haskell-backend proof of an `add` claim was recorded in the second iteration but its claim files are not in the repository; hash claims stall on unfolding. Of the application artifacts only swap `expire` reaches a successful witness. Divergences K1 to K6 recorded, none reported upstream. The compiled definitions are tracked in git by accident.

## The circuit-side gap: three approaches

**A. Full PLONKish model.** Model the Halo2 constraint system midnight-circuits builds: rows, columns, gates, copy constraints, lookups, per chip. A second implementation of midnight-circuits in K, months of work, brittle under every chip change, and still not a proof about the proving system. Rejected for this iteration.

**B. Relational model with a circuit oracle. Chosen.** Keep instruction-level relations, make the modelled witness space explicit, and check verdicts against the real circuit through the halo2 MockProver, which needs no proving keys or structured reference string: `MidnightCircuit::new(&ir, Value::known(pis), Value::known(preprocessed), k)` followed by `MockProver::run(...).verify()` (the stdlib wrapper also configures `used_chips` and loads lookup tables, which calling `Relation::circuit` alone would skip). This makes the verdict layer differentially tested at the granularity the crate exposes, whole-circuit accept or reject on a preimage, and lets the soundness statement be made relative to the modelled relations. Its abstraction assumptions stay explicit in the statement.

**C. Status quo.** Completeness-only verdicts, soundness delegated to the crate. Leaves the Moriarty specification unable to say what the circuit accepts. Rejected.

## What the crate allows, checked before planning

- The base crate resolves midnight-proofs 0.7.3 and midnight-zk-stdlib 2.3.3; the extension crate resolves 0.8.2. One oracle build per version; the 0.8 `MockProver::run` sizes the circuit itself, the 0.7 path takes `k`.
- `Preprocessed` is a public struct and `IrSource::prove_unchecked` exists for malicious-prover tests, so a witness can be injected. But `IrValue` is typed (`JubjubPoint(JubjubSubgroup)`, `JubjubScalar(JubjubFr)`, `Bytes32([u8; 32])`): off-curve, out-of-subgroup, non-canonical and out-of-range values cannot be represented at that interface. Only `Native` values, the public-input vector, the binding input and the commitment can be perturbed there. Synthesis recomputes arithmetic and hash outputs and checks them against memory through `mem_insert`, so an injected register that disagrees surfaces as a synthesis error, not a constraint failure.
- `optimal_k` searches 9 to 25; the ledger circuits may need 2^20 rows or more.
- Real keygen synthesises with unknown witnesses and needs KZG parameters of the right size; the crate's tests use a `TestParams` provider reading `MIDNIGHT_PP/bls_midnight_2p{k}`.
- The harness already seeds every corpus directory's literal `test_preimage`, including the seventeen extension ones; there is no unused quick win there.

## Milestones

**M0. Housekeeping.** Untrack the four `*-kompiled` directories and ignore them. A `check` target that kompiles the four definitions and runs the six check layers in order, writing dated receipts. A drift script that diffs the pinned commits' instruction and type enums against the current upstream heads. Draft issue texts for K1 to K6 from the contradictions rows; filing is a separate decision. Half a day; first.

**M1a. Feasibility spike.** Build the MockProver oracle for the largest base-corpus circuit and the largest extension circuit; record k, wall time and peak memory. This decides M1's exit criterion and whether the corpus runs whole or sampled. One day; before anything else on the circuit track.

**M5a. Contract shape.** Before the circuit work settles, fix the shape of the target contract in three tiers, because the rest of the iteration produces evidence for each tier separately: program-only obligations (well-formedness, the chip set implied by `used_chips`, width limits, in-circuit alignment restrictions, the commitment flag, the version pin); preimage-dependent conditions (the off-circuit run succeeds); and circuit acceptance (the oracle outcome). A K function `targetContract(P)` for the first tier and a JSON export of the same. One to two days; can overlap M1a.

**M1. Circuit oracle.** `zkir-circuit-oracle` in both `_build` workspaces: load program and preimage, run `preprocess`, build `MidnightCircuit` with known instance and witness, run `MockProver`, and report one of six outcomes: preprocessing rejection, witness-consistency rejection (from `mem_insert` or `pi_push`), synthesis failure, constraint failure with the failing constraints, panic, or accepted. A `--circuit` mode of `diff_test.py` comparing K's verdict summary with the oracle outcome on the honest preimage and the four perturbations, and injected Native-level perturbations of the preprocessed memory through `prove_unchecked`. No forced one-to-one mapping between K outcomes and oracle outcomes: the comparison table is written down first, and each cell is either expected agreement or a named, accepted difference. Exit: every base-corpus and divergence-case comparison lands in an expected cell; a comparison outside the table is a blocking finding to fix, not a note. Three to five days depending on M1a.

**M2. The modelled witness space and the soundness statement.** Define `witnessSpace(P, pi)` as a predicate over memories: every emitted relation holds, and every register carries a value of its declared type. State explicitly that typing does the work of the assignment constraints for points, scalars and byte strings, because the crate's typed values cannot be malformed, and record JubjubScalar canonicity with a new explicit outcome `unconstrained` so the gap is visible in verdicts rather than silent. State the projection from circuit cells to registers and the existential treatment of auxiliary cells. The soundness claim reads: any memory in the modelled witness space is accepted by the circuit up to the listed unmodelled items; tested by M1 on the Native-level injected perturbations and the divergence cases. Update the definition page and chapter 08. Two to three days; after M1.

**M3. Symbolic backend.** A `zkir-symbolic.k` main module importing only what the claims need: defining rules of the hashes and curve arithmetic marked `[concrete]`, uninterpreted function symbols for the hashes with functionality as the only axiom, simplification lemmas for `modInt` over `#r`. Recover the recorded `add` claim first as the baseline. Then per-instruction lemmas for the native arithmetic and control instructions, one lemma over a three-instruction program with a hash, and the template of a compiler correctness obligation. Exit: `kprove` receipts with times for every claim, including the `transient_hash` claim that failed before. Independent of M1 and M2; runs in parallel; never on the critical path.

**M4. Real transaction contexts.** Produce preimages for the seven Moriarty artifacts by driving the compiled contract runtime (`output/contract/index.js` with the Compact runtime) or the ledger's proof-preimage construction, so each reaches a successful witness on both sides and through the circuit oracle. Fallback if the runtime path fails within two days: hand-built contexts for two of the seven. Exit: successful-run agreements and circuit outcomes recorded for every artifact reached. After M1.

**M5b. The contract completed.** Add the second and third tiers to the contract output, define the observable semantics `[[P]](pre)` = (status, outputs, public inputs) as one K function over the configuration, and prove provability rather than assume it: for the corpus, unknown-witness keygen with `TestParams`, and prove-and-verify receipts for a representative sample. Chapter 16 describes the contract; the Moriarty artifacts become first-class corpus entries with expected contract results. After M2 and M4.

**M6. Audit and documentation.** The eight-reviewer cross-vendor audit on the changed modules, a fix iteration, updates to chapters 08, 12, 14, 15 and the new 16, the definition and plan pages, and receipts with a new suffix.

## Order

M0, then M1a and M5a in parallel; M1; M2; M4; M5b; M6. M3 runs alongside from M1 onward. Critical path: M1a, M1, M2, M5b, M6.

## The compiler-correctness statement

The statement Moriarty's specification will make has two directions, and the plan names them now so M5 builds the right interfaces. Completeness: an honest run of the source semantics yields a preimage the compiled program accepts off-circuit, the circuit accepts, and whose public inputs are the ones the source semantics predicts. This is what the oracle tests. Soundness: every memory in the modelled witness space corresponds to a source execution. It quantifies over the modelled space, which over-approximates the set the real circuit accepts whenever a constraint is unmodelled, so it is the safe direction, and the unmodelled list is a stated residual of the theorem, not a footnote. Differential tests discharge neither direction universally; they are the evidence that the model is not wrong on the corpus, and the proofs of M3 are where universal obligations begin.

## Risks

MockProver cost on the ledger circuits: M1a decides sampling before M1 commits. Two midnight-proofs versions: two oracle builds, pinned in the workspaces. Injected witnesses cannot express malformed typed values: accepted, and turned into the typing clause of M2 rather than an exit criterion. Haskell backend performance: a reduced symbolic module, off the critical path. Moriarty preimages need the Compact runtime and a transaction context: a two-day timebox with a hand-built fallback. Upstream drift since the pinned commits: the M0 script decides whether to re-pin before M1.

## Process

Every milestone lands with a receipt under `evidence/`, a claim record on the wiki, and a differential comparison where an oracle exists. Implementation by the routine lanes, cross-vendor audit before merge, commits on a branch from main merged with `--no-ff`, source identifiers allocated from the inventory maximum on main.
