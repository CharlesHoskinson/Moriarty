# R4 review: the symbolic backend, the claims and the tooling

Reviewer: Claude Fable 5.1. Repository: /home/charl/Moriarty/.worktrees/zkir-k-iter3.
Scope as in briefs/R4-symbolic-and-tooling.md. Prior-audit items (review-2026-09-05/CONSOLIDATED.md) are not re-reported; item 34 (proof readiness) is superseded by this iteration's claims and is discussed below only where the new treatment falls short.

VERDICT: WARNING — the twelve claims are sound and state what the README says, but the uninterpreted-hash treatment breaks its own functionality argument on the communications commitment (the one gate every Moriarty artifact needs), and the claims runner and Makefile can report success where nothing was proved or where a layer failed.

## Findings

### 1. major — `transientCommit` and `poseidonHash` are two different uninterpreted symbols for the same hash, so no claim with a communications commitment can pin its verdicts

Files: experiments/zkir-k/semantics/zkir-hash.k:80-81; experiments/zkir-k/semantics/zkir-vm.k:581-584 (`#commCheck`); experiments/zkir-k/semantics/zkir-constraints.k:130-134 (`#commOutcome`).

What the K code says. `transientCommit(Vs, Rand) => poseidonHash(ListItem(Rand) Vs) [concrete]` (zkir-hash.k:81). The witness side checks the commitment with `Cm ==Int transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)` (zkir-vm.k:582), the constraint side evaluates `commGate` with `poseidonHash(ListItem(Rand) Enc #encodeAll(Os)) ==Int C` (zkir-constraints.k:132). Under the Haskell backend with a symbolic input, `transientCommit(ListItem(A), R)` never unfolds (its only rule is `[concrete]`), so the two sides talk about two syntactically different terms, `transientCommit(ListItem(A), 5)` and `poseidonHash(ListItem(5) ListItem(A))`, that no axiom relates. The README's justification for the treatment, "two applications to the same argument are the same term, which is what makes a register holding the hash agree with the gate that recomputes it" (claims/README.md:35-37), does not hold for the commitment.

Source of truth. In the crate both are one function: `transient_commit(values, rand)` is `transient_hash([rand] ++ values)` (transient-crypto), and `Relation::circuit` recomputes the same Poseidon over rand, the re-encoded inputs and outputs. The K definition itself says so at zkir-hash.k:79.

Why it matters. Every one of the seven Moriarty artifacts has `do_communications_commitment: true` (experiments/moriarty-core-swap/output/zkir/*.zkir, experiments/moriarty-compact-escrow/output/zkir/*.zkir, checked with a script; see section 5). A claim in the style of add-spec.k (verdicts and `<witnessSpace>` pinned) on any of them cannot be proved. Reproduced: the scratch claim `comm-sym.k` (program `copy %a -> %o`, `do_communications_commitment` true, preimage `comm(C, 5)` with `requires C ==Int transientCommit(ListItem(A), 5)`, verdicts pinned to `holds()`) ends with the residual goal

    #commOutcome ( lcOk ( ListItem ( A:Int ) ) , 5 , .List , transientCommit ( ListItem ( A:Int ) , 5 ) , SetItem ( "poseidon" ) )

unevaluated in `<verdicts>` (scratch log comm-sym.log, exit 1). The observable-only form of the same claim (`comm-sym-obs.k`, verdicts existential, `<observable> noObs() => obs(ok(), .List, ListItem(42) ListItem(C))`) proves in 7 s, because the witness side only ever sees `transientCommit`. So the compiler-obligation template survives, the witness-space claims do not.

Fix. Drop `[concrete]` from the `transientCommit` rule (zkir-hash.k:81): it is a definitional alias, and on a symbolic argument it then unfolds once to `poseidonHash(ListItem(Rand) Vs)`, which is where the `[concrete]` barrier belongs; a concrete argument still evaluates fully. Alternatively make `#commCheck` (zkir-vm.k:582-584) call `poseidonHash(ListItem(Rand) ...)` directly. Add a claim with `do_communications_commitment` true and verdicts pinned to the claim set, so the property is exercised. Update claims/README.md:30-40 and the zkir-symbolic.k header (lines 9-16), which list `transientCommit` among the uninterpreted symbols as if that were harmless.

### 2. minor — the claims runner labels every residual goal a "kprove error"

File: experiments/zkir-k/tools/run_claims.py:138-145.

What the code says. `if '[Error]' in out: return 'not proved (kprove error: ...)'` is tested before the residual-goal branch (line 145). K v7.1.337's `kprove` ends every unproved claim with `[Error] Prover: backend terminated because the configuration cannot be rewritten further. See output for more details.` (scratch logs false-add.log:281, ctb-both.log:266, comm-sym.log:270, hash-inj.log:254), so the "residual goal" branch is unreachable and a false claim is reported as a tool failure. The receipt (evidence/zkir-k-claims-2026-09-06c.txt) is unaffected because every row is `proved`; the vocabulary in the docstring (lines 12-15) and README.md:14-17 is wrong for the common case.

Fix. Match the specific prover message (or the presence of `#Not`/a printed configuration) as "residual goal", and reserve "kprove error" for a non-zero exit with no configuration printed (parse error, missing module).

### 3. minor — `--only` with an unknown name reports `0/0 claims proved` and exits 0

File: experiments/zkir-k/tools/run_claims.py:161-174; the same pattern in experiments/zkir-k/tools/moriarty_contexts.py:136 (`names = [n for n in manifest['artifacts'] if args.only in n]`, a substring test with no empty-set check; an unmatched `--only` prints `all rows as expected` and exits 0, line 151-152).

Reproduced: `uv run --group zkir-k python experiments/zkir-k/tools/run_claims.py --only nonexistent` prints the header, then `0/0 claims proved`, exit 0. A typo in a CI invocation is a green run. Fix: reject names not in `CLAIM_LIST` (respectively not in the manifest) with exit 2, and return 1 when `total == 0`.

### 4. minor — no vacuity check: a claim with an unsatisfiable precondition is "proved"

File: experiments/zkir-k/tools/run_claims.py:138-139; claims/README.md:23 ("`#Top` on standard output is a proof").

Reproduced: `vacuous-add.k`, add-spec.k with the wrong output `(A +Int B +Int 1) modInt #r` and `requires 0 <=Int A andBool A <Int 0`, prints `#Top`, exit 0, 6 s. The same claim with the satisfiable precondition (`false-add.k`) is refuted in 8 s. The twelve committed claims have satisfiable preconditions (`0 <= X < #r`, `C ==Int 1`, `C ==Int 0`, `2 <= C < #r`), so nothing in the receipt is vacuous today; the runner has no way to notice when a future edit makes one so. Fix: for each module also run a companion claim with the same LHS and `requires` whose RHS is unreachable (for instance `<status> ok() => error("vacuity probe")`) and require it to fail, or check the `requires` with Z3 through pyk before proving.

### 5. minor — `make` exit status never reflects a failing layer

File: experiments/zkir-k/Makefile:27, 30, 33-34, 40-43, 46-47.

Every check runs as `$(UV) tool | tee receipt`; make uses /bin/sh without `pipefail`, so the recipe's status is `tee`'s. Reproduced with a two-line Makefile (`false | tee /dev/null` → `make exit 0`). `make check`, `make contexts`, `make circuit` and `make contract` therefore succeed when the tool prints `N failing rows` or a differential mismatch; the receipt text is still honest, the exit code is not. Fix: `SHELL := /bin/bash` and `.SHELLFLAGS := -o pipefail -ec`, or `set -o pipefail;` in each recipe. Related: there is no `claims` or `drift` target, so the two receipts of this brief (zkir-k-claims-*.txt, zkir-k-upstream-drift-*.txt) were assembled by hand (the claims receipt's first four header lines are not printed by the tool) and cannot be regenerated by `make`.

### 6. minor — the `constrain_to_boolean-ok` and `assert-ok` claims are concrete runs, and the natural symbolic form does not prove

Files: experiments/zkir-k/claims/native-ops-spec.k:175 (`requires C ==Int 1`), :232 (`requires C ==Int 1`); README.md:79-80, 84.

What the claims say. Both fix the input to 1, so they quantify over nothing; a `krun` establishes the same. README.md:64 says "All claims range over inputs A, B, C with 0 <= X < #r ... unless stated otherwise" and the two lines do state otherwise, so the README is accurate, but the claim set does not contain the property one would expect here: constrain_to_boolean passes on both boolean values.

Reproduced: `ctb-both.k`, the OK claim with `requires C ==Int 0 orBool C ==Int 1`, is not proved; the residual goal leaves `#boolCheck(asBool(native(C)))` and `#boolean(vOk(native(C)), "constrain_to_boolean")` unevaluated (ctb-both.log:58-89), because `asBool` (zkir-ops.k:351-354) has three rules whose conditions the backend cannot decide under a disjunction without a case split.

Fix. Either two claims (C = 0 and C = 1) per instruction, or a `[simplification]`-free approach: state the OK claim once with `C ==Int 0` and once with `C ==Int 1` in the same module, or add a claim-local case split (`requires C ==Int 0` and `requires C ==Int 1` as two `claim`s under one module name, which kprove proves together). Also add the missing error branch: `assert` on an input in `[2, r)` fails with `Expected boolean, found: C` (asBool, zkir-ops.k:353), a third status the claims never state.

### 7. minor — stale statements about the observable triple and the hash treatment

Files: experiments/zkir-k/claims/README.md:52-56; experiments/zkir-k/semantics/zkir-symbolic.k:30-35; wiki/zkir/zkir-k-definition.md:76.

README.md:52-56 and the zkir-symbolic.k header say the observable result "is read off the `<status>`, `<outputs>` and `<pi>` cells" and that "the triple cannot be packaged into one term without changing `ZKIR-VM`". Since M5b the configuration has `<observable>` and `obs(Status, List, List)` (zkir-vm.k:88, 551-556) and the template pins exactly that cell (run_claims.py:96); the two texts contradict the code they sit next to. wiki/zkir/zkir-k-definition.md:76 (CLM-0746) still records that "the plan's uninterpreted-hash treatment for proofs is not implemented"; no later claim on that page records the `[concrete]` treatment, the lemmas or the receipt. Fix: rewrite the two paragraphs around `<observable>`, and add the M3 claim record to the definition page with the 2026-09-06c receipt.

### 8. minor — the negative controls of `moriarty_contexts.py` never reach the circuit, but the tool says they are rejected "on all three sides"

File: experiments/zkir-k/tools/moriarty_contexts.py:9-15, 21, 120-121.

What the code says. A negative row passes when `outcome != 'accepted'` (line 120). For every negative row in evidence/zkir-k-moriarty-contexts-2026-09-06.txt the circuit column is `preprocess-error` with an empty `k` and `0` ms: the oracle stopped in `preprocess` and `MockProver` never ran. That is inherent (the witness is computed by `preprocess`, a preimage cannot violate a constraint without failing there first), so the columns are honest, but the docstring's "which every side must reject" and the exit criterion's "rejected on all three sides" describe the circuit as having rejected something it never saw. The only circuit evidence on the contexts is the honest acceptance and, elsewhere, the `--inject` mode of diff_test.py. Fix: word the docstring and the receipt summary as "rejected by preprocess on both sides; the circuit is not reached", and require `outcome == 'preprocess-error'` explicitly for a negative row so that a future `load-error` (line 63-64, returned for a crashing or missing binary) cannot count as a rejection.

Verified while here: the `wrong-phase` construction does what the docstring says. On swap-fundAlice the honest first public read is `0`, the K generation run on the perturbed preimage regenerates nine public transcript inputs whose ninth is `1` instead of the honest `0` (the impact that re-verifies the phase read precedes the assertion), and the checked run then fails with `Failed direct assertion`; without the regeneration it fails earlier with `Public transcript input mismatch for input 8`. The regenerated input list is truncated to the impacts before the failing assertion (9 of 108), so "only the ledger value is wrong" (line 12) is not literally true; it does not matter because the assertion fires first, but the sentence should say so.

### 9. minor — `upstream_drift.py` is blind to changes that keep the enums and versions

File: experiments/zkir-k/tools/upstream_drift.py:2-6, 316-339.

The comparison covers the serde names of `Instruction` and `IrType`, the field names of struct variants, and three dependency versions. Between the pin and `origin/ledger-9` (0d364eb9) there are 43 commits; `git diff --stat 92e8bdd3..origin/ledger-9 -- zkir-v3/ transient-crypto/ onchain-runtime/` shows zkir-v3 untouched and transient-crypto/src/proofs.rs changed (the `VerifierKey` serialization; `ProofPreimage` unchanged), so "no drift" is correct for the crate today. A change to a runtime check or an error message in `ir_vm.rs` or `ir_instructions/*.rs`, the things the differential harness compares, would not be reported. Fix: print, per counted comparison, `git diff --stat <pin> <head> -- <crate dir>` (and for the ledger also `transient-crypto/src/proofs.rs`, on which preimage-json depends), and count a non-empty diff under `src/ir_vm.rs` or `src/ir_instructions/` as drift.

### 10. nit — manifest wording on the commitment randomness

File: experiments/zkir-k/tools/moriarty_preimages.mjs:62-63; corpus/moriarty-contexts/manifest.json `fixed_fields`.

"the ledger overwrites them at transaction assembly" is right for `binding_input` (ledger/src/prove.rs:370 passes `Some(intermediate_call.binding_input(binding_commitment))` to the prover) and not for the randomness: the ledger takes `communication_commitment_rand` and the matching commitment from the DApp's `ContractCallPrototype` as given (ledger/src/construct.rs:565-568); it is the caller, not the ledger, that samples it. Say "the DApp supplies a fresh randomness and commitment in the call prototype; the ledger overwrites the binding input when proving".

### 11. nit — the eleven pinned claims leave `<observable>` existential

Files: claims/add-spec.k:31, native-ops-spec.k (every module), transient-hash-spec.k:44.

`<observable> noObs() => ?_` was reasonable before M5b; now the cell is the statement the contract quotes, and pinning it (`obs(ok(), .List, ListItem(42))`, or the error variant) costs nothing and ties the eleven claims to the template's vocabulary.

## Answers to the brief's questions

1. The twelve claims. Each states what README.md:62-104 says. No claim is vacuous (preconditions satisfiable, section 4) and none leaves a cell unconstrained in a way that empties the statement: the eleven pinned claims fix every cell but `<observable>`; the template instance fixes `<observable>` and leaves the rest existential by design, which is a real statement because `obs(...)` carries the status. Claims 7 and 9 (constrain_to_boolean-ok, assert-ok) are concrete runs dressed as claims (finding 6). The six `[simplification]` lemmas (zkir-symbolic.k:38-51) are sound for `P > 0` under K's Euclidean `modInt` (domains.md:1262-1264: the remainder is always non-negative): idempotence, the four `+Int`/`*Int` congruences and the negation congruence all hold because both sides are the canonical representative of the same residue class. `preserves-definedness` is justified by the `requires P >Int 0` guard (the only partial symbol on the right is `modInt`, defined for `P =/= 0`); no rule lacks it. No subtraction congruence is provided; nothing in ZKIR-VM builds `(X modInt P -Int Y) modInt P` on the witness side (`neg` goes through `fneg`, zkir-field.k:49), so nothing is missing for the claim set.

2. `[concrete]`. The attribute is on every rule of `poseidonHash`, `transientCommit`, `varLenSponge`, `hashToCurve`, `sha256Bytes`, `keccak256Bytes` (one rule each, zkir-hash.k:73, 81, 86, 91, 168-170, 238) and both rules of `ecMul` (zkir-curves.k:126-127). The helpers (`absorbAll`, `permute`, `#htc`, `mapToCurve`, `#ecMulAcc`, `#shaBlocks`, `#kAbsorb`) are reachable only through those rules, so nothing unfolds on a symbolic argument; a concrete argument still evaluates (the LLVM receipts and the `add` claim through `run_claims.py --only add`, 7.3 s, confirm both backends). What the prover knows of `poseidonHash(t)` on a symbolic `t`: it is an `Int` (sort), it is defined (`total`), and it is syntactically itself. No range, injectivity or collision fact is derivable: the scratch claim `hash-inj.k` (`poseidonHash(ListItem(A)) =/=Int poseidonHash(ListItem(B))` under `A =/=Int B`) is refuted with the residual `poseidonHash(ListItem(A)) #Equals poseidonHash(ListItem(B))` (hash-inj.log:79-83). "Functionality is the only property" is therefore true as stated, with two qualifications: totality is also assumed (correct, the functions are total), and functionality is only syntactic, which is why finding 1 bites: the two spellings of the commitment hash are not identified. The same risk exists for `persistent_hash`/`keccak256`, where the witness side hashes `alignedBytes(Al, L)` (zkir-vm.k:428) and the gate hashes `alignedBytesCircuit(Al, L)` (zkir-constraints.k:542); on symbolic natives the two byte builders must reduce to the identical `Bytes` term for `#matches` to hold, and no claim exercises it (see coverage).

3. The template `spec_compiled_observable`. Its shape (a program term, a preimage with symbolic `Int`s, an expected `obs(status, outputs, pis)`, everything else existential) is the right statement of the completeness direction's first two conjuncts (docs/16:53). What a compiler proof needs that it does not give: (a) the third conjunct, circuit acceptance; the modelled proxy is `<witnessSpace> true` with no `violated`/`synthErr` verdict, and the template leaves both existential, so an instance says nothing about the constraint side; offer a `witness_space: bool` parameter that pins `<witnessSpace> false => true` and `<unconstrainedRegs>` (after finding 1 is fixed, or the commitment gate blocks it); (b) tier one: the template uses `job` (the VM's own domain, every program `IrSource::load` accepts) and never evaluates `wf`; the Moriarty specification must discharge `targetContract(P)` separately, and the template's docstring should say so; (c) a symbolic binding input: the instance pins `7` in both the preimage and `pis`; the ledger overwrites it with a hash at proving time (prove.rs:370), so the obligation should read `preimage(..., B:Int, ...)` with `pis = ListItem(B) ...` and `requires 0 <=Int B andBool B <Int #r`; the comm-sym-obs probe shows the same works for a symbolic commitment `C` with `requires C ==Int transientCommit(...)`; (d) the extension surface: ZKIR-SYMBOLIC imports ZKIR-VM only and the template fixes `<strictDecode> false`, so no obligation can be stated for midnight-zkir 2ffe2d1; (e) scale: the smallest Moriarty artifact is 49 instructions with 84 public transcript inputs, a `persistent_hash` over symbolic bytes and the commitment; the twelve claims are one to three instructions with no transcript. Nothing in the receipts shows the template proving anything of the shape the specification will instantiate; the persistent_hash and commitment cases are the two that most need a trial before the shape is frozen.

4. Moriarty preimages. `proofDataIntoSerializedPreimage` (onchain-runtime-wasm/src/primitives.rs:227-274 at 92e8bdd3, the same source the runtime's onchain-runtime-v4 4.0.0-rc.3 wraps) builds `inputs`, `private_transcript` (value-only field repr of the private transcript outputs), `public_transcript_inputs` (`field_repr` of every op in order), `public_transcript_outputs` (value-only repr of every `Popeq` result), `binding_input = 0`, and `communications_commitment = Some((transient_hash([0] ++ input ++ output), 0))`. The ledger's own construction (ledger/src/construct.rs:520-570) builds the same five vectors from `guaranteed ++ fallible` ops with `binding_input = 0` and the prototype's randomness, then overwrites the binding input when proving. So, up to the two residuals the manifest names (and the wording nit 10), the preimages are the ledger's; the seven honest rows agreeing on K, `preprocess` and `MockProver` (evidence/zkir-k-moriarty-contexts-2026-09-06.txt) are consistent with that. Scenario order is sound: `decide` and `expire` both start from the post-`fundBob` state, `release` and `refundAfterTimeout` from the post-`fund` state (moriarty_preimages.mjs:112-115, 131-133), each taken from `currentQueryContext.state` after the previous call; the private state is inert (the witnesses ignore it); `time` is the ninth positional parameter of `createCircuitContext` (circuit-context.d.ts:198), which the call passes correctly. `preimage-json` (tools/preimage-json/ledger-92e8bdd3/src/main.rs) deserialises with the crate's own `tagged_deserialize::<ProofPreimage>` and prints `BigUint::from_bytes_le(f.as_le_bytes())`, so it is faithful by construction as long as it is built in the 92e8bdd3 workspace, which the README requires. One thing I could not settle from the sources is in the questions below (checkpoints).

5. Misleading receipts. Findings 2, 3, 4, 5, 8 and 9. Nothing in the three receipts of this brief is wrong; the risks are in what the tools would print or return on the next failure.

## Coverage gaps

- No claim with `do_communications_commitment` true (blocked for verdict-pinned claims by finding 1); every Moriarty artifact needs one.
- No claim with `public_input`, `private_input`, a guarded `impact`, `test_eq`, `constrain_bits`, `persistent_hash`, `bytes32_into_low_high`, `less_than`, `cond_select` with a symbolic bit, or any non-native type; the Moriarty instruction mix (public_input, impact, test_eq, assert, private_input, constrain_bits, persistent_hash, bytes32_into_low_high, cond_select) is covered only for `assert`, immediate-bit `cond_select` and one unguarded `impact`.
- `persistent_hash`/`keccak256` on symbolic natives: whether `sha256Bytes(alignedBytes(Al, L))` and `sha256Bytes(alignedBytesCircuit(Al, L))` become the same term is untested.
- No claim on the ZKIR-EXT surface; `strictDecode` is pinned false everywhere.
- No claim with a symbolic binding input, symbolic transcript entries, or an error path other than the two decoder/assert messages; `assert`/`constrain_to_boolean` on `[2, r)` with `assert`, and `constrain_to_boolean` on 0, are not stated (finding 6).
- Vacuity of preconditions is never checked (finding 4).
- `moriarty_contexts.py`: no negative reaches the circuit (finding 8); `wrong-witness` does not exist for `expire` and `refundAfterTimeout` (no private transcript), as the receipt shows.
- `upstream_drift.py`: instruction bodies, `ir_vm.rs`, and `transient-crypto/src/proofs.rs` (the `ProofPreimage` layout preimage-json depends on) are outside the comparison (finding 9).
- Tooling reference: docs/11-tooling-reference.md documents none of `run_claims.py`, `upstream_drift.py`, `moriarty_contexts.py`, `moriarty_preimages.mjs`, `preimage-json` (only docs/16 mentions `run_claims.py`).

## Questions for the authors

1. Is `[concrete]` on `transientCommit` (zkir-hash.k:81) deliberate? If the intent was to keep `transient_commit` opaque as a distinct primitive, the commitment gate must be rewritten in terms of the same symbol; otherwise drop the attribute.
2. Do any of the seven compiled circuits emit checkpoint ops? The ledger splits the op list at checkpoints into guaranteed and fallible transcripts (ledger/src/construct.rs:1006 `partition_transcripts`) and rebuilds the preimage from `guaranteed ++ fallible` (construct.rs:531-540); the runtime serialises the unsplit list. If a `ckpt` op is dropped or re-encoded by the split, the ledger's `public_transcript_inputs` would differ from the corpus preimages while both still verify. I could not decide this from the sources within the review.
3. Has anyone instantiated `spec_compiled_observable` on a program with `persistent_hash` or with the commitment flag? The answer decides whether the template's shape can be frozen for the specification.
4. Should the completeness obligation pin `<witnessSpace> true`, or is circuit acceptance deliberately left to tier three (the oracle) with the claim covering only status and public inputs? docs/16:53 lists circuit acceptance as a conjunct of completeness, and the template cannot state it.

## What I checked and how

Read in full: briefs/COMMON.md, briefs/R4-symbolic-and-tooling.md, semantics/zkir-symbolic.k, zkir-hash.k, zkir-curves.k, zkir-field.k, zkir-vm.k, the gate-evaluation part of zkir-constraints.k (lines 100-146, 386-412, 505-545), zkir-ops.k:349-354, zkir-values.k:142-143 and 217-227, claims/README.md and the four claim files, tools/run_claims.py, moriarty_contexts.py, moriarty_preimages.mjs, upstream_drift.py, preimage-json (README, Cargo.toml, src/main.rs), diff_test.py:60-130 and 160-200, zkir_run.py (grep of the status logic), the Makefile, plan-iter3/PLAN.md, docs/16 and the grep hits in docs/08, docs/11, wiki/zkir/zkir-k-definition.md and wiki/zkir-k-semantics-plan.md, review-2026-09-05/CONSOLIDATED.md, evidence/zkir-k-claims-2026-09-06{,b,c}.txt, zkir-k-moriarty-contexts-2026-09-06.txt, zkir-k-upstream-drift-2026-09-06.txt, corpus/moriarty-contexts/manifest.json and swap-decide.pre.json.

Ground truth consulted: repos/_build/ledger-92e8bdd3 transient-crypto/src/proofs.rs:700-730 (`ProofPreimage`), onchain-runtime-wasm/src/primitives.rs:227-274 (`proof_data_into_serialized_preimage`), ledger/src/construct.rs:520-570 and 1000-1075, ledger/src/prove.rs:340-430; the compact-runtime 0.19.100 package under /nix/store (circuit-context.d.ts:198, proof-data.d.ts, the onchain-runtime-v4 .d.ts:610-616, package.json); K's domains.md:1235-1264 and 1367-1376 (`modInt`, INT-SYMBOLIC).

Freshness: `zkir-symbolic-kompiled/definition.kore` (03:08:45) is newer than every `.k` it imports (latest zkir-vm.k 03:02:38); the claims (03:07) and the c receipt (03:12) follow it. `kprove --version` is v7.1.337.

Commands run (all output under the scratchpad):

- `uv run --group zkir-k python experiments/zkir-k/tools/run_claims.py --only add --log-dir <scratch>/logs` → `add proved 7.3`, exit 0; `--only nonexistent` → `0/0 claims proved`, exit 0. The regenerated claims/spec-compiled-observable.k is byte-identical to the version before the run (diff against a copy).
- Scratch claims, each `kprove <file> --definition experiments/zkir-k/semantics/zkir-symbolic-kompiled --spec-module <M>`: `false-add.k` (wrong output) exit 1, 8 s, no `#Top`; `vacuous-add.k` (wrong output, unsatisfiable requires) exit 0, `#Top`, 6 s; `ctb-both.k` (constrain_to_boolean-ok under `C ==Int 0 orBool C ==Int 1`) exit 1, residual on `asBool`/`#boolean`; `comm-sym-obs.k` (commitment, symbolic input, observable only) exit 0, `#Top`, 7 s; `comm-sym.k` (same with verdicts pinned) exit 1, residual on `#commOutcome(..., transientCommit(ListItem(A), 5), ...)`; `hash-inj.k` exit 1, residual `poseidonHash(ListItem(A)) #Equals poseidonHash(ListItem(B))`.
- A Python session through `zkir_run.Runner` reproducing the `wrong-phase` construction on swap-fundAlice: generation run `error Failed direct assertion` with 9 regenerated public inputs (`['48','80','1','1','9','12','1','1','1']` against the honest prefix ending in `0`), checked run `error Failed direct assertion`; without the regeneration `Public transcript input mismatch for input 8`.
- A script counting the `op`s and `do_communications_commitment` of the seven `.zkir` programs (all seven `true`; 49 to 225 instructions).
- `git diff --stat 92e8bdd3 origin/ledger-9 -- zkir-v3/ transient-crypto/ onchain-runtime/` and `git log --oneline 92e8bdd3..origin/ledger-9 | wc -l` in repos/midnightntwrk/midnight-ledger (read-only): 43 commits, only transient-crypto/src/proofs.rs changed in those directories.
- `make -s -f <scratch>/Makefile.test x` with recipe `false | tee /dev/null` → exit 0.

Nothing outside this report was modified; the scratch claims and logs are under /tmp/claude-1000/-home-charl/7cf55fca-6b70-4fce-8aa0-2e793cb52ef2/scratchpad/{claims,logs}.
